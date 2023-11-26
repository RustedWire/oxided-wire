#[macro_use]
extern crate rocket;

mod routes;
mod services;

use services::mqtt::MQTT;

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
        .mount("/", routes![index])
        .mount("client/", routes![routes::client::client_post_message])
        .manage(MQTT::new())
}
