use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::path::PathBuf;

/// Application configuration
///
/// Supported config file format (config.toml or config.conf, key=value, no sections):
///
/// host = "127.0.0.1"   # or "0.0.0.0"
/// port = 5002
/// static_dir = "static"
/// content_dir = "content"
/// base_url = "https://nornity.com"
///
/// Comments (# or //) and blank lines are ignored.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: [u8; 4],
    pub port: u16,
    pub static_dir: String,
    pub content_dir: PathBuf,
    pub base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: [0, 0, 0, 0], // Bind to all interfaces
            port: 5000,
            static_dir: "static".to_string(),
            content_dir: PathBuf::from("content"),
            base_url: "https://nornity.com".to_string(),
        }
    }
}

impl Config {
    /// Load config from file (config.toml or config.conf), env, or defaults
    pub fn from_file_or_env() -> Self {
        let mut config = Config::default();
        // Try config.toml, then config.conf
        let paths = ["config.toml", "config.conf"];
        for path in &paths {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                        continue;
                    }
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim().trim_matches('"');
                        match key {
                            "host" => {
                                if let Some(parsed) = parse_host(value) {
                                    config.host = parsed;
                                }
                            }
                            "port" => {
                                if let Ok(port) = value.parse() {
                                    config.port = port;
                                }
                            }
                            "static_dir" => {
                                config.static_dir = value.to_string();
                            }
                            "content_dir" => {
                                config.content_dir = PathBuf::from(value);
                            }
                            "base_url" => {
                                config.base_url = value.to_string();
                            }
                            _ => {}
                        }
                    }
                }
                break; // Stop after first found config file
            }
        }
        // Fallback to env if not set by file
        if let Ok(host) = std::env::var("HOST") {
            if let Some(parsed) = parse_host(&host) {
                config.host = parsed;
            }
        }
        if let Ok(port) = std::env::var("PORT") {
            if let Ok(port) = port.parse() {
                config.port = port;
            }
        }
        if let Ok(static_dir) = std::env::var("STATIC_DIR") {
            config.static_dir = static_dir;
        }
        if let Ok(content_dir) = std::env::var("CONTENT_DIR") {
            config.content_dir = PathBuf::from(content_dir);
        }
        if let Ok(base_url) = std::env::var("BASE_URL") {
            config.base_url = base_url;
        }
        config
    }

    /// Get the socket address for the server
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }

    /// Check if content directory exists
    pub fn content_dir_exists(&self) -> bool {
        self.content_dir.exists()
    }
}

/// Parse host string (e.g. "127.0.0.1" or "0.0.0.0") to [u8; 4]
fn parse_host(s: &str) -> Option<[u8; 4]> {
    let parts: Vec<_> = s.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    let mut arr = [0u8; 4];
    for (i, part) in parts.iter().enumerate() {
        if let Ok(val) = part.parse() {
            arr[i] = val;
        } else {
            return None;
        }
    }
    Some(arr)
}
