use std::fs;
use std::path::PathBuf;

use chrono::Utc;

use crate::models::*;
use crate::parser::redact_secrets;

pub fn export_bundle(
    index: &WorkspaceIndex,
    options: &ExportOptions,
) -> Result<ExportResult, String> {
    let export_dir = PathBuf::from(&index.workspace_root).join("export");
    fs::create_dir_all(&export_dir).map_err(|e| e.to_string())?;

    let mut files_written = Vec::new();
    let sections: Vec<&str> = options.sections.iter().map(|s| s.as_str()).collect();
    let export_all = sections.is_empty() || sections.contains(&"all");

    let readme = build_readme(index, options);
    let readme_path = export_dir.join("README.md");
    fs::write(&readme_path, &readme).map_err(|e| e.to_string())?;
    files_written.push(readme_path.to_string_lossy().to_string());

    if export_all || sections.contains(&"rules") {
        let dir = export_dir.join("rules");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for rule in &index.rules {
            let content = format_rule_md(rule, options.basic_mode);
            let path = dir.join(format!("{}.md", rule.name));
            fs::write(&path, content).map_err(|e| e.to_string())?;
            files_written.push(path.to_string_lossy().to_string());
        }
    }

    if export_all || sections.contains(&"skills") {
        let dir = export_dir.join("skills");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for skill in &index.skills {
            let content = format_skill_md(skill, options.basic_mode);
            let slug = skill.name.replace(['/', '\\'], "-");
            let path = dir.join(format!("{slug}.md"));
            fs::write(&path, content).map_err(|e| e.to_string())?;
            files_written.push(path.to_string_lossy().to_string());
        }
    }

    if export_all || sections.contains(&"agents") {
        let dir = export_dir.join("agents");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for agent in &index.agents {
            let content = format_agent_md(agent, options.basic_mode);
            let slug = agent
                .relative_path
                .rsplit('/')
                .next()
                .unwrap_or("agent.md")
                .replace(".md", ".export.md");
            let path = dir.join(slug);
            fs::write(&path, content).map_err(|e| e.to_string())?;
            files_written.push(path.to_string_lossy().to_string());
        }
    }

    if export_all || sections.contains(&"commands") {
        let dir = export_dir.join("commands");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for cmd in &index.commands {
            let content = if options.basic_mode {
                format!(
                    "# /{}\n\n{}\n\n> Vollständige Befehlsdatei: `{}`\n",
                    cmd.name, cmd.summary, cmd.relative_path
                )
            } else {
                format!("# /{}\n\n{}", cmd.name, cmd.body)
            };
            let path = dir.join(format!("{}.md", cmd.name));
            fs::write(&path, content).map_err(|e| e.to_string())?;
            files_written.push(path.to_string_lossy().to_string());
        }
    }

    if export_all || sections.contains(&"configs") {
        let dir = export_dir.join("configs");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        write_json_export(&dir, "hooks.json", &index.configs.hooks_json, options, &mut files_written)?;
        write_json_export(&dir, "permissions.json", &index.configs.permissions_json, options, &mut files_written)?;
        write_json_export(&dir, "mcp.json", &index.configs.mcp_json, options, &mut files_written)?;
        if !options.basic_mode {
            write_json_export(&dir, "skills-lock.json", &index.configs.skills_lock_json, options, &mut files_written)?;
        }

        if let Some(ref gitignore) = index.configs.gitignore_content {
            let content = if options.basic_mode {
                "# .cursor/.gitignore (Zusammenfassung)\n\nStandardmäßig ausgeschlossen: projects/, extensions/, Runtime-Artefakte.\n".to_string()
            } else {
                gitignore.clone()
            };
            let path = dir.join("gitignore.md");
            fs::write(&path, content).map_err(|e| e.to_string())?;
            files_written.push(path.to_string_lossy().to_string());
        }

        for script in &index.configs.hook_scripts {
            let body = if options.basic_mode {
                format!(
                    "# {}\n\nHook-Skript unter `{}` ({} Bytes). Öffne im Cursor-Explorer für den vollständigen Quelltext.\n",
                    script.name,
                    script.relative_path,
                    script.body.len()
                )
            } else {
                format!("# {}\n\n```sh\n{}\n```\n", script.name, script.body)
            };
            let path = dir.join(format!("hook-{}.md", script.name));
            fs::write(&path, body).map_err(|e| e.to_string())?;
            files_written.push(path.to_string_lossy().to_string());
        }
    }

    if export_all || sections.contains(&"relationships") {
        let content = build_relationships_md(index);
        let path = export_dir.join("relationships.md");
        fs::write(&path, &content).map_err(|e| e.to_string())?;
        files_written.push(path.to_string_lossy().to_string());
    }

    let mut file_count = files_written.len();

    if options.include_pdf_html {
        let html = build_print_html(index, options);
        let path = export_dir.join("cursor-explorer-bundle.html");
        fs::write(&path, &html).map_err(|e| e.to_string())?;
        files_written.push(path.to_string_lossy().to_string());
        file_count = files_written.len();
    }

    Ok(ExportResult {
        export_dir: export_dir.to_string_lossy().to_string(),
        files_written: files_written.clone(),
        message: format!(
            "{} Dateien nach {} exportiert. Öffne cursor-explorer-bundle.html und nutze Drucken → Als PDF speichern für eine PDF-Kopie.",
            file_count,
            export_dir.display()
        ),
    })
}

