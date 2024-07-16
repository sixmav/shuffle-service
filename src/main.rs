use std::{env, io};

use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::web;
use actix_web::{http, App, HttpServer};
use futures::FutureExt;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let app_host = env::var("APP_HOST").expect("APP_HOST not found!");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found!");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found!");

    HttpServer::new(move || {
        App::new()
            .wrap()
            .app_data()
            .wrap()
            .configure()
    })
    .bind(&app_url)?
    .run()
    .await
}
