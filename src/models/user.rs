use diesel::{Queryable, Insertable};
use crate::schema::users;
use crate::utils::id::Id;


#[derive(Queryable)]
pub struct User {
    pub id: Id<User>,
    pub user_name: String,
    pub password_hash: String
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub password_hash: String
}

