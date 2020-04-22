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
mod order;
mod utils;

pub const ADDR: &str = "0.0.0.0:9004";
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
	async_std::task::spawn(update_orders());

	HttpServer::new(|| {
		App::new()
			.wrap(Cors::new())
			.service(
				web::scope("/orders")
					.route("", web::get().to(order::get::get_received)),
			)
	})
	.bind(ADDR)
	.unwrap()
	.run()
	.unwrap();
}

async fn update_orders() {
	use event::*;

	let mut event_store_conn = utils::get_event_store_db_connection().unwrap();

	let mut tracked_orders = HashMap::<String, NaiveDateTime>::new();

	let mut order_query_conn = utils::get_order_query_db_connection().unwrap();
	order_query_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "order" (
			    id              SERIAL PRIMARY KEY,
			    entity_id       TEXT NOT NULL,
			    customer_id     TEXT NOT NULL,
			    date            DATE,
			    frame_color     TEXT,
			    frame_id        TEXT NOT NULL,
			    owner_id        TEXT NOT NULL,
			    quantity        INT,
			    total           REAL,
			    updated_at      TIMESTAMP(6) NOT NULL
			  )"#,
			&[],
		)
		.unwrap();

	let user_rows = &order_query_conn
		.query(r#"SELECT entity_id, updated_at FROM "order""#, &[])
		.unwrap();

	for row in user_rows {
		let user_id = row.get(0);
		let updated_at: NaiveDateTime = row.get(1);
		tracked_orders.insert(user_id, updated_at);
	}

	loop {
		let read_res = (|| {
			let tracked_entity_ids = &tracked_orders
				.keys()
				.into_iter()
				.map(|s| s.clone())
				.collect::<Vec<String>>();
			dbg!(tracked_entity_ids);

			let event_rows = &event_store_conn
				.query(
					r#"SELECT entity_id, body, inserted_at FROM "order"
					WHERE type = $1"#,
					&[&"OrderPlaced"],
				)
				.map_err(|e| e.to_string())?;

			for row in event_rows {
				let persist_res = (|| {
					let entity_id: String = row.get(0);
					dbg!(&entity_id);

					if !tracked_entity_ids.contains(&entity_id) {
						let body: String = row.get(1);
						dbg!(&body);
						let body: OrderPlacedData =
							serde_json::from_str(&body).map_err(|e| e.to_string())?;

						let inserted_at: NaiveDateTime = row.get(2);

						// Save order in Postgres
						order_query_conn
							.execute(
								r#"INSERT INTO "order"
								(entity_id,
								customer_id,
								date,
								frame_color,
								frame_id,
								owner_id,
								quantity,
								total,
								updated_at)
								VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
								&[
									&entity_id,
									&body.customer_id,
									&body.date,
									&body.frame_color,
									&body.frame_id,
									&body.owner_id,
									&body.quantity,
									&body.total,
									&inserted_at,
								],
							)
							.map_err(|e| e.to_string())?;

						tracked_orders.insert(entity_id, inserted_at);
					}
					Ok::<(), String>(())
				})();

				if persist_res.is_err() {
					dbg!(persist_res.unwrap_err());
				}
			}

			Ok::<(), String>(())
		})();

		if read_res.is_err() {
			dbg!("{}", read_res.unwrap_err());
		}

		thread::sleep(Duration::from_secs(30));

		/*
			let t = &"2019-11-30 08:20:14.210265"[..19];
			let a = Utc.datetime_from_str(t, "%Y-%m-%d %H:%M:%S");
			println!("{:?}", a);
		*/
	}
}
