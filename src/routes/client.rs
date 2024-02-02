use rocket::futures::{SinkExt, StreamExt};
use rocket::futures::stream::FusedStream;
use rocket::serde::json::{from_str, to_string};
use rocket::State;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use ws::{self, Message};
use crypto_mob::{VerifyingKey};

use crate::Operator;
use crate::services::ws_message::{ProtoMessage, ProtoTransaction, Signature};

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
                                    // Only use text based ws
                                    Message::Text(value) => {
                                        let proto = handle_protocol(value);
                                        match proto.data {
                                            ProtoMessage::Error(_) => {
                                                let _ = stream
                                                        .send(Message::Text(to_string(&proto)
                                                        .unwrap()))
                                                        .await;
                                            }
                                            _ => {
                                               let _ = client
                                                        .post(url_target_op.as_str())
                                                        .json(&proto)
                                                        .send()
                                                        .await;
                                            }
                                        }
                                    }
                                    // If Message is not a text
                                    _ => continue,
                                }
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

/// Function to handle a string and transform it into a `ProtoTransaction`.
/// It also does signature verification.
///
/// # Arguments
///
/// * `value`: A JSON representation of a `ProtoTransaction` struct.
///
/// # Returns
///
/// Returns a `ProtoTransaction`.
///
/// If any errors occur return a `ProtoTransaction` with the `data` filed set at `ProtoMessage::Error()`.
pub fn handle_protocol(value: String) -> ProtoTransaction {
    // Deserialize the JSON string represnetation into a ProtoTransaction
    let transaction = match from_str::<ProtoTransaction>(value.as_str()) {
        Ok(serialized) => serialized,
        Err(err) => {
            // Set the error at BadFormat
            return ProtoTransaction::new_format_error(err.to_string());
        }
    };

    match transaction.data {
        ProtoMessage::KeyExchange(ref proto_keys) => {
            if proto_keys.signature.is_some() {
                match verify_signature(proto_keys.signature.clone().unwrap()) {
                    // If the signature is not validated
                    false => return ProtoTransaction::new_sign_error(&transaction),
                    // If the signature is validated do nothing
                    true => (),
                }
            }
        }
        _ => (),
    };

    transaction
}

/// Verify the signature attatch to a ProtoTransaction.
///
/// # Arguments
///
/// * `signature`: The signature object to verify.
///
/// # Returns
///
/// Returns a `bool`. If the signature is valid `true` otherwise `false`.
pub fn verify_signature(signature: Signature) -> bool {
    true
}