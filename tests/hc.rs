use std::net::TcpListener;
use zero2prod::run;
#[tokio::test]
async fn hc_works() {
	let add = spawn_app();
	dbg!(&add);
	let cli = reqwest::Client::new();
	let res = cli
		.get(format!("{}/greet", add))
		.send()
		.await
		.expect("Failed to exe req");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());
		
}

fn spawn_app()  -> String{
	let lis = TcpListener::bind("127.0.0.1:0")
		.expect("Failed to bind ");
	let port = lis.local_addr().unwrap().port();
	dbg!(&port);
	let s = run(lis).expect("spawn app err");
	let _ = tokio::spawn(s);
	format!("http://127.0.0.1:{}", port)
}
