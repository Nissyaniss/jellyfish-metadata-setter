use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Artist {
	pub id: u32,
	pub name: String,
	pub link: String,
	pub picture: String,
	pub picture_small: String,
	pub picture_medium: String,
	pub picture_big: String,
	pub picture_xl: String,
	pub tracklist: String,
	pub r#type: String,
}
