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

#[tokio::test]
async fn post_200() {
	let app_addr = spawn_app();
	let cli = reqwest::Client::new();
	let body = "name=le%20guin";
	let res = cli
		.post(&format!("{}/post_200", &app_addr))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await
		.expect("post_200 err");

	assert_eq!(200, res.status().as_u16());
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
