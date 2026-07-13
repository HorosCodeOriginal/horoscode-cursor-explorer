use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use walkdir::WalkDir;

use crate::models::*;
use crate::parser::{extract_references, parse_frontmatter, slugify, summarize_markdown};

const DEFAULT_EXCLUDES: &[&str] = &[
    "projects",
    "extensions",
    "ai-tracking",
    "ide_state.json",
    "argv.json",
    "node_modules",
    "plugins/local",
    "__pycache__",
];

pub fn scan_workspace(workspace_root: &str) -> Result<WorkspaceIndex, String> {
    let workspace = PathBuf::from(workspace_root);
    if !workspace.is_dir() {
        return Err(format!("Workspace nicht gefunden: {workspace_root}"));
    }

    let cursor_root = workspace.join(".cursor");
    if !cursor_root.is_dir() {
        return Err(format!(
            "Kein .cursor-Ordner im Workspace: {}",
            workspace.display()
        ));
    }

    let mut exclude_patterns = load_exclude_patterns(&cursor_root);
    for pattern in DEFAULT_EXCLUDES {
        if !exclude_patterns.iter().any(|p| p == pattern) {
            exclude_patterns.push(pattern.to_string());
        }
    }

    let tree = build_tree(&cursor_root, &cursor_root, &exclude_patterns);

    let rules = scan_rules(&cursor_root, &exclude_patterns)?;
    let skills = scan_skills(&cursor_root, &exclude_patterns)?;
    let agents = scan_agents(&cursor_root, &exclude_patterns)?;
    let commands = scan_commands(&cursor_root, &exclude_patterns)?;
    let configs = scan_configs(&cursor_root, &exclude_patterns)?;

    let mut relationships = build_relationships(&rules, &skills, &agents, &commands);

    // Cross-link orchestrator and agent-context-modes heavily
    add_mode_rule_links(&mut relationships, &rules, &skills, &agents);

    Ok(WorkspaceIndex {
        workspace_root: workspace.to_string_lossy().to_string(),
        cursor_root: cursor_root.to_string_lossy().to_string(),
        scanned_at: Utc::now().to_rfc3339(),
        tree,
        rules,
        skills,
        agents,
        commands,
        configs,
        relationships,
        exclude_patterns,
    })
}

fn load_exclude_patterns(cursor_root: &Path) -> Vec<String> {
    let gitignore = cursor_root.join(".gitignore");
    let mut patterns = Vec::new();

    if let Ok(content) = fs::read_to_string(&gitignore) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("!") {
                continue;
            }
            let pattern = line.trim_end_matches('/').to_string();
            if !pattern.is_empty() && pattern != "*" {
                patterns.push(pattern);
            }
        }
    }

    patterns
}

fn is_excluded(path: &Path, cursor_root: &Path, exclude_patterns: &[String]) -> bool {
    let rel = path
        .strip_prefix(cursor_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    if rel.is_empty() {
        return false;
    }

    for pattern in exclude_patterns {
        let pat = pattern.trim_end_matches('/');
        if rel == pat || rel.starts_with(&format!("{pat}/")) {
            return true;
        }
        if pat.contains('*') {
            let parts: Vec<&str> = pat.split('*').collect();
            if parts.len() == 2 && rel.contains(parts[0]) && rel.contains(parts[1]) {
                return true;
            }
        }
        if rel.ends_with(pat) {
            return true;
        }
    }

    false
}

fn build_tree(cursor_root: &Path, current: &Path, exclude_patterns: &[String]) -> TreeNode {
    let rel = current
        .strip_prefix(cursor_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let _rel_display = if rel.is_empty() {
        ".cursor".to_string()
    } else {
        format!(".cursor/{rel}")
    };

    let name = current
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| ".cursor".to_string());

    let excluded = is_excluded(current, cursor_root, exclude_patterns);

    let mut children = Vec::new();
    if current.is_dir() && !excluded {
        if let Ok(entries) = fs::read_dir(current) {
            let mut dirs: Vec<PathBuf> = Vec::new();
            let mut files: Vec<PathBuf> = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if is_excluded(&path, cursor_root, exclude_patterns) {
                    continue;
                }
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    files.push(path);
                }
            }
            dirs.sort();
            files.sort();
            for p in dirs {
                children.push(build_tree(cursor_root, &p, exclude_patterns));
            }
            for p in files {
                children.push(build_tree(cursor_root, &p, exclude_patterns));
            }
        }
    }

    let note = if excluded {
        Some("Laut .gitignore-Richtlinie von Scan/Export ausgeschlossen".to_string())
    } else if rel == "projects" || rel.starts_with("projects/") {
        Some("Runtime-Projektdaten — standardmäßig nie exportiert".to_string())
    } else {
        None
    };

    TreeNode {
        name,
        path: current.to_string_lossy().to_string(),
        is_dir: current.is_dir(),
        children,
        excluded,
        note,
    }
}

