use std::net::SocketAddr;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub host: [u8; 4],
    pub port: u16,
    pub static_dir: &'static str,
    pub content_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: [0, 0, 0, 0], // Bind to all interfaces
            port: 5002,
            static_dir: "static",
            content_dir: PathBuf::from("content"),
        }
    }
}

impl Config {
    /// Get the socket address for the server
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }

    /// Check if content directory exists
    pub fn content_dir_exists(&self) -> bool {
        self.content_dir.exists()
    }
}
