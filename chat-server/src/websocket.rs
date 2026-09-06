use iced::{futures, message};

use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use log::{error, info};
use shared::types::{ClientToServerFrame, ServerToClientFrame};
use warp::filters::ws::Message;
use warp::ws::WebSocket;

use crate::handlers::{ConnectionContext, Context, handle};
pub async fn user_connected(ws: WebSocket) {
    info!("websocket connected");
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (mut tx, mut rx) = mpsc::unbounded::<ServerToClientFrame>();

    tokio::task::spawn(async move {
        info!("send task running");
        while let Some(frame) = rx.next().await {
            info!("sending frame:\n{:#?}", frame);
            let text: String = serde_json::to_string(&frame).unwrap().into();
            user_ws_tx
                .send(Message::text(text))
                .await
                .unwrap_or_else(|e| {
                    error!("websocket send error: {e}");
                });
        }
        info!("send task stopping");
    });
    tokio::task::spawn(async move {
        info!("recieve task running");
        while let Some(result) = user_ws_rx.next().await {
            match result {
                Ok(message) => match message.to_str() {
                    Ok(text) => {
                        let recieved_frame =
                            serde_json::from_str::<ClientToServerFrame>(text).unwrap();
                        let send_frame = handle(recieved_frame, ConnectionContext, Context);
                        tx.send(send_frame).await.unwrap();
                    }
                    Err(_) => (),
                },
                Err(error) => error!("failed to get next message: {:#?}", error),
            }
        }
        info!("recieve task stopping");
    });
}
