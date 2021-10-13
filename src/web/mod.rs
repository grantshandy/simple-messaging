pub mod json_types;

use crate::{BACKLOG_PATH, SERVER_SETTINGS};
use std::sync::Mutex;

use actix_web::Responder;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer};
use chrono::prelude::*;
use futures_util::StreamExt;
use json_types::{Broadcaster, Error, Message, StoredMessage};

pub async fn server(broadcaster: Data<Mutex<Broadcaster>>) {
    println!("Starting Server!");

    let server = match HttpServer::new(move || {
        App::new()
            .app_data(broadcaster.clone())
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

    let stored_message = StoredMessage {
        text: message.text,
        user: message.user,
        time: now.to_rfc3339(),
    };

    fstream::write_text(BACKLOG_PATH, format!("{}\n", stored_message.to_string()), false);

    // Send the message to the broadcaster
    broadcaster
        .lock()
        .unwrap()
        .send(format!("{}\n", stored_message.to_string()));

    // Show that we're working!
    println!("{} [{}]: {}", now, stored_message.user, stored_message.text);

    Ok(HttpResponse::Ok().finish())
}

async fn stream_messages(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(rx)
}
