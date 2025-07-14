use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct WikipediaResponse {
	#[serde(rename(deserialize = "pageid"))]
	pub page_id: Option<u32>,
	pub ns: u32,
	pub title: String,
	pub missing: Option<String>,
	pub extract: Option<String>,
}
