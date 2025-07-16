use core::fmt::Display;
use std::process::exit;

pub trait HandleError<T> {
	fn handle_case(self, error_message: &str, return_code: i32) -> T;
}

impl<T> HandleError<T> for Option<T> {
	fn handle_case(self, error_message: &str, return_code: i32) -> T {
		#[allow(clippy::single_match_else, clippy::option_if_let_else)]
		match self {
			Some(value) => value,
			None => {
				println!("{error_message}");
				exit(return_code);
			}
		}
	}
}

impl<T, E: Display> HandleError<T> for Result<T, E> {
	fn handle_case(self, error_message: &str, return_code: i32) -> T {
		match self {
			Ok(value) => value,
			Err(error) => {
				println!("{error_message} : {error}");
				exit(return_code);
			}
		}
	}
}
