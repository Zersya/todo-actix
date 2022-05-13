use serde::{ Serialize, Deserialize };
use tokio_postgres::Row;

#[derive(Serialize)]
pub struct Status {
    pub code: u32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UpdateParams {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

impl From<&Row> for TodoList {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
        }
    }
}

impl From<&Row> for TodoItem {
    fn from(row: &Row) -> Self {
        TodoItem {
            id: row.get("id"),
            title: row.get("title"),
            checked: row.get("checked"),
            list_id: row.get("list_id"),
        }
    }
}