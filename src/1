use zero2prod::spawn_app;
#[tokio::test]
async fn hc() {
	spawn_app();;
	let cli = reqwest::Client::new();
	let res = cli
		.get("http://127.0.0.1:8000/hc")
		.send()
		.await
		.expect("failed 222");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());
}
