mod export;
mod models;
mod parser;
mod scanner;

use models::{ExportOptions, ExportResult, ItemSource, ItemType, SearchResult, WorkspaceIndex};
use scanner::{scan_workspace, search_index};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
fn get_default_workspace() -> Result<String, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    if let Some(parent) = exe.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
        let mut candidate = parent.to_path_buf();
        for _ in 0..5 {
            if candidate.join(".cursor").is_dir() {
                return Ok(candidate.to_string_lossy().to_string());
            }
            if let Some(p) = candidate.parent() {
                candidate = p.to_path_buf();
            } else {
                break;
            }
        }
    }

    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(workspace) = manifest_dir.parent().and_then(|p| p.parent()) {
        if workspace.join(".cursor").is_dir() {
            return Ok(workspace.to_string_lossy().to_string());
        }
    }

    Err("Workspace-Root konnte nicht erkannt werden. Nutze „Workspace wechseln“, um einen Ordner auszuwählen.".to_string())
}

#[tauri::command]
fn scan_workspace_cmd(workspace_root: String) -> Result<WorkspaceIndex, String> {
    scan_workspace(&workspace_root)
}

#[tauri::command]
fn search_workspace(
    index: WorkspaceIndex,
    query: String,
    types: Vec<String>,
    sources: Vec<String>,
) -> Vec<SearchResult> {
    let parsed_types: Vec<ItemType> = types
        .iter()
        .filter_map(|t| match t.as_str() {
            "rule" => Some(ItemType::Rule),
            "skill" => Some(ItemType::Skill),
            "agent" => Some(ItemType::Agent),
            "command" => Some(ItemType::Command),
            "config" => Some(ItemType::Config),
            "folder" => Some(ItemType::Folder),
            "hook" => Some(ItemType::Hook),
            _ => None,
        })
        .collect();

    let parsed_sources: Vec<ItemSource> = sources
        .iter()
        .filter_map(|s| match s.as_str() {
            "horosCodeLocal" => Some(ItemSource::HorosCodeLocal),
            "cursorBuiltin" => Some(ItemSource::CursorBuiltin),
            "upstream" => Some(ItemSource::Upstream),
            _ => None,
        })
        .collect();

    search_index(&index, &query, &parsed_types, &parsed_sources)
}

#[tauri::command]
fn export_workspace(
    index: WorkspaceIndex,
    options: ExportOptions,
) -> Result<ExportResult, String> {
    export::export_bundle(&index, &options)
}

#[tauri::command]
fn open_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_default_workspace,
            scan_workspace_cmd,
            search_workspace,
            export_workspace,
            open_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::scanner::scan_workspace;

    #[test]
    fn scans_workspace_cursor_folder() {
        let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root")
            .to_string_lossy()
            .to_string();

        let index = scan_workspace(&workspace).expect("scan should succeed");
        assert!(index.rules.len() >= 5, "expected rules");
        assert!(index.skills.len() >= 10, "expected skills");
        assert!(index.agents.len() >= 5, "expected agents");
        assert!(index.commands.len() >= 5, "expected commands");
        assert!(!index.relationships.is_empty(), "expected relationships");
    }
}
