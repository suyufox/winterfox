use tauri::{AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::WinterfoxServiceExt;

pub mod log;

#[tauri::command]
pub async fn ping<R: Runtime>(app: AppHandle<R>, payload: PingRequest) -> Result<PingResponse> {
  app.winterfox_service().ping(payload)
}
