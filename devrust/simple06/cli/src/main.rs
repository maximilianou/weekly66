use axum::response::Html;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", 
        get(|| async { Html("<h1>Working Rust!</h1>") } )
    );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_hello.into_make_service())
  .await
  .unwrap();
}
