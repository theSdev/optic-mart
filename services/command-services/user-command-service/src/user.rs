use actix_web::{post, HttpResponse, Responder};

#[post("/")]
pub fn register() -> impl Responder{
}
