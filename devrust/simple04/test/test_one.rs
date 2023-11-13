use ::axum::Router;
use ::axum::extract::Json;
use ::axum::routing::put;
use ::axum_test::TestServer;
use ::serde_json::json;
use ::serde_json::Value;


async fn put_user(Json(user): Json<Value>) -> () {
    // todo
}


let my_app = Router::new()
    .route("/users", put(put_user));

let server = TestServer::new(my_app)?;

let response = server.put("/users")
    .content_type(&"application/json")
    .json(&json!({
        "username": "Terrance Pencilworth",
    }))
    .await;