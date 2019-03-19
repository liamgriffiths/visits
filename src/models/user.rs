use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{pg::Connection, schema::users};

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    /// Find or create a user record.
    pub fn find_or_create(conn: &Connection, username: &str) -> QueryResult<User> {
        use crate::schema::users::dsl;
        use diesel::insert_into;

        let new_user = NewUser {
            username: username.to_string(),
        };

        insert_into(dsl::users)
            .values(new_user)
            .on_conflict(dsl::username)
            .do_nothing()
            .execute(conn)?;

        dsl::users.filter(dsl::username.eq(username)).first(conn)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
}
