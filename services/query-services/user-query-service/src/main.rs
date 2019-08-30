use actix_web::{App, HttpServer};

mod user;

fn main() {
	// "postgres://YourUserName:YourPassword@YourHost:5432/YourDatabase"
	// let con_str = "postgres://postgres:s1031374@localhost:5432/optic_mart";
	HttpServer::new(|| {
		App::new()
			.service(user::get_all)
	})
		.bind("127.0.0.1:8088")
		.unwrap()
		.run()
		.unwrap();
}
