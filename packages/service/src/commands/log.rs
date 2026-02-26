use crate::models::log::*;
use crate::WinterfoxServiceExt;
use tauri::{AppHandle, Runtime};

#[tauri::command]
pub async fn info<R: Runtime>(app: AppHandle<R>, message: InfoRequest) {
  app.winterfox_service().log_info(message)
}

#[tauri::command]
pub async fn debug<R: Runtime>(app: AppHandle<R>, message: DebugRequest) {
  app.winterfox_service().log_debug(message)
}

#[tauri::command]
pub async fn warn<R: Runtime>(app: AppHandle<R>, message: WarnRequest) {
  app.winterfox_service().log_warn(message)
}

#[tauri::command]
pub async fn error<R: Runtime>(app: AppHandle<R>, message: ErrorRequest) {
  app.winterfox_service().log_error(message)
}

#[tauri::command]
pub async fn trace<R: Runtime>(app: AppHandle<R>, message: TraceRequest) {
  app.winterfox_service().log_trace(message)
}
