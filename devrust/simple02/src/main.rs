use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/hello", get(|| async { "We are moving forward!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub mod filters{
    use warp::Filter;
    use super::handlers;

    pub fn list() ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{ 
        warp::path!("hello")
            .and(warp::get())
            .and_then(handlers::handle_list)
    }
}

mod handlers{
    use warp::http::StatusCode;
    use std::convert::Infallible;

    pub async fn handle_list() -> Result<impl warp::Reply, Infallible> {
        // "Alright, alright, alright", Matthew said.
        Ok(StatusCode::ACCEPTED)
    }
}







