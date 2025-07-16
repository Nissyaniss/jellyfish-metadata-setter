pub mod acoustid;
pub mod deezer;
pub mod fpcalc_result;
pub mod handle_error;
pub mod musicbrainz_id;
pub mod recording_info;
pub mod utils;
pub mod video;
pub mod wikipedia_response;

use anyhow::Result;
use handle_error::HandleError;
use video::Video;

#[tokio::main]
async fn main() -> Result<()> {
	let mut video = Video::search().await;

	video.download().await;
	let recording_info = video.clone().to_recording_info().await;

	recording_info
		.gather_information(
			video.file_path.handle_case("file_path not set", 1),
			&video.duration,
		)
		.await;

	Ok(())
}
