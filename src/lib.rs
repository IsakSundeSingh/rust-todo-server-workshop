#![allow(unused)] // Remove me when developing, if you want to

use axum::{
    routing::{get, post},
    Json, Router,
};

mod todo;

pub use todo::Todo;

/// Empty handler, returns 200
async fn empty() {}

async fn todos() -> Json<Vec<Todo>> {
    Json(Vec::new())
}

async fn create_todo(Json(todo): Json<Todo>) {}

pub fn app() -> Router {
    Router::new()
        .route("/", get(empty))
        .route("/todos", get(todos))
        .route("/todos", post(create_todo))
}
