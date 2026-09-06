use shared::types::ChatMessage;
use shared::types::ClientToServerFrame::{self};
use shared::types::ServerToClientFrame::RecivedMessage;
use shared::types::ServerToClientFrame::{self};
use shared::types::UserHandle;

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
        author: UserHandle::new_random(),
    };

    RecivedMessage(response_message)
}
