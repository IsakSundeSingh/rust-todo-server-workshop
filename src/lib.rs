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

pub use todo::Todo;
use tokio_rusqlite::Connection;

pub async fn app(_db_path: String) -> Router {
    Router::new()
}
