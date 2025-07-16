use serde::Deserialize;
use serde_json::{Value, from_str};

use crate::handle_error::HandleError;

use super::{album::Album, artist::Artist};

#[derive(Debug, Deserialize)]
pub struct Track {
	pub id: u32,
	pub readable: bool,
	pub title: String,
	pub title_short: String,
	pub title_version: String,
	pub link: String,
	pub duration: u16,
	pub rank: u32,
	pub explicit_lyrics: bool,
	pub explicit_content_lyrics: u16,
	pub explicit_content_cover: u16,
	pub preview: String,
	pub md5_image: String,
	pub artist: Artist,
	pub album: Album,
	pub r#type: String,
}

#[allow(dead_code)]
const DEEZER_API: &str = "https://api.deezer.com";

impl Track {
	pub async fn search_track(artist: String, track: String) -> Vec<Self> {
		#[allow(clippy::manual_let_else, clippy::option_if_let_else)]
		let data = reqwest::get(format!(
			"{DEEZER_API}/search?q=artist:\"{artist}\" track:\"{track}\""
		))
		.await
		.handle_case("Error while trying to communicate to the Deezer api", 1)
		.text()
		.await
		.handle_case("Error while trying to conver deezer response to &str", 1);

		let value: Value = serde_json::from_str(&data)
			.handle_case("Error while trying to build Json from deezer response", 1);

		from_str::<Vec<Self>>(&value["data"].to_string())
			.handle_case("Error trying to convert Json to Track", 1)
	}
}
