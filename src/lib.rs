pub mod conf;
pub mod routes;
pub mod startup;

use actix_web::{
	HttpServer,
	web,
	App,
	HttpRequest,
	Responder,
	HttpResponse,
	dev::Server,
};

use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
	email: String,
	name: String,
}

async fn hc() -> HttpResponse {
	HttpResponse::Ok().finish()
}

async fn subs(_form: web::Form<FormData>) -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run(lis: TcpListener) -> Result<Server, std::io::Error> {
	let s = HttpServer::new(|| {
		App::new()
			.route("/hc", web::get().to(hc))
			.route("/subs", web::post().to(subs))
	}).listen(lis)?
	.run();
	Ok(s)
}



