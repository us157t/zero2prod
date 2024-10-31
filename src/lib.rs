use actix_web::{
	web,
	App,
	HttpServer,
	HttpRequest,
	Responder,
	HttpResponse,
};

use actix_web::dev::Server;

async fn greet() -> impl Responder {
	HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
	let s = HttpServer::new(|| {
		App::new()
			.route("/greet", web::get().to(greet))
	})
	.bind("127.0.0.1:3000")?
	.run();

	Ok(s)
}

