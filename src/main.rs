use axum::{Router, routing::post};

mod routes;
mod utils;

use routes::{receiver, sender};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/receive", post(receiver::receive))
        .route("/send", post(sender::send));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
