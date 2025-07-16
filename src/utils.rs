use inquire::{CustomType, ui::RenderConfig};

pub fn inquire_number<'a>(
	max: usize,
	help_message: &'a str,
	parser: &'a dyn Fn(&str) -> Result<usize, ()>,
) -> CustomType<'a, usize> {
	CustomType {
		message: "What cover is the correct one",
		starting_input: None,
		formatter: &|i| format!("${i}"),
		default_value_formatter: &|i| format!("${i}"),
		default: None,
		validators: vec![],
		placeholder: None,
		error_message: format!("Please type a valid number. (1-{max})"),
		help_message: Some(help_message),
		parser,
		render_config: RenderConfig::default(),
	}
}
