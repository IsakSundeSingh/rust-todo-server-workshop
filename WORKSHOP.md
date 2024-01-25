# Web-server todo workshop

During this workshop you'll get to know some async Rust, some webserver-handling and some database-persistence.
Afterwards, you'll hopefully have enough to go on to get started with your own project.

## Getting started

You'll be gradually supporting more and more of our wanted "To do"-api. The pre-code comes with a set of tests to help you gradually solve the problems,
so after each test, you can run `cargo test` to run the tests and see if you've completed the assignment or not.
Or if you want to run specific tests, run with a prefix of the test path: `cargo test part1`.

## Requirements

At the end, you'll have the following:

- A HTTP-webserver running locally on port 8080
- The following endpoints
  - `GET /todos` - returns all todos
  - `GET /todos/:id` - returns a specific todo
  - `POST /todos` - Add a new todo, where the todo is a JSON-encoded body
  - `POST /toggle/:id` - Toggle a todo id to complete/uncomplete the todo. Each todo has a unique id
  - `PUT /toggle/:id` - Update a todo with new information
- Todos are persisted between server runs
- Persistence is done with SQLite

## Assignments

### 1. Health check

Add an endpoint that just returns the status code `200 OK` on the path `/`

<details>
<summary>Solution</summary>

Start by opening the `./src/lib.rs`-file, and add an empty handler:

```rust
async fn empty() {}
```

It needs to be async as Axum expects async handlers.
Then you need to add the handler to the router in the `app`-function:

```rust
pub fn app() -> Router {
    Router::new()
        .route("/", get(empty))
}
```

Run your tests, that's it! Empty handlers implicitly return a successful status code.

</details>

### 2. List todos

Add an endpoint that lists all todos, as a JSON-encoded response. The path of the endpoint should be `/todos` and should accept the `GET`-method.

💡 Tip: This endpoint just needs to return an empty list for now, we don't actually have any todos to return yet.

<details>
<summary>Solution</summary>

Add a function called `todos` (or whatever you like, the name doesn't matter). Make the return-type `Json<Vec<Todo>>`. This tells Axum to serialize the return value to JSON, add some info about the content encoding as an header, a return type, etc. Just return an empty vector for now, which will be inferred as a correct type, wrap it in a `Json`-constructor:

```rust
async fn todos() -> Json<Vec<Todo>> {
    Json(Vec::new())
}
```

Register the handler in the router:

```rust
Router::new()
    .route("/", get(empty))
    .route("/todos", get(todos))
```

</details>

### 3. Add a todo

#### 3.1 Accept a todo

Add an endpoint that accepts a JSON-encoded todo and returns `201 Created` and an empty response on creation.

### 3.2 Store the todo

Add app state to store the todos in-memory while the server is running.

This will be tested by creating a todo through the previous endpoint, and fetching it through the first.
