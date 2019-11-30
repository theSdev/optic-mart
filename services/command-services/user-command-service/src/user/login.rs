use crate::utils;
use actix_web::{error, web, HttpResponse};
use bcrypt::verify;
use chrono::Utc;
use serde::Serialize;
use utils::Claims;

//#region Event

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLoggedInData {
	token: String,
}

//#endregion

//#region Web

pub async fn login_async(
	path: web::Path<(String,)>,
	password: String,
) -> Result<HttpResponse, actix_web::Error> {
	let username = &path.0;

	// Hash password to compare with the stored one.
	let conn =
		utils::get_user_command_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	// Check username and password.
	let user_rows = &conn
		.query(
			r#"SELECT id, password FROM "user" WHERE username = $1"#,
			&[username],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;
	let stored_user = user_rows
		.into_iter()
		.next()
		.ok_or(error::ErrorBadRequest("User not found."))?;
	let stored_id = stored_user.get::<usize, String>(0);
	let stored_pass = stored_user.get::<usize, String>(1);

	verify(password, &stored_pass).map_err(|_e| error::ErrorBadRequest("User not found."))?;

	// Generate token.
	let claims = Claims {
		sub: username.clone(),
		exp: (Utc::now().timestamp() + (2 * 60 * 60)) as usize,
	};
	let jwt_key = crate::SECRETS
		.get("jwt_key")
		.ok_or(error::ErrorInternalServerError("Failed to get jwt_key"))?;
	let token = jwt::encode(&jwt::Header::default(), &claims, jwt_key.as_ref())
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Construct event.
	let event_data = &UserLoggedInData {
		token: token.clone(),
	};

	let event_store_conn =
		utils::get_event_store_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	// Store event.
	event_store_conn
		.execute(
			r#"INSERT INTO "user" (uuid, type, body) VALUES ($1, $2, $3)"#,
			&[
				&stored_id,
				&"UserLoggedIn",
				&serde_json::to_string(event_data)
					.map_err(|e| error::ErrorInternalServerError(e))?,
			],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	Ok(HttpResponse::Created().body(token))
}

pub fn login(
	path: web::Path<(String,)>,
	password: String,
) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(login_async(path, password))
}

//#endregion
