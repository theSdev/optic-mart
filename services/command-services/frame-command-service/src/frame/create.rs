use super::Frame;
use crate::event::{Event, ExpectedVersion};
use crate::utils;
use actix_web::{error, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;
use surf;

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
			owner_id: "".to_string(),
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
	price: f32,
	privacy_mode: u8,
}

pub async fn create_async(
	req: HttpRequest,
	user_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	// Validate and convert user_model to UserRegisteredData.
	let frame = FrameCreatedData::try_from(user_model.into_inner())
		.map_err(|e| error::ErrorBadRequest(e))?;

	// Construct event
	let owner_username = "".to_owned();
	let brand_name = frame.brand_name.clone();
	let model_name = frame.model_name.clone();
	let event_data = &FrameCreatedData::from(frame);
	let event = Event::FrameCreated(event_data.clone());
	let event_id = utils::get_correlation_id(req.headers().get("X-Correlation-ID"));

	// Persist to Event Store.
	let es_res = surf::post(
		format!(
			"http://localhost:2113/streams/frame-{}-{}-{}",
			owner_username, brand_name, model_name
		)
		.as_str(),
	)
	.set_header("Content-Type", "application/json")
	.set_header(
		"ES-ExpectedVersion",
		(ExpectedVersion::StreamShouldNotExist as i8).to_string(),
	)
	.set_header("ES-EventId", event_id)
	.set_header("ES-EventType", event.to_string())
	.body_json(event_data)
	.map_err(|e| error::ErrorInternalServerError(e))?
	.await;

	es_res.map_err(|e| error::ErrorInternalServerError(e))?;

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
	user_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(create_async(req, user_model))
}

//#endregion
