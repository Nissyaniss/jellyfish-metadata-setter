use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Video {
	pub title: String,
	pub yt_id: String,
	pub duration: String,
}

impl Video {
	#[must_use]
	pub fn from_yt_dlp(input: &str) -> Vec<Self> {
		let lines: Vec<&str> = input.split('\n').collect();
		let mut res = vec![];
		let mut i = 0;
		while i < lines.len() {
			res.push(Self {
				title: lines[i].to_string(),
				yt_id: lines[i + 1].to_string(),
				duration: lines[i + 2].to_string(),
			});
			if i + 3 < lines.len() - 1 {
				i += 3;
			} else {
				break;
			}
		}
		res
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
