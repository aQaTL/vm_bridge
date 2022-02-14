use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use thiserror::Error;

const CONFIG_PATH: &str = "config.toml";
const CONFIG_TEMPLATE_PATH: &str = "config.toml.template";

#[derive(Debug, Deserialize)]
pub struct Config {
	pub host_ip: IpAddr,
	pub vm_ip: IpAddr,
	pub port: u16,
	pub env: EnvMap,
	pub vm_apps: VmAppsMap,
}

pub type EnvMap = HashMap<String, String>;
pub type VmAppsMap = HashMap<String, PathBuf>;

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
		let config: String = fs::read_to_string(&config_path)?;
		let config: Config = toml::from_str(&config)?;
		println!("Config path: \"{}\"", config_path.display());
		Ok(config)
	}
}

fn config_path() -> PathBuf {
	#[cfg(not(unix))]
	if let Some(file_path) = std::env::args().next() {
		let mut path = PathBuf::from(file_path);
		path.pop(); // Pop the binary name
		path.push(CONFIG_PATH);

		if path.exists() {
			return path;
		}
	}
	#[cfg(unix)]
	if let Ok(mut path) = std::fs::read_link("/proc/self/exe") {
		path.pop(); // Pop the binary name
		path.push(CONFIG_PATH);

		if path.exists() {
			return path;
		}
	}

	let path_in_cwd = Path::new(CONFIG_PATH);
	if path_in_cwd.exists() {
		return path_in_cwd.to_owned();
	}

	let template_path = PathBuf::from(CONFIG_TEMPLATE_PATH);
	template_path
}
