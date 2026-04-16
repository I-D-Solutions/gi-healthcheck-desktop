use super::CheckResult;
use std::net::ToSocketAddrs;
use std::time::Duration;

pub async fn check_dns() -> CheckResult {
    let cat = "Network";
    match "dns.google:443".to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                CheckResult::pass("DNS Resolution", cat, "DNS is resolving correctly")
                    .with_details(&format!("Resolved dns.google -> {}", addr.ip()))
            } else {
                CheckResult::fail("DNS Resolution", cat, "DNS resolved but returned no addresses")
            }
        }
        Err(e) => CheckResult::fail("DNS Resolution", cat, "DNS resolution failed")
            .with_details(&format!("{}", e)),
    }
}

pub async fn check_reachability() -> CheckResult {
    let cat = "Network";
    let timeout = Duration::from_secs(5);

    match tokio::time::timeout(timeout, async {
        tokio::net::TcpStream::connect("1.1.1.1:443").await
    })
    .await
    {
        Ok(Ok(_)) => CheckResult::pass(
            "Network Reachability",
            cat,
            "Internet is reachable",
        )
        .with_details("TCP connection to 1.1.1.1:443 succeeded"),
        Ok(Err(e)) => {
            CheckResult::fail("Network Reachability", cat, "Cannot reach the internet")
                .with_details(&format!("{}", e))
        }
        Err(_) => CheckResult::warn(
            "Network Reachability",
            cat,
            "Network check timed out after 5s",
        ),
    }
}
