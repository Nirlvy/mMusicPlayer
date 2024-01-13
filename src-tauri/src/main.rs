// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mpd_fn;
mod ws;

#[tokio::main]
async fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            mpd_fn::connect,
            mpd_fn::get_currentsong,
            mpd_fn::set_playstate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
