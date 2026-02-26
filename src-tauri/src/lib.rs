use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(winterfox_configs::init())
    .plugin(winterfox_plugins::init())
    .setup(|app| {
      let mut service = winterfox_service::Builder::new();
      let app_log_path = app.path().resolve("", tauri::path::BaseDirectory::AppLog)?;
      let app_log_path_string = format!("{}", app_log_path.display());

      service.set_app_handle(app.handle());
      service.set_logger_enable(true);
      service.set_logger_pattern(winterfox_service::Pattern::Basic);
      service.set_logger_file_path(app_log_path_string);
      #[cfg(debug_assertions)]
      {
        service.set_logger_file_name("winterfox.debug".to_string());
        service.set_logger_log_level(winterfox_service::LogLevel::Trace);
      }
      #[cfg(not(debug_assertions))]
      {
        service.set_logger_file_name("winterfox".to_string());
        service.set_logger_log_level(winterfox_service::LogLevel::Info);
      }
      service.set_logger_max_size(50 * 1024 * 1024);
      service.set_logger_max_file_number(10);

      // 创建插件但不立即构建，而是返回插件以便注册
      let plugin = service.build();

      // 注册插件
      app.app_handle().plugin(plugin)?;

      winterfox_service::log_info("service init");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
