use crate::database::types::*;
use crate::models::user::{User, UserError};
use crate::utils::errors::RyobError;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginForm {
    user_name: String,
    password: String,
}

#[derive(Serialize)]
struct PreviousLoginForm {
    user_name: String,
}

fn sanitize_login_form(form: &LoginForm) -> LoginForm {
    LoginForm {
        user_name: form.user_name.trim().to_owned(),
        password: form.password.clone(),
    }
}

fn login_form_to_previous(form: &LoginForm) -> PreviousLoginForm {
    PreviousLoginForm {
        user_name: form.user_name.to_owned(),
    }
}

pub fn post(hb: web::Data<Handlebars>, pool: web::Data<DatabasePool>, session: Session, form: web::Form<LoginForm>) -> Result<HttpResponse, RyobError> {
    let connection = pool.get()?;
    let sanitized_form = sanitize_login_form(&form);
    let user_result = User::login(&connection, &form.user_name, &form.password);
    match user_result {
        Err(UserError::BadLogin) => {
            let previous = login_form_to_previous(&sanitized_form);
            let data = json!({ "errors": ["Incorrect username or password"], "previous": previous });
            let page = hb.render("pages/login", &data)?;
            Ok(HttpResponse::BadRequest().body(page))
        }
        Err(error) => Err(RyobError::from(error)),
        Ok(user) => {
            user.to_session(&session)?;
            Ok(HttpResponse::Found().header(actix_web::http::header::LOCATION, "/").finish())
        }
    }
}

pub fn get(hb: web::Data<Handlebars>) -> Result<HttpResponse, RyobError> {
    let data = json!({});
    let page = hb.render("pages/login", &data)?;
    Ok(HttpResponse::Ok().body(page))
}
