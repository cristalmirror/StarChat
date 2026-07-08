use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::Response,
};
use crate::models::{ClientEvent, ServerEvent};

/*handler of websocket for servers connections*/
pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handler_socket)
}

/**/
async fn handler_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(_) = socket.send(Message::Text(text)).await {
                    break;
                }
            }
            Ok(_) => {
                
            }
            Err(_) => break,
            
        }
    }
}

