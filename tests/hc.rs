use zero2prod::run;
#[tokio::test]
async fn hc_works() {
	spawn_app();

	let cli = reqwest::Client::new();
	let res = cli
		.get("http://127.0.0.1:3000/greet")
		.send()
		.await
		.expect("Failed to exe req");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());
		
}

fn spawn_app()  {
	let s = run().expect("spawn app err");
	let _ = tokio::spawn(s);
}
