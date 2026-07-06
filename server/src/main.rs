
mod models;
mod client_connection_manager;
/*axum headers*/
use axum::{routing::get, Router, Json};
/*seder headers*/
use serde_json::{json, Value};
use client_connection_manager::ws_handler;

async fn health_check() -> Json<Value> {
    // return 
    Json(json!({ "status":"ok"}))
}

/*Main funcitions*/
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ws", get(ws_handler));
    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listen Server: [{}]",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
