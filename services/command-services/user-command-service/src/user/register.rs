use super::User;
use crate::utils;
use crate::utils::generate_uuid;
use actix_web::{error, web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;

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
	user_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	// Validate and convert user_model to UserRegisteredData.
	let user = UserRegisteredData::try_from(user_model.into_inner())
		.map_err(|e| error::ErrorBadRequest(e))?;

	let user_command_conn =
		utils::get_user_command_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	user_command_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "user" (
			    id              UUID NOT NULL,
			    username        TEXT NOT NULL UNIQUE,
			    password        TEXT NOT NULL,
			    email           TEXT NOT NULL UNIQUE
			  )"#,
			&[],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Check for username and email uniqueness.
	for row in &user_command_conn
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

	let user_id = generate_uuid();
	let username = user.username.clone();

	// Save user in Postgres
	user_command_conn
		.execute(
			r#"INSERT INTO "user" (id, username, password, email) VALUES ($1, $2, $3, $4)"#,
			&[&user_id, &user.username, &user.password, &user.email],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Persist to Event Store.
	let event_data = &UserRegisteredData::from(user);

	let event_store_conn =
		utils::get_event_store_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	event_store_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "user" (
				id              SERIAL PRIMARY KEY,
				entity_id       UUID NOT NULL,
				type            TEXT NOT NULL,
				body            JSON NOT NULL,
				inserted_at     TIMESTAMP(6) NOT NULL DEFAULT (statement_timestamp() at time zone 'utc')
			  )"#,
			&[],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Store event.
	event_store_conn
		.execute(
			r#"INSERT INTO "user" (entity_id, type, body) VALUES ($1, $2, $3)"#,
			&[
				&user_id,
				&"UserRegistered",
				&serde_json::to_string(event_data)
					.map_err(|e| error::ErrorInternalServerError(e))?,
			],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Return successfully.
	Ok(HttpResponse::Created()
		.header("Location", format!("{}/users/{}", crate::ADDR, &username))
		.body("user registered successfully"))
}

pub fn register(user_model: web::Json<WebModel>) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(register_async(user_model))
}

//#endregion
