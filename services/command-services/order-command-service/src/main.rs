use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::Config;
use std::collections::HashMap;
mod order;
mod utils;

pub const ADDR: &str = "0.0.0.0:8004";
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
		App::new()
			.wrap(Cors::new())
			.service(web::scope("/orders")
			.route("", web::post().to(order::place::place))
			.route("/{id}", web::put().to(order::mark::mark)))
	})
	.bind(ADDR)
	.unwrap()
	.run()
	.unwrap();
}
