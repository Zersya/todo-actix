use std::io;

use crate::db;
use crate::models::{RequestTodoItem, RequestTodoList, Status, TodoItem, TodoList};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        success: true,
        message: "OK".to_string(),
        data: Some(()),
    })
}

pub async fn get_todo_list(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_todo_list(&client).await;

    match result {
        Ok(todo_list) => HttpResponse::Ok().json(Status::<Vec<TodoList>> {
            success: true,
            message: "OK".to_string(),
            data: Some(todo_list),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn create_todo_list(
    db_pool: web::Data<Pool>,
    todo_list: web::Json<RequestTodoList>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::create_todo_list(&client, &todo_list.title).await;

    match result {
        Ok(todo_list) => HttpResponse::Created().json(Status {
            success: true,
            message: "OK".to_string(),
            data: Some(todo_list),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn update_todo_list(
    db_pool: web::Data<Pool>,
    params: web::Path<(i32,)>,
    todo_list: web::Json<RequestTodoList>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::update_todo_list(&client, &todo_list.title, params.0).await;

    // TODO: need new method for duplicates response
    match result {
        Ok(todo_list) => HttpResponse::Ok().json(Status {
            success: true,
            message: "OK".to_string(),
            data: Some(todo_list),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn delete_todo_list(
    db_pool: web::Data<Pool>,
    params: web::Path<(i32,)>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::delete_todo_list(&client, params.0).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(Status {
            success: true,
            message: "OK".to_string(),
            data: Some(()),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn get_todo_items(db_pool: web::Data<Pool>, params: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_todo_items(&client, params.0).await;

    match result {
        Ok(todo_items) => HttpResponse::Ok().json(Status::<Vec<TodoItem>> {
            success: true,
            message: "OK".to_string(),
            data: Some(todo_items),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn create_todo_item(
    db_pool: web::Data<Pool>,
    params: web::Path<(i32,)>,
    todo_item: web::Json<RequestTodoItem>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::create_todo_item(&client, &todo_item.title, params.0).await;

    match result {
        Ok(todo_item) => HttpResponse::Created().json(Status {
            success: true,
            message: "OK".to_string(),
            data: Some(todo_item),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn update_todo_item(
    db_pool: web::Data<Pool>,
    params: web::Path<(i32, i32)>,
    todo_item: web::Json<RequestTodoItem>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::update_todo_item(&client, &todo_item.title, todo_item.checked, params.1).await;

    match result {
        Ok(todo_item) => HttpResponse::Ok().json(Status {
            success: true,
            message: "OK".to_string(),
            data: Some(todo_item),
        }),
        Err(e) => error_handler(e),
    }
}

pub async fn delete_todo_item(
    db_pool: web::Data<Pool>,
    params: web::Path<(i32, i32)>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::delete_todo_item(&client, params.1).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(Status {
            success: true,
            message: "OK".to_string(),
            data: Some(()),
        }),
        Err(e) => error_handler(e),
    }
}

fn error_handler(error: io::Error) -> HttpResponse {
    match error.kind() {
        io::ErrorKind::NotFound => HttpResponse::NotFound().json(Status {
            success: false,
            message: "Not Found".to_string(),
            data: Some(()),
        }),
        io::ErrorKind::InvalidInput => HttpResponse::BadRequest().json(Status {
            success: false,
            message: error.to_string(),
            data: Some(()),
        }),
        _ => HttpResponse::InternalServerError().json(Status {
            success: false,
            message: "Internal Server Error".to_string(),
            data: Some(()),
        }),
    }
}
