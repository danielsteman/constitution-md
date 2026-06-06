use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

/// Notice prepended to every generated file so humans know not to hand-edit it.
const NOTICE: &str = "AUTO-GENERATED FROM CONSTITUTION.md — DO NOT EDIT DIRECTLY";

/// Output format for a generated agent config file.
///
/// Each variant owns how the auto-generation [`NOTICE`] and the constitution
/// content are serialized, so adding a new format only requires a new variant
/// and a match arm in [`Format::render`].
#[derive(Debug, Clone, PartialEq)]
enum Format {
    /// Raw markdown: the notice is an HTML comment and content is emitted verbatim.
    Markdown,
    /// YAML: the notice is a `#` comment and content is stored under a `conventions` key.
    Yaml,
    /// JSON: the notice and content are stored as string fields in an object.
    Json,
}

impl Format {
    /// Render `content` into this format, including the auto-generation notice.
    fn render(&self, content: &str) -> String {
        match self {
            Format::Markdown => format!("<!-- {NOTICE} -->\n\n{content}"),
            Format::Yaml => format!("# {NOTICE}\n\n{}", wrap_yaml(content)),
            Format::Json => wrap_json(content),
        }
    }
}

/// Where and how a single agent's config file is generated.
struct AgentConfig {
    path: &'static str,
    format: Format,
}

fn agent_configs() -> HashMap<&'static str, AgentConfig> {
    let mut m = HashMap::new();
    m.insert(
        "claude",
        AgentConfig {
            path: "CLAUDE.md",
            format: Format::Markdown,
        },
    );
    m.insert(
        "cursor",
        AgentConfig {
            path: ".cursorrules",
            format: Format::Markdown,
        },
    );
    m.insert(
        "copilot",
        AgentConfig {
            path: ".github/copilot-instructions.md",
            format: Format::Markdown,
        },
    );
    m.insert(
        "windsurf",
        AgentConfig {
            path: ".windsurfrules",
            format: Format::Markdown,
        },
    );
    m.insert(
        "aider",
        AgentConfig {
            path: ".aider.conf.yml",
            format: Format::Yaml,
        },
    );
    m.insert(
        "continue",
        AgentConfig {
            path: ".continue/config.json",
            format: Format::Json,
        },
    );
    m
}

/// Escape `content` for embedding in a YAML double-quoted scalar.
fn wrap_yaml(content: &str) -> String {
    let escaped = content.replace('\\', "\\\\").replace('"', "\\\"");
    format!("conventions: \"{escaped}\"\n")
}

/// Escape `content` for a JSON string and wrap it in an object alongside the notice.
///
/// Backslashes are escaped first so the escapes introduced for quotes and
/// control characters are not themselves doubled.
fn wrap_json(content: &str) -> String {
    let escaped = content
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("{{\n  \"_generated\": \"{NOTICE}\",\n  \"conventions\": \"{escaped}\"\n}}\n")
}

