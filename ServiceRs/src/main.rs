use axum::{http::StatusCode, Router, Json, routing::get};
use rand::Rng;
use serde_json::{Value, json};
use tokio::net::TcpListener;

fn init_router() -> Router {
  Router::new()
    .route("/api", get(handle_api))
}

async fn handle_api() -> Result<Json<Value>, StatusCode> {
  let random_float: f64 = rand::thread_rng().r#gen();
  //70% chance of success
  if random_float <= 0.7 {
    return Ok(Json(json!({
      "value": random_float
    })));
  }
  return Err(StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::main]
async fn main() {
  let app = init_router();

  // Bind to a port using tokio::net::TcpListener
  let addr = "127.0.0.1:4000";
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Listening on {}", addr);

  // Use axum::serve with the listener and the router
  axum::serve(listener, app)
      .await
      .unwrap(); 

}
