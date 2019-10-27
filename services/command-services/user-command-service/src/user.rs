use actix_web::{web, HttpResponse, Responder};
use bcrypt::{DEFAULT_COST, hash};
use chrono::prelude::*;
use crate::event::{Event, ExpectedVersion};
use futures::{future, Future};
use regex::Regex;
use reqwest;
use serde::{Serialize, Deserialize};
use std::borrow::Borrow;
use std::convert::TryFrom;
use uuid::Uuid;

//#region User

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	id: String,
	name: String,
	start_date: Option<DateTime<Utc>>,
	address: Option<String>,
	phone_number: Option<String>,
	username: String,
	password: String,
	email: String,
	photo: Option<String>,
}

impl User {
	pub fn id(&self) -> &String{
		&self.id
	}
	
	pub fn name(&self) -> &String{
		&self.name
	}
	
	pub fn start_date(&self) -> &Option<DateTime<Utc>>{
		&self.start_date
	}
	
	pub fn address(&self) -> &Option<String>{
		&self.address
	}
	
	pub fn phone_number(&self) -> &Option<String>{
		&self.phone_number
	}
	
	pub fn username(&self) -> &String{
		&self.username
	}
	
	pub fn password(&self) -> &String{
		&self.password
	}
	
	pub fn email(&self) -> &String{
		&self.email
	}
	
	pub fn photo(&self) -> &Option<String>{
		&self.photo
	}
	
	fn get_field_regex_pattern(field_name: &str) -> Option<&'static str>
	{
		match field_name {
			"name" => Some(r"^\w{2,50}$"),
			"address" => Some(r"^\w{1,500}$"),
			"phone_number" => Some(r"^[\d+]{4,20}$"),
			"username" => Some(r"^\w{1,20}$"),
			"email" => Some(r"^\w+@\w{2,}\.\w{2,}$"),
			_ => None
		}
	}
	
	fn validate_field_str(field_name: &str, value: &str) -> Result<(), String> {
		let pattern = Self::get_field_regex_pattern(field_name);
		
		if let Some(pattern) = pattern {
			if !Regex::new(pattern).unwrap().is_match(value) {
				return Err(format!("Field '{}' is invalid. Expected pattern: {}", field_name, pattern));
			}
		}
		
		Ok(())
	}
}

impl TryFrom<WebModel> for User {
	type Error = String;
	
	fn try_from(user_model: WebModel) -> Result<Self, Self::Error> {
		Self::validate_field_str("name", user_model.name.as_str())?;
		
		if let Some(address) = user_model.address.borrow() {
			Self::validate_field_str("address", address)?;
		}
		
		if let Some(phone_number) = user_model.phone_number.borrow() {
			Self::validate_field_str("phone_number", phone_number)?;
		}
		
		Self::validate_field_str("username", user_model.name.as_str())?;
		Self::validate_field_str("email", user_model.name.as_str())?;
		
		Ok(Self {
			id: format!("{}", Uuid::new_v4().to_hyphenated()),
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

//#region event

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

impl From<User> for UserRegisteredData {
	fn from(user_model: User) -> Self {
		Self {
			name: user_model.name,
			start_date: user_model.start_date,
			address: user_model.address,
			phone_number: user_model.phone_number,
			username: user_model.username,
			password: user_model.password,
			email: user_model.email,
			photo: user_model.photo,
		}
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

pub fn register(user_model: web::Json<WebModel>) -> Box<dyn Future<Item = impl Responder, Error = actix_web::Error>> {
	let user = User::try_from(user_model.into_inner());
	
	let user = match user {
		Ok(user) => user,
		Err(error) => return Box::new(future::ok(HttpResponse::BadRequest().body(error))),
	};
	
	println!("{:#?}", &user);
	
	let event = Event::UserRegistered(UserRegisteredData::from(user));
	
	Box::new(web::block(move || {
		let mut event_data: Option<UserRegisteredData> = None;
		
		if let Event::UserRegistered(event_data_pat) = event.clone() {
			event_data = Some(event_data_pat);
		}
		
		reqwest::Client::new()
			.post("http://0.0.0.0:2113/streams/user")
			.header("Content-Type", "application/json")
			.header("ES-ExpectedVersion", (ExpectedVersion::StreamShouldNotExist as i8).to_string())
			.header("ES-EventId", format!("{}", Uuid::new_v4().to_hyphenated()))
			.header("ES-EventType", event.to_string())
			.json(&event_data.unwrap())
			.send()
	})
		.from_err()
		.and_then(|res| {
			println!(">>> testing res: {:?}", &res);
			HttpResponse::Created().body("user registered successfully")
		}))
}

//#endregion
