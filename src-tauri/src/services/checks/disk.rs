use super::CheckResult;
use sysinfo::Disks;

const LOW_DISK_THRESHOLD_GB: f64 = 5.0;

pub fn check_disk() -> CheckResult {
    let cat = "Storage";
    let disks = Disks::new_with_refreshed_list();

    if disks.list().is_empty() {
        return CheckResult::warn("Disk Space", cat, "No disks detected");
    }

    let mut worst = CheckResult::pass("Disk Space", cat, "Disk space is healthy");
    let mut details = Vec::new();

    for disk in disks.list() {
        let total_gb = disk.total_space() as f64 / 1_073_741_824.0;
        let avail_gb = disk.available_space() as f64 / 1_073_741_824.0;
        let mount = disk.mount_point().to_string_lossy().to_string();

        details.push(format!(
            "{}: {:.1} GB free / {:.1} GB total",
            mount, avail_gb, total_gb
        ));

        if avail_gb < LOW_DISK_THRESHOLD_GB {
            worst = CheckResult::fail(
                "Disk Space",
                cat,
                &format!("Low disk space on {}", mount),
            );
        }
    }

    worst.with_details(&details.join("\n"))
}
