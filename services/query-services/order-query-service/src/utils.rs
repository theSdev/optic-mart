use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub id: String,
	pub exp: usize,
	pub sub: String,
}

pub fn get_event_store_db_connection() -> Result<Client, postgres::Error> {
	Client::connect(
		crate::SECRETS
			.get("event_store_connection_string")
			.map_or(&"", |s| &s),
		NoTls,
	)
}

pub fn get_order_query_db_connection() -> Result<Client, postgres::Error> {
	Client::connect(
		crate::SECRETS
			.get("order_query_connection_string")
			.map_or(&"", |s| &s),
		NoTls,
	)
}
