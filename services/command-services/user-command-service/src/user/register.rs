use super::User;
use crate::event::{Event, ExpectedVersion};
use crate::utils;
use actix_web::{error, web, HttpRequest, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;
use surf;

//#region Event

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisteredData {
	name: String,
	start_date: Option<DateTime<Utc>>,
	address: Option<String>,
	phone_number: Option<String>,
	username: String,
	password: String,
	email: String,
	photo: Option<String>,
}

impl TryFrom<WebModel> for UserRegisteredData {
	type Error = String;

	fn try_from(user_model: WebModel) -> Result<Self, Self::Error> {
		User::validate_field_str("name", user_model.name.as_str())?;

		if let Some(address) = user_model.address.borrow() {
			User::validate_field_str("address", address)?;
		}

		if let Some(phone_number) = user_model.phone_number.borrow() {
			User::validate_field_str("phone_number", phone_number)?;
		}

		User::validate_field_str("username", user_model.username.as_str())?;
		User::validate_field_str("email", user_model.email.as_str())?;

		Ok(Self {
			name: user_model.name,
			start_date: user_model.start_date,
			address: user_model.address,
			phone_number: user_model.phone_number,
			username: user_model.username,
			password: hash(user_model.password, DEFAULT_COST).map_err(|e| e.to_string())?,
			email: user_model.email,
			photo: user_model.photo,
		})
	}
}

//#endregion

//#region Web

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebModel {
	name: String,
	start_date: Option<DateTime<Utc>>,
	address: Option<String>,
	phone_number: Option<String>,
	username: String,
	password: String,
	email: String,
	photo: Option<String>,
}

pub async fn register_async(
	req: HttpRequest,
	user_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	// Validate and convert user_model to UserRegisteredData.
	let user = UserRegisteredData::try_from(user_model.into_inner())
		.map_err(|e| error::ErrorBadRequest(e))?;

	let conn = utils::get_postgres_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	conn.execute(
		r#"CREATE TABLE IF NOT EXISTS "user" (
			    username        VARCHAR PRIMARY KEY,
			    password        VARCHAR NOT NULL,
			    email           VARCHAR NOT NULL UNIQUE
			  )"#,
		&[],
	)
	.map_err(|e| error::ErrorInternalServerError(e))?;

	// Check for username and email uniqueness.
	for row in &conn
		.query(r#"SELECT username, email FROM "user""#, &[])
		.map_err(|e| error::ErrorInternalServerError(e))?
	{
		let username: String = row.get(0);
		let email: String = row.get(1);
		if username == user.username {
			return Err(error::ErrorBadRequest("username exists"));
		}
		if email == user.email {
			return Err(error::ErrorBadRequest("email exists"));
		}
	}

	// Save user in postgres
	conn.execute(
		r#"INSERT INTO "user" (username, password, email) VALUES ($1, $2, $3)"#,
		&[&user.username, &user.password, &user.email],
	)
	.map_err(|e| error::ErrorInternalServerError(e))?;

	let username = user.username.clone();

	// Construct event
	let event_data = &UserRegisteredData::from(user);
	let event = Event::UserRegistered(event_data.clone());
	let event_id = utils::get_correlation_id(req.headers().get("X-Correlation-ID"));

	// Persist to Event Store.
	let es_res = surf::post(format!("http://localhost:2113/streams/user-{}", username).as_str())
		.set_header("Content-Type", "application/json")
		.set_header(
			"ES-ExpectedVersion",
			(ExpectedVersion::StreamShouldNotExist as i8).to_string(),
		)
		.set_header("ES-EventId", event_id)
		.set_header("ES-EventType", event.to_string())
		.body_json(event_data)
		.map_err(|e| error::ErrorInternalServerError(e))?
		.await;

	// Remove user from postgres if event registration is not successful and return error.
	if es_res.is_err() {
		let _delete_res = conn.execute(r#"DELETE FROM "user" where username = $1"#, &[&username]);
		es_res.map_err(|e| error::ErrorInternalServerError(e))?;
	}

	// Return successfully.
	Ok(HttpResponse::Created()
		.header("Location", format!("{}/users/{}", crate::ADDR, username))
		.body("user registered successfully"))
}

pub fn register(
	req: HttpRequest,
	user_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(register_async(req, user_model))
}

//#endregion