fn scan_rules(cursor_root: &Path, exclude_patterns: &[String]) -> Result<Vec<RuleEntry>, String> {
    let rules_dir = cursor_root.join("rules");
    let mut rules = Vec::new();

    if !rules_dir.is_dir() {
        return Ok(rules);
    }

    for entry in WalkDir::new(&rules_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "mdc"))
    {
        let path = entry.path();
        if is_excluded(path, cursor_root, exclude_patterns) {
            continue;
        }

        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let fm = parse_frontmatter(&content);
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let relative_path = path
            .strip_prefix(cursor_root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
            .replace('\\', "/");

        let always_apply = fm
            .fields
            .get("alwaysApply")
            .is_some_and(|v| v == "true");

        let references = extract_references(&content);
        let summary = summarize_markdown(&fm.body, 240);

        rules.push(RuleEntry {
            id: format!("rule:{name}"),
            name: name.clone(),
            path: path.to_string_lossy().to_string(),
            relative_path,
            description: fm.fields.get("description").cloned(),
            globs: fm.fields.get("globs").cloned(),
            always_apply,
            summary,
            body: fm.body,
            source: ItemSource::HorosCodeLocal,
            references,
        });
    }

    rules.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(rules)
}

fn scan_skills(cursor_root: &Path, exclude_patterns: &[String]) -> Result<Vec<SkillEntry>, String> {
    let lock_map = load_skills_lock(cursor_root);
    let mut skills = Vec::new();
    let skill_roots = [cursor_root.join("skills"), cursor_root.join("skills-cursor")];

    for root in skill_roots {
        if !root.is_dir() {
            continue;
        }
        let source = if root.ends_with("skills-cursor") {
            ItemSource::CursorBuiltin
        } else {
            ItemSource::HorosCodeLocal
        };

        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "SKILL.md")
        {
            let path = entry.path();
            if is_excluded(path, cursor_root, exclude_patterns) {
                continue;
            }

            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let fm = parse_frontmatter(&content);
            let folder_name = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();

            let skill_name = fm
                .fields
                .get("name")
                .cloned()
                .unwrap_or(folder_name.clone());

            let relative_path = path
                .strip_prefix(cursor_root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default()
                .replace('\\', "/");

            let lock_key = folder_name.clone();
            let (lock_source, lock_source_type, resolved_source) =
                if let Some(lock) = lock_map.get(&lock_key) {
                    let src = if lock.source_type == "local" {
                        ItemSource::HorosCodeLocal
                    } else {
                        ItemSource::Upstream
                    };
                    (
                        Some(lock.source.clone()),
                        Some(lock.source_type.clone()),
                        if source == ItemSource::CursorBuiltin {
                            ItemSource::CursorBuiltin
                        } else {
                            src
                        },
                    )
                } else if source == ItemSource::CursorBuiltin {
                    (None, None, ItemSource::CursorBuiltin)
                } else {
                    (None, None, ItemSource::HorosCodeLocal)
                };

            let source_urls: Vec<String> = fm
                .fields
                .get("source_urls")
                .map(|s| s.split(',').map(|v| v.trim().to_string()).collect())
                .unwrap_or_default();

            let references = extract_references(&content);
            let summary = summarize_markdown(&fm.body, 240);

            skills.push(SkillEntry {
                id: format!("skill:{lock_key}"),
                name: skill_name,
                path: path.to_string_lossy().to_string(),
                relative_path,
                description: fm.fields.get("description").cloned(),
                version: fm.fields.get("version").cloned(),
                source_urls,
                lock_source,
                lock_source_type,
                summary,
                body: fm.body,
                source: resolved_source,
                references,
            });
        }
    }

    skills.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(skills)
}

struct LockEntry {
    source: String,
    source_type: String,
}

