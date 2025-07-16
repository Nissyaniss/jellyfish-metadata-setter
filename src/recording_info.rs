use std::{
	fs::{self, File, create_dir, rename},
	path::{Path, PathBuf},
};

use core::fmt::Write;
use mp4ameta::Tag;
use musicbrainz_rs::{
	Fetch,
	entity::{artist::Artist, recording::Recording, release::Release},
};
use serde_json::{Value, from_str};
use std::io::Write as IoWrite;

use crate::{
	acoustid::response::AcoustidResponse, deezer::track::Track, handle_error::HandleError,
	utils::inquire_number, wikipedia_response::WikipediaResponse,
};

#[derive(Debug, Clone)]
pub struct RecordingInfo {
	pub recording: Recording,
	pub artist: Artist,
	pub album: Release,
}

impl RecordingInfo {
	pub async fn from_query(query: String) -> Vec<Self> {
		let acoustid_response = from_str::<AcoustidResponse>(&query)
			.handle_case("Error while trying to convert Json to AcoustidResponse", 1);
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
						.handle_case("Error while fetching recording", 1);
					let artist = Artist::fetch()
						.id(&recording
							.artist_credit
							.clone()
							.handle_case("Error while trying to get artist credit", 1)[0]
							.artist
							.id)
						.with_annotations()
						.execute()
						.await
						.handle_case("Error while fetching artist", 1);

					let album = Release::fetch()
						.id(&recording
							.releases
							.clone()
							.handle_case("Error while trying to get the release", 1)[0]
							.id)
						.with_recordings()
						.execute()
						.await
						.handle_case("Error while fetching album", 1);
					recording_info_list.push(Self {
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

	pub async fn gather_information(&self, file_path: PathBuf, duration: &String) {
		let artist = self.artist.clone();
		let album = self.album.clone();
		let recording = self.recording.clone();
		let mut recording_disc_number = 0;

		for media in album
			.media
			.clone()
			.handle_case("Error no release for this recording", 1)
		{
			for track in media
				.tracks
				.handle_case("Error no tracks for this release", 1)
			{
				if track.title == recording.title {
					recording_disc_number = track.position;
				}
			}
		}

		let renamed_file = &format!("{} - {}.m4a", recording_disc_number, recording.title);
		let _ = rename(file_path, renamed_file);

		let artist_path = Path::new(&artist.name);
		let album_path = artist_path.join(Path::new(&album.title));

		let (is_new_artist, is_new_album, has_description, description) =
			check_metadata(renamed_file, self.clone(), artist_path, &album_path).await;

		if is_new_album {
			get_cover(self.clone(), &album_path, artist_path).await;
		}

		let album_path_str = album_path
			.to_str()
			.handle_case("Error while trying to convert album_path to &str", 1);

		println!("Moving music to {album_path_str}");
		let _ = rename(renamed_file, format!("{album_path_str}/{renamed_file}"));

		if is_new_artist {
			generate_artist_nfo(self.clone(), artist_path, has_description, &description);
		}

		if is_new_album {
			generate_album_nfo(self.clone(), &album_path, recording_disc_number, duration);
		}
	}
}

fn generate_album_nfo(
	recording_info: RecordingInfo,
	album_path: &Path,
	recording_disc_number: u32,
	duration: &String,
) {
	let artist = recording_info.artist;
	let album = recording_info.album;
	let recording = recording_info.recording;
	println!("Creating nfo file for album {}.", album.title);
	let mut album_nfo = String::new();

	let _ = writeln!(
		album_nfo,
		"<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>"
	);
	let _ = writeln!(album_nfo, "<album>");
	let _ = writeln!(album_nfo, "\t<title>{}</title>", album.title);
	let _ = writeln!(
		album_nfo,
		"\t<musicbrainzalbumid>{}</musicbrainzalbumid>",
		album.id
	);
	let _ = writeln!(
		album_nfo,
		"\t<musicbrainzalbumartistid>{}</musicbrainzalbumartistid>",
		artist.id
	);
	let _ = writeln!(album_nfo, "\t<art>");
	let _ = writeln!(
		album_nfo,
		"\t\t<poster>{}\\folder.png</poster>",
		fs::canonicalize(album_path).unwrap().to_str().unwrap()
	);
	let _ = writeln!(album_nfo, "\t</art>");
	let _ = writeln!(album_nfo, "\t<track>");
	let _ = writeln!(album_nfo, "\t\t<title>{}</title>", recording.title);
	let _ = writeln!(
		album_nfo,
		"\t\t<position>{recording_disc_number}</position>"
	);
	let _ = writeln!(album_nfo, "\t\t<duration>{duration}</duration>");
	let _ = writeln!(album_nfo, "\t</track>");
	let _ = writeln!(album_nfo, "</album>");

	let mut nfo_file = File::create(album_path.join(Path::new("album.nfo"))).unwrap();

	let _ = write!(nfo_file, "{album_nfo}");
}

fn generate_artist_nfo(
	recording_info: RecordingInfo,
	artist_path: &Path,
	has_description: bool,
	description: &String,
) {
	let artist = recording_info.artist;
	let album = recording_info.album;
	println!("Creating nfo file for artist {}.", artist.name);
	let mut artist_nfo = String::new();

	let _ = writeln!(
		artist_nfo,
		"<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>"
	);
	let _ = writeln!(artist_nfo, "<artist>");
	if has_description {
		let _ = writeln!(artist_nfo, "\t<biography>WIP</biography>"); //WIP
	}
	let _ = writeln!(artist_nfo, "\t<biography>{description}</biography>"); //WIP
	let _ = writeln!(artist_nfo, "\t<title>{}</title>", artist.name);
	let _ = writeln!(
		artist_nfo,
		"\t<musicbrainzartistid>{}</musicbrainzartistid>",
		artist.id
	);
	let _ = writeln!(artist_nfo, "\t<art>");
	let _ = writeln!(
		artist_nfo,
		"\t\t<poster>{}\\folder.png</poster>",
		fs::canonicalize(artist_path).unwrap().to_str().unwrap()
	);
	let _ = writeln!(artist_nfo, "\t</art>");
	let _ = writeln!(artist_nfo, "\t<album>");
	let _ = writeln!(artist_nfo, "\t\t<title>{}</title>", album.title);
	let _ = writeln!(artist_nfo, "\t\t<year>{}</year>", album.date.unwrap().0);
	let _ = writeln!(artist_nfo, "\t</album>");
	let _ = writeln!(artist_nfo, "</artist>");

	let mut nfo_file = File::create(artist_path.join(Path::new("artist.nfo"))).unwrap();

	let _ = write!(nfo_file, "{artist_nfo}");
}

async fn get_cover(recording_info: RecordingInfo, album_path: &Path, artist_path: &Path) {
	let artist = recording_info.artist;
	let recording = recording_info.recording;
	let tracks = Track::search_track(artist.name.clone(), recording.title.clone()).await;
	let mut album_covers = vec![];
	let mut artist_covers = vec![];

	for track in tracks {
		if track.title == "Believer" && track.artist.name == "Imagine Dragons" {
			album_covers.push(track.album.cover_xl);
			artist_covers.push(track.artist.picture_xl);
		}
	}
	let correct_track = if album_covers.len() != 1 && !album_covers.is_empty() {
		let mut album_covers_number = 1;
		for cover in album_covers.clone() {
			println!("{album_covers_number} - {cover}");
			album_covers_number += 1;
		}

		album_covers_number -= 1;

		let help_message = format!("Number between 1-{album_covers_number}");

		#[allow(clippy::option_if_let_else)]
		let cover_parser = &|i: &str| match i.parse::<usize>() {
			Ok(val) => {
				if val > album_covers_number {
					Err(())
				} else {
					Ok(val)
				}
			}
			Err(_) => Err(()),
		};

		let select_cover = inquire_number(album_covers_number, help_message.as_str(), cover_parser);

		select_cover.prompt().unwrap() - 1
	} else {
		1
	};

	println!("Selected : {}", album_covers[correct_track]);

	let album_cover_bytes = reqwest::get(album_covers[correct_track].clone())
		.await
		.unwrap()
		.bytes()
		.await
		.unwrap();

	let artist_cover_bytes = reqwest::get(artist_covers[correct_track].clone())
		.await
		.unwrap()
		.bytes()
		.await
		.unwrap();

	let album_cover_image = image::load_from_memory(&album_cover_bytes).unwrap();
	let artist_cover_image = image::load_from_memory(&artist_cover_bytes).unwrap();
	let _ = album_cover_image.save(format!("{}/folder.png", album_path.to_str().unwrap()));
	let _ = artist_cover_image.save(format!("{}/folder.png", artist_path.to_str().unwrap()));
}

async fn check_metadata(
	renamed_file: &String,
	recording_info: RecordingInfo,
	artist_path: &Path,
	album_path: &Path,
) -> (bool, bool, bool, String) {
	let recording = recording_info.recording;
	let artist = recording_info.artist;
	let album = recording_info.album;
	let mut is_new_artist = false;
	let mut is_new_album = false;
	let mut tag = Tag::read_from_path(renamed_file).unwrap();
	let mut descritpion = WikipediaResponse::default();

	println!("Is there a title in metadata ? : {}", tag.title().is_some());
	if tag.title().is_none() {
		println!("Applying found title: {}...", recording.title);
		tag.set_title(recording.title.clone());
	}

	println!(
		"Is there an artist in metadata ? : {}",
		tag.artist().is_some()
	);
	if tag.artist().is_none() {
		println!("Applying found artist : {}...", artist.name);
		tag.set_artist(artist.name.clone());
	}

	println!(
		"Is there an album in metadata ? : {}",
		tag.album().is_some()
	);
	if tag.album().is_none() {
		println!("Applying found album : {}...", album.title);
		tag.set_album(album.clone().title);
	}

	println!("Is there a year in metadata ? : {}", tag.year().is_some());
	if tag.year().is_none() {
		let year = recording.first_release_date.unwrap().0;
		println!("Applying found year : {}...", &year);
		tag.set_year(year);
	}

	println!(
		"Checking if {} is already in jellyfin: {}",
		artist.name,
		artist_path.exists()
	);
	if !artist_path.exists() {
		is_new_artist = true; // For finding cover of artist
		create_dir(artist.name.clone()).unwrap();
	}

	println!(
		"Checking if album {} is already in jellyfin: {}",
		album.title,
		album_path.exists()
	);
	if !album_path.exists() {
		is_new_album = true;
		create_dir(album_path).unwrap();
	}

	if is_new_artist {
		println!("Checking if {} as a Wikipedia description.", artist.name);
		let body = reqwest::get(format!(
			"https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&titles={}&format=json",
			artist.name
		)).await.unwrap().text().await.unwrap();
		let res: Value = from_str(&body).unwrap();
		if let Some(page) = res["query"]["pages"].as_object() {
			if let Some((random_key, _)) = page.iter().next() {
				descritpion =
					from_str::<WikipediaResponse>(&res["query"]["pages"][random_key].to_string())
						.unwrap();
			}
		}

		if descritpion.missing.is_none() {
			println!("Description found.");
		} else {
			println!("{} has no description.", artist.name);
		}
	}

	println!("Clearing comments...");
	tag.set_comment("");
	tag.write_to_path(renamed_file).unwrap();
	if descritpion.missing.is_none() {
		(
			is_new_artist,
			is_new_album,
			descritpion.missing.is_none(),
			descritpion.extract.unwrap(),
		)
	} else {
		(
			is_new_artist,
			is_new_album,
			descritpion.missing.is_none(),
			String::new(),
		)
	}
}
