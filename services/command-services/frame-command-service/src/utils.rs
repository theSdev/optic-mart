use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,
	pub exp: usize,
}

pub fn generate_uuid() -> String {
	format!("{}", Uuid::new_v4().to_hyphenated())
}

use actix_web::http::header::HeaderValue;
/// Set event_id to X-Correlation-ID header if exists or a generated uuid otherwise.
pub fn get_correlation_id(header: Option<&HeaderValue>) -> String {
	header.map_or_else(generate_uuid, |header| {
		header
			.to_str()
			.ok()
			.map_or_else(generate_uuid, |header_str| header_str.to_owned())
	})
}
