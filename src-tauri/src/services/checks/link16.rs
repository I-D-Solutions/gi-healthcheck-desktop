use super::CheckResult;
use super::manifest::Link16Config;
use super::serial;
use super::process;
use std::time::Duration;

const CATEGORY: &str = "Link 16";

pub async fn run_checks(config: &Link16Config) -> Vec<CheckResult> {
    let mut results = vec![
        serial::check_serial_port(&config.serial_port, config.baud_rate, CATEGORY),
        process::check_process_running(&config.process_name, CATEGORY),
    ];

    results.push(check_tcp_reachability(&config.tcp_host, config.tcp_port).await);

    results
}

async fn check_tcp_reachability(host: &str, port: u16) -> CheckResult {
    let addr = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(3);

    match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => CheckResult::pass(
            "Link 16 Terminal",
            CATEGORY,
            &format!("Terminal reachable at {}", addr),
        ),
        Ok(Err(e)) => CheckResult::warn(
            "Link 16 Terminal",
            CATEGORY,
            &format!("Terminal at {} not reachable", addr),
        )
        .with_details(&format!("{}", e)),
        Err(_) => CheckResult::warn(
            "Link 16 Terminal",
            CATEGORY,
            &format!("Connection to {} timed out after 3s", addr),
        ),
    }
}
