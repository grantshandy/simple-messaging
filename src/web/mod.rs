mod json_types;

use crate::SERVER_SETTINGS;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::RwLock;

use actix_web::Responder;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer};
use chrono::prelude::*;
use futures_util::StreamExt;
use json_types::{Broadcaster, Error, Message};

pub async fn server() {
    let data = Broadcaster::create();

    let server = match HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/message").route(web::post().to(send_message)))
            .route("/stream_messages", web::get().to(stream_messages))
        })
    .bind(SERVER_SETTINGS)
    {
        Ok(data) => data,
        Err(error) => {
            eprintln!(
                "Can't bind to {}:{}, Error: {}",
                SERVER_SETTINGS.0, SERVER_SETTINGS.1, error
            );
            std::process::exit(1);
        }
    };

    match server.run().await {
        Ok(_) => (),
        Err(error) => eprintln!("Error while running the server: {}", error),
    }
}

async fn send_message(
    mut payload: web::Payload,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, Error> {
    let now = Utc::now();

    // Stream our payload into bytes...
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        let item = match item {
            Ok(data) => data,
            Err(error) => {
                return Err(Error {
                    msg: format!("Payload Error: {}", error),
                    status: 100,
                })
            }
        };

        bytes.extend_from_slice(&item);
    }

    // Create a json object from the bytes...
    let message = match serde_json::from_slice::<Message>(&bytes) {
        Ok(data) => data,
        Err(error) => {
            return Err(Error {
                msg: format!("Bad JSON syntax: {}", error),
                status: 200,
            })
        }
    };

    // Send the message to the broadcaster
    broadcaster
        .lock()
        .unwrap()
        .send(format!("{}\n", message).as_str());

    // Show that we're working!
    println!("{} [{}]: {}", now, message.user, message.text);

    Ok(HttpResponse::Ok().finish())
}

async fn stream_messages(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(rx)
}
