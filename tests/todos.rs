use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use todo_server_workshop::app;
use tower::ServiceExt;

mod part_1 {
    use super::*;

    #[tokio::test]
    async fn returns_empty_200_at_index() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

mod part2 {
    use super::*;

    #[tokio::test]
    async fn returns_empty_list_of_todos() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/todos")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }
}
