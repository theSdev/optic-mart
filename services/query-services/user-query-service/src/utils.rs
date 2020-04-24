use postgres::{Client, NoTls};

pub fn get_event_store_db_connection() -> Result<Client, postgres::Error> {
	Client::connect(
		crate::SECRETS
			.get("event_store_connection_string")
			.map_or(&"", |s| &s),
		NoTls,
	)
}

pub fn get_user_query_db_connection() -> Result<Client, postgres::Error> {
	Client::connect(
		crate::SECRETS
			.get("user_query_connection_string")
			.map_or(&"", |s| &s),
		NoTls,
	)
}
