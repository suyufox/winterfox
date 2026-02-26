use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::log::*;
use crate::models::*;
use crate::{log_debug, log_error, log_info, log_trace, log_warning};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<WinterfoxService<R>> {
  Ok(WinterfoxService(app.clone()))
}

/// Access to the winterfox-service APIs.
pub struct WinterfoxService<R: Runtime>(AppHandle<R>);

impl<R: Runtime> WinterfoxService<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse { value: payload.value })
  }

  pub fn log_debug(&self, message: DebugRequest) {
    log_debug(message.value.as_str())
  }

  pub fn log_info(&self, message: InfoRequest) {
    log_info(message.value.as_str())
  }

  pub fn log_error(&self, message: ErrorRequest) {
    log_error(message.value.as_str())
  }

  pub fn log_trace(&self, message: TraceRequest) {
    log_trace(message.value.as_str())
  }

  pub fn log_warn(&self, message: WarnRequest) {
    log_warning(message.value.as_str())
  }
}
