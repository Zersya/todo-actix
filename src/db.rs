use crate::models::{ TodoList };
use deadpool_postgres::Client;
use std::io;
use actix_web::web::Json;

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

pub async fn create_todo_list(client: &Client, todo_list: Json<TodoList>) -> Result<(), io::Error> {

    let statement = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1)", )
        .await
        .unwrap();

    client
        .execute(&statement, &[&todo_list.title])
        .await
        .expect("Failed to create todo list");

    Ok(())
}