use serde::Deserialize;
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;
use thiserror::Error;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
	pub host_ip: IpAddr,
	pub vm_ip: IpAddr,
	pub port: u16,
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("{self:?}")]
	Io(#[from] std::io::Error),
	#[error("{self:?}")]
	Deserialize(#[from] toml::de::Error),
}

impl Config {
	pub fn load() -> Result<Self, Error> {
		let config_path = config_path();
		let config: String = fs::read_to_string(config_path)?;
		let config: Config = toml::from_str(&config)?;
		Ok(config)
	}
}

fn config_path() -> PathBuf {
	match std::env::args().next() {
		Some(file_path) => {
			let mut path = PathBuf::from(file_path);
			path.pop(); // Pop the binary name
			path.push(CONFIG_PATH);
			path
		}
		None => PathBuf::from(CONFIG_PATH),
	}
}
