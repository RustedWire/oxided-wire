use bincode;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use crate::services::mqtt::MQTT;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    size: u64,
    data: Vec<u8>,
}

#[post("/message", format = "json", data = "<message>")]
pub fn client_post_message(mqtt_borker: &State<MQTT>, message: Json<Message>) -> Status {
    let msg = match bincode::serialize(&message.0) {
        Ok(ser) => ser,
        Err(_) => return Status::InternalServerError,
    };
    // TODO: Do a post to other operator
    mqtt_borker.publish("client/message", &msg).unwrap();

    Status::Accepted
}

#[get("/message")]
pub fn client_get_message(mqtt_borker: &State<MQTT>) -> Vec<Json<Message>> {
    let messages = mqtt_borker.get_messages("client/message");

    let mut new_vec: Vec<Json<Message>> = Vec::new();
    for msg in messages {
        let msg_struct: Message = bincode::deserialize(msg.payload()).unwrap();
        new_vec.push(Json(msg_struct));
    }

    new_vec
}