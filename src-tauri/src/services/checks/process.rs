use super::CheckResult;
use sysinfo::System;

pub fn check_process_running(expected_name: &str, category: &str) -> CheckResult {
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let found = sys.processes().values().any(|p| {
        p.name().to_string_lossy().eq_ignore_ascii_case(expected_name)
    });

    if found {
        CheckResult::pass(
            &format!("Process: {}", expected_name),
            category,
            &format!("{} is running", expected_name),
        )
    } else {
        CheckResult::warn(
            &format!("Process: {}", expected_name),
            category,
            &format!("{} not detected", expected_name),
        )
    }
}
