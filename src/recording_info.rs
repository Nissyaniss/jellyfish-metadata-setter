use musicbrainz_rs::{
	Fetch,
	entity::{artist::Artist, recording::Recording, release::Release},
};
use serde_json::from_str;

use crate::acoustid::response::AcoustidResponse;

#[derive(Debug, Clone)]
pub struct RecordingInfo {
	pub recording: Recording,
	pub artist: Artist,
	pub album: Release,
}

impl RecordingInfo {
	pub async fn from_query(query: String) -> Vec<Self> {
		let acoustid_response = from_str::<AcoustidResponse>(&query).unwrap();
		let mut recording_number = 1;
		let mut recording_info_list = vec![];
		if acoustid_response.results.len() != 1 {
			for result in &acoustid_response.results {
				for recording in &result.recordings {
					let recording = Recording::fetch()
						.id(&recording.id)
						.with_artists()
						.with_releases()
						.execute()
						.await
						.unwrap();
					let artist = Artist::fetch()
						.id(&recording.artist_credit.clone().unwrap()[0].artist.id)
						.with_annotations()
						.execute()
						.await
						.unwrap();
					let album = Release::fetch()
						.id(&recording.releases.clone().unwrap()[0].id)
						.with_recordings()
						.execute()
						.await
						.unwrap();
					recording_info_list.push(RecordingInfo {
						recording: recording.clone(),
						artist: artist.clone(),
						album: album.clone(),
					});
					println!(
						"{recording_number} - {} by {} from the {} album",
						recording.title, artist.name, album.title
					);
					recording_number += 1;
				}
			}
		}
		recording_info_list
	}
}
