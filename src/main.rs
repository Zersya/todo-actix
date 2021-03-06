mod models;
mod config;
mod handlers;
mod db;

use actix_web::{web, App, HttpServer, middleware};
use deadpool_postgres::{Runtime, tokio_postgres};
use dotenv::dotenv;
use handlers::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    let pool = config.pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();

    println!("Server running at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todo_list", web::get().to(get_todo_list))
            .route("/todo_list", web::post().to(create_todo_list))
            .route("/todo_list/{list_id}", web::patch().to(update_todo_list))
            .route("/todo_list/{list_id}", web::delete().to(delete_todo_list))
            .route("/todo_list/{list_id}/items", web::get().to(get_todo_items))
            .route("/todo_list/{list_id}/item", web::post().to(create_todo_item))
            .route("/todo_list/{list_id}/item/{item_id}", web::patch().to(update_todo_item))
            .route("/todo_list/{list_id}/item/{item_id}", web::delete().to(delete_todo_item))


    })
        .bind(format!("{}:{}", &config.server.host, &config.server.port))?
        .run()
        .await
}
