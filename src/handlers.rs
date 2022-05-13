use crate::models::{ Status, TodoList };
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, HttpResponse, Responder};


pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        code: 200,
        message: "OK".to_string(),
    })
}

pub async fn get_todo_list(db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_todo_list(&client).await;

    match result {
        Ok(todo_list) => {
            HttpResponse::Ok().json(todo_list)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub async fn create_todo_list(db_pool: web::Data<Pool>, todo_list: web::Json<TodoList>) -> impl Responder {

    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::create_todo_list(&client, todo_list).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(Status {
                code: 201,
                message: "OK".to_string(),
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}