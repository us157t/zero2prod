use zero2prod::run;
use zero2prod::conf::get_configuration;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()>{
	let conf = get_configuration().expect("Failed to read conf");
	let addr = format!("127.0.0.1:{}", conf.application_port);
	let lis = TcpListener::bind(
		addr
	)?;
	run(lis)?.await
}
