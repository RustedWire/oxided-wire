use crate::services::ws_message::ProtoTransaction;
use crate::Operator;
use rocket::futures::stream::FusedStream;
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::json::{to_string, from_str};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::State;
use ws::{self, Message};

/// Enables the client to connect to a websocket to exchanges with the server
#[get("/connect")]
pub async fn connect(
    config: &State<Operator>,
    queue: &State<Sender<ProtoTransaction>>,
    ws: ws::WebSocket,
) -> ws::Channel<'static> {
    let mut rx = queue.subscribe();

    let url_target_op = format!(
        "http://{}:{}/operator/transmit",
        config.address_op, config.port_op
    );
    let client = reqwest::Client::new();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            // Main loop of the connection
            loop {
                select! {
                    // Branch that sends data from the server to the client
                    recv_message = rx.recv() => {
                        match recv_message {
                            Ok(recv_msg) => {
                                let json_string = to_string(&recv_msg);
                                let _ = stream.send(Message::Text(json_string.unwrap())).await;
                            }
                            Err(RecvError::Closed) => break,
                            // POC system is unlikely to lag, if it does ignore it
                            Err(RecvError::Lagged(_)) => (),
                        }

                    }
                    // Branch that send data from the client to the server
                    send_message = stream.next() => {
                        match send_message {
                            Some(message) => {
                                match message.unwrap() {
                                    Message::Text(value) => {
                                        let proto_struct: ProtoTransaction = from_str(value.as_str()).unwrap();
                                        let _ = client.post(url_target_op.as_str()).json(&proto_struct).send().await;
                                    }
                                    _ => continue,
                                }
                                //
                            },
                            None => continue,
                        }
                    }
                }
                //If the stream cannot be polled for more data end the loop to end connection
                if stream.is_terminated() {
                    break;
                }
            }

            Ok(())
        })
    })
}
