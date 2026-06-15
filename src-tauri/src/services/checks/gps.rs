use super::manifest::GpsConfig;
use super::process;
use super::serial;
use super::CheckResult;
use std::io::Read;
use std::process::Command;
use std::time::Duration;

const CATEGORY: &str = "GPS";

pub fn run_checks(config: &GpsConfig) -> Vec<CheckResult> {
    let mut results = vec![
        serial::check_serial_port(&config.serial_port, config.baud_rate, CATEGORY),
        process::check_process_running(&config.process_name, CATEGORY),
    ];

    if config.nmea_required {
        results.push(check_nmea_data(&config.serial_port, config.baud_rate));
    }

    if let Some(expected_ip) = &config.expected_ip {
        results.push(check_network_gps(expected_ip));
    }

    results
}

fn check_network_gps(expected_ip: &str) -> CheckResult {
    let mut command = Command::new("ping");

    if cfg!(target_os = "windows") {
        command.args(["-n", "1", "-w", "1000", expected_ip]);
    } else {
        command.args(["-c", "1", "-W", "1", expected_ip]);
    }

    match command.output() {
        Ok(output) if output.status.success() => CheckResult::pass(
            "GPS Network Reachability",
            CATEGORY,
            &format!("GPS reachable at {}", expected_ip),
        ),
        Ok(output) => CheckResult::warn(
            "GPS Network Reachability",
            CATEGORY,
            &format!("GPS at {} did not respond to ping", expected_ip),
        )
        .with_details(&String::from_utf8_lossy(&output.stderr)),
        Err(e) => CheckResult::warn(
            "GPS Network Reachability",
            CATEGORY,
            "Could not run ping for GPS network check",
        )
        .with_details(&format!("{}", e)),
    }
}

fn check_nmea_data(port: &str, baud: u32) -> CheckResult {
    let port_result = serialport::new(port, baud)
        .timeout(Duration::from_secs(2))
        .open();

    match port_result {
        Ok(mut port) => {
            let mut buf = [0u8; 256];
            match port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buf[..n]);
                    if data.contains("$GP") || data.contains("$GN") || data.contains("$GL") {
                        CheckResult::pass("NMEA Data", CATEGORY, "GPS is producing NMEA sentences")
                            .with_details(&format!(
                                "Read {} bytes: {}...",
                                n,
                                &data[..data.len().min(80)]
                            ))
                    } else {
                        CheckResult::warn(
                            "NMEA Data",
                            CATEGORY,
                            "Data received but no NMEA sentences detected",
                        )
                        .with_details(&format!(
                            "Read {} bytes: {}...",
                            n,
                            &data[..data.len().min(80)]
                        ))
                    }
                }
                Ok(_) => CheckResult::warn("NMEA Data", CATEGORY, "Port open but no data received"),
                Err(e) => CheckResult::warn("NMEA Data", CATEGORY, "Could not read from GPS port")
                    .with_details(&format!("{}", e)),
            }
        }
        Err(_) => CheckResult::warn(
            "NMEA Data",
            CATEGORY,
            "GPS port not available for NMEA check",
        ),
    }
}
