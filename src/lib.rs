use actix_web::{
	web,
	App,
	HttpServer,
	HttpRequest,
	Responder,
	HttpResponse,
};
use std::net::TcpListener;
use actix_web::dev::Server;

async fn greet() -> impl Responder {
	HttpResponse::Ok()
}

pub fn run(lis: TcpListener) -> Result<Server, std::io::Error> {
	let s = HttpServer::new(|| {
		App::new()
			.route("/greet", web::get().to(greet))
	})
	.listen(lis)?
	.run();

	Ok(s)
}

