use chrono::NaiveDate;
use failure::Error;

use crate::{
    models::user::User,
    models::visit::{NewVisit, Visit},
    pg::Connection,
};

/// A Session represents a someone using the app and let's them do things.
pub struct Session {
    conn: Connection,
    user: User,
}

impl Session {
    pub fn new(conn: Connection, username: &str) -> Result<Session, Error> {
        let user = User::find_or_create(&conn, &username)?;
        Ok(Session { conn, user })
    }

    pub fn all_visits(&self) -> Result<Vec<Visit>, Error> {
        let visits = Visit::for_user(&self.conn, &self.user)?;
        Ok(visits)
    }

    /// Add a new visit to the users' log.
    pub fn add_visit(&self, enter_at: NaiveDate, exit_at: NaiveDate) -> Result<Visit, Error> {
        let visit = NewVisit {
            user_id: self.user.id,
            enter_at,
            exit_at,
        }
        .create(&self.conn)?;

        Ok(visit)
    }

    pub fn remove_visit(&self, id: i32) -> Result<usize, Error> {
        let res = Visit::delete_for_user(&self.conn, &self.user, id)?;
        Ok(res)
    }

    pub fn next_visit(&self, period: i64, max_days: i64, length: i64) -> Result<Visit, Error> {
        let visit = Visit::next_for_user(&self.conn, &self.user, period, max_days, length)?;
        Ok(visit)
    }
}
