use tokio_rusqlite::Connection;

use crate::Todo;

const CREATE_TODO_TABLE_SQL: &str = include_str!("./create_todo_table.sql");

pub(crate) async fn create_todos_table(connection: &Connection) {
    connection
        .call(|conn| Ok(conn.execute(CREATE_TODO_TABLE_SQL, [])?))
        .await
        .expect("creating todo table failed");
}

pub(crate) async fn insert_todo(connection: &Connection, todo: Todo) {
    connection
        .call_unwrap(move |connection| {
            connection
                .execute(
                    "INSERT INTO todos (id, name, completed) VALUES (?1, ?2, ?3)",
                    (todo.id, todo.name, todo.completed),
                )
                .unwrap();
        })
        .await
}

pub(crate) async fn get_todo(connection: &Connection, id: u32) -> Option<Todo> {
    connection
        .call(move |conn| {
            let mut stmt = conn
                .prepare("SELECT id, name, completed FROM todos WHERE id = ?1;")
                .unwrap();
            let result = stmt.query_row([id], |row| {
                Ok(Todo {
                    id: row.get_unwrap(0),
                    name: row.get_unwrap(1),
                    completed: row.get_unwrap(2),
                })
            });

            Ok(result?)
        })
        .await
        .ok()
}

pub(crate) async fn get_todos(connection: &Connection) -> Vec<Todo> {
    connection
        .call_unwrap(|connection| {
            let mut stmt = connection
                .prepare("SELECT id, name, completed FROM todos;")
                .unwrap();
            let result: Result<Vec<_>, _> = stmt
                .query([])
                .unwrap()
                .mapped(|row| {
                    Ok(Todo {
                        id: row.get_unwrap(0),
                        name: row.get_unwrap(1),
                        completed: row.get_unwrap(2),
                    })
                })
                .collect();
            result
        })
        .await
        .expect("fetching todos failed")
}

pub(crate) async fn update_todo(connection: &Connection, updated: Todo) -> Result<(), ()> {
    let result = connection
        .call(move |connection| {
            connection
                .execute(
                    "UPDATE todos SET name = ?1, completed = ?2 WHERE id = ?3",
                    (updated.name, updated.completed, updated.id),
                )
                .map_err(Into::into)
        })
        .await;

    match result {
        // If the connection updated zero rows, it did not exist
        Ok(0) => Err(()),
        Ok(_) => Ok(()),
        _ => Err(()),
    }
}
