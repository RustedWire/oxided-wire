#[macro_use]
extern crate rocket;

mod routes;
mod services;

use rocket::serde::Deserialize;
use rocket::tokio::sync::broadcast;
use services::ws_message::ProtoTransaction;

use crate::services::ws_message::{ProtoKeys, ProtoMessage};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Operator {
    address_op: String,
    port_op: u16,
}

#[get("/")]
fn index() -> &'static str {
    "
    API status: up
    "
}

#[launch]
fn rocket() -> _ {

    let test = ProtoTransaction{
        uuid: uuid::Uuid::now_v7(),
        step: 1,
        data: ProtoMessage::KeyExchange(ProtoKeys{
            key: Some([4;32]),
            signature: None,
        }),
    };

    println!("{}", rocket::serde::json::to_pretty_string(&test).unwrap());

    let rocket = rocket::build()
        .manage(broadcast::channel::<ProtoTransaction>(1).0)
        .mount("/", routes![index])
        .mount(
            "/client/",
            routes![
                routes::client::connect,
            ],
        )
        .mount(
            "/operator/",
            routes![
                routes::operator::transmit,
            ],
        );

    let figment = rocket.figment();

    let config: Operator = figment.extract().expect("config");

    println!("{:?}", config);

    rocket.manage(config)
}
