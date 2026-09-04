use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientToServerFrame {
    Login(String),
    SendMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerToClientFrame {
    RecivedMessage,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub author: Arc<User>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub participant: Arc<User>,
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
}
