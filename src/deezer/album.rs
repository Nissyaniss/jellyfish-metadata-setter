use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Album {
	pub id: u32,
	pub title: String,
	pub link: Option<String>,
	pub cover: String,
	pub cover_small: String,
	pub cover_medium: String,
	pub cover_big: String,
	pub cover_xl: String,
	pub md5_image: String,
	pub tracklist: String,
	pub r#type: String,
}
