use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;

pub async fn get_todo_list(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client
        .prepare("SELECT * FROM todo_list ORDER BY id DESC limit 10")
        .await
        .unwrap();

    let todo_list = client
        .query(&statement, &[])
        .await
        .expect("Failed to get todo list")
        .iter()
        .map(|row| TodoList::from(row))
        .collect::<Vec<TodoList>>();

    Ok(todo_list)
}

pub async fn create_todo_list(client: &Client, title: &String) -> Result<TodoList, io::Error> {
    let statement = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING *")
        .await
        .unwrap();

    client
        .query(&statement, &[&title])
        .await
        .expect("Failed to create todo list")
        .iter()
        .map(|row| TodoList::from(row))
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Failed to parsed return todo list",
        ))
}

pub async fn update_todo_list(
    client: &Client,
    title: &String,
    id: i32,
) -> Result<TodoList, io::Error> {
    let statement = client
        .prepare("UPDATE todo_list SET title = $1 WHERE id = $2 RETURNING *")
        .await
        .unwrap();

    client
        .query(&statement, &[&title, &id])
        .await
        .expect("Failed to update todo list")
        .iter()
        .map(|row| TodoList::from(row))
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no_todo_list_found",
        ))
}

pub async fn delete_todo_list(client: &Client, id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM todo_list WHERE id = $1")
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&id])
        .await
        .expect("Failed to delete todo list");

    match result {
        ref deleted if (*deleted == 1) => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "nothing_changed",
        )),
    }
}

pub async fn create_todo_item(
    client: &Client,
    title: &String,
    todo_list_id: i32,
) -> Result<TodoItem, io::Error> {
    let statement = client
        .prepare("INSERT INTO todo_item (title, todo_list_id) VALUES ($1, $2) RETURNING *")
        .await
        .unwrap();

    client
        .query(&statement, &[&title, &todo_list_id])
        .await
        .expect("Failed to create todo item")
        .iter()
        .map(|row| TodoItem::from(row))
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Failed to parsed return todo item",
        ))
}

pub async fn get_todo_items(
    client: &Client,
    todo_list_id: i32,
) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("SELECT * FROM todo_item WHERE todo_list_id = $1 ORDER BY id DESC limit 10")
        .await
        .unwrap();

    let todo_items = client
        .query(&statement, &[&todo_list_id])
        .await
        .expect("Failed to get todo items")
        .iter()
        .map(|row| TodoItem::from(row))
        .collect::<Vec<TodoItem>>();

    Ok(todo_items)
}

pub async fn update_todo_item(
    client: &Client,
    title: &String,
    checked: Option<bool>,
    id: i32,
) -> Result<TodoItem, io::Error> {
    let statement = client
        .prepare("UPDATE todo_item SET title = $1, checked = $2  WHERE id = $3 RETURNING *")
        .await
        .unwrap();

    client
        .query(&statement, &[&title, &checked, &id])
        .await
        .expect("Failed to update todo item")
        .iter()
        .map(|row| TodoItem::from(row))
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no_todo_item_found",
        ))
}

pub async fn delete_todo_item(client: &Client, id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM todo_item WHERE id = $1")
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&id])
        .await
        .expect("Failed to delete todo list");

    match result {
        ref deleted if (*deleted == 1) => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "nothing_changed",
        )),
    }
}
