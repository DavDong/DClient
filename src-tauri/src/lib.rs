mod commands;
mod pty_manager;

use pty_manager::PtyManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(PtyManager::new())
        .invoke_handler(tauri::generate_handler![
            commands::spawn_pty,
            commands::write_pty,
            commands::resize_pty,
            commands::kill_pty,
            commands::get_current_version,
            commands::check_update,
            commands::download_update,
            commands::read_config,
            commands::write_config,
            commands::get_claude_skills,
            commands::get_mcp_servers,
            commands::get_claude_plugins,
            commands::get_claude_history,
            commands::get_session_messages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
