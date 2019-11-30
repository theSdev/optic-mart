use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
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

fn main() {
	println!("Listening on {}", ADDR);

	thread::spawn(update_users);

	HttpServer::new(|| {
		App::new()
			.wrap(Cors::new())
			.service(web::scope("/users").route("", web::get().to(user::get_all::get_all)))
	})
	.bind(ADDR)
	.unwrap()
	.run()
	.unwrap();
}

async fn update_users() {
	use event::*;

	let event_store_conn = utils::get_event_store_db_connection().unwrap();

	let mut tracked_users = HashMap::<String, String>::new();

	let user_query_conn = utils::get_user_query_db_connection().unwrap();
	user_query_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "user" (
			    id              SERIAL PRIMARY KEY,
			    entity_id       UUID NOT NULL,
			    name            TEXT NOT NULL,
			    start_date      DATE,
			    address         TEXT,
			    phone_number    TEXT,
			    username        TEXT NOT NULL UNIQUE,
			    password        TEXT NOT NULL,
			    email           TEXT NOT NULL UNIQUE,
			    photo           TEXT,
			    updated_at      TIMESTAMP(6) NOT NULL
			  )"#,
			&[],
		)
		.unwrap();

	let user_rows = &user_query_conn
		.query(r#"SELECT entity_id, updated_at FROM "user""#, &[])
		.map_err(|e| e.to_string())
		.unwrap();

	for row in user_rows {
		tracked_users.insert(row.get(0), row.get(1));
	}

	loop {
		let read_res = (|| {
			let event_rows = &event_store_conn
				.query(
					r#"SELECT entity_id, body, inserted_at FROM "user"
					WHERE type = $1
					AND entity_id NOT IN ($2)"#,
					&[
						&"UserRegistered",
						&tracked_users
							.keys()
							.into_iter()
							.map(|s| s.clone())
							.collect::<Vec<String>>()
							.join(", "),
					],
				)
				.map_err(|e| e.to_string())?;

			for row in event_rows {
				let persist_res = (|| {
					let entity_id: String = row.get(0);
					let body: String = row.get(1);
					let body: UserRegisteredData =
						serde_json::from_str(&body).map_err(|e| e.to_string())?;
					let inserted_at: String = row.get(2);

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
							password,
							email,
							photo,
							updated_at)
							VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
							&[
								&entity_id,
								&body.name,
								&body
									.start_date
									.map_or("".to_owned(), |start_date| start_date.to_string()),
								&body.address,
								&body.phone_number,
								&body.username,
								&body.password,
								&body.email,
								&body.photo,
								&inserted_at,
							],
						)
						.map_err(|e| e.to_string())?;

					tracked_users.insert(entity_id, inserted_at);

					Ok::<(), String>(())
				})();

				if persist_res.is_err() {
					dbg!("{}", persist_res.unwrap_err());
				}
			}

			Ok::<(), String>(())
		})();

		if read_res.is_err() {
			dbg!("{}", read_res.unwrap_err());
		}

		thread::sleep(Duration::from_secs(60));

		/*
			let t = &"2019-11-30 08:20:14.210265"[..19];
			let a = Utc.datetime_from_str(t, "%Y-%m-%d %H:%M:%S");
			println!("{:?}", a);
		*/
	}
}
