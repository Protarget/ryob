use crate::database::types::DatabaseConnection;
use crate::models::post::{NewPost, Post};
use crate::models::topic::Topic;
use crate::models::user::User;
use crate::utils::id::Id;
use diesel::result::Error as DieselError;
use log::info;

#[derive(Debug)]
pub enum PostError {
    UnknownDatabaseError(DieselError),
    Unknown(String),
}

impl From<DieselError> for PostError {
    fn from(error: DieselError) -> Self {
        PostError::UnknownDatabaseError(error)
    }
}

impl Post {
    pub fn create(connection: &DatabaseConnection, creator: Id<User>, topic: Id<Topic>, content: &String) -> Result<Post, PostError> {
        let timestamp = chrono::Utc::now();
        let new_post = NewPost {
            posted_in: topic,
            created_by: creator,
            created_at: timestamp,
            content: content.clone(),
        };

        let result = {
            use crate::schema::posts;
            use diesel::prelude::*;
            diesel::insert_into(posts::table).values(&new_post).get_result(connection)?
        };

        info!("User {:?} has created a post in topic {:?}", creator, topic);

        Ok(result)
    }

    pub fn in_topic_by_date(connection: &DatabaseConnection, topic: Id<Topic>, offset: i64, limit: i64) -> Result<Vec<(Post, User)>, PostError> {
        Ok({
            use crate::schema::posts::dsl::*;
            use crate::schema::users::dsl::users;
            use diesel::prelude::*;
            posts
                .order(created_at.desc())
                .filter(posted_in.eq(topic))
                .limit(limit)
                .offset(offset)
                .inner_join(users)
                .load(connection)?
        })
    }
}
