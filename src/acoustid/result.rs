use serde::Deserialize;

use crate::musicbrainz_id::MusicBrainzId;

#[derive(Debug, Deserialize)]
pub struct AcoustidResult {
	pub id: String,
	pub recordings: Vec<MusicBrainzId>,
	pub score: f64,
}
