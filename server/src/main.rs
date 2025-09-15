#![allow(unused)] // temporary
use axum::response::Html;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("hello <strong>world!!</strong>") }),
    );
    // region --start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    print!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap()
}
