mod web;

use web::json_types::Broadcaster;

pub const SERVER_SETTINGS: (&str, u16) = ("0.0.0.0", 8080);

#[actix_web::main]
async fn main() {
    // Ethan: this broadcaster is a type that you can .send() and .recv() messages from on a different thread so you can save it if we need that.
    let broadcaster = Broadcaster::create();

    web::server(broadcaster).await;
}