fn load_skills_lock(cursor_root: &Path) -> HashMap<String, LockEntry> {
    let path = cursor_root.join("skills-lock.json");
    let mut map = HashMap::new();

    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(skills) = json.get("skills").and_then(|s| s.as_object()) {
                for (name, entry) in skills {
                    let source = entry
                        .get("source")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let source_type = entry
                        .get("sourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    map.insert(name.clone(), LockEntry { source, source_type });
                }
            }
        }
    }

    map
}

fn scan_agents(cursor_root: &Path, exclude_patterns: &[String]) -> Result<Vec<AgentEntry>, String> {
    let agents_dir = cursor_root.join("agents");
    let mut agents = Vec::new();

    if !agents_dir.is_dir() {
        return Ok(agents);
    }

    for entry in WalkDir::new(&agents_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        if is_excluded(path, cursor_root, exclude_patterns) {
            continue;
        }

        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let fm = parse_frontmatter(&content);
        let file_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let relative_path = path
            .strip_prefix(cursor_root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
            .replace('\\', "/");

        let name = fm
            .fields
            .get("name")
            .cloned()
            .unwrap_or(file_name.clone());

        let role = infer_agent_role(&file_name, &fm.body);
        let references = extract_references(&content);
        let summary = summarize_markdown(&fm.body, 240);

        agents.push(AgentEntry {
            id: format!("agent:{file_name}"),
            name,
            path: path.to_string_lossy().to_string(),
            relative_path,
            description: fm.fields.get("description").cloned(),
            model: fm.fields.get("model").cloned(),
            summary,
            body: fm.body,
            source: ItemSource::HorosCodeLocal,
            references,
            role,
        });
    }

    agents.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(agents)
}

fn infer_agent_role(file_name: &str, body: &str) -> Option<String> {
    let lower = body.to_lowercase();
    if lower.contains("coordinator") || lower.contains("can spawn") {
        return Some("Koordinator".to_string());
    }
    if lower.contains("worker") || lower.contains("leaf") {
        return Some("Worker".to_string());
    }
    match file_name {
        "aang" | "sokka" | "katara" | "appa" | "suki" => Some("Koordinator".to_string()),
        "toph" | "momo" | "iroh" | "zuko" | "horos-ui" => Some("Worker".to_string()),
        _ => Some("Agent".to_string()),
    }
}

fn scan_commands(
    cursor_root: &Path,
    exclude_patterns: &[String],
) -> Result<Vec<CommandEntry>, String> {
    let commands_dir = cursor_root.join("commands");
    let mut commands = Vec::new();

    if !commands_dir.is_dir() {
        return Ok(commands);
    }

    for entry in fs::read_dir(&commands_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.extension().is_some_and(|ext| ext == "md") {
            continue;
        }
        if is_excluded(&path, cursor_root, exclude_patterns) {
            continue;
        }

        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let relative_path = path
            .strip_prefix(cursor_root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
            .replace('\\', "/");

        let references = extract_references(&content);
        let summary = summarize_markdown(&content, 240);

        commands.push(CommandEntry {
            id: format!("command:{name}"),
            name: name.clone(),
            path: path.to_string_lossy().to_string(),
            relative_path,
            summary,
            body: content,
            source: ItemSource::HorosCodeLocal,
            references,
        });
    }

    commands.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(commands)
}

fn scan_configs(cursor_root: &Path, exclude_patterns: &[String]) -> Result<ConfigBundle, String> {
    let read_json = |name: &str| -> Option<serde_json::Value> {
        let path = cursor_root.join(name);
        fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
    };

    let gitignore_content = fs::read_to_string(cursor_root.join(".gitignore")).ok();

    let mut hook_scripts = Vec::new();
    let hooks_dir = cursor_root.join("hooks");
    if hooks_dir.is_dir() {
        for entry in fs::read_dir(&hooks_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() && !is_excluded(&path, cursor_root, exclude_patterns) {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                let body = fs::read_to_string(&path).unwrap_or_default();
                let relative_path = path
                    .strip_prefix(cursor_root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
                    .replace('\\', "/");
                hook_scripts.push(HookScriptEntry {
                    name,
                    path: path.to_string_lossy().to_string(),
                    relative_path,
                    body,
                });
            }
        }
    }

    let hooks_json = read_json("hooks.json");
    let mut hook_entries = Vec::new();
    if let Some(ref hooks) = hooks_json {
        if let Some(hook_map) = hooks.get("hooks").and_then(|h| h.as_object()) {
            for (event, arr) in hook_map {
                if let Some(items) = arr.as_array() {
                    for item in items {
                        let command = item
                            .get("command")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let fail_closed = item
                            .get("failClosed")
                            .and_then(|v| v.as_bool());
                        let script_path = extract_hook_script_path(&command);
                        hook_entries.push(HookEntry {
                            event: event.clone(),
                            command,
                            fail_closed,
                            script_path,
                        });
                    }
                }
            }
        }
    }

    Ok(ConfigBundle {
        hooks_json,
        permissions_json: read_json("permissions.json"),
        mcp_json: read_json("mcp.json"),
        skills_lock_json: read_json("skills-lock.json"),
        gitignore_content,
        hook_scripts,
    })
}

fn extract_hook_script_path(command: &str) -> Option<String> {
    if command.contains(".cursor/hooks/") {
        command
            .split(".cursor/hooks/")
            .nth(1)
            .map(|s| s.split_whitespace().next().unwrap_or(s).to_string())
    } else if command.contains("hooks/") {
        command
            .split("hooks/")
            .nth(1)
            .map(|s| s.split_whitespace().next().unwrap_or(s).to_string())
    } else {
        None
    }
}

fn build_relationships(
    rules: &[RuleEntry],
    skills: &[SkillEntry],
    agents: &[AgentEntry],
    commands: &[CommandEntry],
) -> Vec<Relationship> {
    let mut id_map: HashMap<String, (String, ItemType)> = HashMap::new();

    for r in rules {
        id_map.insert(r.name.clone(), (r.id.clone(), ItemType::Rule));
        id_map.insert(
            format!("{}.mdc", r.name),
            (r.id.clone(), ItemType::Rule),
        );
    }
    for s in skills {
        let folder = s
            .relative_path
            .split('/')
            .nth(1)
            .unwrap_or(&s.name)
            .to_string();
        id_map.insert(folder.clone(), (s.id.clone(), ItemType::Skill));
        id_map.insert(s.name.clone(), (s.id.clone(), ItemType::Skill));
        id_map.insert(slugify(&s.name), (s.id.clone(), ItemType::Skill));
    }
    for a in agents {
        let stem = a
            .relative_path
            .rsplit('/')
            .next()
            .unwrap_or(&a.name)
            .trim_end_matches(".md")
            .to_string();
        id_map.insert(stem.clone(), (a.id.clone(), ItemType::Agent));
        id_map.insert(a.name.clone(), (a.id.clone(), ItemType::Agent));
    }
    for c in commands {
        id_map.insert(c.name.clone(), (c.id.clone(), ItemType::Command));
    }

    let mut relationships = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let add_ref = |from_id: &str,
                   from_type: ItemType,
                   ref_key: &str,
                   relationships: &mut Vec<Relationship>,
                   seen: &mut HashSet<String>| {
        let normalized = ref_key
            .trim_end_matches(".md")
            .trim_end_matches(".mdc")
            .to_string();
        if let Some((to_id, to_type)) = id_map.get(&normalized).or_else(|| id_map.get(ref_key)) {
            let key = format!("{from_id}->{to_id}");
            if seen.insert(key) {
                relationships.push(Relationship {
                    from_id: from_id.to_string(),
                    from_type,
                    to_id: to_id.clone(),
                    to_type: to_type.clone(),
                    label: "verweist auf".to_string(),
                });
            }
        }
    };

    for r in rules {
        for ref_key in &r.references {
            add_ref(
                &r.id,
                ItemType::Rule,
                ref_key,
                &mut relationships,
                &mut seen,
            );
        }
    }
    for s in skills {
        for ref_key in &s.references {
            add_ref(
                &s.id,
                ItemType::Skill,
                ref_key,
                &mut relationships,
                &mut seen,
            );
        }
    }
    for a in agents {
        for ref_key in &a.references {
            add_ref(
                &a.id,
                ItemType::Agent,
                ref_key,
                &mut relationships,
                &mut seen,
            );
        }
    }
    for c in commands {
        for ref_key in &c.references {
            add_ref(
                &c.id,
                ItemType::Command,
                ref_key,
                &mut relationships,
                &mut seen,
            );
        }
    }

    relationships
}

fn add_mode_rule_links(
    relationships: &mut Vec<Relationship>,
    rules: &[RuleEntry],
    skills: &[SkillEntry],
    agents: &[AgentEntry],
) {
    let hub_rules = ["orchestrator", "agent-context-modes"];
    for hub in hub_rules {
        if let Some(rule) = rules.iter().find(|r| r.name == hub) {
            for agent in agents {
                if rule.body.to_lowercase().contains(&agent.name.to_lowercase())
                    || rule.body.contains(&format!("Task({})", agent.name.split_whitespace().next().unwrap_or(&agent.name)))
                {
                    let key = format!("{}->{}", rule.id, agent.id);
                    if !relationships.iter().any(|r| format!("{}->{}", r.from_id, r.to_id) == key)
                    {
                        relationships.push(Relationship {
                            from_id: rule.id.clone(),
                            from_type: ItemType::Rule,
                            to_id: agent.id.clone(),
                            to_type: ItemType::Agent,
                            label: "leitet weiter zu".to_string(),
                        });
                    }
                }
            }
            for skill in skills {
                if rule.body.contains(&skill.name)
                    || rule.body.contains(&format!("@{}", skill.name))
                    || rule
                        .body
                        .contains(skill.relative_path.trim_start_matches("skills/"))
                {
                    let key = format!("{}->{}", rule.id, skill.id);
                    if !relationships.iter().any(|r| format!("{}->{}", r.from_id, r.to_id) == key)
                    {
                        relationships.push(Relationship {
                            from_id: rule.id.clone(),
                            from_type: ItemType::Rule,
                            to_id: skill.id.clone(),
                            to_type: ItemType::Skill,
                            label: "aktiviert".to_string(),
                        });
                    }
                }
            }
        }
    }
}

pub fn search_index(index: &WorkspaceIndex, query: &str, types: &[ItemType], sources: &[ItemSource]) -> Vec<SearchResult> {
    let q = query.trim().to_lowercase();
    let mut results = Vec::new();

    let type_allowed = |t: &ItemType| types.is_empty() || types.contains(t);
    let source_allowed = |s: &ItemSource| sources.is_empty() || sources.contains(s);

    if q.is_empty() {
        return results;
    }

    for r in &index.rules {
        if !type_allowed(&ItemType::Rule) || !source_allowed(&r.source) {
            continue;
        }
        if matches_query(&q, &r.name, &r.summary, &r.body) {
            results.push(SearchResult {
                id: r.id.clone(),
                item_type: ItemType::Rule,
                source: r.source.clone(),
                title: r.name.clone(),
                snippet: r.summary.clone(),
                path: r.relative_path.clone(),
            });
        }
    }

    for s in &index.skills {
        if !type_allowed(&ItemType::Skill) || !source_allowed(&s.source) {
            continue;
        }
        if matches_query(&q, &s.name, &s.summary, &s.body) {
            results.push(SearchResult {
                id: s.id.clone(),
                item_type: ItemType::Skill,
                source: s.source.clone(),
                title: s.name.clone(),
                snippet: s.summary.clone(),
                path: s.relative_path.clone(),
            });
        }
    }

    for a in &index.agents {
        if !type_allowed(&ItemType::Agent) || !source_allowed(&a.source) {
            continue;
        }
        if matches_query(&q, &a.name, &a.summary, &a.body) {
            results.push(SearchResult {
                id: a.id.clone(),
                item_type: ItemType::Agent,
                source: a.source.clone(),
                title: a.name.clone(),
                snippet: a.summary.clone(),
                path: a.relative_path.clone(),
            });
        }
    }

    for c in &index.commands {
        if !type_allowed(&ItemType::Command) || !source_allowed(&c.source) {
            continue;
        }
        if matches_query(&q, &c.name, &c.summary, &c.body) {
            results.push(SearchResult {
                id: c.id.clone(),
                item_type: ItemType::Command,
                source: c.source.clone(),
                title: c.name.clone(),
                snippet: c.summary.clone(),
                path: c.relative_path.clone(),
            });
        }
    }

    results
}

fn matches_query(q: &str, title: &str, summary: &str, body: &str) -> bool {
    let title_l = title.to_lowercase();
    let summary_l = summary.to_lowercase();
    let body_l = body.to_lowercase();
    title_l.contains(q) || summary_l.contains(q) || body_l.contains(q)
}
