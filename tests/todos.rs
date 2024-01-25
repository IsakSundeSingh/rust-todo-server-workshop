use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use todo_server_workshop::app;
use tower::ServiceExt;

#[tokio::test]
async fn returns_empty_200_at_index() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
