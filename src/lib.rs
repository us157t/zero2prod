use std::thread;
use std::time::Duration;
use actix_web::{App, web, Responder, HttpResponse,HttpServer};
use actix_web::dev::Server;

async fn hc() -> impl Responder {
	HttpResponse::Ok()
}

pub fn run() -> Result<Server,std::io::Error>{
	let s = HttpServer::new(|| {
		App::new()
			.route("/hc",web::get().to(hc))
	})
	.bind("127.0.0.1:8000")?
	.run();
	Ok(s)
}

pub fn spawn_app() {
	let s = run().expect("failed to bind addr");
	let _ = tokio::spawn(s);
}
