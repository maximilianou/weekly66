use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    routing::{get, post},
    Json, Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_testing=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

//    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//        .await
//        .unwrap();
//    tracing::debug!("listening on {}", listener.local_addr().unwrap());
//    axum::serve(listener, app()).await.unwrap();
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
  .serve(app().into_make_service())
  .await
  .unwrap();
}

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        .route(
            "/requires-connect-into",
            get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move { format!("Hi {addr}") }),
        )
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
}



//use axum::{
//    routing::get,
//    Router,
//};
//
//#[tokio::main]
//async fn main() {
//    // build our application with a single route
//    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
//
//    // run it with hyper on localhost:3000
//    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//        .serve(app.into_make_service())
//        .await
//        .unwrap();
//}





//#[test]
//fn my_test() {
//    tokio::runtime::Builder::new_multi_thread()
//        .enable_all()
//        .build()
//        .unwrap()
//        .block_on(async {
//            assert!(true);
//        })
//}