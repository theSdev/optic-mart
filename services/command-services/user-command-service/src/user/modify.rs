use super::User;
use crate::utils;
use actix_web::{error, web, HttpRequest, HttpResponse};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;
use utils::Claims;

//#region Event

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserModifiedData {
	name: String,
	start_date: Option<DateTime<Utc>>,
	address: Option<String>,
	phone_number: Option<String>,
	username: String,
	email: String,
	photo: Option<String>,
}

impl TryFrom<WebModel> for UserModifiedData {
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
	email: String,
	photo: Option<String>,
}

pub fn modify(
	req: HttpRequest,
	user_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	let auth_header = req
		.headers()
		.get("Authorization")
		.ok_or(error::ErrorUnauthorized("Auth required."))?
		.to_str()
		.map_err(|e| error::ErrorBadRequest(e))?;

	let token = auth_header.replace(&"Bearer", &"");
	let token = token.as_str().trim();

	let jwt_key = crate::SECRETS
		.get("jwt_key")
		.ok_or(error::ErrorInternalServerError("Failed to get jwt_key"))?;

	let token = jwt::decode::<Claims>(token, jwt_key.as_ref(), &jwt::Validation::default())
		.map_err(|e| error::ErrorBadRequest(e))?;

	let id = token.claims.id.clone();

	// Validate and convert user_model to UserModifiedData.
	let user = UserModifiedData::try_from(user_model.into_inner())
		.map_err(|e| error::ErrorBadRequest(e))?;

	let user_command_conn =
		utils::get_user_command_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	// Check for username and email uniqueness.
	for row in &user_command_conn
		.query(
			r#"SELECT username, email FROM "user" WHERE id <> $1"#,
			&[&id],
		)
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

	let username = user.username.clone();

	// Save user in Postgres
	user_command_conn
		.execute(
			r#"UPDATE "user" SET 
				username = $1,
				email = $2
				WHERE id = $3"#,
			&[&user.username, &user.email, &id],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Persist to Event Store.
	let event_data = &UserModifiedData::from(user);

	let event_store_conn =
		utils::get_event_store_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	event_store_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "user" (
				id              SERIAL PRIMARY KEY,
				entity_id       TEXT NOT NULL,
				type            TEXT NOT NULL,
				body            TEXT NOT NULL,
				inserted_at     TIMESTAMP(6) NOT NULL DEFAULT (statement_timestamp() at time zone 'utc')
			  )"#,
			&[],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	event_store_conn
		.execute(
			r#"INSERT INTO "user" (entity_id, type, body) VALUES ($1, $2, $3)"#,
			&[
				&id,
				&"UserModified",
				&serde_json::to_string(event_data)
					.map_err(|e| error::ErrorInternalServerError(e))?,
			],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Return successfully.
	Ok(HttpResponse::Ok()
		.header("Location", format!("{}/users/{}", crate::ADDR, &username))
		.body("user modified successfully"))
}

//#endregion
