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

### 2. List todos

Add an endpoint that lists all todos, as a JSON-encoded response. The path of the endpoint should be `/todos` and should accept the `GET`-method.

💡 Tip: This endpoint just needs to return an empty list for now, we don't actually have any todos to return yet.
