extern crate iced;
extern crate vm_bridge;

use iced::window::{Icon, Mode};
use iced::{
	Application, Clipboard, Color, Column, Command, Container, Element, Radio, Row, Settings,
	Subscription,
};
use log::{error, info};
use std::path::PathBuf;
use vm_bridge::config::{Config, VmAppsMap};

mod commands;

fn main() -> anyhow::Result<()> {
	flexi_logger::Logger::try_with_env_or_str("vm_bridge_linux_client_gui=info")?.start()?;
	let config: &'static Config = Box::leak(Box::new(Config::load()?));

	let settings = iced::Settings {
		window: iced::window::Settings {
			size: (500, 500),
			min_size: None,
			max_size: None,
			resizable: true,
			decorations: true,
			transparent: false,
			always_on_top: false,
			icon: None,
		},
		flags: AppFlags { config },
		default_font: None,
		default_text_size: 20,
		exit_on_close_request: true,
		antialiasing: false,
	};
	<App as Application>::run(settings)?;
	Ok(())
}

struct AppFlags {
	config: &'static Config,
}

struct App {
	config: &'static Config,

	apps: VmAppsMap,
}

#[derive(Debug)]
enum Msg {
	GetAppsResult(Result<VmAppsMap, reqwest::Error>),
}

impl App {
	fn new(flags: AppFlags) -> Self {
		App {
			config: flags.config,
			apps: Default::default(),
		}
	}
}

impl iced::Application for App {
	type Executor = iced::executor::Default;
	type Message = Msg;
	type Flags = AppFlags;

	fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
		let config = flags.config;
		(
			App::new(flags),
			Command::perform(commands::get_apps(config), |res| Msg::GetAppsResult(res)),
		)
	}

	fn title(&self) -> String {
		String::from("VM Bridge")
	}

	fn update(
		&mut self,
		message: Self::Message,
		_clipboard: &mut Clipboard,
	) -> Command<Self::Message> {
		match message {
			Msg::GetAppsResult(Ok(vm_apps)) => {
				info!("Apps: {:#?}", vm_apps);
				self.apps = vm_apps;
			}
			Msg::GetAppsResult(Err(e)) => {
				error!("Failed to fetch apps: {}. Caused by {:?}", e, e);
			}
		}
		Command::none()
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		Subscription::none()
	}

	fn view(&mut self) -> Element<'_, Self::Message> {
		Column::with_children(vec![Row::with_children(vec![]).into()]).into()
	}

	fn mode(&self) -> Mode {
		Mode::Windowed
	}

	fn background_color(&self) -> Color {
		Color::from_rgb8(0, 0x2b, 0x36)
	}

	// fn scale_factor(&self) -> f64 {
	//     1.0
	// }
}
