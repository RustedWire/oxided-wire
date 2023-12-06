#[macro_use]
extern crate rocket;

mod routes;
mod services;

use rocket::serde::Deserialize;
use rocket::tokio::sync::broadcast;
use services::utils::Data;

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
    let rocket = rocket::build()
        .manage(broadcast::channel::<Data>(1).0)
        .mount("/", routes![index])
        .mount(
            "/client/",
            routes![
                routes::client::client_post_message,
                routes::client::client_get_message,
                routes::client::client_post_pubkey,
                routes::client::client_get_pubkey,
            ],
        )
        .mount(
            "/operator/",
            routes![
                routes::operator::operator_post_message,
                routes::operator::operator_post_pubkey,
            ],
        );

    let figment = rocket.figment();

    let config: Operator = figment.extract().expect("config");

    println!("{:?}", config);

    rocket.manage(config)
}
