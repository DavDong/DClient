use crate::pty_manager::PtyManager;
use tauri::State;

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