fn write_json_export(
    dir: &PathBuf,
    name: &str,
    value: &Option<serde_json::Value>,
    options: &ExportOptions,
    files_written: &mut Vec<String>,
) -> Result<(), String> {
    if let Some(json) = value {
        let text = serde_json::to_string_pretty(json).map_err(|e| e.to_string())?;
        let content = if options.basic_mode {
            redact_secrets(&text)
        } else {
            text
        };
        let path = dir.join(name);
        fs::write(&path, content).map_err(|e| e.to_string())?;
        files_written.push(path.to_string_lossy().to_string());
    }
    Ok(())
}

fn build_readme(index: &WorkspaceIndex, options: &ExportOptions) -> String {
    format!(
        r#"# HorosCode .cursor Export

Erstellt von **Cursor-Explorer** (HorosCode) am {date}.

- Workspace: `{workspace}`
- Modus: {mode}
- Regeln: {rules} | Skills: {skills} | Agenten: {agents} | Befehle: {commands}

## Inhalt

Dieses Bundle dokumentiert die `.cursor`-Konfigurationsebene: Regeln, Skills, Agenten, Slash-Befehle, Hooks und MCP-Tooling.

> HorosCloud ist ein HorosCode-Produkt. Dieser Export beschreibt Cursor-IDE-Anpassungen, keinen Anwendungs-Runtime-Code.

## PDF

Öffne `cursor-explorer-bundle.html` im Browser und nutze **Drucken → Als PDF speichern**, wenn du ein PDF-Archiv brauchst.
"#,
        date = Utc::now().format("%Y-%m-%d %H:%M UTC"),
        workspace = index.workspace_root,
        mode = if options.basic_mode { "Einfach (geschwärzt)" } else { "Erweitert (vollständig)" },
        rules = index.rules.len(),
        skills = index.skills.len(),
        agents = index.agents.len(),
        commands = index.commands.len(),
    )
}

fn format_rule_md(rule: &RuleEntry, basic: bool) -> String {
    if basic {
        format!(
            r#"# Regel: {name}

**Beschreibung:** {desc}
**Immer anwenden:** {always}
**Pfad:** `{path}`

## Zusammenfassung

{summary}
"#,
            name = rule.name,
            desc = rule.description.as_deref().unwrap_or("—"),
            always = rule.always_apply,
            path = rule.relative_path,
            summary = rule.summary,
        )
    } else {
        format!(
            r#"# Regel: {name}

**Beschreibung:** {desc}
**Globs:** {globs}
**Immer anwenden:** {always}
**Pfad:** `{path}`

---

{body}
"#,
            name = rule.name,
            desc = rule.description.as_deref().unwrap_or("—"),
            globs = rule.globs.as_deref().unwrap_or("—"),
            always = rule.always_apply,
            path = rule.relative_path,
            body = rule.body,
        )
    }
}

