use crate::models::topic::Topic;
use crate::models::user::User;
use crate::schema::posts;
use crate::utils::id::Id;
use chrono;
use diesel::{Insertable, Queryable};

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
