use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

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
use desktop::WinterfoxConfigs;
#[cfg(mobile)]
use mobile::WinterfoxConfigs;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the winterfox-configs APIs.
pub trait WinterfoxConfigsExt<R: Runtime> {
  fn winterfox_configs(&self) -> &WinterfoxConfigs<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WinterfoxConfigsExt<R> for T {
  fn winterfox_configs(&self) -> &WinterfoxConfigs<R> {
    self.state::<WinterfoxConfigs<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("winterfox-configs")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let winterfox_configs = mobile::init(app, api)?;
      #[cfg(desktop)]
      let winterfox_configs = desktop::init(app, api)?;
      app.manage(winterfox_configs);
      Ok(())
    })
    .build()
}
