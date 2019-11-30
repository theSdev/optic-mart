use actix_web::{HttpResponse, Responder};

pub fn get_all() -> impl Responder {
	let users = ["me", "you", "others"];
	HttpResponse::Ok().body(format!("{:#?}", users))
}
