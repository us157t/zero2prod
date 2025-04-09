use std::thread;
use std::time::Duration;
use actix_web::{App, web, Responder, HttpResponse,HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
	email: String,
	name: String
}

pub fn fmt(x: &str, y: &str) -> String {
	format!("{}/{}", x, y)
}

async fn hc() -> impl Responder {
	HttpResponse::Ok()
}

async fn subs(_form: web::Form<FormData>) -> impl Responder {
	HttpResponse::Ok()
}

pub fn run(lis: TcpListener) -> Result<Server,std::io::Error>{
	let s = HttpServer::new(|| {
		App::new()
			.route("/hc",web::get().to(hc))
			.route("/subs",web::post().to(subs))
	})
	.listen(lis)?
	.run();
	Ok(s)
}

pub fn spawn_app() -> String{
	let lis = TcpListener::bind("127.0.0.1:0")
		.expect("failed to bind random port");
	let port = lis.local_addr().unwrap().port();
	let s = run(lis).expect("failed to bind addr");
	let _ = tokio::spawn(s);
	format!("http://127.0.0.1:{}", port)
}
