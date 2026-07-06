use serde::{Deserialize, Serialize};

/// Eventos que el cliente puede mandar al servidor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientEvent {
    SendMessage { conversation_id: String, content: String },
    Typing { conversation_id: String },
    DeleteMessage { conversation_id: String, message_id: String },
}

/// Eventos que el servidor manda de vuelta al cliente
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    Ack { detail: String },
    Error { detail: String },
}
