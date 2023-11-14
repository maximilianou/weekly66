// cli/src/web/routes_login.rs
use crate::{Result, Error};
use serde::Deserialize;
use axum::Json;
use serde_json::Value;
use serde_json::json;
use axum::Router;
use axum::routing::post;

// export interface routes
pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}


async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>>{
  println!("--> {:<12} - api_login","HANDLER");

  //TODO: Implement db

  if payload.username != "demo" || payload.pwd != "demo" {
    return Err(Error::LoginFail);
  }

  //TODO: Set cookies

  let body = Json(json!({
    "result":{
        "success": true,
    }
  }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
