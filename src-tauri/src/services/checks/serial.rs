use super::CheckResult;

pub fn check_serial_port(expected_port: &str, expected_baud: u32, category: &str) -> CheckResult {
    let available = serialport::available_ports();

    match available {
        Ok(ports) => {
            let found = ports.iter().any(|p| p.port_name == expected_port);
            if found {
                match serialport::new(expected_port, expected_baud)
                    .timeout(std::time::Duration::from_secs(1))
                    .open()
                {
                    Ok(_) => CheckResult::pass(
                        &format!("Serial Port {}", expected_port),
                        category,
                        &format!("Port {} open at {} baud", expected_port, expected_baud),
                    ),
                    Err(e) => CheckResult::warn(
                        &format!("Serial Port {}", expected_port),
                        category,
                        &format!("Port {} found but cannot open", expected_port),
                    )
                    .with_details(&format!("{}", e)),
                }
            } else {
                let available_list: Vec<&str> = ports.iter().map(|p| p.port_name.as_str()).collect();
                CheckResult::warn(
                    &format!("Serial Port {}", expected_port),
                    category,
                    &format!("Expected port {} not found", expected_port),
                )
                .with_details(&format!("Available ports: {:?}", available_list))
            }
        }
        Err(e) => CheckResult::fail(
            &format!("Serial Port {}", expected_port),
            category,
            "Cannot enumerate serial ports",
        )
        .with_details(&format!("{}", e)),
    }
}

pub fn list_serial_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.port_name)
        .collect()
}
