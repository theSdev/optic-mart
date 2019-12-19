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
mod frame;
mod utils;

pub const ADDR: &str = "0.0.0.0:9003";
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
	async_std::task::spawn(update_frames());
	dbg!();

	HttpServer::new(|| {
		App::new().wrap(Cors::new()).service(
			web::scope("/frames")
				.route("", web::get().to(frame::get::get_all))
				.route("/{id}", web::get().to(frame::get::get)),
		)
	})
	.bind(ADDR)
	.unwrap()
	.run()
	.unwrap();
}

async fn update_frames() {
	use event::*;

	let mut event_store_conn = utils::get_event_store_db_connection().unwrap();

	let mut tracked_frames = HashMap::<String, NaiveDateTime>::new();

	let mut frame_query_conn = utils::get_frame_query_db_connection().unwrap();
	frame_query_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "frame" (
			    id              SERIAL PRIMARY KEY,
			    entity_id       TEXT NOT NULL,
			    brand_name      TEXT NOT NULL,
			    colors          TEXT NOT NULL,
			    cover_image     TEXT,
			    description     TEXT,
			    has_case        BOOL,
			    materials       TEXT,
			    model_name      TEXT NOT NULL,
			    other_images    TEXT,
			    owner_id        TEXT NOT NULL,
			    price           REAL,
			    privacy_mode    SMALLINT,
			    updated_at      TIMESTAMP(6) NOT NULL
			  )"#,
			&[],
		)
		.unwrap();

	let user_rows = &frame_query_conn
		.query(r#"SELECT entity_id, updated_at FROM "frame""#, &[])
		.unwrap();

	for row in user_rows {
		let user_id = row.get(0);
		let updated_at: NaiveDateTime = row.get(1);
		tracked_frames.insert(user_id, updated_at);
	}

	loop {
		let read_res = (|| {
			let tracked_entity_ids = &tracked_frames
				.keys()
				.into_iter()
				.map(|s| s.clone())
				.collect::<Vec<String>>();
			dbg!(tracked_entity_ids);

			let event_rows = &event_store_conn
				.query(
					r#"SELECT entity_id, body, inserted_at FROM "frame"
					WHERE type = $1"#,
					&[&"FrameCreated"],
				)
				.map_err(|e| e.to_string())?;

			for row in event_rows {
				let persist_res = (|| {
					let entity_id: String = row.get(0);
					dbg!(&entity_id);

					if !tracked_entity_ids.contains(&entity_id) {
						let body: String = row.get(1);
						dbg!(&body);
						let body: FrameCreatedData =
							serde_json::from_str(&body).map_err(|e| e.to_string())?;

						let inserted_at: NaiveDateTime = row.get(2);

						// Save frame in Postgres
						frame_query_conn
							.execute(
								r#"INSERT INTO "frame"
								(entity_id,
								brand_name,
								colors,
			    					cover_image,
			    					description,
			    					has_case,
			    					materials,
			    					model_name,
			    					other_images,
			    					owner_id,
			    					price,
			    					privacy_mode,
								updated_at)
								VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"#,
								&[
									&entity_id,
									&body.brand_name,
									&body.colors.join(","),
									&body.cover_image,
									&body.description,
									&body.has_case,
									&body.materials.join(","),
									&body.model_name,
									&body.other_images.join(","),
									&body.owner_id,
									&body.price,
									&body.privacy_mode,
									&inserted_at,
								],
							)
							.map_err(|e| e.to_string())?;

						tracked_frames.insert(entity_id, inserted_at);
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
