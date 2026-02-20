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
use desktop::WinterfoxPlugins;
#[cfg(mobile)]
use mobile::WinterfoxPlugins;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the winterfox-plugins APIs.
pub trait WinterfoxPluginsExt<R: Runtime> {
  fn winterfox_plugins(&self) -> &WinterfoxPlugins<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WinterfoxPluginsExt<R> for T {
  fn winterfox_plugins(&self) -> &WinterfoxPlugins<R> {
    self.state::<WinterfoxPlugins<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("winterfox-plugins")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let winterfox_plugins = mobile::init(app, api)?;
      #[cfg(desktop)]
      let winterfox_plugins = desktop::init(app, api)?;
      app.manage(winterfox_plugins);
      Ok(())
    })
    .build()
}
