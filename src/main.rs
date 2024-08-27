extern crate controller_macro;
use axum::Router;
use test_macro::controller::{v1_controller::V1Controller, v2_controller::V2Controller};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(V1Controller::router())
        .merge(V2Controller::router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
