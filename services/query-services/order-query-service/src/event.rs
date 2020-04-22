use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderPlacedData {
	pub customer_id: String,
	pub date: Option<NaiveDate>,
	pub frame_color: Option<String>,
	pub frame_id: String,
	pub owner_id: String,
	pub quantity: i32,
	pub total: f32,
}
