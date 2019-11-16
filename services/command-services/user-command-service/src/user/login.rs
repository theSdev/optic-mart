use crate::event::{Event, ExpectedVersion};
use crate::utils;
use actix_web::{error, web, HttpRequest, HttpResponse};
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
	req: HttpRequest,
	path: web::Path<(String,)>,
	password: String,
) -> Result<HttpResponse, actix_web::Error> {
	let username = &path.0;

	// Hash password to compare with the stored one.
	let conn = utils::get_postgres_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	// Check username and password.
	let user_rows = &conn
		.query(
			r#"SELECT password FROM "user" WHERE username = $1"#,
			&[username],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;
	let stored_pass = user_rows
		.into_iter()
		.next()
		.ok_or(error::ErrorBadRequest("User not found."))?
		.get::<usize, String>(0);
	verify(password, &stored_pass).map_err(|_e| error::ErrorBadRequest("User not found."))?;

	// Generate token.
	let claims = Claims {
		sub: username.clone(),
		exp: (Utc::now().timestamp() + (2 * 60 * 60)) as usize,
	};
	let jwt_key = crate::SECRETS
		.get("connection_string")
		.ok_or(error::ErrorInternalServerError("Failed to get jwt_key"))?;
	let token = jwt::encode(&jwt::Header::default(), &claims, jwt_key.as_ref())
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Construct event.
	let event_data = UserLoggedInData { token };
	let event = Event::Login(event_data.clone());
	let event_id = utils::get_correlation_id(req.headers().get("X-Correlation-ID"));

	// Persist to Event Store.
	surf::post(format!("http://0.0.0.0:2113/streams/user-{}", username).as_str())
		.set_header("Content-Type", "application/json")
		.set_header(
			"ES-ExpectedVersion",
			(ExpectedVersion::StreamShouldExist as i8).to_string(),
		)
		.set_header("ES-EventId", event_id)
		.set_header("ES-EventType", event.to_string())
		.body_json(&event_data)
		.map_err(|e| error::ErrorInternalServerError(e))?
		.await
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Return successfully.
	Ok(HttpResponse::Created().body(event_data.token))
}

pub fn login(
	req: HttpRequest,
	path: web::Path<(String,)>,
	password: String,
) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(login_async(req, path, password))
}

//#endregion
