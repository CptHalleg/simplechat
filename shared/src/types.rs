use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{range, sync::Arc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientToServerFrame {
    Login(String),
    SendMessage(ChatMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerToClientFrame {
    RecivedMessage(ChatMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserHandle {
    id: u64,
}

impl UserHandle {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        UserHandle { id: rng.random() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub author: UserHandle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub handle: UserHandle,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub participant: Arc<User>,
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
}
