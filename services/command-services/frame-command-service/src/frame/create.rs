use super::Frame;
use crate::utils;
use crate::utils::{generate_uuid, Claims};
use actix_web::{error, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;

//#region Event

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameCreatedData {
	brand_name: String,
	colors: Vec<String>,
	cover_image: Option<String>,
	description: Option<String>,
	has_case: bool,
	materials: Vec<String>,
	model_name: String,
	other_images: Vec<String>,
	owner_id: String,
	price: f32,
	privacy_mode: u8,
}

impl TryFrom<WebModel> for FrameCreatedData {
	type Error = String;

	fn try_from(frame_model: WebModel) -> Result<Self, Self::Error> {
		Frame::validate_field_str("brand_name", frame_model.brand_name.as_str())?;

		for color in &frame_model.colors {
			Frame::validate_field_str("color", color.as_str())?;
		}

		if let Some(description) = frame_model.description.borrow() {
			Frame::validate_field_str("description", description)?;
		}

		for material in &frame_model.materials {
			Frame::validate_field_str("material", material.as_str())?;
		}

		Frame::validate_field_str("model_name", frame_model.model_name.as_str())?;
		Frame::validate_field("price", Box::new(frame_model.price))?;
		Frame::validate_field("privacy_mode", Box::new(frame_model.privacy_mode))?;

		Ok(Self {
			brand_name: frame_model.brand_name,
			colors: frame_model.colors,
			cover_image: frame_model.cover_image,
			description: frame_model.description,
			has_case: frame_model.has_case,
			materials: frame_model.materials,
			model_name: frame_model.model_name,
			other_images: frame_model.other_images,
			owner_id: frame_model.owner_id.ok_or("Missing owner_id")?,
			price: frame_model.price,
			privacy_mode: frame_model.privacy_mode,
		})
	}
}

//#endregion

//#region Web

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebModel {
	brand_name: String,
	colors: Vec<String>,
	cover_image: Option<String>,
	description: Option<String>,
	has_case: bool,
	materials: Vec<String>,
	model_name: String,
	other_images: Vec<String>,
	owner_id: Option<String>,
	price: f32,
	privacy_mode: u8,
}

pub async fn create_async(
	req: HttpRequest,
	mut frame_model: web::Json<WebModel>,
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
	frame_model.owner_id = Some(token.claims.id);

	// Validate and convert frame_model to UserRegisteredData.
	let frame = FrameCreatedData::try_from(frame_model.into_inner())
		.map_err(|e| error::ErrorBadRequest(e))?;

	let frame_id = generate_uuid();
	let owner_username = token.claims.sub.to_owned();
	let brand_name = frame.brand_name.clone();
	let model_name = frame.model_name.clone();

	// Persist to Event Store.
	let event_data = &FrameCreatedData::from(frame);

	let event_store_conn =
		utils::get_event_store_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	event_store_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "frame" (
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
			r#"INSERT INTO "frame" (entity_id, type, body) VALUES ($1, $2, $3)"#,
			&[
				&frame_id,
				&"FrameCreated",
				&serde_json::to_string(event_data)
					.map_err(|e| error::ErrorInternalServerError(e))?,
			],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Return successfully.
	Ok(HttpResponse::Created()
		.header(
			"Location",
			format!(
				"{}/users/{}/brands/{}/models/{}",
				crate::ADDR,
				owner_username,
				brand_name,
				model_name
			),
		)
		.body("frame created successfully"))
}

pub fn create(
	req: HttpRequest,
	frame_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(create_async(req, frame_model))
}

//#endregion
