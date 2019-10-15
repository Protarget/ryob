use crate::database::types::*;
use crate::models::user::{User, UserError};
use crate::utils::errors::RyobError;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterForm {
    user_name: String,
    password: String,
    confirm_password: String,
}

#[derive(Serialize)]
struct PreviousRegisterForm {
    user_name: String,
}

#[derive(Clone, Copy)]
enum RegisterFormValidationError {
    UserNameTooShort,
    UserNameTooLong,
    UserNameInvalidCharacters,
    PasswordTooShort,
    PasswordInsecure,
    PasswordNotConfirmed,
    PasswordInvalidCharacters,
}

const MIN_PASSWORD_SIZE: usize = 8;
const MIN_USER_NAME_SIZE: usize = 2;
const MAX_USER_NAME_SIZE: usize = 128;

fn sanitize_register_form(form: &RegisterForm) -> RegisterForm {
    RegisterForm {
        user_name: form.user_name.trim().to_owned(),
        password: form.password.clone(),
        confirm_password: form.confirm_password.clone(),
    }
}

fn register_form_to_previous(form: &RegisterForm) -> PreviousRegisterForm {
    PreviousRegisterForm {
        user_name: form.user_name.to_owned(),
    }
}

fn is_valid_user_name_char(username_char: char) -> bool {
    username_char.is_alphanumeric() || username_char == ' '
}

fn validate_user_name(name: &String) -> Vec<RegisterFormValidationError> {
    let mut errors: Vec<RegisterFormValidationError> = vec![];
    if name.len() < MIN_USER_NAME_SIZE {
        errors.push(RegisterFormValidationError::UserNameTooShort);
    } else if name.len() > MAX_USER_NAME_SIZE {
        errors.push(RegisterFormValidationError::UserNameTooLong);
    }

    if !name.chars().all(is_valid_user_name_char) {
        errors.push(RegisterFormValidationError::UserNameInvalidCharacters);
    }
    errors
}

fn is_valid_password_char(password_char: char) -> bool {
    password_char.is_alphanumeric() || password_char == ' '
}

fn validate_password(password: &String, confirm_password: &String) -> Vec<RegisterFormValidationError> {
    let mut errors: Vec<RegisterFormValidationError> = vec![];
    if password != confirm_password {
        errors.push(RegisterFormValidationError::PasswordNotConfirmed);
    }
    if password.len() < MIN_PASSWORD_SIZE {
        errors.push(RegisterFormValidationError::PasswordTooShort);
    }
    if !password.chars().all(is_valid_password_char) {
        errors.push(RegisterFormValidationError::PasswordInvalidCharacters);
    }
    errors
}

fn validate_register_form(form: &RegisterForm) -> Vec<RegisterFormValidationError> {
    let mut user_name_errors = validate_user_name(&form.user_name);
    let mut password_errors = validate_password(&form.password, &form.confirm_password);
    let mut errors = vec![];
    errors.append(&mut user_name_errors);
    errors.append(&mut password_errors);
    errors
}

fn validation_error_to_string(error: RegisterFormValidationError) -> String {
    match error {
        RegisterFormValidationError::UserNameTooShort => format!("Username must be between {} and {} characters long", MIN_USER_NAME_SIZE, MAX_USER_NAME_SIZE),
        RegisterFormValidationError::UserNameTooLong => format!("Username must be between {} and {} characters long", MIN_USER_NAME_SIZE, MAX_USER_NAME_SIZE),
        RegisterFormValidationError::UserNameInvalidCharacters => "Username must consist of alphanumeric characters and spaces".to_owned(),
        RegisterFormValidationError::PasswordInsecure => "Password not secure".to_owned(),
        RegisterFormValidationError::PasswordTooShort => format!("Password must be {} or more characters long", MIN_PASSWORD_SIZE),
        RegisterFormValidationError::PasswordInvalidCharacters => "Password must consist of alphanumeric characters and spaces".to_owned(),
        RegisterFormValidationError::PasswordNotConfirmed => "Passwords did not match".to_owned(),
    }
}

pub fn post(hb: web::Data<Handlebars>, pool: web::Data<DatabasePool>, session: Session, form: web::Form<RegisterForm>) -> Result<HttpResponse, RyobError> {
    let sanitized_form = sanitize_register_form(&form);
    let validation_errors = validate_register_form(&sanitized_form);
    if validation_errors.len() > 0 {
        let previous = register_form_to_previous(&sanitized_form);
        let validation_error_strings: Vec<String> = validation_errors.iter().map(|e| validation_error_to_string(*e)).collect();
        let data = json!({ "errors": validation_error_strings, "previous": previous });
        let page = hb.render("pages/register", &data)?;
        Ok(HttpResponse::BadRequest().body(page))
    } else {
        let connection = pool.get()?;
        let registered_user_result = User::register(&connection, &sanitized_form.user_name, &sanitized_form.password);
        match registered_user_result {
            Err(UserError::NameAlreadyInUse) => {
                let previous = register_form_to_previous(&sanitized_form);
                let data = json!({"errors": ["Username is already in use"], "previous": previous });
                let page = hb.render("pages/register", &data)?;
                Ok(HttpResponse::Conflict().body(page))
            }
            Err(error) => Err(RyobError::from(error)),
            Ok(registered_user) => {
                registered_user.to_session(&session)?;
                Ok(HttpResponse::Found().header(actix_web::http::header::LOCATION, "/").finish())
            }
        }
    }
}

pub fn get(hb: web::Data<Handlebars>) -> Result<HttpResponse, RyobError> {
    let data = json!({});
    let page = hb.render("pages/register", &data)?;
    Ok(HttpResponse::Ok().body(page))
}
