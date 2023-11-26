#[macro_use]
extern crate rocket;

mod routes;
mod services;

use rocket::tokio::sync::broadcast;
use services::utils::Data;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build()
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
        )
}
