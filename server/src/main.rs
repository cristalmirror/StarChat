use axum::{routing::get, Router,Json};
use serde_json::{json, Value};

async fn health_check() -> Json<Value> {

    // return 
    Json(json!({ "status":"ok"}))
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listen Server: [{}]",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
