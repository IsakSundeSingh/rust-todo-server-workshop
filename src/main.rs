use todo_server_workshop::app;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app("todo_server_workshop_db.db".into()).await)
        .await
        .unwrap();
}
