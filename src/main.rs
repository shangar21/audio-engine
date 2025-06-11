use axum::{Router, routing::post};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod routes;
mod utils;

use routes::{receiver, sender};

#[tokio::main]
async fn main() {
    let session_store: utils::SessionMap = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/receive", post(receiver::receive))
        .route("/stop_receive", post(receiver::stop_receive))
        .route("/send", post(sender::send))
        .route("/stop_send", post(sender::stop_send))
        .with_state(session_store.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
