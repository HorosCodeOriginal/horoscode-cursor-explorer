use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Rule,
    Skill,
    Agent,
    Command,
    Config,
    Folder,
    Hook,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ItemSource {
    HorosCodeLocal,
    CursorBuiltin,
    Upstream,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
    pub excluded: bool,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub description: Option<String>,
    pub globs: Option<String>,
    pub always_apply: bool,
    pub summary: String,
    pub body: String,
    pub source: ItemSource,
    pub references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub source_urls: Vec<String>,
    pub lock_source: Option<String>,
    pub lock_source_type: Option<String>,
    pub summary: String,
    pub body: String,
    pub source: ItemSource,
    pub references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub description: Option<String>,
    pub model: Option<String>,
    pub summary: String,
    pub body: String,
    pub source: ItemSource,
    pub references: Vec<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub summary: String,
    pub body: String,
    pub source: ItemSource,
    pub references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookEntry {
    pub event: String,
    pub command: String,
    pub fail_closed: Option<bool>,
    pub script_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigBundle {
    pub hooks_json: Option<serde_json::Value>,
    pub permissions_json: Option<serde_json::Value>,
    pub mcp_json: Option<serde_json::Value>,
    pub skills_lock_json: Option<serde_json::Value>,
    pub gitignore_content: Option<String>,
    pub hook_scripts: Vec<HookScriptEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookScriptEntry {
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub from_id: String,
    pub from_type: ItemType,
    pub to_id: String,
    pub to_type: ItemType,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceIndex {
    pub workspace_root: String,
    pub cursor_root: String,
    pub scanned_at: String,
    pub tree: TreeNode,
    pub rules: Vec<RuleEntry>,
    pub skills: Vec<SkillEntry>,
    pub agents: Vec<AgentEntry>,
    pub commands: Vec<CommandEntry>,
    pub configs: ConfigBundle,
    pub relationships: Vec<Relationship>,
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub item_type: ItemType,
    pub source: ItemSource,
    pub title: String,
    pub snippet: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportOptions {
    pub sections: Vec<String>,
    pub basic_mode: bool,
    pub include_pdf_html: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub export_dir: String,
    pub files_written: Vec<String>,
    pub message: String,
}
