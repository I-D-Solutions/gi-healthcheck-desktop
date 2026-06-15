use super::manifest::ExtraConnectedDeviceConfig;
use super::CheckResult;
use std::process::Command;
use std::time::Duration;

const CATEGORY: &str = "Connected Devices";

pub async fn run_checks(configs: &[ExtraConnectedDeviceConfig]) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for config in configs {
        if let Some(expected_ip) = &config.expected_ip {
            results.push(check_ip_reachability(config, expected_ip));

            if let Some(ports) = &config.ports {
                for port in ports {
                    results.push(check_tcp_port(config, expected_ip, *port).await);
                }
            }
        } else if config.required.unwrap_or(false) {
            results.push(
                CheckResult::fail(
                    &format!("{} Address", config.name),
                    CATEGORY,
                    &format!(
                        "{} is required but has no expected_ip configured",
                        config.name
                    ),
                )
                .with_details(&device_details(config)),
            );
        }
    }

    results
}

fn check_ip_reachability(config: &ExtraConnectedDeviceConfig, expected_ip: &str) -> CheckResult {
    let mut command = Command::new("ping");

    if cfg!(target_os = "windows") {
        command.args(["-n", "1", "-w", "1000", expected_ip]);
    } else {
        command.args(["-c", "1", "-W", "1", expected_ip]);
    }

    match command.output() {
        Ok(output) if output.status.success() => CheckResult::pass(
            &format!("{} Reachability", config.name),
            CATEGORY,
            &format!("{} reachable at {}", config.name, expected_ip),
        )
        .with_details(&device_details(config)),
        Ok(output) => status_for_required(config)(
            &format!("{} Reachability", config.name),
            CATEGORY,
            &format!("{} at {} did not respond to ping", config.name, expected_ip),
        )
        .with_details(&format!(
            "{}\n{}",
            device_details(config),
            String::from_utf8_lossy(&output.stderr)
        )),
        Err(e) => status_for_required(config)(
            &format!("{} Reachability", config.name),
            CATEGORY,
            &format!("Could not run ping for {}", config.name),
        )
        .with_details(&format!("{}\n{}", device_details(config), e)),
    }
}

async fn check_tcp_port(
    config: &ExtraConnectedDeviceConfig,
    expected_ip: &str,
    port: u16,
) -> CheckResult {
    let addr = format!("{}:{}", expected_ip, port);
    let timeout = Duration::from_secs(3);

    match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => CheckResult::pass(
            &format!("{} Port {}", config.name, port),
            CATEGORY,
            &format!("{} service reachable at {}", config.name, addr),
        )
        .with_details(&device_details(config)),
        Ok(Err(e)) => status_for_required(config)(
            &format!("{} Port {}", config.name, port),
            CATEGORY,
            &format!("{} service at {} is not reachable", config.name, addr),
        )
        .with_details(&format!("{}\n{}", device_details(config), e)),
        Err(_) => status_for_required(config)(
            &format!("{} Port {}", config.name, port),
            CATEGORY,
            &format!("Connection to {} timed out after 3s", addr),
        )
        .with_details(&device_details(config)),
    }
}

fn status_for_required(config: &ExtraConnectedDeviceConfig) -> fn(&str, &str, &str) -> CheckResult {
    if config.required.unwrap_or(false) {
        CheckResult::fail
    } else {
        CheckResult::warn
    }
}

fn device_details(config: &ExtraConnectedDeviceConfig) -> String {
    let mut details = vec![format!("kind: {}", config.kind)];

    if let Some(baud_rate) = config.baud_rate {
        details.push(format!("baud_rate: {}", baud_rate));
    }

    if let Some(notes) = &config.notes {
        details.push(format!("notes: {}", notes));
    }

    details.join("\n")
}
