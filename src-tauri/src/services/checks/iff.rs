use super::CheckResult;
use super::manifest::IffConfig;
use super::serial;
use super::process;

const CATEGORY: &str = "IFF";

pub fn run_checks(config: &IffConfig) -> Vec<CheckResult> {
    vec![
        serial::check_serial_port(&config.serial_port, config.baud_rate, CATEGORY),
        process::check_process_running(&config.process_name, CATEGORY),
    ]
}
