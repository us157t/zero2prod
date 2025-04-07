use zero2prod::spawn_app;
use zero2prod::fmt;
#[tokio::test]
async fn hc() {
	let addr = spawn_app();
	let cli = reqwest::Client::new();
	let res = cli
		.get(fmt(&addr,"hc"))
		.send()
		.await
		.expect("failed 222");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn _200() {
	let app = spawn_app();
	let cli = reqwest::Client::new();

	let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
	let res = cli
		.post(fmt(&app, "subs"))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await
		.expect("Failed to execute req");	
		assert_eq!(200, res.status().as_u16());
}
