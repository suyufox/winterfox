use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<WinterfoxPlugins<R>> {
  Ok(WinterfoxPlugins(app.clone()))
}

/// Access to the winterfox-plugins APIs.
pub struct WinterfoxPlugins<R: Runtime>(AppHandle<R>);

impl<R: Runtime> WinterfoxPlugins<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
