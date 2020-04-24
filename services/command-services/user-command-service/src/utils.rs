use postgres::{Connection, TlsMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub id: String,
	pub exp: usize,
	pub sub: String,
}

pub fn generate_uuid() -> String {
	format!("{}", Uuid::new_v4().to_hyphenated())
}

pub fn get_event_store_db_connection() -> Result<Connection, postgres::Error> {
	Connection::connect(
		crate::SECRETS
			.get("event_store_connection_string")
			.map_or("".to_owned(), |s| s.to_owned()),
		TlsMode::None,
	)
}

pub fn get_user_command_db_connection() -> Result<Connection, postgres::Error> {
	Connection::connect(
		crate::SECRETS
			.get("user_command_connection_string")
			.map_or("".to_owned(), |s| s.to_owned()),
		TlsMode::None,
	)
}
