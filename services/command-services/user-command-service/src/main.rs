use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod event;
mod user;

fn main() {
	// "postgres://YourUserName:YourPassword@YourHost:5432/YourDatabase"
	// let con_str = "postgres://postgres:s1031374@localhost:5432/optic_mart";
	println!("Listening on localhost:8090");

	HttpServer::new(|| {
		App::new()
			.wrap(Cors::new())
			.route("/", web::post().to_async(user::register))
	})
	.bind("0.0.0.0:8090")
	.unwrap()
	.run()
	.unwrap();
}
