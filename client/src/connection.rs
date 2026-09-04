use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::{Stream, StreamExt, select};
use iced::stream;
use shared::types::ClientToServerFrame;
use shared::types::ServerToClientFrame;

use crate::connection::ConnectionCommand::Connect;

pub enum ConnectionEvent {
    Connected(mpsc::Sender<ConnectionCommand>),
    Received(ServerToClientFrame),
}

pub enum ConnectionCommand {
    Connect { url: String, user_name: String },
    Send(ClientToServerFrame),
}

pub fn connect() -> impl Stream<Item = ConnectionEvent> {
    stream::channel(100, async |mut output| {
        let (cmd_sender, mut cmd_receiver) = mpsc::channel::<ConnectionCommand>(100);
        output
            .send(ConnectionEvent::Connected(cmd_sender))
            .await
            .unwrap();
        let mut connection: Option<(ewebsock::WsSender, ewebsock::WsReceiver)> = None;

        loop {
            match connection.as_mut() {
                Some(_) => (),
                None => {
                    let cmd = cmd_receiver.select_next_some().await;
                    match cmd {
                        ConnectionCommand::Connect { url, user_name } => {
                            let options = ewebsock::Options::default();
                            let (mut web_sender, web_reciver) =
                                ewebsock::connect(url, options).unwrap();

                            web_sender.send(ewebsock::WsMessage::Text("Hello!".into()));
                            connection = Some((web_sender, web_reciver));
                        }
                        ConnectionCommand::Send(frame) => match connection.as_mut() {
                            None => (),
                            Some((web_sender, _)) => {
                                web_sender.send(ewebsock::WsMessage::Text(
                                    serde_json::to_string(&frame).unwrap(),
                                ));
                            }
                        },
                    }
                }
            }
        }

        //web_sender.send(ewebsock::WsMessage::Text("Hello!".into()));
        //while let Some(event) = web_reciver.try_recv() {
        //    println!("Received {:?}", event);
        //}
    })
}
