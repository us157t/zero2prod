use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn hc_works() {
	let addr = spawn_app();
	println!("{}", &addr);
	let cli = reqwest::Client::new();
	let res = cli
		.get(format!("{}/hc", addr))
		.send()
		.await
		.expect("Failed to exe req!!!!!!!!!!!!!!!!!");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());
}

fn spawn_app()  -> String {
	let lis = TcpListener::bind(
		"127.0.0.1:0"
	).expect("Failed to lis!!!");
	let port = lis.local_addr().unwrap().port();
	let s = run(lis).expect("Failed to run");;
	let _ = tokio::spawn(s);
	format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subs_200() {
	let app = spawn_app();
	let cli = reqwest::Client::new();
	let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
	let res = cli
		.post(&format!("{}/subs", &app))
		.header(
			"Content-Type",
			"application/x-www-form-urlencoded"
		).body(body)
		.send()
		.await
		.expect("Failed to execute request.");

	assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn subs_400() {
	let app = spawn_app();
	let cli = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=ursula_le_guin%40gmail.com", "missing the name"),
		("", "missing both name and email")
	];

	for (invalid_body, error_message) in test_cases {
		let res = cli
			.post(&format!("{}/subs", &app))
			.header(
				"Content-Type",
				"application/x-www-form-urlencoded"
			).body(invalid_body)
			.send()
			.await
			.expect("Failed to execute request.");

		assert_eq!(400, res.status().as_u16(),
		"The API did not fail with 400 Bad Request {}",
		error_message
		);

	}


}
