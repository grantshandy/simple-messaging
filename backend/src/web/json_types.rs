use std::fmt::{Display, Formatter, Result as FmtResult};
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};

use actix_web::http::StatusCode;
use actix_web::web::{Bytes, Data};
use actix_web::{web, ResponseError};
use futures::Stream;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub msg: String,
    pub status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string(self).unwrap())
    }
}

impl ResponseError for Error {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub text: String,
    pub user: String,
    pub time: String,
}

impl Display for StoredMessage {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string(self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub user: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string(self).unwrap())
    }
}

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        Data::new(Mutex::new(Broadcaster::new()))
    }

    pub fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&self, msg: String) {
        let msg = Bytes::from(["data: ", msg.as_str(), "\n\n"].concat());

        for client in self.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
