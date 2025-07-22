use axum::{
    routing::get,
    Router,
};
use std::env;
use dotenvy::dotenv;
use tracing_subscriber;
use crate::db::init_db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = init_db(&db_url).await;

    let app = Router::new().route("/", get(|| async {"welcome to lexigraph"}));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
