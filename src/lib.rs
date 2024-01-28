#![allow(unused)] // Remove me when developing, if you want to

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};

mod solutions;
mod todo;

use solutions::db;
pub use todo::Todo;
use tokio_rusqlite::Connection;

/// Empty handler, returns 200
async fn empty() {}

async fn todos(State(AppState(connection)): State<AppState>) -> Json<Vec<Todo>> {
    let todos = db::get_todos(&connection).await;
    Json(todos)
}

async fn create_todo(
    State(AppState(connection)): State<AppState>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    db::insert_todo(&connection, todo).await;
    StatusCode::CREATED
}

async fn get_todo(
    State(AppState(connection)): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = db::get_todo(&connection, id).await;
    todo.ok_or(StatusCode::BAD_REQUEST).map(Json)
}

async fn toggle(State(AppState(connection)): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let maybe_todo = db::get_todo(&connection, id).await;

    if let Some(todo) = maybe_todo {
        let toggled = db::update_todo(
            &connection,
            Todo {
                completed: !todo.completed,
                ..todo
            },
        )
        .await;

        if toggled.is_ok() {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        }
    } else {
        StatusCode::BAD_REQUEST
    }
}

async fn update_todo(
    State(AppState(connection)): State<AppState>,
    Json(updated_todo): Json<Todo>,
) -> StatusCode {
    let updated = db::update_todo(&connection, updated_todo).await;

    if updated.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

#[derive(Clone)]
struct AppState(Connection);
pub async fn app(db_path: String) -> Router {
    let connection = Connection::open(db_path).await.unwrap();

    // Ensure table exists
    db::create_todos_table(&connection).await;

    let app_state = AppState(connection);
    Router::new()
        .route("/", get(empty))
        .route("/todos", get(todos))
        .route("/todos", post(create_todo))
        .route("/todos", put(update_todo))
        .route("/todos/:id", get(get_todo))
        .route("/toggle/:id", post(toggle))
        .with_state(app_state)
}
