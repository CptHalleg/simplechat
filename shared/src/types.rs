use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

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
    pub url: String,
    pub user_name: String,
}

impl UserHandle {
    pub fn new_random() -> Self {
        let mut rng = rand::rng();
        UserHandle {
            url: String::from(rng.random::<char>()),
            user_name: String::from(rng.random::<char>()),
        }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub participant: User,
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
}
