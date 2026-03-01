use std::sync::Arc;

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
pub struct Converstation {
    pub participant: Arc<User>,
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
}
