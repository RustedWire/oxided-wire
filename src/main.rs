#[macro_use]
extern crate rocket;

mod routes;
mod services;

use std::net::IpAddr;
use std::str::FromStr;

use rand::Rng;
use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::{broadcast, Mutex};
use rocket::{Build, Rocket};
use services::utils::Data;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Operator {
    address: String,
    address_op: String,
    port_op: u16,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum ServerRole {
    Leader,
    Follower,
    None,
}

pub struct ServerInfo {
    id: usize,
    role: Mutex<ServerRole>,
}

#[rocket::async_trait]
impl Fairing for ServerInfo {
    fn info(&self) -> Info {
        Info {
            name: "ID & role informations",
            kind: Kind::Ignite | Kind::Liftoff | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        // If the string doesn't have an ip format this should fail
        let config: Operator = rocket.figment().extract().expect("config");
        match IpAddr::from_str(&config.address.as_str()) {
            Ok(_) => (),
            Err(_) => return fairing::Result::Err(rocket),
        }

        let mut rng = rand::thread_rng();

        let id_ip = &config.address.as_str().replace(".", "");
        let id_ext = rng.gen_range(0..99);

        let formated_id = format!("{}{}", id_ip, id_ext);

        let uid = formated_id.parse::<usize>().unwrap();

        let id_role = ServerInfo {
            id: uid,
            role: Mutex::new(ServerRole::None),
        };

        let rocket = rocket.manage(id_role);

        fairing::Result::Ok(rocket)
    }
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
        .attach(ServerInfo {
            id: 0,
            role: Mutex::new(ServerRole::None),
        })
        .manage(broadcast::channel::<Data>(1).0) // Use the channel to pass data from the operator receiving route and the client sending websocket
        .mount("/", routes![index])
        .mount(
            "/client/",
            routes![
                routes::client::client_post_message,
                routes::client::client_get_message,
                routes::client::client_post_pubkey,
                routes::client::client_get_pubkey,
                routes::client::client_get_role_operator,
            ],
        )
        .mount(
            "/operator/",
            routes![
                routes::operator::operator_post_message,
                routes::operator::operator_post_pubkey,
                routes::operator::operator_suid,
            ],
        );

    let config: Operator = rocket.figment().extract().expect("config");
    rocket.manage(config)
}
