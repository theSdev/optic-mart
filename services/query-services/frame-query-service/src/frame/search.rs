use super::Frame;
use crate::utils;
use actix_web::{error, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
	pub term: String,
}

pub fn search(info: web::Query<Info>) -> Result<HttpResponse, actix_web::Error> {
	let term = &info.term;
	dbg!(&term);

	let mut frame_query_conn = utils::get_frame_query_db_connection().unwrap();
	let frame_rows = frame_query_conn
		.query(
			r#"SELECT
				entity_id,
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
				updated_at
				FROM "frame"
			WHERE brand_name LIKE $1 
				OR colors LIKE $1 
				OR description LIKE $1 
				OR materials LIKE $1 
				OR model_name LIKE $1 "#,
			&[&(term.to_owned() + &"%".to_owned())],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	let mut frames: Vec<Frame> = vec![];

	for stored_frame in frame_rows {
		let colors = stored_frame.get::<usize, String>(2);
		let materials = stored_frame.get::<usize, String>(6);
		let other_images = stored_frame.get::<usize, String>(8);

		let frame = Frame {
			id: stored_frame.get(0),
			brand_name: stored_frame.get(1),
			colors: colors.split(",").map(|s| s.to_string()).collect(),
			cover_image: stored_frame.get(3),
			description: stored_frame.get(4),
			has_case: stored_frame.get(5),
			materials: materials.split(",").map(|s| s.to_string()).collect(),
			model_name: stored_frame.get(7),
			other_images: other_images.split(",").map(|s| s.to_string()).collect(),
			owner_id: stored_frame.get(9),
			price: stored_frame.get(10),
			privacy_mode: stored_frame.get(11),
		};

		frames.push(frame);
	}

	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(serde_json::to_string(&frames).unwrap()))
}
