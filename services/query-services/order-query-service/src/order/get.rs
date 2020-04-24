use super::Order;
use crate::utils;
use crate::utils::Claims;
use actix_web::{error, HttpRequest, HttpResponse};

pub fn get_received(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
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
	let owner_id = token.claims.id;
	dbg!(&owner_id);

	let mut order_query_conn = utils::get_order_query_db_connection().unwrap();
	let order_rows = order_query_conn
		.query(
			r#"SELECT
				entity_id,
				customer_id,
				date,
				frame_color,
				frame_id,
				owner_id,
				quantity,
				total,
				updated_at
				FROM "order"
				WHERE owner_id = $1"#,
			&[&owner_id],
		)
		.map_err(|e| error::ErrorInternalServerError(e))?;

	let mut orders: Vec<Order> = vec![];

	for stored_order in order_rows {
		let order = Order {
			id: stored_order.get(0),
			customer_id: stored_order.get(1),
			date: stored_order.get(2),
			frame_color: stored_order.get(3),
			frame_id: stored_order.get(4),
			owner_id: stored_order.get(5),
			quantity: stored_order.get(6),
			total: stored_order.get(7),
		};

		orders.push(order);
	}

	Ok(HttpResponse::Ok()
		.content_type("application/json")
		.body(serde_json::to_string(&orders).unwrap()))
}
