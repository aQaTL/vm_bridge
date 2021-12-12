use vm_bridge::config::{Config, VmAppsMap};

const GET_APPS: &str = "get_apps";
const OPEN_APP: &str = "open_app";

pub async fn get_apps(config: &Config) -> Result<VmAppsMap, reqwest::Error> {
	reqwest::get(address_for(config, GET_APPS))
		.await?
		.json::<VmAppsMap>()
		.await
}

fn address_for(config: &Config, url: &'static str) -> String {
	format!("http://{}:{}/{}", config.vm_ip, config.port, url)
}
