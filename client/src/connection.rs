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
        let connection: Option<(ewebsock::WsSender, ewebsock::WsReceiver)> = None;

        loop {
            match connection {
                Some((mut web_sender, web_reciever)) => {
                    select! {
                        cmd = cmd_receiver.select_next_some() => println!(""),
                        con = web_reciever.try_recv() => println!(""),
                    }
                }
                None => {
                    let cmd = cmd_receiver.select_next_some().await;
                    match cmd {
                        ConnectionCommand::Connect { url, user_name } => {
                            let options = ewebsock::Options::default();
                            let (mut web_sender, mut web_reciver) =
                                ewebsock::connect(url, options).unwrap();

                            web_sender.send(ewebsock::WsMessage::Text("Hello!".into()));
                        }
                        ConnectionCommand::Send(frame) => match connection {
                            None => (),
                            Some((mut web_sender, web_reciever)) => {
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
