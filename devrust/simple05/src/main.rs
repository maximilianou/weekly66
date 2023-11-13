use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
      .route("/", get(|| async { "main Working..!" }))
      .route("/ping", get(get_ping));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// get 
async fn get_ping() -> &'static str {
    "pong!"
}

#[cfg(test)]
mod test_main{
  use ::axum_test::TestServer;
  use axum::{
    routing::get,
    Router,
  };

  // main test    
  #[tokio::test]
  async fn it_should_work() {
    // Build an application with a route.
    let app = Router::new()
    .route("/", get(|| async { "main Working..!" }));
    // Run the application for testing.
    let server = TestServer::new(app).unwrap();
    // Get the request.
    let response = server
        .get("/")
        .await;
    assert_eq!(response.text(), "main Working..!");
  }
}


#[cfg(test)]
mod test_ping{
    use ::axum_test::TestServer;
    use axum::{
        routing::get,
        Router,
      };    
    use super::{get_ping};
  // get test 
  #[tokio::test]
  async fn it_should_get() {
    // Build an application with a route.
    let app = Router::new()
        .route("/ping", get(get_ping));
    // Run the application for testing.
    let server = TestServer::new(app).unwrap();
    // Get the request.
    let response = server
        .get("/ping")
        .await;
    assert_eq!(response.text(), "pong!");
  }
}

