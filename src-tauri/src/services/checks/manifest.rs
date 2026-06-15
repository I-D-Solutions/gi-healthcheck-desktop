use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct DeviceManifest {
    pub iff: IffConfig,
    pub gps: GpsConfig,
    pub link16: Link16Config,
    pub secure_link: SecureLinkConfig,
    #[serde(default)]
    pub extra_connected_devices: Vec<ExtraConnectedDeviceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct IffConfig {
    pub serial_port: String,
    pub baud_rate: u32,
    pub process_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GpsConfig {
    pub serial_port: String,
    pub baud_rate: u32,
    pub nmea_required: bool,
    pub expected_ip: Option<String>,
    pub process_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Link16Config {
    pub serial_port: String,
    pub baud_rate: u32,
    pub tcp_host: String,
    pub tcp_port: u16,
    pub process_name: String,
}

#[derive(Debug, Deserialize)]
pub struct SecureLinkConfig {
    pub process_name: String,
    pub expected_interface: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtraConnectedDeviceConfig {
    pub name: String,
    pub kind: String,
    pub expected_ip: Option<String>,
    pub ports: Option<Vec<u16>>,
    pub baud_rate: Option<u32>,
    pub required: Option<bool>,
    pub notes: Option<String>,
}

impl DeviceManifest {
    pub fn load() -> Result<Self, String> {
        let path = manifest_path();
        let contents = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read manifest at {}: {}", path.display(), e))?;
        toml::from_str(&contents).map_err(|e| format!("Failed to parse manifest: {}", e))
    }
}

fn manifest_path() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    // In dev mode, the config lives next to the source tree
    let candidates = [
        exe_dir
            .as_ref()
            .map(|d| d.join("config/expected-devices.toml")),
        Some(PathBuf::from("config/expected-devices.toml")),
        Some(PathBuf::from("src-tauri/config/expected-devices.toml")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from("config/expected-devices.toml")
}
