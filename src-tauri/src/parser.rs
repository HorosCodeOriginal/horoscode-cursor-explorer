use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    pub fields: HashMap<String, String>,
    pub body: String,
}

pub fn parse_frontmatter(content: &str) -> Frontmatter {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Frontmatter {
            fields: HashMap::new(),
            body: content.to_string(),
        };
    }

    let rest = &trimmed[3..];
    if let Some(end) = rest.find("\n---") {
        let yaml_block = &rest[..end];
        let body = rest[end + 4..].trim_start_matches('\n').to_string();
        let fields = parse_simple_yaml(yaml_block);
        return Frontmatter { fields, body };
    }

    Frontmatter {
        fields: HashMap::new(),
        body: content.to_string(),
    }
}

fn parse_simple_yaml(yaml: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut list_values: Vec<String> = Vec::new();

    for line in yaml.lines() {
        let line = line.trim_end();
        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("- ") && current_key.is_some() {
            list_values.push(line[2..].trim().to_string());
            continue;
        }

        if let Some(key) = current_key.take() {
            if !list_values.is_empty() {
                fields.insert(key, list_values.join(", "));
                list_values.clear();
            }
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_string();
            let value = value.trim();
            if value.is_empty() {
                current_key = Some(key);
                list_values.clear();
            } else {
                fields.insert(key, value.trim_matches('"').to_string());
            }
        }
    }

    if let Some(key) = current_key {
        if !list_values.is_empty() {
            fields.insert(key, list_values.join(", "));
        }
    }

    fields
}

pub fn summarize_markdown(body: &str, max_chars: usize) -> String {
    let mut summary = String::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('|') {
            continue;
        }
        if trimmed.starts_with("```") {
            break;
        }
        summary.push_str(trimmed);
        summary.push(' ');
        if summary.chars().count() >= max_chars {
            break;
        }
    }
    let summary = summary.trim();
    if summary.chars().count() > max_chars {
        let truncated: String = summary.chars().take(max_chars).collect();
        format!("{}.", truncated)
    } else {
        summary.to_string()
    }
}

pub fn extract_references(content: &str) -> Vec<String> {
    let mut refs = Vec::new();

    let patterns = [
        r"Task\(([a-zA-Z0-9_-]+)\)",
        r"@([a-zA-Z0-9_-]+)",
        r"\.cursor/rules/([a-zA-Z0-9_.-]+\.mdc)",
        r"\.cursor/skills(?:-cursor)?/([a-zA-Z0-9_.-]+)/SKILL\.md",
        r"skills(?:-cursor)?/([a-zA-Z0-9_.-]+)/SKILL\.md",
        r"\.cursor/agents/([a-zA-Z0-9_.-]+\.md)",
        r"agents/([a-zA-Z0-9_.-]+\.md)",
        r"rules/([a-zA-Z0-9_.-]+\.mdc)",
        r"commands/([a-zA-Z0-9_.-]+\.md)",
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(content) {
                if let Some(m) = cap.get(1) {
                    let value = m.as_str().to_string();
                    if !refs.contains(&value) {
                        refs.push(value);
                    }
                }
            }
        }
    }

    refs
}

pub fn slugify(name: &str) -> String {
    name.to_lowercase()
        .replace([' ', '_'], "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

pub fn redact_secrets(content: &str) -> String {
    let secret_keys = [
        "API_KEY",
        "SECRET",
        "TOKEN",
        "PASSWORD",
        "PRIVATE_KEY",
        "ACCESS_KEY",
        "AUTH",
    ];

    let mut result = content.to_string();
    for key in secret_keys {
        if let Ok(re) = Regex::new(&format!(r"(?i)({key}\s*[=:]\s*)(\S+)")) {
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    format!("{}{}", &caps[1], "[REDACTED]")
                })
                .to_string();
        }
        if let Ok(re) = Regex::new(&format!(r#"(?i)"{key}"\s*:\s*"[^"]*""#)) {
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    caps[0].split_once(':').map_or_else(
                        || caps[0].to_string(),
                        |(k, _)| format!("{k}: \"[REDACTED]\""),
                    )
                })
                .to_string();
        }
    }
    result
}
