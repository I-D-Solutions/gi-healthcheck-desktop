use super::CheckResult;
use super::manifest::SecureLinkConfig;
use super::process;
use sysinfo::Networks;

const CATEGORY: &str = "Secure Link";

pub fn run_checks(config: &SecureLinkConfig) -> Vec<CheckResult> {
    vec![
        check_interface(&config.expected_interface),
        process::check_process_running(&config.process_name, CATEGORY),
    ]
}

fn check_interface(expected: &str) -> CheckResult {
    let networks = Networks::new_with_refreshed_list();
    let found = networks.iter().any(|(name, _)| name == expected);

    if found {
        CheckResult::pass(
            &format!("Interface: {}", expected),
            CATEGORY,
            &format!("Secure link interface {} is present", expected),
        )
    } else {
        let available: Vec<&str> = networks.iter().map(|(name, _)| name.as_str()).collect();
        CheckResult::warn(
            &format!("Interface: {}", expected),
            CATEGORY,
            &format!("Expected interface {} not found", expected),
        )
        .with_details(&format!("Available interfaces: {:?}", available))
    }
}
