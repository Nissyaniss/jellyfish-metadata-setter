use std::{env, fmt::Display, io::Write, path::PathBuf, process::Stdio, time::Duration};

use inquire::Text;
use serde_json::from_str;
use tokio::{
	io::{AsyncBufReadExt, BufReader},
	process::Command,
	select,
	time::interval,
};

use crate::{
	fpcalc_result::FpcalcResult, handle_error::HandleError, recording_info::RecordingInfo,
	utils::inquire_number,
};

extern crate dotenv;

#[derive(Debug, Clone)]
pub struct Video {
	pub title: String,
	pub yt_id: String,
	pub duration: String,
	pub file_path: Option<PathBuf>,
}

impl Video {
	pub async fn download(&mut self) {
		println!("Downloading the video...");
		println!("This may take a while");

		let _ = Command::new("./yt-dlp.exe")
			.args([
				self.yt_id.clone(),
				"-x".to_string(),
				"--audio-format".to_string(),
				"m4a".to_string(),
				"-o".to_string(),
				"%(title)s.%(ext)s".to_string(),
			])
			.spawn()
			.handle_case("Error trying to invoque yt-dlp", 1)
			.wait()
			.await
			.handle_case("Error trying to execute yt-dlp", 1);

		println!("Video Downloaded!");
		self.file_path = Some(PathBuf::from(format!("{}.m4a", self.title)));
	}

	pub async fn search() -> Self {
		let music_name = Text::new("What music do you want ?")
			.with_help_message(
				"Please use this template for better results : <artist> <song_name> audio",
			)
			.prompt()
			.handle_case("Error while getting your response", 1);

		println!("The search may take some time be patient.");

		let mut ytb_search_output = Command::new("./yt-dlp.exe")
			.args([
				format!("ytsearch10:\"{music_name}\""),
				"--get-id".to_string(),
				"--get-title".to_string(),
				"--get-duration".to_string(),
			])
			.stdout(Stdio::piped())
			.stderr(Stdio::null())
			.spawn()
			.handle_case("Error trying to execute yt-dlp", 1);

		let stdout = ytb_search_output
			.stdout
			.take()
			.handle_case("Error trying to take stdout from yt-dlp", 1);

		let mut ticker = interval(Duration::from_millis(100));
		let mut stdout_reader = BufReader::new(stdout);
		let mut buffer = Vec::new();
		let mut time_passed = 0.0;
		let mut line_counter = 0;
		let mut lines: Vec<String> = Vec::new();
		let mut video_list: Vec<Self> = Vec::new();

		loop {
			select! {
				_ = ticker.tick() => {
					print!("\rSearching for {time_passed:.1}s... (Usually around 30s)");
					let _ = std::io::stdout().flush();
					time_passed += 0.1;
				}

				maybe_line = stdout_reader.read_until(b'\n', &mut buffer) => {
					#[allow(clippy::single_match_else)]
					match maybe_line {
						Ok(_) => {
							if line_counter == 3 {
								if lines[0].is_empty() {
									println!();
									break;
								}
								let video = Self {
									title: lines[0].trim().to_string(),
									yt_id: lines[1].trim().to_string(),
									duration: lines[2].trim().to_string(),
									file_path: None
								};
								video_list.push(video);
								lines.clear();
								line_counter = 1;
							} else {
								line_counter += 1;
							}
							let line = String::from_utf8_lossy(&buffer);
							lines.push(format!("{line}"));
							buffer.clear();
						}
						Err(_) => {
							println!();
							break;
						}
					}
				}
				_ = ytb_search_output.wait() => {
					println!();
					break;
				}
			}
		}

		for (i, video) in video_list.iter().enumerate() {
			println!("{} - {video}", i + 1);
		}

		let video_list_len = video_list.len();

		#[allow(clippy::option_if_let_else)]
		let video_select_parser = &|i: &str| match i.parse::<usize>() {
			Ok(val) => {
				if val > video_list_len {
					Err(())
				} else {
					Ok(val)
				}
			}
			Err(_) => Err(()),
		};

		let help_message = format!("Le nombre doit etre entre 1-{video_list_len}");

		let select_video =
			inquire_number(video_list_len, help_message.as_str(), video_select_parser).prompt();

		video_list[select_video.handle_case("Error while getting your response", 1) - 1].clone()
	}

	pub async fn to_recording_info(self) -> RecordingInfo {
		dotenv::dotenv().ok();

		let file_path = self
			.file_path
			.handle_case("file_path not set", 1)
			.to_str()
			.handle_case("Unable to convert file_path to &str", 1)
			.to_string();

		let fpcalc_output = Command::new("./fpcalc.exe")
			.args(["-json", file_path.as_str()])
			.stdout(Stdio::piped())
			.spawn()
			.handle_case("Error while trying to invoque fpcalc", 1)
			.wait_with_output()
			.await
			.handle_case("Error while execution of fpacal", 1);

		let mut api_key = String::new();
		for (key, value) in env::vars() {
			if key == "ACOUSTID_API_KEY" {
				api_key = value;
			}
		}

		let fpcal_result = from_str::<FpcalcResult>(
			&String::from_utf8(fpcalc_output.stdout)
				.handle_case("Error while trying to convert fpcalc result to Utf8", 1),
		)
		.handle_case("Error while trying to convert fpcalc result to Json", 1);
		let request = format!(
			"https://api.acoustid.org/v2/lookup?client={api_key}&duration={}&fingerprint={}&meta=recordingids",
			fpcal_result.duration.round(),
			fpcal_result.fingerprint
		);
		let body = reqwest::get(request)
			.await
			.handle_case("Error while trying to communicate with Acoutid", 1)
			.text()
			.await
			.handle_case(
				"Error while trying to convert the Acoutid response to String",
				1,
			);

		let recording_info_list = RecordingInfo::from_query(body).await;
		let recording_number = recording_info_list.len();

		let help_message = format!("Number between 1-{recording_number}");

		#[allow(clippy::option_if_let_else)]
		let recording_parser = &|i: &str| match i.parse::<usize>() {
			Ok(val) => {
				if val > recording_number {
					Err(())
				} else {
					Ok(val)
				}
			}
			Err(_) => Err(()),
		};

		let select_recording =
			inquire_number(recording_number, help_message.as_str(), recording_parser);

		let res = select_recording
			.prompt()
			.handle_case("Error while trying to get your response", 1)
			- 1;

		let recording_info = &recording_info_list[res];

		println!(
			"Selected : {} - {} by {} from the {} album",
			res + 1,
			recording_info.recording.title,
			recording_info.artist.name,
			recording_info.album.title
		);

		recording_info.clone()
	}
}

impl Display for Video {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {}. Youtube link = https://www.youtube.com/watch?v={}",
			self.title, self.duration, self.yt_id
		)
	}
}
