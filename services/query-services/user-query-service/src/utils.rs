use postgres::{Connection, TlsMode};

pub fn get_event_store_db_connection() -> Result<Connection, postgres::Error> {
	Connection::connect(
		crate::SECRETS
			.get("event_store_connection_string")
			.map_or("".to_owned(), |s| s.to_owned()),
		TlsMode::None,
	)
}

pub fn get_user_query_db_connection() -> Result<Connection, postgres::Error> {
	Connection::connect(
		crate::SECRETS
			.get("user_query_connection_string")
			.map_or("".to_owned(), |s| s.to_owned()),
		TlsMode::None,
	)
}
