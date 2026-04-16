use super::CheckResult;
use super::manifest::GpsConfig;
use super::serial;
use super::process;
use std::io::Read;
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

    results
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
                            .with_details(&format!("Read {} bytes: {}...", n, &data[..data.len().min(80)]))
                    } else {
                        CheckResult::warn(
                            "NMEA Data",
                            CATEGORY,
                            "Data received but no NMEA sentences detected",
                        )
                        .with_details(&format!("Read {} bytes: {}...", n, &data[..data.len().min(80)]))
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
