use serde::{Deserialize, Serialize};
use std::env;
use std::io;
use std::process::Command;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct OpenUrl {
	pub url: String,
}

#[cfg(target_os = "linux")]
pub fn open_url(url: Url) -> io::Result<()> {
	let app = match env::var("XDG_DESKTOP_SESSION").as_deref() {
		Ok("KDE") => "kde-open5",
		_ => "xdg-open",
	};

	let _output = Command::new(app).arg(url.as_str()).output()?;

	Ok(())
}

#[cfg(target_os = "windows")]
pub fn open_url(url: Url) -> io::Result<()> {
	unimplemented!()
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn open_url(url: Url) -> io::Result<()> {
	unimplemented!()
}
