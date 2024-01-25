#![allow(unused)] // Remove me when developing, if you want to

use axum::{routing::get, Router};

/// Empty handler, returns 200
async fn empty() {}

pub fn app() -> Router {
    Router::new()
}
