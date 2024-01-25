#[derive(Debug, serde::Serialize)]
pub(crate) struct Todo {
    id: u32,
    name: String,
    completed: bool,
}
