use crate::models::{ TodoList };
use deadpool_postgres::Client;
use std::io;

pub async fn get_todo_list(client: &Client) -> Result<Vec<TodoList>, io::Error> {

    let statement = client
        .prepare("SELECT * FROM todo_list ORDER BY id DESC")
        .await
        .unwrap();

    let todo_list = client.query(&statement, &[])
        .await
        .expect("Failed to get todo list")
        .iter()
        .map(|row| TodoList::from(row))
        .collect::<Vec<TodoList>>();

    Ok(todo_list)
}

pub async fn create_todo_list(client: &Client, title: &String) -> Result<(), io::Error> {

    let statement = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1)")
        .await
        .unwrap();

    client
        .execute(&statement, &[&title])
        .await
        .expect("Failed to create todo list");

    Ok(())
}

pub async fn update_todo_list(client: &Client, title: &String, id: i32) -> Result<(), io::Error> {

    let statement = client
        .prepare("UPDATE todo_list SET title = $1 WHERE id = $2")
        .await
        .unwrap();

    client
        .execute(&statement, &[&title, &id])
        .await
        .expect("Failed to update todo list");

    Ok(())
}

pub async fn delete_todo_list(client: &Client, id: i32) -> Result<(), io::Error> {

    let statement = client
        .prepare("DELETE FROM todo_list WHERE id = $1")
        .await
        .unwrap();

    client
        .execute(&statement, &[&id])
        .await
        .expect("Failed to delete todo list");

    Ok(())
}