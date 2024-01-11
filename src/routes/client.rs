use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{Shutdown, State};
use rocket_ws;
use serde_json;

use crate::services::utils::{get_data_file, save_to_file, Data, DataType};
use crate::{Operator, ServerInfo, ServerRole};

/// Route to send message to the operator that will distribute it.
#[post("/message", format = "json", data = "<message>")]
pub async fn client_post_message(message: Json<Data>, config: &State<Operator>) -> Status {
    match message.data_type {
        DataType::MESSAGE => (),
        DataType::PUBKEY => return Status::NotAcceptable,
    }

    let url = format!(
        "http://{}:{}/operator/message",
        config.address_op, config.port_op
    );

    let client = reqwest::Client::new();
    let _res = client
        .post(url)
        .json(&message.into_inner())
        .send()
        .await
        .unwrap();
    Status::Accepted
}

/// Route used to get messages from the other client via the operator using Websockets.
#[get("/message")]
pub async fn client_get_message(
    queue: &State<Sender<Data>>,
    mut end: Shutdown,
    ws: rocket_ws::WebSocket,
) -> rocket_ws::Stream!['static] {
    let mut rx = queue.subscribe();

    rocket_ws::Stream! { ws =>
        let _ = ws;
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield rocket_ws::Message::Text(serde_json::to_string(&msg).unwrap());
        }
    }
}

#[post("/pubkey", format = "json", data = "<pubkey>")]
pub async fn client_post_pubkey(pubkey: Json<Data>, config: &State<Operator>) -> Status {
    match pubkey.data_type {
        DataType::MESSAGE => return Status::NotAcceptable,
        DataType::PUBKEY => (),
    }

    save_to_file("alice.pub", &pubkey.data).await;

    let url = format!(
        "http://{}:{}/operator/pubkey",
        config.address_op, config.port_op
    );

    let client = reqwest::Client::new();
    let _res = client
        .post(url)
        .json(&pubkey.into_inner())
        .send()
        .await
        .unwrap();

    Status::Accepted
}

#[get("/pubkey")]
pub async fn client_get_pubkey() -> Option<Json<Data>> {
    let data = match get_data_file("bob.pub").await {
        Ok(data) => {
            println!("{:?}", data);
            Some(Json(Data {
                data_type: DataType::PUBKEY,
                data,
            }))
        }
        Err(_) => None,
    };

    data
}

#[get("/role_operator")]
pub async fn client_get_role_operator(
    server_info: &State<ServerInfo>,
    config: &State<Operator>,
) ->Json<ServerRole> {
    let mut role_lock = server_info.role.lock().await;
    let role = match *role_lock{
        ServerRole::None => {
            let client = reqwest::Client::new();
            let other_server_url = format!(
                "http://{}:{}/operator/suid",
                config.address_op, config.port_op
            );

            let role = match client.get(other_server_url).send().await {
                Ok(resp) => {
                    let mut role = ServerRole::None;
                    if resp.status().is_success() {
                        role = match server_info
                            .id
                            .cmp(&resp.text().await.unwrap().parse::<usize>().unwrap())
                        {
                            std::cmp::Ordering::Less => ServerRole::Leader,
                            std::cmp::Ordering::Equal => ServerRole::None,
                            std::cmp::Ordering::Greater => ServerRole::Follower,
                        };

                        *role_lock = role.clone();
                        
                    }
                    role
                }
                Err(_) => ServerRole::None,
            };

            role
        }
        ServerRole::Follower => ServerRole::Follower,
        ServerRole::Leader => ServerRole::Leader,
    };

    Json(role)
}
