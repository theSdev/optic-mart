use super::User;
use crate::event::{Event, ExpectedVersion};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use chrono::prelude::*;
use futures::{future, Future};
use reqwest;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;
use uuid::Uuid;

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

		User::validate_field_str("username", user_model.name.as_str())?;
		User::validate_field_str("email", user_model.name.as_str())?;

		Ok(Self {
			name: user_model.name,
			start_date: user_model.start_date,
			address: user_model.address,
			phone_number: user_model.phone_number,
			username: user_model.username,
			password: hash(user_model.password, DEFAULT_COST).unwrap(),
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

pub fn register(
	req: HttpRequest,
	user_model: web::Json<WebModel>,
) -> Box<dyn Future<Item = impl Responder, Error = actix_web::Error>> {
	let user = UserRegisteredData::try_from(user_model.into_inner());

	let user = match user {
		Ok(user) => user,
		Err(error) => return Box::new(future::ok(HttpResponse::BadRequest().body(error))),
	};

	println!("{:#?}", &user);

	fn uuid() -> String {
		format!("{}", Uuid::new_v4().to_hyphenated())
	}

	let event = Event::UserRegistered(UserRegisteredData::from(user));
	let event_id = req
		.headers()
		.get("X-Correlation-ID")
		.map_or_else(uuid, |header| {
			header
				.to_str()
				.ok()
				.map_or_else(uuid, |header_str| header_str.to_owned())
		});

	let entity_id = format!("{}", Uuid::new_v4().to_hyphenated());

	Box::new(
		web::block(move || {
			let Event::UserRegistered(event_data) = &event;

			reqwest::Client::new()
				.post(format!("http://0.0.0.0:2113/streams/user-{}", entity_id).as_str())
				.header("Content-Type", "application/json")
				.header(
					"ES-ExpectedVersion",
					(ExpectedVersion::StreamShouldNotExist as i8).to_string(),
				)
				.header("ES-EventId", event_id)
				.header("ES-EventType", event.to_string())
				.json(event_data)
				.send()
		})
		.from_err()
		.and_then(|res| {
			println!(">>> testing res: {:?}", &res);
			HttpResponse::Created().body("user registered successfully")
		}),
	)
}

//#endregion
