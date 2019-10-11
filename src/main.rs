#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate env_logger;
extern crate log;
extern crate serde_json;

pub mod controllers;
pub mod database;
pub mod models;
pub mod schema;
pub mod services;
pub mod utils;

use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use database::types::{DatabaseManager, DatabasePool};

fn main() -> std::io::Result<()> {
    env_logger::init();
    let manager = DatabaseManager::new(std::env::var("DATABASE_URL").expect("DATABASE_URL"));
    let pool = DatabasePool::builder().build(manager).expect("Failed to create connection pool");
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .expect("Failed to load templates");

    let handlebars_data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(pool.clone())
            .register_data(handlebars_data.clone())
            .service(actix_files::Files::new("/styles", "static/styles/"))
    })
    .bind("127.0.0.1:8088")?
    .run()
}
