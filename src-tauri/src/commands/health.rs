use crate::services::checks::{self, CheckResult};

#[tauri::command]
pub async fn run_health_checks() -> Result<Vec<CheckResult>, String> {
    Ok(checks::run_all_checks().await)
}
