use iced::message;
use shared::types::{
    ChatMessage,
    ClientToServerFrame::{self, SendMessage},
    ServerToClientFrame::{self, RecivedMessage},
    UserHandle,
};

pub struct Context;
pub struct ConnectionContext;

pub fn handle(
    frame: ClientToServerFrame,
    connection_context: ConnectionContext,
    context: Context,
) -> ServerToClientFrame {
    match frame {
        ClientToServerFrame::SendMessage(message) => {
            send_message(message, connection_context, context)
        }
        ClientToServerFrame::Login(_) => todo!(),
    }
}

fn send_message(
    message: ChatMessage,
    connection_context: ConnectionContext,
    context: Context,
) -> ServerToClientFrame {
    let response_message = ChatMessage {
        content: format!("{} to you!", message.content),
        author: UserHandle::new(),
    };
    RecivedMessage(response_message)
}
