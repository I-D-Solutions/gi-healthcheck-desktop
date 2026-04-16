use super::CheckResult;

pub fn check_hostname() -> CheckResult {
    let cat = "Runtime";
    match hostname::get() {
        Ok(name) => {
            let name = name.to_string_lossy().to_string();
            CheckResult::pass("Hostname", cat, &format!("Hostname: {}", name))
        }
        Err(e) => {
            CheckResult::warn("Hostname", cat, "Could not determine hostname")
                .with_details(&format!("{}", e))
        }
    }
}

pub fn check_os_info() -> CheckResult {
    let cat = "Runtime";
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let details = format!("OS: {} | Arch: {}", os, arch);

    CheckResult::pass("OS Info", cat, &format!("{} ({})", os, arch)).with_details(&details)
}

pub fn check_app_data_dir() -> CheckResult {
    let cat = "Runtime";
    match dirs_next() {
        Some(dir) => {
            if dir.exists() {
                CheckResult::pass(
                    "App Data Directory",
                    cat,
                    "App data directory is accessible",
                )
                .with_details(&format!("{}", dir.display()))
            } else {
                match std::fs::create_dir_all(&dir) {
                    Ok(_) => CheckResult::pass(
                        "App Data Directory",
                        cat,
                        "App data directory created successfully",
                    )
                    .with_details(&format!("{}", dir.display())),
                    Err(e) => CheckResult::fail(
                        "App Data Directory",
                        cat,
                        "Cannot create app data directory",
                    )
                    .with_details(&format!("{}: {}", dir.display(), e)),
                }
            }
        }
        None => CheckResult::fail(
            "App Data Directory",
            cat,
            "Cannot determine app data directory",
        ),
    }
}

fn dirs_next() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "macos")]
    {
        dirs::data_dir().map(|d| d.join("com.idsolutions.gi-healthcheck"))
    }
    #[cfg(target_os = "windows")]
    {
        dirs::data_local_dir().map(|d| d.join("GI Health Check"))
    }
    #[cfg(target_os = "linux")]
    {
        dirs::data_dir().map(|d| d.join("gi-healthcheck"))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        None
    }
}
