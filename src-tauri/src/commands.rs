use crate::pty_manager::PtyManager;
use serde::Serialize;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tauri::State;

#[derive(Serialize, Clone)]
pub struct VersionInfo {
    pub current: String,
    pub latest: String,
    pub has_update: bool,
    pub download_url: String,
    pub release_notes: String,
}

#[tauri::command]
pub fn get_current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub async fn check_update() -> Result<VersionInfo, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();

    // 从 GitHub API 获取最新 release
    let url = "https://api.github.com/repos/DavDong/DClient/releases/latest";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "DClient")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    let latest = body["tag_name"]
        .as_str()
        .unwrap_or("")
        .trim_start_matches('v')
        .to_string();

    let release_notes = body["body"].as_str().unwrap_or("").to_string();

    // 获取当前平台的下载链接
    let platform = if cfg!(target_os = "macos") {
        "dmg"
    } else {
        "msi"
    };

    let download_url = body["assets"]
        .as_array()
        .and_then(|assets| {
            assets.iter().find(|a| {
                a["name"]
                    .as_str()
                    .map(|n| n.contains(platform))
                    .unwrap_or(false)
            })
        })
        .and_then(|a| a["browser_download_url"].as_str())
        .unwrap_or("")
        .to_string();

    let has_update = !latest.is_empty() && latest != current;

    Ok(VersionInfo {
        current,
        latest,
        has_update,
        download_url,
        release_notes,
    })
}

#[tauri::command]
pub async fn download_update(url: String) -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let download_dir = PathBuf::from(&home).join("Downloads");

    // 从 URL 提取文件名
    let filename = url.split('/').last().unwrap_or("DClient-update");
    let save_path = download_dir.join(filename);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "DClient")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(&save_path, &bytes).map_err(|e| e.to_string())?;

    Ok(save_path.to_string_lossy().to_string())
}

fn dclient_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let dir = PathBuf::from(home).join(".ZhtyDClient");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(dir)
}

#[tauri::command]
pub fn read_config(filename: String) -> Result<String, String> {
    let path = dclient_dir()?.join(&filename);
    if !path.exists() {
        return Ok("{}".to_string());
    }
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_config(filename: String, data: String) -> Result<(), String> {
    let path = dclient_dir()?.join(&filename);
    std::fs::write(&path, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn spawn_pty(
    app_handle: tauri::AppHandle,
    state: State<'_, PtyManager>,
    cols: u16,
    rows: u16,
) -> Result<String, String> {
    state.spawn(app_handle, cols, rows)
}

#[tauri::command]
pub fn write_pty(state: State<'_, PtyManager>, id: String, data: String) -> Result<(), String> {
    state.write(&id, &data)
}

#[tauri::command]
pub fn resize_pty(
    state: State<'_, PtyManager>,
    id: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    state.resize(&id, cols, rows)
}

#[tauri::command]
pub fn kill_pty(state: State<'_, PtyManager>, id: String) -> Result<(), String> {
    state.kill(&id)
}

#[derive(Serialize, Clone)]
pub struct ClaudeSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content: String,      // 完整 markdown 内容
    pub source: String,       // "global" 或项目路径
}

#[tauri::command]
pub fn get_claude_skills(project_path: Option<String>) -> Result<Vec<ClaudeSkill>, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let mut skills = Vec::new();

    // 读取全局技能 ~/.claude/skills/*/skill.md
    let global_dir = format!("{}/.claude/skills", home);
    read_skills_from_dir(&global_dir, "global", &mut skills);

    // 读取项目级技能 <project>/.claude/skills/*/skill.md
    if let Some(ref project) = project_path {
        let project_dir = format!("{}/.claude/skills", project);
        read_skills_from_dir(&project_dir, project, &mut skills);
        // 也检查 .claude/commands/
        let commands_dir = format!("{}/.claude/commands", project);
        read_skills_from_dir(&commands_dir, project, &mut skills);
    }

    Ok(skills)
}

fn read_skills_from_dir(dir: &str, source: &str, skills: &mut Vec<ClaudeSkill>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }

        let skill_file = path.join("skill.md");
        if !skill_file.exists() { continue; }

        let content = match std::fs::read_to_string(&skill_file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let id = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        // 解析 frontmatter
        let (name, description) = parse_skill_frontmatter(&content);

        skills.push(ClaudeSkill {
            id: id.clone(),
            name: if name.is_empty() { id } else { name },
            description,
            content: content.clone(),
            source: source.to_string(),
        });
    }
}

fn parse_skill_frontmatter(content: &str) -> (String, String) {
    let mut name = String::new();
    let mut description = String::new();

    if !content.starts_with("---") {
        return (name, description);
    }

    // 找第二个 ---
    if let Some(end) = content[3..].find("---") {
        let frontmatter = &content[3..3 + end];
        for line in frontmatter.lines() {
            let line = line.trim();
            if let Some(v) = line.strip_prefix("name:") {
                name = v.trim().trim_matches('"').to_string();
            } else if let Some(v) = line.strip_prefix("description:") {
                description = v.trim().trim_matches('"').to_string();
            }
        }
    }

    (name, description)
}

#[derive(Serialize, Clone)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub source: String,   // "global" 或项目路径
}

#[tauri::command]
pub fn get_mcp_servers(project_path: Option<String>) -> Result<Vec<McpServer>, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let mut servers = Vec::new();

    // 全局 MCP: ~/.claude/.claude.json
    let global_path = format!("{}/.claude/.claude.json", home);
    read_mcp_from_file(&global_path, "global", &mut servers);

    // 项目级 MCP: <project>/.mcp.json
    if let Some(ref project) = project_path {
        let project_mcp = format!("{}/.mcp.json", project);
        read_mcp_from_file(&project_mcp, project, &mut servers);
    }

    Ok(servers)
}

