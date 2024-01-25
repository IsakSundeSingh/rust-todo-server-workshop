#![allow(unused)] // Remove me when developing, if you want to

use std::{borrow::BorrowMut, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

mod todo;

pub use todo::Todo;
use tokio::sync::RwLock;

/// Empty handler, returns 200
async fn empty() {}

async fn todos(State(AppState(todos)): State<AppState>) -> Json<Vec<Todo>> {
    Json(todos.read().await.to_vec())
}

async fn create_todo(
    State(AppState(todos)): State<AppState>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    let mut todos = todos.write().await;
    todos.push(todo);
    StatusCode::CREATED
}

#[derive(Clone)]
struct AppState(Arc<RwLock<Vec<Todo>>>);
pub fn app() -> Router {
    let app_state = AppState(Arc::new(RwLock::new(Vec::new())));
    Router::new()
        .route("/", get(empty))
        .route("/todos", get(todos))
        .route("/todos", post(create_todo))
        .with_state(app_state)
}
