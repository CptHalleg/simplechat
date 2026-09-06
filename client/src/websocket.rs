use async_tungstenite::tokio::ConnectStream;
use iced::futures;
use iced::futures::stream::Fuse;
use iced::task::Never;
use iced::task::Sipper;
use iced::task::sipper;

use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use async_tungstenite::WebSocketStream;
use async_tungstenite::tungstenite;
use log::debug;
use log::info;
use shared::types::ClientToServerFrame;
use shared::types::ServerToClientFrame;

pub fn connect() -> impl Sipper<Never, Event> {
    sipper(async |mut output| {
        let (sender, mut instructions) = mpsc::channel::<Instruction>(100);
        output.send(Event::Initialized(WebSocket(sender))).await;

        let mut connection: Option<Fuse<WebSocketStream<ConnectStream>>> = None;

        loop {
            match connection.as_mut() {
                Some(websocket) => {
                    debug!("websocket is initialized, waiting for instruction or frame...");
                    futures::select! {
                        received = websocket.select_next_some() => {
                            debug!("websocket frame received:\n{:#?}", received);
                            match received {
                                Ok(tungstenite::Message::Text(message)) => {
                                    let parsed = serde_json::from_str::<ServerToClientFrame>(&message);
                                    match parsed {
                                        Ok(frame) => output.send(Event::MessageReceived(frame)).await,
                                        Err(_) => info!("failed to parse message {:#?}", message),
                                    }
                                }
                                Err(_) => {
                                    output.send(Event::Disconnected).await;
                                    connection = None;
                                }
                                Ok(_) => debug!("recieved unknown message type"),
                            }
                        }
                        instruction = instructions.select_next_some() => {
                            debug!("instruction received:\n{:#?}", instruction);
                            match instruction {
                                Instruction::SendMessage(frame) =>{
                                    let text = serde_json::to_string(&frame).unwrap().into();
                                    match websocket.send(tungstenite::Message::Text(text)).await {
                                        Ok(()) => info!("send message:\n{:#?}", frame),
                                        Err(_) => info!("failed to send message:\n{:#?}", frame),
                                    }},
                                Instruction::Disconnect => {
                                    websocket.close().await.unwrap();
                                },
                                Instruction::Connect(_) => debug!("cant connect without websocket"),
                            }
                        }
                    }
                }
                None => {
                    debug!("websocket is not initialized, waiting for instructions...");
                    let instruction = instructions.select_next_some().await;
                    debug!("instruction received:\n{:#?}", instruction);
                    match instruction {
                        Instruction::Connect(address) => {
                            match async_tungstenite::tokio::connect_async(format!(
                                "ws://{}",
                                address
                            ))
                            .await
                            {
                                Ok((websocket, _)) => {
                                    connection = Some(websocket.fuse());
                                    output.send(Event::Connected).await;
                                }
                                Err(error) => {
                                    output
                                        .send(Event::Error(format!(
                                            "connection failed:\n{:#?}",
                                            error
                                        )))
                                        .await;
                                }
                            }
                        }
                        Instruction::SendMessage(_) => {
                            debug!("cant send message without websocket")
                        }
                        Instruction::Disconnect => {
                            debug!("cant disconnect without websocket")
                        }
                    }
                }
            }
        }
    })
}

#[derive(Debug, Clone)]
pub enum Event {
    Initialized(WebSocket),
    Connected,
    Disconnected,
    MessageReceived(ServerToClientFrame),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct WebSocket(mpsc::Sender<Instruction>);

#[derive(Debug, Clone)]
pub enum Instruction {
    Connect(String),
    Disconnect,
    SendMessage(ClientToServerFrame),
}

impl WebSocket {
    pub async fn send(&mut self, intsruction: Instruction) {
        self.0.send(intsruction).await.unwrap()
    }
}
