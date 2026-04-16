pub mod network;
pub mod disk;
pub mod memory;
pub mod cpu;
pub mod runtime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub category: String,
    pub status: CheckStatus,
    pub message: String,
    pub details: Option<String>,
    pub timestamp: String,
}

impl CheckResult {
    pub fn pass(name: &str, category: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            status: CheckStatus::Pass,
            message: message.to_string(),
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn warn(name: &str, category: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            status: CheckStatus::Warn,
            message: message.to_string(),
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn fail(name: &str, category: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            status: CheckStatus::Fail,
            message: message.to_string(),
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }
}

pub async fn run_all_checks() -> Vec<CheckResult> {
    let mut results = Vec::new();

    results.push(runtime::check_hostname());
    results.push(runtime::check_os_info());
    results.push(runtime::check_app_data_dir());
    results.push(memory::check_memory());
    results.push(cpu::check_cpu());
    results.push(disk::check_disk());
    results.push(network::check_dns().await);
    results.push(network::check_reachability().await);

    results
}
