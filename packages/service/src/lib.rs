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

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("winterfox-service")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let winterfox_service = mobile::init(app, api)?;
      #[cfg(desktop)]
      let winterfox_service = desktop::init(app, api)?;
      app.manage(winterfox_service);
      Ok(())
    })
    .build()
}
