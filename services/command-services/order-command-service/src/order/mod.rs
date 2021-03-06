use chrono::prelude::*;

pub mod place;
pub mod mark;

//#region Order

#[derive(Debug)]
pub struct Order {
	id: String,
	customer_id: String,
	date: Date<Utc>,
	frame_color: Option<String>,
	frame_id: String,
	owner_id: String,
	quantity: u16,
	total: f32,
}

//#endregion
