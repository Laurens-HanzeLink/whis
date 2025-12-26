mod commands;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::transcribe_audio,
            commands::validate_api_key,
            commands::list_presets,
            commands::get_preset_details,
            commands::set_active_preset,
            commands::get_active_preset,
            commands::create_preset,
            commands::update_preset,
            commands::delete_preset,
            commands::transcribe_streaming_start,
            commands::transcribe_streaming_send_chunk,
            commands::transcribe_streaming_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
