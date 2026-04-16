use super::CheckResult;
use sysinfo::System;

pub fn check_cpu() -> CheckResult {
    let cat = "System";
    let mut sys = System::new();
    sys.refresh_cpu_all();

    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    if cpus.is_empty() {
        return CheckResult::warn("CPU", cat, "Could not enumerate CPUs");
    }

    let avg_usage: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;
    let core_count = cpus.len();
    let brand = cpus.first().map(|c| c.brand().to_string()).unwrap_or_default();

    let details = format!(
        "{} cores, {} | Avg usage: {:.0}%",
        core_count, brand, avg_usage
    );

    if avg_usage > 90.0 {
        CheckResult::warn("CPU", cat, "CPU usage is very high").with_details(&details)
    } else {
        CheckResult::pass("CPU", cat, "CPU usage is normal").with_details(&details)
    }
}
