use crate::database::types::*;
use crate::schema::users;
use crate::utils::errors::RyobError;
use crate::utils::id::Id;
use actix_session::Session;
use actix_web::error::Error as ActixError;
use bcrypt::BcryptError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{Insertable, Queryable, RunQueryDsl};
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Id<User>,
    pub user_name: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub enum UserError {
    NameAlreadyInUse,
    BadLogin,
    NoSuchUser,
    UnknownHashError(BcryptError),
    UnknownDatabaseError(DieselError),
    UnknownActixError(ActixError),
    Unknown(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NameAlreadyInUse => write!(f, "Name already in use"),
            UserError::BadLogin => write!(f, "Bad login"),
            UserError::NoSuchUser => write!(f, "No such user"),
            UserError::UnknownActixError(err) => write!(f, "{}", err),
            UserError::UnknownDatabaseError(err) => write!(f, "{}", err),
            UserError::UnknownHashError(err) => write!(f, "{}", err),
            UserError::Unknown(err) => write!(f, "{}", err),
        }
    }
}

impl From<UserError> for RyobError {
    fn from(error: UserError) -> RyobError {
        match error {
            UserError::NameAlreadyInUse => RyobError::from_display(actix_web::http::StatusCode::CONFLICT, error),
            UserError::BadLogin => RyobError::from_display(actix_web::http::StatusCode::UNAUTHORIZED, error),
            UserError::NoSuchUser => RyobError::from_display(actix_web::http::StatusCode::NOT_FOUND, error),
            _ => RyobError::from_display(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, error),
        }
    }
}

impl User {
    pub fn register(connection: &DatabaseConnection, user_name: &String, password: &String) -> Result<User, UserError> {
        let hash = bcrypt::hash(password, 10).map_err(UserError::UnknownHashError)?;

        let new_user = NewUser {
            user_name: user_name.clone(),
            password_hash: hash,
        };

        let user: User = {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(connection)
                .map_err(|err| match err {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, error_info) => {
                        let constraint_name_option = (*error_info).constraint_name();
                        match constraint_name_option {
                            Some("users_user_name_key") => UserError::NameAlreadyInUse,
                            _ => UserError::UnknownDatabaseError(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, error_info)),
                        }
                    }
                    _ => UserError::UnknownDatabaseError(err),
                })?
        };

        info!("User {:?} has been registered with username \"{}\"", user.id, user_name);

        Ok(user)
    }

    pub fn login(connection: &DatabaseConnection, user_name: &String, password: &String) -> Result<User, UserError> {
        let user = User::by_user_name(connection, user_name)?;
        let user_verified = bcrypt::verify(password, &user.password_hash).map_err(UserError::UnknownHashError)?;
        if user_verified {
            info!("User {:?} with username \"{}\" has logged in", user.id, user_name);
            Ok(user)
        } else {
            Err(UserError::BadLogin)
        }
    }

    pub fn by_user_name(connection: &DatabaseConnection, target_user_name: &String) -> Result<User, UserError> {
        Ok({
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;
            users
                .filter(user_name.eq(target_user_name))
                .first::<User>(connection)
                .map_err(UserError::UnknownDatabaseError)?
        })
    }

    pub fn by_user_id(connection: &DatabaseConnection, target_user_id: Id<User>) -> Result<User, UserError> {
        Ok({
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;
            users
                .filter(id.eq(target_user_id))
                .first::<User>(connection)
                .map_err(UserError::UnknownDatabaseError)?
        })
    }

    pub fn from_session(connection: &DatabaseConnection, session: &Session) -> Result<Option<User>, UserError> {
        let user_id_maybe = User::id_from_session(session)?;
        match user_id_maybe {
            None => Ok(None),
            Some(id) => Ok(Some(User::by_user_id(connection, id)?)),
        }
    }

    pub fn id_from_session(session: &Session) -> Result<Option<Id<User>>, UserError> {
        Ok(session.get::<Id<User>>("user").map_err(UserError::UnknownActixError)?)
    }

    pub fn to_session(&self, session: &Session) -> Result<(), UserError> {
        Ok(session.set("user", &self.id).map_err(UserError::UnknownActixError)?)
    }
}
