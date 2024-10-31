use zero2prod::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
	let lis = TcpListener::bind("127.0.0.1:0").expect("Main!!!");;
	run(lis)?.await
}
