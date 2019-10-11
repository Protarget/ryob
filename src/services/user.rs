use crate::database::types::DatabaseConnection;
use crate::models::user::{NewUser, User};
use crate::utils::id::Id;
use actix_session::Session;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::RunQueryDsl;
use log::info;

#[derive(Debug)]
pub enum UserError {
    NameAlreadyInUse,
    BadLogin,
    NoSuchUser,
    UnknownHashError(bcrypt::BcryptError),
    UnknownDatabaseError(DieselError),
    UnknownActixError(actix_web::Error),
    Unknown(String),
}

impl From<bcrypt::BcryptError> for UserError {
    fn from(error: bcrypt::BcryptError) -> Self {
        UserError::UnknownHashError(error)
    }
}

impl From<DieselError> for UserError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, error_info) => {
                let column_name_option = (*error_info).column_name();
                match column_name_option {
                    Some("user_name") => UserError::NameAlreadyInUse,
                    _ => UserError::UnknownDatabaseError(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, error_info)),
                }
            }
            _ => UserError::UnknownDatabaseError(error),
        }
    }
}

impl From<actix_web::Error> for UserError {
    fn from(error: actix_web::Error) -> Self {
        UserError::UnknownActixError(error)
    }
}

impl User {
    pub fn register(connection: &DatabaseConnection, user_name: &String, password: &String) -> Result<User, UserError> {
        let hash = bcrypt::hash(password, 10)?;

        let new_user = NewUser {
            user_name: user_name.clone(),
            password_hash: hash,
        };

        let user: User = {
            use crate::schema::users;
            diesel::insert_into(users::table).values(&new_user).get_result(connection)?
        };

        info!("User {:?} has been registered with username \"{}\"", user.id, user_name);

        Ok(user)
    }

    pub fn login(connection: &DatabaseConnection, user_name: &String, password: &String) -> Result<User, UserError> {
        let user = User::by_user_name(connection, user_name)?;
        let user_verified = bcrypt::verify(password, &user.password_hash)?;
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
            users.filter(user_name.eq(target_user_name)).first::<User>(connection)?
        })
    }

    pub fn by_user_id(connection: &DatabaseConnection, target_user_id: Id<User>) -> Result<User, UserError> {
        Ok({
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;
            users.filter(id.eq(target_user_id)).first::<User>(connection)?
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
        Ok(session.get::<Id<User>>("user")?)
    }

    pub fn to_session(&self, session: &Session) -> Result<(), UserError> {
        Ok(session.set("user", &self.id)?)
    }
}
