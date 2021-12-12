extern crate vm_bridge;

use anyhow::bail;
use std::env;

use vm_bridge::config::Config;
use vm_bridge::models;

const OPEN_URL: &str = "open_url";

fn main() -> anyhow::Result<()> {
	let config = Config::load()?;

	let args: Vec<_> = env::args().skip(1).collect();
	if args.is_empty() {
		bail!("No arguments provided");
	}

	let open_url_payload = models::OpenUrl {
		url: args[0].clone(),
	};
	let open_url_payload_json = serde_json::to_value(open_url_payload)?;
	let response = ureq::post(&address_for(&config, OPEN_URL)).send_json(open_url_payload_json)?;

	println!("Response: {:#?}", response);

	Ok(())
}

fn address_for(config: &Config, url: &'static str) -> String {
	format!("http://{}:{}/{}", config.host_ip, config.port, url)
}
