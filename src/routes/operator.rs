use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::broadcast::Sender;
use rocket::State;

use crate::services::ws_message::ProtoTransaction;

#[post("/transmit", format = "json", data = "<transac_struct>")]
pub async fn transmit(transac_struct: Json<ProtoTransaction>, queue: &State<Sender<ProtoTransaction>>) -> Status {
    let _res = queue.send(transac_struct.into_inner());

    Status::Accepted
}