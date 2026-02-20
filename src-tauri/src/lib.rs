#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(winterfox_configs::init())
        .plugin(winterfox_plugins::init())
        .plugin(winterfox_service::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
