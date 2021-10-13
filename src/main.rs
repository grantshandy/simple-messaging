mod web;

pub const SERVER_SETTINGS: (&str, u16) = ("127.0.0.1", 8080);

#[actix_web::main]
async fn main() {
    web::server().await;
}
