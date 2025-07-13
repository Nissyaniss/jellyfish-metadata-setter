use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MusicBrainzId {
	pub id: String,
}
