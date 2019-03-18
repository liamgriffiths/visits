use chrono::NaiveDate;
use prettytable;
use prettytable::{Attr, Cell, Row, Table};
use time::Duration;

use crate::models::{NewVisit, User, Visit};
use crate::pg::Connection;

pub struct Session {
    conn: Connection,
    user: User,
}

impl Session {
    pub fn new(conn: Connection, username: String) -> Session {
        let user = User::find_or_create(&conn, &username).unwrap();
        Session { conn, user }
    }

    pub fn add_visit(&self, enter_at: NaiveDate, exit_at: NaiveDate) -> Visit {
        let visit = NewVisit {
            user_id: self.user.id,
            enter_at,
            exit_at,
        };
        visit.create(&self.conn).unwrap()
    }

    // TODO: make the period length and max-days-per-period to be parameters
    pub fn print_summary(&self) {
        let visits = Visit::for_user(&self.conn, &self.user).unwrap();

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("id").with_style(Attr::Bold),
            Cell::new("entry").with_style(Attr::Bold),
            Cell::new("exit").with_style(Attr::Bold),
            Cell::new("length").with_style(Attr::Bold),
            // Cell::new("period").with_style(Attr::Bold),
            Cell::new("days left").with_style(Attr::Bold),
        ]));

        for v in &visits {
            let start_at = v.exit_at - Duration::days(365);
            let used_days = v.sum_all_days_since(start_at, &visits);
            let days_left = 182 - used_days;

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
    }
}
