use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use todo_server_workshop::{app, Todo};
use tower::{Service, ServiceExt};

mod part1 {
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

fn get_todos_request() -> Request<Body> {
    Request::builder()
        .uri("/todos")
        .body(Body::empty())
        .unwrap()
}

fn post_todo_request(todo: Todo) -> Request<Body> {
    Request::builder()
        .uri("/todos")
        .method(axum::http::Method::POST)
        .header(
            axum::http::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .body(Body::from(serde_json::to_string(&todo).unwrap()))
        .unwrap()
}

fn default_todo() -> Todo {
    Todo {
        id: 1,
        name: "Remember to store the todo".into(),
        completed: false,
    }
}

mod part2 {
    use super::*;

    #[tokio::test]
    async fn returns_empty_list_of_todos() {
        let app = app();

        let response = app.oneshot(get_todos_request()).await.unwrap();

        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }
}

mod part3 {
    use super::*;

    #[tokio::test]
    async fn returns_201_created_on_new_todo() {
        let app = app();

        let todo = default_todo();

        let response = app.oneshot(post_todo_request(todo)).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn persists_a_todo() {
        let mut app = app();

        let todo = default_todo();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(post_todo_request(todo.clone()))
            .await
            .unwrap();

        assert!(response.status().is_success());

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(get_todos_request())
            .await
            .unwrap();

        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(
            serde_json::from_slice::<Vec<Todo>>(&body).unwrap(),
            vec![todo]
        );
    }
}

mod part4 {
    use super::*;

    #[tokio::test]
    async fn can_get_specific_todo() {
        let mut app = app();

        let todo = default_todo();

        // Create todo
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(post_todo_request(todo.clone()))
            .await
            .unwrap();

        assert!(response.status().is_success());

        // Fetch specific todo
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(
                Request::builder()
                    .uri("/todos/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(serde_json::from_slice::<Todo>(&body).unwrap(), todo);
    }

    #[tokio::test]
    async fn fetching_nonexisting_todo_returns_400() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/todos/123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

mod part5 {
    use super::*;

    #[tokio::test]
    async fn can_toggle_todo() {
        let mut app = app();

        let todo = default_todo();

        // Create todo
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(post_todo_request(todo.clone()))
            .await
            .unwrap();

        assert!(response.status().is_success());

        // Toggle the todo
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(
                Request::builder()
                    .uri("/toggle/1")
                    .method(axum::http::Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_success());

        // Fetch the todo and assert its completion status
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(
                Request::builder()
                    .uri("/todos/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_success());

        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(
            serde_json::from_slice::<Todo>(&body).unwrap(),
            Todo {
                completed: true,
                ..todo
            }
        );
    }

    #[tokio::test]
    async fn toggling_nonexisting_todo_returns_400() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/toggle/1")
                    .method(axum::http::Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
