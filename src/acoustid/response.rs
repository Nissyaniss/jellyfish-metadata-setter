use serde::Deserialize;

use crate::acoustid::result::AcoustidResult;

#[derive(Debug, Deserialize)]
pub struct AcoustidResponse {
	pub results: Vec<AcoustidResult>,
	pub status: String,
}
