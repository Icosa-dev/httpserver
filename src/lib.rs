use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

pub mod log;

/// Configuration data for the server.
#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

impl ServerConfig {
    /// Returns both the ip and port
    /// of the server in the format
    /// of `{ip}:{port}``
    pub fn get_socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

/// Reads and parses the `config.toml` file for
/// the web server instance.
pub fn get_server_config() -> Result<ServerConfig> {
    let contents = fs::read_to_string("config.toml").context("Server config couldn't be read")?;
    let config: ServerConfig =
        toml::from_str(&contents).context("Server config could not be deserialized")?;
    Ok(config)
}
