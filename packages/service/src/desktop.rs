use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

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
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
