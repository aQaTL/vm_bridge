//! A server that's supposed to run on a linux VM, allowing a Windows host to open an app by using
//! a web request

extern crate actix_web;
extern crate vm_bridge;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::process::Command;
use vm_bridge::config::Config;

fn main() -> anyhow::Result<()> {
	flexi_logger::Logger::try_with_env_or_str("info")?.start()?;

	let config: &'static Config = Box::leak(Box::new(Config::load()?));
	println!("Config loaded.");
	println!("{:#?}", config);

	actix_web::rt::System::new("main").block_on(async move {
		HttpServer::new(move || {
			App::new()
				.wrap(Logger::default())
				.app_data(config)
				.service(get_apps_service)
				.service(open_app_service)
		})
		.bind((config.vm_ip, config.port))?
		.run()
		.await
	})?;
	Ok(())
}

#[get("/get_apps")]
async fn get_apps_service(req: HttpRequest) -> HttpResponse {
	let config: &&'static Config = req.app_data().unwrap();
	println!("hi");
	HttpResponse::Ok().json2(&config.vm_apps)
}

#[post("/open_app")]
async fn open_app_service(req: HttpRequest, app: web::Json<OpenApp>) -> impl Responder {
	let config: &&'static Config = req.app_data().unwrap();

	let app_path = match config.vm_apps.get(&app.app) {
		Some(v) => v,
		None => return None,
	};

	let child = Command::new(app_path).envs(&config.env).spawn();
	if child.is_ok() {
		Some(":)")
	} else {
		Some(":(")
	}
}

#[derive(Deserialize, serde::Serialize)]
struct OpenApp {
	app: String,
}

#[cfg(test)]
mod tests {
	use actix_web::{test, App};
	use vm_bridge::config::Config;

	#[actix_rt::test]
	async fn test_get_apps() {
		let config: &'static _ = Box::leak(Box::new(Config::load().unwrap()));

		let mut app =
			test::init_service(App::new().app_data(config).service(super::get_apps_service)).await;

		let req = test::TestRequest::get().uri("/get_apps").to_request();
		let resp = test::call_service(&mut app, req).await;
		println!("{:?}", resp);
		assert!(resp.status().is_success());
	}
}
