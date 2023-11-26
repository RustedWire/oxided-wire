use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{Shutdown, State};

use crate::services::utils::{save_to_file, Data, DataType};

#[post("/message", format = "json", data = "<message>")]
pub async fn client_post_message(message: Json<Data>) -> Status {
    match message.data_type {
        DataType::MESSAGE => (),
        DataType::PUBKEY => return Status::NotAcceptable,
    }

    let client = reqwest::Client::new();
    let _res = client
        .post("http://127.0.0.1:8000/operator/message")
        .json(&message.into_inner())
        .send()
        .await
        .unwrap();
    Status::Accepted
}

#[get("/message")]
pub async fn client_get_message(queue: &State<Sender<Data>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[post("/pubkey", format = "json", data = "<pubkey>")]
pub async fn client_post_pubkey(pubkey: Json<Data>) -> Status {
    match pubkey.data_type {
        DataType::MESSAGE => return Status::NotAcceptable,
        DataType::PUBKEY => (),
    }

    save_to_file("alice.pub", &pubkey.data).await;

    let client = reqwest::Client::new();
    let _res = client
        .post("http://127.0.0.1:8000/operator/pubkey")
        .json(&pubkey.into_inner())
        .send()
        .await
        .unwrap();

    Status::Accepted
}

#[get("/pubkey")]
pub async fn client_get_pubkey(queue: &State<Sender<Data>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}
