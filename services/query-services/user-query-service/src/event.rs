use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisteredData {
	pub name: String,
	pub start_date: Option<NaiveDate>,
	pub address: Option<String>,
	pub phone_number: Option<String>,
	pub username: String,
	pub password: String,
	pub email: String,
	pub photo: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserModifiedData {
	pub name: String,
	pub start_date: Option<DateTime<Utc>>,
	pub address: Option<String>,
	pub phone_number: Option<String>,
	pub username: String,
	pub email: String,
	pub photo: Option<String>,
}
