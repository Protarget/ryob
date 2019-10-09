use actix_web::Responder;
use actix_web::web::{self, HttpResponse};
use actix_session::Session;
use crate::database::types::DatabasePool;
use crate::services::user::UserError;
use crate::models::user::User;
use log;

const MIN_USERNAME_SIZE: usize = 2;
const MAX_USERNAME_SIZE: usize = 64;
const MIN_PASSWORD_SIZE: usize = 8;
const MAX_PASSWORD_SIZE: usize = 256;

#[derive(serde::Deserialize)]
pub struct RegistrationForm {
    pub user_name: String,
    pub password: String,
    pub password_confirm: String
}

#[derive(serde::Serialize)]
pub struct RegistrationFormPrevious {
    pub user_name: String
}

enum RegistrationFormValidationError {
    NameTooShort,
    NameTooLong,
    NameInvalidCharacters,
    PasswordTooShort,
    PasswordTooLong,
    PasswordInsecure,
    PasswordInvalidCharacters,
    PasswordMismatch
}

fn registration_form_sanitize(form: &RegistrationForm) -> RegistrationForm {
    RegistrationForm {
        user_name: form.user_name.trim().to_owned(),
        password: form.password.clone(),
        password_confirm: form.password_confirm.clone()
    }
}

fn registration_form_validate(form: &RegistrationForm) -> Result<(), RegistrationFormValidationError> {
    if form.user_name.len() < MIN_USERNAME_SIZE {
        Err(RegistrationFormValidationError::NameTooShort)
    }
    else if form.user_name.len() > MAX_USERNAME_SIZE {
        Err(RegistrationFormValidationError::NameTooLong)
    }
    else if !form.user_name.chars().all(|c| c == ' ' || c.is_alphanumeric()) {
        Err(RegistrationFormValidationError::NameInvalidCharacters)
    }
    else if form.password.len() < MIN_PASSWORD_SIZE {
        Err(RegistrationFormValidationError::PasswordTooShort)
    }
    else if form.password.len() > MAX_PASSWORD_SIZE {
        Err(RegistrationFormValidationError::PasswordTooLong)
    }
    else if !form.password.chars().all(|c| c.is_ascii_punctuation() || c.is_alphanumeric()) {
        Err(RegistrationFormValidationError::PasswordInvalidCharacters)
    }
    else if form.password != form.password_confirm {
        Err(RegistrationFormValidationError::PasswordMismatch)
    }
    else {
        Ok(())
    }
}

fn registration_form_to_previous(form: &RegistrationForm) -> RegistrationFormPrevious {
    RegistrationFormPrevious {
        user_name: form.user_name.clone()
    }
}

fn registration_form_invalid_handler(hb: &handlebars::Handlebars, form: &RegistrationForm, error: RegistrationFormValidationError) -> HttpResponse {
    log::info!("Failed to register user: \"{}\"", form.user_name);
    let message = match error {
        RegistrationFormValidationError::NameTooShort => "Name must be between 2 and 64 alpha-numeric characters or spaces",
        RegistrationFormValidationError::NameTooLong => "Name must be between 2 and 64 alpha-numeric characters or spaces",
        RegistrationFormValidationError::NameInvalidCharacters => "Name must be between 2 and 64 alpha-numeric characters or spaces",
        RegistrationFormValidationError::PasswordTooShort => "Password must be between 8 and 256 alpha-numeric characters or punctuation symbols",
        RegistrationFormValidationError::PasswordTooLong => "Password must be between 8 and 256 alpha-numeric characters or punctuation symbols",
        RegistrationFormValidationError::PasswordInvalidCharacters => "Password must be between 8 and 256 alpha-numeric characters or punctuation symbols",
        RegistrationFormValidationError::PasswordInsecure => "Password is not secure enough",
        RegistrationFormValidationError::PasswordMismatch => "Passwords do not match"
    };
    
    let data = json!({"error": message, "previous": registration_form_to_previous(form)});
    let page = hb.render("register", &data);
    match page {
        Ok(p) => HttpResponse::BadRequest().header(actix_web::http::header::CONTENT_TYPE, "text/html; charset=utf-8").body(p),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

fn registration_form_ok_handler(hb: &handlebars::Handlebars, user: &User, session: &Session) -> HttpResponse {
    log::info!("Successfully to registered user: \"{}\" with ID {}", user.user_name, user.id);
    match user.to_session(session) {
        Ok(_) => HttpResponse::Found().header(actix_web::http::header::LOCATION, "/").finish(),
        Err(e) => registration_form_error_handler(hb, e)
    }
}

fn registration_form_error_handler(hb: &handlebars::Handlebars, error: UserError) -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

pub fn register_get(hb: web::Data<handlebars::Handlebars>) -> impl Responder {
    let data = json!({});
    let page = hb.render("register", &data);
    match page {
        Ok(p) => HttpResponse::Ok().body(p),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub fn register_post(hb: web::Data<handlebars::Handlebars>, pool: web::Data<DatabasePool>, form: web::Form<RegistrationForm>, session: Session) -> impl Responder {
    let registration_form = registration_form_sanitize(&form);
    let registration_form_validation_result = registration_form_validate(&registration_form);
    match registration_form_validation_result {
        Err(e) => registration_form_invalid_handler(&hb, &registration_form, e),
        Ok(_) => {
            let connection = pool.get().unwrap();
            let user_result = User::register(&connection, &registration_form.user_name, &registration_form.password);
            match user_result {
                Ok(u) => registration_form_ok_handler(&hb, &u, &session),
                Err(e) => registration_form_error_handler(&hb, e)
            }
        }
    }
}