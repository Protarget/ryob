use crate::database::types::*;
use crate::models::topic::Topic;
use crate::models::user::User;
use crate::schema::posts;
use crate::utils::errors::RyobError;
use crate::utils::id::Id;
use chrono;
use diesel::result::Error as DieselError;
use diesel::{Insertable, Queryable};
use log::info;

#[derive(Queryable)]
pub struct Post {
    pub id: Id<Post>,
    pub posted_in: Id<Topic>,
    pub created_by: Id<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub posted_in: Id<Topic>,
    pub created_by: Id<User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub content: String,
}

#[derive(Debug)]
pub enum PostError {
    UnknownDatabaseError(DieselError),
    Unknown(String),
}

impl std::fmt::Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PostError::UnknownDatabaseError(err) => write!(f, "{}", err),
            PostError::Unknown(err) => write!(f, "{}", err),
        }
    }
}

impl From<PostError> for RyobError {
    fn from(error: PostError) -> RyobError {
        RyobError::from_display(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, error)
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
            use diesel::prelude::*;
            diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result(connection)
                .map_err(PostError::UnknownDatabaseError)?
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
                .load(connection)
                .map_err(PostError::UnknownDatabaseError)?
        })
    }
}
