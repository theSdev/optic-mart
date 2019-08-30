//use actix_web::{App, HttpServer};
use rdkafka::{
	config::{ClientConfig, FromClientConfig},
	producer::future_producer::{FutureProducer, FutureRecord},
};
use futures::*;

//mod user;

fn main() {
	// "postgres://YourUserName:YourPassword@YourHost:5432/YourDatabase"
	// let con_str = "postgres://postgres:s1031374@localhost:5432/optic_mart";
//	HttpServer::new(|| {
//		App::new()
//			.service(user::register)
//	})
//		.bind("127.0.0.1:8088")
//		.unwrap()
//		.run()
//		.unwrap();
	
	
	let fut_producer: FutureProducer = ClientConfig::new()
		.set("bootstrap.servers", "localhost:9092")
		.set("produce.offset.report", "true")
		.set("message.timeout.ms", "5000")
		.create()
		.expect("Producer creation error");
	
	let fut_record = FutureRecord::to("test")
		.payload("lol")
		.key("2");
	fut_producer.send(fut_record, 1000)
		.map(move |delivery_status| {   // This will be executed onw the result is received
                    	println!("Delivery status for message received ");
                })
		.wait();
	
}
