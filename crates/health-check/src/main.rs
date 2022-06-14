use std::process::exit;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
  let res = reqwest::get(format!(
    "{}://{}:{}/api/ping",
    std::env::var("HC_SCHEME").unwrap_or_else(|_| "http".to_string()),
    std::env::var("HC_HOST").unwrap_or_else(|_| "localhost".to_string()),
    std::env::var("HC_PORT").unwrap_or_else(|_| "8080".to_string())
  ))
  .await
  .expect("Failed to get response from server");

  if res.status().is_success() {
    exit(0)
  } else {
    exit(1)
  }
}
