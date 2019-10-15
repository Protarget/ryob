use crate::database::types::*;
use crate::models::user::User;
use crate::utils::errors::RyobError;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

pub fn get(hb: web::Data<Handlebars>, pool: web::Data<DatabasePool>, session: Session) -> Result<HttpResponse, RyobError> {
    let connection = pool.get()?;
    let user = User::from_session(&connection, &session)?;
    let data = json!({ "user": user });
    let page = hb.render("pages/index", &data)?;
    Ok(HttpResponse::Ok().body(page))
}
