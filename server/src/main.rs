mod models;
/*code mod of client connection*/
mod client_connection_manager;
/*code mod of server connection*/
mod server_connection_manager;


/*axum headers*/
use axum::{routing::get, Router, Json};
/*seder headers*/
use serde_json::{json, Value};
use client_connection_manager::ws_handler;
use server_connection_manager::{
    replication::server_link_server::ServerLinkServer,
    MyServerLink,
};

use tonic::transport::Server as TonicServer;

async fn health_check() -> Json<Value> {
    // return 
    Json(json!({ "status":"ok"}))
}

/*Main funcitions*/
#[tokio::main]
async fn main() {
    /*Client connections manager usin HTTP/WebSockets*/
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ws", get(ws_handler));
    

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[HTTP/WS] -> Listen Server: [{}]",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    /*Server connections manager usin gRPC*/
    let grpc_addr = "0.0.0.0:50051".parse().unwrap();
    println!("[gRPC] -> Listen Server: [{}]",grpc_addr);

    let grpc_server = TonicServer::builder()
        .add_service(ServerLinkServer::new(MyServerLink::default()))
        .serve(grpc_addr);

    let _ = tokio::join!(http_server, grpc_server);
}
