use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main(){
    //creating health check
    let app = Router::new().route("/", get(|| async {"Welcome to Lexigraph"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
