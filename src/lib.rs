use std::fs;

use anyhow::{Context, Result};
use serde::{Deserialize, ser::Error};

pub mod log;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

pub fn get_server_config() -> Result<ServerConfig> {
    let contents = fs::read_to_string("config.toml").context("Server config couldn't be read")?;
    let config: ServerConfig =
        toml::from_str(&contents).context("Server config could not be deserialized")?;
    Ok(config)
}
