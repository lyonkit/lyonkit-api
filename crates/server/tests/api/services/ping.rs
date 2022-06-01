use crate::helpers::spawn_app;

#[tokio::test]
async fn ping_works() {
  let app = spawn_app().await;

  let response = app.get("/ping").await;

  assert!(response.status().is_success());
  assert_ne!(Some(0), response.content_length());
  assert_eq!(
    Some("application/json"),
    response
      .headers()
      .get("Content-Type")
      .and_then(|v| v.to_str().ok())
  );

  app.terminate().await;
}
