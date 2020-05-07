use chrono::prelude::*;
use crate::utils;
use crate::utils::{generate_uuid, Claims};
use actix_web::{error, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

//#region Event

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderPlacedData {
	customer_id: String,
	date: Option<DateTime<Utc>>,
	frame_color: Option<String>,
	frame_id: String,
	owner_id: String,
	quantity: u16,
	total: f32,
}

impl TryFrom<WebModel> for OrderPlacedData {
	type Error = String;

	fn try_from(order_model: WebModel) -> Result<Self, Self::Error> {
		Ok(Self {
			customer_id: order_model.customer_id.ok_or("Missing customer_id")?,
			date: order_model.date,
			frame_color: order_model.frame_color,
			frame_id: order_model.frame_id,
			owner_id: order_model.owner_id.ok_or("Missing owner_id")?,
			quantity: order_model.quantity,
			total: order_model.total.ok_or("Missing total")?,
		})
	}
}

//#endregion

//#region Web

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Frame {
	owner_id: String,
	price: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebModel {
	customer_id: Option<String>,
	date: Option<DateTime<Utc>>,
	frame_color: Option<String>,
	frame_id: String,
	owner_id: Option<String>,
	quantity: u16,
	total: Option<f32>,
}

pub async fn place_async(
	req: HttpRequest,
	mut order_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	// Auth
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
	order_model.customer_id = Some(token.claims.id);

	// Get frame
	let frame_query_service_address = crate::SECRETS.get("frame_query_service_address")
		.ok_or(error::ErrorInternalServerError("Failed to get frame_query_service_address"))?;
	let mut response = surf::get(format!("{}/frames/{}", frame_query_service_address, order_model.frame_id))
		.await
		.map_err(|e| error::ErrorInternalServerError(e))?;
	let Frame { owner_id, price } = response.body_json()
		.await
		.map_err(|e| error::ErrorInternalServerError(e))?;
	order_model.owner_id = Some(owner_id);
	order_model.total = Some(order_model.quantity as f32 * price);
	
	// Validate and convert order_model to OrderPlacedData.
	let order = OrderPlacedData::try_from(order_model.into_inner())
		.map_err(|e| error::ErrorBadRequest(e))?;

	let order_id = generate_uuid();

	// Save in order_command db before event_store for future checks
	let order_command_conn =
		utils::get_order_command_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	order_command_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "order" (
				id              TEXT NOT NULL PRIMARY KEY,
				owner_id        TEXT NOT NULL,
				customer_id     TEXT NOT NULL
			  )"#,
			&[],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	order_command_conn
		.execute(
			r#"INSERT INTO "order" (id, owner_id, customer_id) VALUES ($1, $2, $3)"#,
			&[&order_id, &order.owner_id, &order.customer_id],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Persist to Event Store.
	let event_data = &OrderPlacedData::from(order);

	let event_store_conn =
		utils::get_event_store_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	event_store_conn
		.execute(
			r#"CREATE TABLE IF NOT EXISTS "order" (
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
			r#"INSERT INTO "order" (entity_id, type, body) VALUES ($1, $2, $3)"#,
			&[
				&order_id,
				&"OrderPlaced",
				&serde_json::to_string(event_data)
					.map_err(|e| error::ErrorInternalServerError(e))?,
			],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Return successfully.
	Ok(HttpResponse::Created()
		.body("order created successfully"))
}

pub fn place(
	req: HttpRequest,
	order_model: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	async_std::task::block_on(place_async(req, order_model))
}

//#endregion
