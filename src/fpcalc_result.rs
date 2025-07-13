use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FpcalcResult {
	pub duration: f64,
	pub fingerprint: String,
}
