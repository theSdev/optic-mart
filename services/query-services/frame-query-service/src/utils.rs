use postgres::{Client, NoTls};

pub fn get_event_store_db_connection() -> Result<Client, postgres::Error> {
	Client::connect(
		crate::SECRETS
			.get("event_store_connection_string")
			.map_or(&"", |s| &s),
		NoTls,
	)
}

pub fn get_frame_query_db_connection() -> Result<Client, postgres::Error> {
	Client::connect(
		crate::SECRETS
			.get("frame_query_connection_string")
			.map_or(&"", |s| &s),
		NoTls,
	)
}
