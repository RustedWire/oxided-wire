use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::broadcast::Sender;
use rocket::State;

use crate::services::utils::{save_to_file, Data};

#[post("/message", format = "json", data = "<message>")]
pub async fn operator_post_message(message: Json<Data>, queue: &State<Sender<Data>>) -> Status {
    let _res = queue.send(message.into_inner());

    Status::Accepted
}

#[post("/pubkey", format = "json", data = "<pubkey>")]
pub async fn operator_post_pubkey(pubkey: Json<Data>, queue: &State<Sender<Data>>) -> Status {
    save_to_file("bob.pub", &pubkey.data).await;

    let _res = queue.send(pubkey.into_inner());

    Status::Accepted
}