fn read_mcp_from_file(path: &str, source: &str, servers: &mut Vec<McpServer>) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };
    let v: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return,
    };

    if let Some(obj) = v["mcpServers"].as_object() {
        for (name, cfg) in obj {
            let command = cfg["command"].as_str().unwrap_or("").to_string();
            let args = cfg["args"]
                .as_array()
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            servers.push(McpServer {
                name: name.clone(),
                command,
                args,
                source: source.to_string(),
            });
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ClaudePlugin {
    pub id: String,
    pub name: String,
    pub marketplace: String,
    pub version: String,
    pub description: String,
    pub installed_at: String,
}

#[tauri::command]
pub fn get_claude_plugins() -> Result<Vec<ClaudePlugin>, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let installed_path = format!("{}/.claude/plugins/installed_plugins.json", home);

    let content = std::fs::read_to_string(&installed_path).map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let mut plugins = Vec::new();

    if let Some(obj) = v["plugins"].as_object() {
        for (key, entries) in obj {
            // key 格式: "name@marketplace"
            let parts: Vec<&str> = key.splitn(2, '@').collect();
            let name = parts.first().unwrap_or(&"").to_string();
            let marketplace = parts.get(1).unwrap_or(&"").to_string();

            if let Some(arr) = entries.as_array() {
                if let Some(entry) = arr.first() {
                    let version = entry["version"].as_str().unwrap_or("").to_string();
                    let installed_at = entry["installedAt"].as_str().unwrap_or("").to_string();
                    let install_path = entry["installPath"].as_str().unwrap_or("");

                    // 尝试读取 README 第三行作为描述
                    let readme_path = format!("{}/README.md", install_path);
                    let description = if let Ok(readme) = std::fs::read_to_string(&readme_path) {
                        readme.lines()
                            .find(|l| !l.is_empty() && !l.starts_with('#'))
                            .unwrap_or("")
                            .to_string()
                    } else {
                        String::new()
                    };

                    plugins.push(ClaudePlugin {
                        id: key.clone(),
                        name,
                        marketplace,
                        version,
                        description,
                        installed_at,
                    });
                }
            }
        }
    }

    // 按安装时间倒序
    plugins.sort_by(|a, b| b.installed_at.cmp(&a.installed_at));
    Ok(plugins)
}

#[derive(Serialize, Clone)]
pub struct ClaudeSession {
    pub session_id: String,
    pub project: String,
    pub first_message: String,
    pub last_timestamp: u64,
    pub message_count: usize,
}

#[derive(Serialize, Clone)]
pub struct SessionMessage {
    pub role: String,       // "user" | "assistant"
    pub text: String,       // 消息文本
    pub timestamp: String,  // ISO 时间
}

#[tauri::command]
pub fn get_session_messages(session_id: String) -> Result<Vec<SessionMessage>, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let projects_dir = format!("{}/.claude/projects", home);

    // 遍历 projects 目录找到包含 session_id 的文件
    let projects = std::fs::read_dir(&projects_dir).map_err(|e| e.to_string())?;
    let filename = format!("{}.jsonl", session_id);

    let mut target_path = None;
    for entry in projects.flatten() {
        let path = entry.path().join(&filename);
        if path.exists() {
            target_path = Some(path);
            break;
        }
    }

    let path = target_path.ok_or("Session file not found")?;
    let file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut messages = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let v: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg_type = v["type"].as_str().unwrap_or("");
        if msg_type != "user" && msg_type != "assistant" {
            continue;
        }

        let timestamp = v["timestamp"].as_str().unwrap_or("").to_string();

        // message 字段可能是 JSON 对象或字符串化的 JSON
        let msg_val = if v["message"].is_object() {
            v["message"].clone()
        } else if let Some(msg_str) = v["message"].as_str() {
            match serde_json::from_str(msg_str) {
                Ok(v) => v,
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let role = msg_val["role"].as_str().unwrap_or(msg_type).to_string();

        // 提取文本内容
        let text = if let Some(content) = msg_val["content"].as_str() {
            content.to_string()
        } else if let Some(arr) = msg_val["content"].as_array() {
            arr.iter()
                .filter(|c| c["type"].as_str() == Some("text"))
                .filter_map(|c| c["text"].as_str())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            continue;
        };

        // 跳过空消息和纯 tool_use
        if text.trim().is_empty() {
            continue;
        }

        messages.push(SessionMessage {
            role,
            text,
            timestamp,
        });
    }

    Ok(messages)
}

#[tauri::command]
pub fn get_claude_history() -> Result<Vec<ClaudeSession>, String> {
    let home = std::env::var("HOME").map_err(|e| e.to_string())?;
    let history_path = format!("{}/.claude/history.jsonl", home);

    let file = std::fs::File::open(&history_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut sessions: HashMap<String, ClaudeSession> = HashMap::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let v: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let session_id = v["sessionId"].as_str().unwrap_or("").to_string();
        let project = v["project"].as_str().unwrap_or("").to_string();
        let display = v["display"].as_str().unwrap_or("").to_string();
        let timestamp = v["timestamp"].as_u64().unwrap_or(0);

        if session_id.is_empty() {
            continue;
        }

        let entry = sessions.entry(session_id.clone()).or_insert(ClaudeSession {
            session_id,
            project: project.clone(),
            first_message: display.clone(),
            last_timestamp: timestamp,
            message_count: 0,
        });

        entry.message_count += 1;
        if timestamp > entry.last_timestamp {
            entry.last_timestamp = timestamp;
        }
    }

    let mut result: Vec<ClaudeSession> = sessions.into_values().collect();
    result.sort_by(|a, b| b.last_timestamp.cmp(&a.last_timestamp));

    Ok(result)
}
