use chrono::NaiveDate;
use failure::Error;

use prettytable::{cell, row};
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

    pub fn next(&self, period: i64, max_days: i64, length: i64) -> Result<(), Error> {
        let visits = Visit::for_user(&self.conn, &self.user)?;
        let today = chrono::Utc::now().naive_utc().date();

        let mut v = Visit {
            id: 123,
            user_id: self.user.id,
            enter_at: today,
            exit_at: today + Duration::days(length - 1),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Next possible visit").with_style(Attr::Bold)
        ]));

        table.add_row(row![
            "Id",
            "Entry",
            "Exit",
            "Length",
            &format!("Days left (of {})", max_days)
        ]);

        let mut start_at = v.exit_at - Duration::days(period);
        let mut done = false;
        let mut days_left;

        while !done {
            v.enter_at += Duration::days(1);
            v.exit_at += Duration::days(1);
            start_at += Duration::days(1);
            days_left = max_days - v.sum_all_days_since(start_at, &visits);
            done = days_left >= length;
        }

        days_left =
            max_days - v.sum_all_days_since(v.exit_at - Duration::days(period + length), &visits);

        table.add_row(row![v.id, v.enter_at, v.exit_at, v.days(), days_left]);

        table.printstd();

        Ok(())
    }

    /// Prints out a summary of the users' visits.
    // TODO: make the period length and max-days-per-period to be parameters
    pub fn print_summary(&self, period: i64, max_days: i64) -> Result<(), Error> {
        let visits = Visit::for_user(&self.conn, &self.user)?;

        let mut table = Table::new();

        table.add_row(row![
            "Id",
            "Entry",
            "Exit",
            "Length",
            &format!("Days left (of {})", max_days)
        ]);

        for v in &visits {
            let start_at = v.exit_at - Duration::days(period);
            let used_days = v.sum_all_days_since(start_at, &visits);
            let days_left = max_days - used_days;
            table.add_row(row![v.id, v.enter_at, v.exit_at, v.days(), days_left]);
        }

        table.printstd();

        Ok(())
    }
}
