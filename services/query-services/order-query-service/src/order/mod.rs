use chrono::prelude::*;
use serde::Serialize;

pub mod get;

//#region Order

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
	id: String,
	customer_id: String,
	date: Option<NaiveDate>,
	frame_color: Option<String>,
	frame_id: String,
	owner_id: String,
	quantity: i32,
	total: f32,
	processed: bool,
	rejected: bool,
}

//#endregion
