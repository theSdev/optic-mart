use regex::Regex;
use serde::Serialize;
use std::any::Any;

pub mod get;

//#region Frame

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
	brand_name: String,
	colors: Vec<String>,
	cover_image: Option<String>,
	description: Option<String>,
	id: String,
	has_case: bool,
	materials: Vec<String>,
	model_name: String,
	other_images: Vec<String>,
	owner_id: String,
	price: f32,
	privacy_mode: i16,
}

impl Frame {
	fn get_field_regex_pattern(field_name: &str) -> Option<&'static str> {
		match field_name {
			"brand_name" => Some(r"^.{2,50}$"),
			"color" => Some(r"^.{2,25}$"),
			"material" => Some(r"^.{2,25}$"),
			"model_name" => Some(r"^.{2,50}$"),
			"description" => Some(r"^.{0,500}$"),
			_ => None,
		}
	}

	fn validate_field(field_name: &str, value: Box<dyn Any>) -> Result<(), String> {
		match field_name {
			"price" => {
				if let Some(price) = value.downcast_ref::<f32>() {
					if *price >= 0.0 {
						return Ok(());
					}
					Err("Price has to be non-negative.")?
				}
				Err("Price has to be a number.")?
			}
			"privacy_mode" => {
				if let Some(privacy_mode) = value.downcast_ref::<u8>() {
					println!("{}", *privacy_mode);
					if *privacy_mode >= 1 && *privacy_mode <= 4 {
						return Ok(());
					}
					Err("Privacy Mode has to be between 1 and 3.")?
				}
				Err("Privacy Mode has to be a number.")?
			}
			_ => {
				if let Some(s) = value.downcast_ref::<String>() {
					Frame::validate_field_str(field_name, s)?;
					return Ok(());
				}
				Err("Cast error.")?
			}
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
