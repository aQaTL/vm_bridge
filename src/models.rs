use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenUrl {
	pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenApp {
	pub app: String,
}