fn sync_agent(repo_root: &Path, agent: &str, content: &str) -> Result<PathBuf, String> {
    let configs = agent_configs();
    let config = configs
        .get(agent)
        .ok_or_else(|| format!("Unknown agent '{agent}'"))?;

    let output_path = repo_root.join(config.path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    let output = config.format.render(content);
    fs::write(&output_path, output).map_err(|e| format!("Failed to write {}: {e}", config.path))?;

    Ok(output_path)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        let configs = agent_configs();
        eprintln!("Error: No agents specified. Pass agent names as args.");
        eprintln!(
            "Supported agents: {}",
            configs.keys().copied().collect::<Vec<_>>().join(", ")
        );
        process::exit(1);
    }

    let repo_root = env::var("PRE_COMMIT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().expect("Cannot determine current directory"));

    let constitution_path = repo_root.join("CONSTITUTION.md");
    let content = match fs::read_to_string(&constitution_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: CONSTITUTION.md not found in repo root.");
            process::exit(1);
        }
    };

    let configs = agent_configs();
    let mut generated_files = Vec::new();

    for agent in &args {
        match sync_agent(&repo_root, agent, &content) {
            Ok(_) => generated_files.push(configs[agent.as_str()].path),
            Err(e) => {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
    }

    if !generated_files.is_empty() {
        let status = Command::new("git")
            .arg("add")
            .args(&generated_files)
            .current_dir(&repo_root)
            .status()
            .expect("Failed to run git add");

        if !status.success() {
            eprintln!("Error: git add failed");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_wrap_yaml_basic() {
        let result = wrap_yaml("Use TypeScript\nWrite tests");
        assert_eq!(result, "conventions: \"Use TypeScript\nWrite tests\"\n");
    }

    #[test]
    fn test_wrap_yaml_escapes_quotes() {
        let result = wrap_yaml("Use \"strict\" mode");
        assert_eq!(result, "conventions: \"Use \\\"strict\\\" mode\"\n");
    }

    #[test]
    fn test_wrap_yaml_escapes_backslashes() {
        let result = wrap_yaml("path\\to\\file");
        assert_eq!(result, "conventions: \"path\\\\to\\\\file\"\n");
    }

    #[test]
    fn test_wrap_json_basic() {
        let result = wrap_json("Use Rust");
        assert_eq!(
            result,
            format!("{{\n  \"_generated\": \"{NOTICE}\",\n  \"conventions\": \"Use Rust\"\n}}\n")
        );
    }

    #[test]
    fn test_wrap_json_escapes_special_chars() {
        // Input chars: a " b \ c <newline> d
        let result = wrap_json("a\"b\\c\nd");
        // " -> \" , \ -> \\ , newline -> \n
        assert!(result.contains(r#"a\"b\\c\nd"#));
    }

    #[test]
    fn test_render_markdown() {
        let result = Format::Markdown.render("# Rules\n- Be nice");
        assert_eq!(result, format!("<!-- {NOTICE} -->\n\n# Rules\n- Be nice"));
    }

    #[test]
    fn test_render_yaml() {
        let result = Format::Yaml.render("Do things");
        assert_eq!(
            result,
            format!("# {NOTICE}\n\nconventions: \"Do things\"\n")
        );
    }

    #[test]
    fn test_render_json() {
        let result = Format::Json.render("Do things");
        assert_eq!(result, wrap_json("Do things"));
    }

    #[test]
    fn test_agent_configs_contains_all_agents() {
        let configs = agent_configs();
        assert!(configs.contains_key("claude"));
        assert!(configs.contains_key("cursor"));
        assert!(configs.contains_key("copilot"));
        assert!(configs.contains_key("windsurf"));
        assert!(configs.contains_key("aider"));
        assert!(configs.contains_key("continue"));
    }

    #[test]
    fn test_sync_agent_claude() {
        let tmp = TempDir::new().unwrap();
        let content = "# My Rules\n- Rule 1";
        sync_agent(tmp.path(), "claude", content).unwrap();

        let output = fs::read_to_string(tmp.path().join("CLAUDE.md")).unwrap();
        assert!(output.starts_with("<!-- AUTO-GENERATED FROM CONSTITUTION.md"));
        assert!(output.contains("# My Rules\n- Rule 1"));
    }

    #[test]
    fn test_sync_agent_copilot_creates_subdirectory() {
        let tmp = TempDir::new().unwrap();
        let content = "rules here";
        sync_agent(tmp.path(), "copilot", content).unwrap();

        let output_path = tmp.path().join(".github/copilot-instructions.md");
        assert!(output_path.exists());
        let output = fs::read_to_string(output_path).unwrap();
        assert!(output.contains("rules here"));
    }

    #[test]
    fn test_sync_agent_aider_wraps_yaml() {
        let tmp = TempDir::new().unwrap();
        let content = "Use Rust";
        sync_agent(tmp.path(), "aider", content).unwrap();

        let output = fs::read_to_string(tmp.path().join(".aider.conf.yml")).unwrap();
        assert!(output.starts_with("# AUTO-GENERATED FROM CONSTITUTION.md"));
        assert!(output.contains("conventions: \"Use Rust\""));
    }

    #[test]
    fn test_sync_agent_continue_wraps_json() {
        let tmp = TempDir::new().unwrap();
        let content = "Use Rust";
        sync_agent(tmp.path(), "continue", content).unwrap();

        let output_path = tmp.path().join(".continue/config.json");
        assert!(output_path.exists());
        let output = fs::read_to_string(output_path).unwrap();
        assert!(output.contains("\"conventions\": \"Use Rust\""));
        assert!(output.contains("\"_generated\""));
    }

    #[test]
    fn test_sync_agent_unknown_returns_error() {
        let tmp = TempDir::new().unwrap();
        let result = sync_agent(tmp.path(), "unknown_agent", "content");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown agent"));
    }

    #[test]
    fn test_sync_agent_cursor() {
        let tmp = TempDir::new().unwrap();
        let content = "cursor rules";
        sync_agent(tmp.path(), "cursor", content).unwrap();

        let output = fs::read_to_string(tmp.path().join(".cursorrules")).unwrap();
        assert!(output.contains("cursor rules"));
    }

    #[test]
    fn test_sync_agent_windsurf() {
        let tmp = TempDir::new().unwrap();
        let content = "windsurf rules";
        sync_agent(tmp.path(), "windsurf", content).unwrap();

        let output = fs::read_to_string(tmp.path().join(".windsurfrules")).unwrap();
        assert!(output.contains("windsurf rules"));
    }
}
