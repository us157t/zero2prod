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


#[derive(serde::Deserialize)]
struct FD {
	name: String
}

async fn greet() -> impl Responder {
	HttpResponse::Ok()
}

async fn post_200(_form: web::Form<FD>) -> HttpResponse {
	HttpResponse::Ok().finish()
}

fn index(form: web::Form<FD>) -> String {
	format!("Welcome {}", form.name)
}

pub fn run(lis: TcpListener) -> Result<Server, std::io::Error> {
	let s = HttpServer::new(|| {
		App::new()
			.route("/greet", web::get().to(greet))
			.route("/post_200", web::post().to(post_200))
	})
	.listen(lis)?
	.run();

	Ok(s)
}

