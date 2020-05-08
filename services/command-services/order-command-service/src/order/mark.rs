use crate::utils;
use crate::utils::Claims;
use actix_web::{error, web, HttpRequest, HttpResponse};
use serde::Deserialize;

//#region Web

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebModel {
	processed: bool,
	rejected: bool,
}

pub fn mark(
	req: HttpRequest,
	id: web::Path<String>,
	flags: web::Json<WebModel>,
) -> Result<HttpResponse, actix_web::Error> {
	let order_id = id.to_string();
	dbg!(&order_id);

	if !flags.processed && !flags.rejected {
		Err(error::ErrorBadRequest("One of the flags should be set."))?;
	}

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
	let user_id = token.claims.id;

	// Checks before saving
	let order_command_conn =
		utils::get_order_command_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	let order_rows = order_command_conn
		.query(r#"SELECT owner_id FROM "order""#, &[])
		.map_err(|e| error::ErrorInternalServerError(e))?;

	let stored_order = order_rows
		.into_iter()
		.next()
		.ok_or(error::ErrorBadRequest("Order not found."))?;

	if stored_order.get::<usize, String>(0) != user_id {
		Err(error::ErrorUnauthorized(
			"You need to be the owner of the frame in order to mark its orders.",
		))?;
	}

	let event_store_conn =
		utils::get_event_store_db_connection().map_err(|e| error::ErrorInternalServerError(e))?;

	let event_type = if flags.processed {
		"OrderProcessed".to_string()
	} else {
		"OrderRejected".to_string()
	};

	event_store_conn
		.execute(
			r#"INSERT INTO "order" (entity_id, type, body) VALUES ($1, $2, $3)"#,
			&[&order_id, &event_type, &"{}".to_string()],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	// Return successfully.
	Ok(HttpResponse::Ok().body("order marked successfully"))
}

//#endregion
