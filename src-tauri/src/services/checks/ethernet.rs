use super::CheckResult;
use sysinfo::Networks;

const LOOPBACK_NAMES: &[&str] = &["lo", "lo0"];

pub fn check_interfaces() -> Vec<CheckResult> {
    let cat = "Ethernet";
    let networks = Networks::new_with_refreshed_list();
    let mut results = Vec::new();

    for (name, data) in &networks {
        if LOOPBACK_NAMES.contains(&name.as_str()) {
            continue;
        }

        let rx = data.total_received();
        let tx = data.total_transmitted();
        let mac = data.mac_address();

        let details = format!(
            "MAC: {}\nRx: {} bytes\nTx: {} bytes",
            mac, rx, tx
        );

        let result = if rx > 0 || tx > 0 {
            CheckResult::pass(name, cat, "Interface active")
                .with_details(&details)
        } else {
            CheckResult::warn(name, cat, "Interface has no traffic")
                .with_details(&details)
        };

        results.push(result);
    }

    if results.is_empty() {
        results.push(CheckResult::warn(
            "Interfaces",
            cat,
            "No network interfaces detected",
        ));
    }

    results
}
