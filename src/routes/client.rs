use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{Shutdown, State};
use rocket_ws;
use serde_json;

use crate::Operator;
use crate::services::utils::{get_data_file, save_to_file, Data, DataType};

#[post("/message", format = "json", data = "<message>")]
pub async fn client_post_message(message: Json<Data>, config: &State<Operator>) -> Status {
    match message.data_type {
        DataType::MESSAGE => (),
        DataType::PUBKEY => return Status::NotAcceptable,
    }

    let url = format!("http://{}:{}/operator/message", config.address_op, config.port_op);

    let client = reqwest::Client::new();
    let _res = client
        .post(url)
        .json(&message.into_inner())
        .send()
        .await
        .unwrap();
    Status::Accepted
}

#[get("/message")]
pub async fn client_get_message(queue: &State<Sender<Data>>, mut end: Shutdown, ws: rocket_ws::WebSocket) -> rocket_ws::Stream!['static] {
    let mut rx = queue.subscribe();

    rocket_ws::Stream! { ws => 
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

    let url = format!("http://{}:{}/operator/pubkey", config.address_op, config.port_op);

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
