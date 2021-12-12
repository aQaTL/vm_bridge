extern crate vm_bridge;

use anyhow::{anyhow, bail};
use pico_args::Arguments as PicoArgs;

use vm_bridge::config::{Config, VmAppsMap};
use vm_bridge::models;

const GET_APPS: &str = "get_apps";
const OPEN_APP: &str = "open_app";

struct Args {
	cmd: Command,
}

enum Command {
	GetApps,
	OpenApp { app_name: String },
}

fn parse_args() -> anyhow::Result<Args> {
	let mut args: PicoArgs = PicoArgs::from_env();
	let cmd = args
		.subcommand()?
		.ok_or_else(|| anyhow!("expected a command"))?;
	let cmd = match cmd.as_str() {
		"get-apps" => Command::GetApps,
		"open-app" => Command::OpenApp {
			app_name: args.free_from_str()?,
		},
		_ => bail!("unknown command"),
	};

	Ok(Args { cmd })
}

fn main() -> anyhow::Result<()> {
	let args = parse_args()?;
	let config = Config::load()?;

	match args.cmd {
		Command::GetApps => {
			let resp = ureq::get(&address_for(&config, GET_APPS)).call()?;
			let apps: VmAppsMap = resp.into_json()?;
			println!("{}", "=".repeat(80));
			println!("| {:<25}| {:<50}|", "App name", "Path");
			println!("{}", "=".repeat(80));
			for (name, path) in apps {
				println!("| {:<25}| {:<50}|", name, path.display());
			}
			println!("{}", "=".repeat(80));
		}
		Command::OpenApp { app_name } => {
			let open_app_payload = serde_json::to_value(models::OpenApp { app: app_name })?;
			let response =
				ureq::post(&address_for(&config, OPEN_APP)).send_json(open_app_payload)?;
			println!("{}", response.into_string()?);
		}
	}

	Ok(())
}

fn address_for(config: &Config, url: &'static str) -> String {
	format!("http://{}:{}/{}", config.vm_ip, config.port, url)
}
