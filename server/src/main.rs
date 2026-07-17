mod models;
/*code mod of client connection*/
mod client_connection_manager;
/*code mod of server connection*/
mod server_connection_manager;


/*axum headers*/
use axum::{routing::get, Router, Json};
/*seder headers*/
use serde_json::{json, Value};
use tokio::join;
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
    
    let http_addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(http_addr).await.unwrap();
    println!("[HTTP/WS] -> Listen Server: [{}]",listener.local_addr().unwrap());
    let http_server = axum::serve(listener, app);

    /*Server connections manager usin gRPC*/
    let grpc_addr = "0.0.0.0:50051".parse().unwrap();
    println!("[gRPC] -> Listen Server: [{}]",grpc_addr);

    let grpc_server = TonicServer::builder()
        .add_service(ServerLinkServer::new(MyServerLink::default()))
        .serve(grpc_addr);
    /**/
    println!("[ST_MAIN] <<Starting HTTP on {} and gPRC on {}>>", http_addr, grpc_addr);

    println!("[ST-MAIN] -> Server init success. Perss Ctrl+C if close these Server.");
    let _ = join!(http_server, grpc_server);
}
