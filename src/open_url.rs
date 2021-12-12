use serde::{Deserialize, Serialize};
use std::io;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct OpenUrl {
	pub url: String,
}

#[cfg(target_os = "linux")]
pub fn open_url(url: Url) -> io::Result<()> {
	use std::env;
	use std::process::Command;

	let app = match env::var("XDG_DESKTOP_SESSION").as_deref() {
		Ok("KDE") => "kde-open5",
		_ => "xdg-open",
	};

	let _output = Command::new(app).arg(url.as_str()).output()?;

	Ok(())
}

#[cfg(target_os = "windows")]
pub fn open_url(url: Url) -> io::Result<()> {
	use std::{mem, ptr};

	let url_utf16: Vec<u16> = url.as_str().encode_utf16().collect();

	let result = unsafe {
		let result = winapi::um::shellapi::ShellExecuteW(
			ptr::null_mut(),
			ptr::null(),
			url_utf16.as_ptr(),
			ptr::null(),
			ptr::null(),
			winapi::um::winuser::SW_SHOW,
		);
		mem::transmute::<_, isize>(result)
	};
	if result > 32 {
		Ok(())
	} else {
		Err(io::Error::last_os_error())
	}
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn open_url(_url: Url) -> io::Result<()> {
	unimplemented!()
}
