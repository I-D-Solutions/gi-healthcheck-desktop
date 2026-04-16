use super::CheckResult;
use sysinfo::System;

const LOW_MEM_THRESHOLD_PERCENT: f64 = 10.0;

pub fn check_memory() -> CheckResult {
    let cat = "System";
    let mut sys = System::new();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = total.saturating_sub(used);
    let available_pct = if total > 0 {
        (available as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let total_gb = total as f64 / 1_073_741_824.0;
    let avail_gb = available as f64 / 1_073_741_824.0;

    let details = format!(
        "{:.1} GB available / {:.1} GB total ({:.0}% free)",
        avail_gb, total_gb, available_pct
    );

    if available_pct < LOW_MEM_THRESHOLD_PERCENT {
        CheckResult::fail("Memory", cat, "Memory critically low").with_details(&details)
    } else if available_pct < 20.0 {
        CheckResult::warn("Memory", cat, "Memory usage is high").with_details(&details)
    } else {
        CheckResult::pass("Memory", cat, "Memory usage is normal").with_details(&details)
    }
}
