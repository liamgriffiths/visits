use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::schema::users;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn find(conn: &PgConnection, user_id: i32) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.find(user_id).first(conn)
    }

    pub fn find_or_create(conn: &PgConnection, _username: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        // create and ignore conflicts.
        diesel::insert_into(users)
            .values(NewUser {
                username: _username.to_string(),
            })
            .on_conflict(username)
            .do_nothing()
            .execute(conn)
            .unwrap();

        users.filter(username.eq(_username)).first(conn)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
}

impl NewUser {
    pub fn create(&self, conn: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users).values(self).get_result(conn)
    }
}
