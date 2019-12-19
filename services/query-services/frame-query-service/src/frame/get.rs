use super::Frame;
use crate::utils;
use actix_web::{error, web, HttpResponse, Responder};

pub fn get(path: web::Path<(String,)>) -> Result<HttpResponse, actix_web::Error> {
	let entity_id = &path.0;
	dbg!(&entity_id);

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
				WHERE entity_id = $1"#,
			&[&entity_id],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	let stored_frame = frame_rows
		.into_iter()
		.next()
		.ok_or(error::ErrorBadRequest("Frame not found."))?;

	let colors = stored_frame.get::<usize, String>(2);
	let materials = stored_frame.get::<usize, String>(6);
	let other_images = stored_frame.get::<usize, String>(8);

	let frame = Frame {
		id: stored_frame.get(0),
		brand_name: stored_frame.get(1),
		colors: colors.split(",").collect::<Vec<&str>>(),
		cover_image: stored_frame.get(3),
		description: stored_frame.get(4),
		has_case: stored_frame.get(5),
		materials: materials.split(",").collect::<Vec<&str>>(),
		model_name: stored_frame.get(7),
		other_images: other_images.split(",").collect::<Vec<&str>>(),
		owner_id: stored_frame.get(9),
		price: stored_frame.get(10),
		privacy_mode: stored_frame.get(11),
	};

	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(serde_json::to_string(&frame).unwrap()))
}

pub fn get_all() -> impl Responder {
	let users = ["me", "you", "others"];
	HttpResponse::Ok().body(format!("{:#?}", users))
}
