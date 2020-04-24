use super::User;
use crate::utils;
use actix_web::{error, web, HttpResponse, Responder};

pub fn get(path: web::Path<(String,)>) -> Result<HttpResponse, actix_web::Error> {
	let entity_id = &path.0;
	dbg!(&entity_id);

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
				WHERE entity_id = $1"#,
			&[&entity_id],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	let stored_user = user_rows
		.into_iter()
		.next()
		.ok_or(error::ErrorBadRequest("User not found."))?;

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

	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(serde_json::to_string(&user).unwrap()))
}

pub fn get_all() -> impl Responder {
	let users = ["me", "you", "others"];
	HttpResponse::Ok().body(format!("{:#?}", users))
}
