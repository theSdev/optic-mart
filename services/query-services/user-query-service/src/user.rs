use actix_web::{get, HttpResponse, Responder};

#[get("/user")]
pub fn get_all() -> impl Responder{
	let users = ["me", "you", "others"];
	HttpResponse::Ok().body(format!("{:#?}", users))
}
