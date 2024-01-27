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

<details>
<summary>Solution</summary>

Accepting JSON is quite simple, really. Just create a handler with a signature where the last argument is a type `T` wrapped in `Json<T>` to deserialize the input as JSON. The deserialized value is wrapped in the constructor `Json`, so you can deconstruct and extract your value directly:

```rust
async fn create_todo(Json(todo): Json<Todo>) { }
```

This will not pass the test, however, because a non-failing response returns `200 OK`, not `201 Created`.
Add a return value of `impl IntoResponse`, which means something that can be turned into a response. In our case, we just want to return a status code without a body, so we'll return 201:

```rust
async fn create_todo(Json(todo): Json<Todo>) -> impl IntoResponse {
    StatusCode::CREATED
}
```

And then add it to the router:

```rust
Router::new()
    // other routes
    .route("/todos", post(create_todo))
```

</details>

### 3.2 In-memory shared storage

Add app state to store the todos in-memory while the server is running.

This will be tested by creating a todo through the previous endpoint, and fetching it through the first.

💡 Tip: if you're getting difficult to read error messages when registering handlers (unfortunately common), add a temporary `#[axum_macros::debug_handler]` and compile again for a more descriptive error message!

First, create a shared state. Axum expects it to be clonable and shared among threads, so wrapping the state in `Arc`, makes sense, but you'll also need exclusive access to the list when pushing new todos. You'll need to wrap the list in another layer of a concurrency locking primitive, something that will give you _mutually exclusive_ access between threads.

<details>
<summary>Solution</summary>

**Shared state**

Create a struct `AppState` to store your state, make it cloneable using a derive-macro. Add your vector of todos inside.

```rust
#[derive(Clone)]
struct AppState(Vec<Todo>);
```

This alone will make your data cloned between threads, but won't work because it will mutate state on a single thread and would give data races. Mutually exclusive access can be done through a `Mutex` or some other lock. `std` provides an implementation, but so does tokio, which supports async and doesn't block the thread.

Using a `Mutex` would be a fine solution, however we know that our program will be highly concurrent with many readers and few writers, so a read-write lock would be preferrable, instead of locking other readers out while reading todos:

```rust
#[derive(Clone)]
struct AppState(Arc<RwLock<Vec<Todo>>>);
```

**Register the state**

To register a shared state, simply add `with_state` to your router. Remember that the state applies to all routes registered above it, so if you register a route that needs the state beneath the `.with_state`-call, you'll get a confusing error.
Initialize an empty state and register it as such:

```rust
let app_state = AppState(Arc::new(RwLock::new(Vec::new())));
Router::new()
    // other routes
    .with_state(app_state)
```

**Adding shared mutable state to handlers**

Now we need to modify our handlers to actually add the todos and extract them on the getter.
In Axum, type-safety is important, however the error messages are not always easily understood. If you arrange extractors of input data (form data/json) and shared state extractors the wrong way, you get a confusing error message. However, adding `#[axum_macros::debug_handler]` to your handlers will make them significantly easier to debug.

Modify the post handler to be as such:

```rust
async fn create_todo(
    State(AppState(todos)): State<AppState>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    let mut todos = todos.write().await;
    todos.push(todo);
    StatusCode::CREATED
}
```

And your getter to be as such:

```rust
async fn todos(State(AppState(todos)): State<AppState>) -> Json<Vec<Todo>> {
    Json(todos.read().await.to_vec())
}
```

And that's it!

</details>

### 4 Fetching a specific todo

Add an endpoint that fetches a specific todo. Make sure to return 400 on nonexisting todos. The endpoint should be on the form `/todos/123` for the todo with id 123

💡 Tip: Handlers can return results, returning different things upon success or errors

<details>
<summary>Solution</summary>

Add a handler which extracts a reference to the shared state, and also a path to extract the id:

```rust
async fn get_todo(
    State(AppState(todos)): State<AppState>,
    Path(id): Path<u32>,
)
```

On success, we want to return the JSON-encoded todo, but on failure we want to return 400 Bad request (it may not be exactly the best return code, but let's forget about that for a while). Change the signature to add a return type:

```rust
async fn get_todo(
    State(AppState(todos)): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode>
```

In the body, we want to get a reader lock to the todos and find the specific todo. If we don't find a todo, map it to a result with the error as a status-code and the ok value as a JSON-encoded todo. It can actually be done as a one-liner like this:

```rust
todos
    // Get reader-lock future
    .read()
    // Await it, while not blocking the thread
    .await
    // Create an iterator over the todos, returns references
    .iter()
    // Find the specific id
    .find(|todo| todo.id == id)
    // Convert from an Option<&T> to Option<T> by cloning it
    .cloned()
    // Map the option to a result where the error value is a status code
    .ok_or(StatusCode::BAD_REQUEST)
    // Wrap the Ok-value in the Json-constructor
    .map(Json)
```

Add the handler as a route, using `:id` to signify a path variable:

```rust
Router::new()
    // other routes
    .route("/todos/:id", get(get_todo))
    .with_state(app_state)
```

</details>

### 5. Toggling todos

Add an endpoint that toggles the completion of a todo using `POST /toggle/:id` as the path and method. Return 400 on an invalid todo id and 200 with an empty body on success.

<details>
<summary>Solution</summary>

Add an endpoint with the same arguments signature types as the previous, returning a `StatusCode`:

```rust
async fn toggle(State(AppState(todos)): State<AppState>, Path(id): Path<u32>) -> StatusCode
```

The body is similar to the previous, and can be mostly pipelined:

```rust
let toggled = todos
    // Get a writer-lock
    .write()
    .await
    // Iterate over the list mutably
    .iter_mut()
    // Find the specific todo
    .find(|todo| todo.id == id)
    // Toggle its completion state if found
    .map(|todo| todo.completed = !todo.completed);

// Return the appropriate status code
if toggled.is_some() {
    StatusCode::OK
} else {
    StatusCode::BAD_REQUEST
}
```

And register the handler:

```rust
Router::new()
    // other routes
    .route("/toggle/:id", post(toggle))
```

</details>
