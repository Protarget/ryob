use diesel::{Queryable, Insertable};
use chrono;
use crate::models::user::{User};
use crate::schema::topics;
use crate::utils::id::Id;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Topic {
    pub id: Id<Topic>,
    pub title: String,
    pub created_by: Id<User>,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Insertable)]
#[table_name = "topics"]
pub struct NewTopic {
    pub title: String,
    pub created_by: Id<User>,
    pub created_at: chrono::DateTime<chrono::Utc>
}

