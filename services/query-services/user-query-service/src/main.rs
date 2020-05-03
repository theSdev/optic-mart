use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use chrono::prelude::*;
use config::Config;
use lazy_static;
use serde_json;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

mod event;
mod user;
mod utils;

pub const ADDR: &str = "0.0.0.0:9002";
lazy_static::lazy_static! {
	static ref SECRETS: HashMap<String, String> = {
		let mut config = Config::default();
		config.merge(config::File::with_name("secrets")).unwrap();
		config.try_into::<HashMap<String, String>>().unwrap()
	};
}

#[async_std::main]
async fn main() {
	println!("Listening on {}", ADDR);
	async_std::task::spawn(update_users());

	HttpServer::new(|| {
		App::new()
			.wrap(Cors::new())
			.service(
				web::scope("/users")
					.route("", web::get().to(user::get::get_all))
					.route("/{id}", web::get().to(user::get::get)),
			)
			.service(web::scope("/search").route("", web::get().to(user::search::search)))
	})
	.bind(ADDR)
	.unwrap()
	.run()
	.unwrap();
}

async fn update_users() {
	use event::*;

	let mut event_store_conn = utils::get_event_store_db_connection().unwrap();

	let mut tracked_users = HashMap::<String, NaiveDateTime>::new();

	let mut user_query_conn = utils::get_user_query_db_connection().unwrap();
	user_query_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "settings" (
			    updated_at      TIMESTAMP(6) NOT NULL
			  )"#,
			&[],
		)
		.unwrap();

	user_query_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "user" (
			    id              SERIAL PRIMARY KEY,
			    entity_id       TEXT NOT NULL UNIQUE,
			    name            TEXT NOT NULL,
			    start_date      DATE,
			    address         TEXT,
			    phone_number    TEXT,
			    username        TEXT NOT NULL UNIQUE,
			    email           TEXT NOT NULL UNIQUE,
			    photo           TEXT,
			    updated_at      TIMESTAMP(6) NOT NULL
			  )"#,
			&[],
		)
		.unwrap();

	let user_rows = &user_query_conn
		.query(r#"SELECT entity_id, updated_at FROM "user""#, &[])
		.unwrap();

	for row in user_rows {
		let user_id = row.get(0);
		let updated_at: NaiveDateTime = row.get(1);
		tracked_users.insert(user_id, updated_at);
	}

	loop {
		let recent = tracked_users.iter().max_by(|p, q| p.1.cmp(q.1));
		let updated_at: NaiveDateTime = if let Some(max_pair) = recent {
			max_pair.1.clone()
		} else {
			NaiveDateTime::new(NaiveDate::from_yo(1970, 1), NaiveTime::from_hms(0, 0, 0))
		};

		dbg!(&updated_at);

		let read_res = (|| {
			let event_rows = &event_store_conn
				.query(
					r#"SELECT entity_id, body, inserted_at, type FROM "user"
					WHERE inserted_at > $1"#,
					&[&updated_at],
				)
				.map_err(|e| e.to_string())?;

			for row in event_rows {
				let entity_id: String = row.get(0);
				dbg!(&entity_id);

				let body: String = row.get(1);
				dbg!(&body);

				let inserted_at: NaiveDateTime = row.get(2);
				dbg!(&inserted_at);

				let r#type: String = row.get(3);
				dbg!(&r#type);

				let persist_res = (|| {
					match r#type.as_str() {
						"UserRegistered" => {
							let body: UserRegisteredData =
								serde_json::from_str(&body).map_err(|e| e.to_string())?;

							// Save user in Postgres
							user_query_conn
								.execute(
									r#"INSERT INTO "user"
								(entity_id,
								name,
								start_date,
								address,
								phone_number,
								username,
								email,
								photo,
								updated_at)
								VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
									&[
										&entity_id,
										&body.name,
										&body.start_date,
										&body.address,
										&body.phone_number,
										&body.username,
										&body.email,
										&body.photo,
										&inserted_at,
									],
								)
								.map_err(|e| e.to_string())?;

							Ok::<(), String>(())
						}
						"UserModified" => {
							let body: UserModifiedData =
								serde_json::from_str(&body).map_err(|e| e.to_string())?;

							// Save user in Postgres
							user_query_conn
								.execute(
									r#"UPDATE "user" SET
										name = $1,
										start_date = $2,
										address = $3,
										phone_number = $4,
										username = $5,
										email = $6,
										photo = $7,
										updated_at = $8
									WHERE entity_id = $9"#,
									&[
										&body.name,
										&body.start_date,
										&body.address,
										&body.phone_number,
										&body.username,
										&body.email,
										&body.photo,
										&inserted_at,
										&entity_id,
									],
								)
								.map_err(|e| e.to_string())?;

							Ok::<(), String>(())
						}
						other => {
							println!("Unknown event type {}", other);

							Ok::<(), String>(())
						}
					}
				})();

				persist_res.unwrap();
				tracked_users.insert(entity_id, inserted_at);
			}

			Ok::<(), String>(())
		})();

		if read_res.is_err() {
			dbg!("{}", read_res.unwrap_err());
		}

		thread::sleep(Duration::from_secs(30));
	}
}
