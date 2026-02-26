use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::log::*;
use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_winterfox_service);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<WinterfoxService<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("wen.suyufox.winterfox.service", "ServicePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_winterfox_service)?;
  Ok(WinterfoxService(handle))
}

/// Access to the winterfox-service APIs.
pub struct WinterfoxService<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> WinterfoxService<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self.0.run_mobile_plugin("ping", payload).map_err(Into::into)
  }

  pub fn log_info(&self, payload: InfoRequest) {
    self.0.run_mobile_plugin("info", payload).map_err(Into::into)
  }

  pub fn log_debug(&self, payload: DebugRequest) {
    self.0.run_mobile_plugin("debug", payload).map_err(Into::into)
  }

  pub fn log_trace(&self, payload: TraceRequest) {
    self.0.run_mobile_plugin("trace", payload).map_err(Into::into)
  }

  pub fn log_error(&self, payload: ErrorRequest) {
    self.0.run_mobile_plugin("error", payload).map_err(Into::into)
  }

  pub fn log_warn(&self, payload: WarnRequest) {
    self.0.run_mobile_plugin("warn", payload).map_err(Into::into)
  }
}
