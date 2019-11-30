use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::Config;
use lazy_static;
use std::collections::HashMap;

mod user;
mod utils;

pub const ADDR: &str = "0.0.0.0:8002";
lazy_static::lazy_static! {
	static ref SECRETS: HashMap<String, String> = {
		let mut config = Config::default();
		config.merge(config::File::with_name("secrets")).unwrap();
		config.try_into::<HashMap<String, String>>().unwrap()
	};
}

fn main() {
	println!("Listening on {}", ADDR);

	HttpServer::new(|| {
		App::new().wrap(Cors::new()).service(
			web::scope("/users")
				.route("", web::post().to(user::register::register))
				.route("/{username}/tokens", web::post().to(user::login::login)),
		)
	})
	.bind(ADDR)
	.unwrap()
	.run()
	.unwrap();
}
