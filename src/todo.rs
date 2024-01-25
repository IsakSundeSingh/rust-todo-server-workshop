#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Todo {
    pub id: u32,
    pub name: String,
    pub completed: bool,
}
