use chrono::prelude::*;
use regex::Regex;

mod register;
pub use register::register;
pub use register::UserRegisteredData;

//#region User

#[derive(Debug)]
pub struct User {
	id: String,
	name: String,
	start_date: Option<DateTime<Utc>>,
	address: Option<String>,
	phone_number: Option<String>,
	username: String,
	password: String,
	email: String,
	photo: Option<String>,
}

impl User {
	fn get_field_regex_pattern(field_name: &str) -> Option<&'static str> {
		match field_name {
			"name" => Some(r"^\w{2,50}$"),
			"address" => Some(r"^\w{1,500}$"),
			"phone_number" => Some(r"^[\d+]{4,20}$"),
			"username" => Some(r"^\w{1,20}$"),
			"email" => Some(r"^\w+@\w+\.\w{2,}$"),
			_ => None,
		}
	}

	fn validate_field_str(field_name: &str, value: &str) -> Result<(), String> {
		let pattern = Self::get_field_regex_pattern(field_name);

		if let Some(pattern) = pattern {
			if !Regex::new(pattern).unwrap().is_match(value) {
				return Err(format!(
					"Field '{}' is invalid. Expected pattern: {}",
					field_name, pattern
				));
			}
		}

		Ok(())
	}
}

//#endregion
