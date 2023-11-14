pub use self::error::{Error, Result};

use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use axum::extract::Path;
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::routing::get_service;
use axum::middleware;
use axum::response::Response;
use tower_cookies::CookieManagerLayer;

mod error;
mod web;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
  .merge( routes_hello() )
  .merge( web::routes_login::routes() )
  .layer( middleware::map_response(main_response_mapper) )
  .layer( CookieManagerLayer::new() )
  .fallback_service( routes_static() );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_all.into_make_service())
  .await
  .unwrap();
}


async fn main_response_mapper( res: Response ) -> Response {
    println!("--> {:<12} main_response_mapper","RES_MAPPER");
    println!();
    res
}


fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route( "/hello", get( handler_hello ) )
    .route( "/hello2/:name", get( handler_hello2 ) )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// /hello?name=Maximiliano
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}

// /hello/CarloAcuti
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" ->> {:<12} - hello_handler2 {name:?}","HANDLER");

    Html(format!("<h1>Working Rust! {name}!</h1>"))
}