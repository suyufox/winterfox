use tauri::{plugin::TauriPlugin, Manager, Runtime};

use tauri::plugin::Builder as PluginBuilder;

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::WinterfoxService;
#[cfg(mobile)]
use mobile::WinterfoxService;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the winterfox-service APIs.
pub trait WinterfoxServiceExt<R: Runtime> {
  fn winterfox_service(&self) -> &WinterfoxService<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WinterfoxServiceExt<R> for T {
  fn winterfox_service(&self) -> &WinterfoxService<R> {
    self.state::<WinterfoxService<R>>().inner()
  }
}

///
/// service main config
/// 服务 主配置
///
pub struct ServiceConfigs<'a, R: Runtime> {
  app_handle: Option<&'a tauri::AppHandle<R>>,
}

pub use FreedomLogger::LogLevel;
pub use FreedomLogger::Pattern;
pub use FreedomLogger::{log_debug, log_error, log_info, log_trace, log_warning};

///
///
///
///
pub struct LoggerConfig {
  enable: bool,
  pattern: Pattern,
  file_path: Option<String>,
  file_name: String,
  log_level: FreedomLogger::LogLevel,
  max_size: u64,
  max_file_number: u32,
}

///
/// Tauri Plugins - Winterfox Service Builder
///
pub struct Builder<'a, R: Runtime> {
  pub service: ServiceConfigs<'a, R>,
  pub logger: LoggerConfig,
  _runtime: std::marker::PhantomData<R>,
}

impl<'a, R: Runtime> Builder<'a, R> {
  pub fn new() -> Self {
    Self {
      service: ServiceConfigs { app_handle: None },
      logger: LoggerConfig {
        enable: false,
        pattern: Pattern::Basic,
        file_path: Some("log".to_string()),
        file_name: "app".to_string(),
        log_level: LogLevel::Info,
        max_size: 50 * 1024 * 1024,
        max_file_number: 10,
      },
      _runtime: std::marker::PhantomData,
    }
  }

  pub fn set_app_handle(&mut self, app_handle: &'a tauri::AppHandle<R>) -> &mut Self {
    self.service.app_handle = Some(app_handle);
    self
  }

  pub fn set_logger_enable(&mut self, enable: bool) -> &mut Self {
    self.logger.enable = enable;
    self
  }

  pub fn set_logger_pattern(&mut self, pattern: Pattern) -> &mut Self {
    self.logger.pattern = pattern;
    self
  }

  pub fn set_logger_file_path(&mut self, file_path: String) -> &mut Self {
    self.logger.file_path = Some(file_path);
    self
  }

  pub fn set_logger_file_name(&mut self, file_name: String) -> &mut Self {
    self.logger.file_name = file_name;
    self
  }

  pub fn set_logger_log_level(&mut self, log_level: LogLevel) -> &mut Self {
    self.logger.log_level = log_level;
    self
  }

  pub fn set_logger_max_size(&mut self, max_size: u64) -> &mut Self {
    self.logger.max_size = max_size;
    self
  }

  pub fn set_logger_max_file_number(&mut self, max_file_number: u32) -> &mut Self {
    self.logger.max_file_number = max_file_number;
    self
  }

  pub fn build(self) -> TauriPlugin<R> {
    if self.logger.enable {
      use FreedomLogger::log_init_with_rotation;

      log_init_with_rotation(
        self.logger.pattern,
        self.logger.file_path.unwrap_or("logs".to_string()),
        &self.logger.file_name,
        self.logger.log_level,
        self.logger.max_size,
        self.logger.max_file_number,
      );

      log_info("Logger Initialization completed | Logger 初始化完成");
    };

    log_info("Initialize service plugin | 初始化 Service 插件");
    PluginBuilder::new("winterfox-service")
      .invoke_handler(
        tauri::generate_handler![
          commands::ping,
          commands::log::info,
          commands::log::debug,
          commands::log::error,
          commands::log::warn,
          commands::log::trace
        ])
      .setup(|app, api| {
        #[cfg(mobile)]
        let winterfox_service = mobile::init(app, api)?;
        #[cfg(desktop)]
        let winterfox_service = desktop::init(app, api)?;
        app.manage(winterfox_service);

        log_info("Service plugin initialization completed | 服务插件初始化完成");
        Ok(())
      })
      .build()
  }
}
