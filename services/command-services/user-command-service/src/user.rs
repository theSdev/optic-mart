use std::convert::From;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use bcrypt::{DEFAULT_COST, hash};
use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
use futures::Future;

#[derive(Deserialize)]
pub struct UserModel {
	username: String,
	password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
	id: String,
	username: String,
	password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Event {
	event_id: String,
	event_type: String,
	data: String,
}

impl From<UserModel> for User {
	fn from(user_model: UserModel) -> Self {
		Self {
			id: format!("{}", Uuid::new_v4().to_hyphenated()),
			username: user_model.username,
			password: hash(user_model.password, DEFAULT_COST).unwrap(),
		}
	}
}

pub fn register(user_model: web::Json<UserModel>) -> impl Future<Item = impl Responder, Error = actix_web::Error> {
	let user = User::from(user_model.into_inner());
	
	let event = Event {
		event_id: format!("{}", Uuid::new_v4().to_hyphenated()),
		event_type: "register".to_owned(),
		data: serde_json::to_string(&user).unwrap(),
	};
	
	println!("{}", event.event_id);
	
	web::block(move ||{
		reqwest::Client::new()
			.post("http://0.0.0.0:2113/streams/user")
			.header("Content-Type", "application/json")
			.header("ES-CurrentVersion", "-2")
			.header("ES-EventId", event.event_id)
			.header("ES-EventType", event.event_type)
			.json(&user)
			.send()
	})
		.from_err()
		.and_then(|res| {
			println!(">>> testing res: {:?}", &res);
			HttpResponse::Created().body("user registered successfully")
		})
}