fn format_skill_md(skill: &SkillEntry, basic: bool) -> String {
    if basic {
        format!(
            r#"# Skill: {name}

**Quelle:** {source:?}
**Pfad:** `{path}`
**Version:** {version}

## Zusammenfassung

{summary}
"#,
            name = skill.name,
            source = skill.source,
            path = skill.relative_path,
            version = skill.version.as_deref().unwrap_or("—"),
            summary = skill.summary,
        )
    } else {
        format!(
            r#"# Skill: {name}

**Quelle:** {source:?}
**Lock-Quelle:** {lock}
**Pfad:** `{path}`

---

{body}
"#,
            name = skill.name,
            source = skill.source,
            lock = skill.lock_source.as_deref().unwrap_or("—"),
            path = skill.relative_path,
            body = skill.body,
        )
    }
}

fn format_agent_md(agent: &AgentEntry, basic: bool) -> String {
    if basic {
        format!(
            r#"# Agent: {name}

**Rolle:** {role}
**Modell:** {model}
**Pfad:** `{path}`

## Zusammenfassung

{summary}
"#,
            name = agent.name,
            role = agent.role.as_deref().unwrap_or("—"),
            model = agent.model.as_deref().unwrap_or("—"),
            path = agent.relative_path,
            summary = agent.summary,
        )
    } else {
        format!(
            r#"# Agent: {name}

**Beschreibung:** {desc}
**Rolle:** {role}
**Modell:** {model}

---

{body}
"#,
            name = agent.name,
            desc = agent.description.as_deref().unwrap_or("—"),
            role = agent.role.as_deref().unwrap_or("—"),
            model = agent.model.as_deref().unwrap_or("—"),
            body = agent.body,
        )
    }
}

fn build_relationships_md(index: &WorkspaceIndex) -> String {
    let mut md = String::from("# Beziehungskarte\n\n");
    for rel in &index.relationships {
        md.push_str(&format!(
            "- `{}` ({:?}) **{}** → `{}` ({:?})\n",
            rel.from_id, rel.from_type, rel.label, rel.to_id, rel.to_type
        ));
    }
    md
}

fn build_print_html(index: &WorkspaceIndex, options: &ExportOptions) -> String {
    let mut body = String::new();
    body.push_str("<h1>HorosCode .cursor Bundle</h1>");
    body.push_str(&format!("<p>Workspace: {}</p>", index.workspace_root));

    body.push_str("<h2>Regeln</h2>");
    for r in &index.rules {
        body.push_str(&format!("<h3>{}</h3><p>{}</p>", r.name, html_escape(&r.summary)));
        if !options.basic_mode {
            body.push_str(&format!("<pre>{}</pre>", html_escape(&r.body)));
        }
    }

    body.push_str("<h2>Skills</h2>");
    for s in &index.skills {
        body.push_str(&format!("<h3>{}</h3><p>{}</p>", s.name, html_escape(&s.summary)));
    }

    body.push_str("<h2>Agenten</h2>");
    for a in &index.agents {
        body.push_str(&format!("<h3>{}</h3><p>{}</p>", a.name, html_escape(&a.summary)));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="utf-8"/>
<title>HorosCode .cursor Export</title>
<style>
body {{ font-family: Segoe UI, system-ui, sans-serif; margin: 2rem; color: #111; }}
h1,h2,h3 {{ color: #0d3b66; }}
pre {{ background: #f4f4f5; padding: 1rem; overflow-x: auto; font-size: 12px; }}
@media print {{ body {{ margin: 1cm; }} }}
</style>
</head>
<body>
{body}
</body>
</html>"#
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
