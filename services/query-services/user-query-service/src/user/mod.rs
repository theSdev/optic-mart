use chrono::prelude::*;
use regex::Regex;
use serde::Serialize;

pub mod get;
pub mod search;

//#region User

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	id: String,
	name: String,
	start_date: Option<NaiveDate>,
	address: Option<String>,
	phone_number: Option<String>,
	username: String,
	email: String,
	photo: Option<String>,
}

impl User {
	fn get_field_regex_pattern(field_name: &str) -> Option<&'static str> {
		match field_name {
			"name" => Some(r"^.{2,50}$"),
			"address" => Some(r"^.{1,500}$"),
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
					"Field '{}' is invalid. Expected pattern: {}.",
					field_name, pattern
				));
			}
		}

		Ok(())
	}
}

//#endregion
