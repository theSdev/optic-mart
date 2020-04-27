use super::User;
use crate::utils;
use actix_web::{error, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
	pub term: String,
}

pub fn search(info: web::Query<Info>) -> Result<HttpResponse, actix_web::Error> {
	let term = &info.term;
	dbg!(&term);

	let mut user_query_conn = utils::get_user_query_db_connection().unwrap();
	let user_rows = user_query_conn
		.query(
			r#"SELECT
				entity_id,
				name,
				start_date,
				address,
				phone_number,
				username,
				email,
				photo,
				updated_at
				FROM "user"
			WHERE name LIKE $1
				OR address LIKE $1
				OR phone_number LIKE $1
				OR username LIKE $1 "#,
			&[&term],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	let mut users: Vec<User> = vec![];

	for stored_user in user_rows {
		let user = User {
			id: stored_user.get(0),
			name: stored_user.get(1),
			start_date: stored_user.get(2),
			address: stored_user.get(3),
			phone_number: stored_user.get(4),
			username: stored_user.get(5),
			email: stored_user.get(6),
			photo: stored_user.get(7),
		};

		users.push(user);
	}

	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(serde_json::to_string(&users).unwrap()))
}
