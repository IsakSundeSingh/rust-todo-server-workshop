#![allow(unused)] // Remove me when developing, if you want to

use std::{borrow::BorrowMut, sync::Arc};

use axum::{
    extract::{Path, State},
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

async fn get_todo(
    State(AppState(todos)): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    todos
        .read()
        .await
        .iter()
        .find(|todo| todo.id == id)
        .cloned()
        .ok_or(StatusCode::BAD_REQUEST)
        .map(Json)
}

async fn toggle(State(AppState(todos)): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let toggled = todos
        .write()
        .await
        .iter_mut()
        .find(|todo| todo.id == id)
        .map(|todo| todo.completed = !todo.completed);

    if toggled.is_some() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

#[derive(Clone)]
struct AppState(Arc<RwLock<Vec<Todo>>>);
pub fn app() -> Router {
    let app_state = AppState(Arc::new(RwLock::new(Vec::new())));
    Router::new()
        .route("/", get(empty))
        .route("/todos", get(todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/toggle/:id", post(toggle))
        .with_state(app_state)
}
