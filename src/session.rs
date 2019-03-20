use chrono::NaiveDate;
use failure::Error;
use prettytable;
use prettytable::{Attr, Cell, Row, Table};
use time::Duration;

use crate::{
    models::{NewVisit, User, Visit},
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

    /// Prints out a summary of the users' visits.
    // TODO: make the period length and max-days-per-period to be parameters
    pub fn print_summary(&self, period: i64, max_days: i64) -> Result<(), Error> {
        let visits = Visit::for_user(&self.conn, &self.user)?;

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Id").with_style(Attr::Bold),
            Cell::new("Entry").with_style(Attr::Bold),
            Cell::new("Exit").with_style(Attr::Bold),
            Cell::new("Length").with_style(Attr::Bold),
            // Cell::new("period").with_style(Attr::Bold),
            Cell::new(&format!("Days left (of {})", max_days)).with_style(Attr::Bold),
        ]));

        for v in &visits {
            let start_at = v.exit_at - Duration::days(period);
            let used_days = v.sum_all_days_since(start_at, &visits);
            let days_left = max_days - used_days;

            table.add_row(Row::new(vec![
                Cell::new(&format!("{}", v.id)),
                Cell::new(&format!("{}", v.enter_at)),
                Cell::new(&format!("{}", v.exit_at)),
                Cell::new(&format!("{}", v.days())),
                // Cell::new(&format!("{} -> {}", start_at, end_at)),
                Cell::new(&format!("{}", days_left)),
            ]));
        }

        table.printstd();

        Ok(())
    }
}
