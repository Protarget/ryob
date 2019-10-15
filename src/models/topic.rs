use crate::database::types::*;
use crate::models::user::User;
use crate::schema::topics;
use crate::utils::errors::RyobError;
use crate::utils::id::Id;
use chrono;
use diesel::result::Error as DieselError;
use diesel::{Insertable, Queryable};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Topic {
    pub id: Id<Topic>,
    pub title: String,
    pub created_by: Id<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[table_name = "topics"]
pub struct NewTopic {
    pub title: String,
    pub created_by: Id<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum TopicError {
    UnknownDatabaseError(DieselError),
    Unknown(String),
}

impl std::fmt::Display for TopicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TopicError::UnknownDatabaseError(err) => write!(f, "{}", err),
            TopicError::Unknown(err) => write!(f, "{}", err),
        }
    }
}

impl From<TopicError> for RyobError {
    fn from(error: TopicError) -> RyobError {
        RyobError::from_display(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, error)
    }
}

impl Topic {
    pub fn create(connection: &DatabaseConnection, creator: Id<User>, title: &String) -> Result<Topic, TopicError> {
        let timestamp = chrono::Utc::now();

        let new_topic = NewTopic {
            title: title.clone(),
            created_by: creator,
            created_at: timestamp,
        };

        let result = {
            use diesel::prelude::*;
            diesel::insert_into(topics::table)
                .values(&new_topic)
                .get_result(connection)
                .map_err(TopicError::UnknownDatabaseError)?
        };

        info!("User {:?} has created a topic titled \"{}\"", creator, title);

        Ok(result)
    }

    pub fn by_date(connection: &DatabaseConnection, offset: i64, limit: i64) -> Result<Vec<(Topic, User)>, TopicError> {
        Ok({
            use crate::schema::topics::dsl::*;
            use crate::schema::users::dsl::users;
            use diesel::prelude::*;
            topics
                .order(created_at.desc())
                .limit(limit)
                .offset(offset)
                .inner_join(users)
                .load(connection)
                .map_err(TopicError::UnknownDatabaseError)?
        })
    }
}
