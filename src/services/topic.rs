use crate::database::types::DatabaseConnection;
use crate::models::topic::{NewTopic, Topic};
use crate::models::user::User;
use crate::utils::id::Id;
use diesel::result::Error as DieselError;
use log::info;

#[derive(Debug)]
pub enum TopicError {
    UnknownDatabaseError(DieselError),
    Unknown(String),
}

impl From<DieselError> for TopicError {
    fn from(error: DieselError) -> Self {
        TopicError::UnknownDatabaseError(error)
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
            use crate::schema::topics;
            use diesel::prelude::*;
            diesel::insert_into(topics::table).values(&new_topic).get_result(connection)?
        };

        info!("User {:?} has created a topic titled \"{}\"", creator, title);

        Ok(result)
    }

    pub fn by_date(connection: &DatabaseConnection, offset: i64, limit: i64) -> Result<Vec<(Topic, User)>, TopicError> {
        Ok({
            use crate::schema::topics::dsl::*;
            use crate::schema::users::dsl::users;
            use diesel::prelude::*;
            topics.order(created_at.desc()).limit(limit).offset(offset).inner_join(users).load(connection)?
        })
    }
}
