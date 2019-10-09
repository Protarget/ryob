#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;
extern crate actix_web;
#[macro_use]
extern crate log;

pub mod utils;
pub mod models;
pub mod schema;
pub mod database;
pub mod controllers;
pub mod services;

use utils::id::Id;
use models::user; 

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_session::{Session, CookieSession};
use database::types::{DatabaseConnection, DatabaseManager, DatabasePool};

fn main() -> std::io::Result<()> {
    let manager = DatabaseManager::new(std::env::var("DATABASE_URL").expect("DATABASE_URL"));
    let pool = DatabasePool::builder()
        .build(manager)
        .expect("Failed to create connection pool");
        
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .expect("Failed to load templates");
        
    let handlebars_data = web::Data::new(handlebars);
    
    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32])
                .secure(false))
            .data(pool.clone())
            .register_data(handlebars_data.clone())
            .service(actix_files::Files::new("/styles", "static/styles/"))
            .route("/register", web::get().to(controllers::register::register_get))
            .route("/register", web::post().to(controllers::register::register_post))
    })
    .bind("127.0.0.1:8088")?
    .run()
}
