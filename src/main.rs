extern crate controller_macro;
use axum::Router;
mod controller;
mod middleware;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(controller::v1_controller::V1Controller::router())
        .merge(controller::v2_controller::V2Controller::router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
