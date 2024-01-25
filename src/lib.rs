#![allow(unused)] // Remove me when developing, if you want to

use axum::{routing::get, Json, Router};

mod todo;

use todo::Todo;

/// Empty handler, returns 200
async fn empty() {}

async fn todos() -> Json<Vec<Todo>> {
    Json(Vec::new())
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(empty))
        .route("/todos", get(todos))
}
