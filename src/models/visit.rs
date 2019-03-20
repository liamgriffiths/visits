use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use failure::Error;
use time::Duration;

use crate::{models::User, pg::Connection, schema::visits};

#[derive(Debug, Queryable, Associations, Identifiable, Clone)]
#[table_name = "visits"]
#[belongs_to(User)]
pub struct Visit {
    pub id: i32,
    pub user_id: i32,
    pub enter_at: NaiveDate,
    pub exit_at: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Visit {
    /// Find all visits for a user
    pub fn for_user(conn: &Connection, user: &User) -> QueryResult<Vec<Visit>> {
        use crate::schema::visits::dsl;
        Visit::belonging_to(user)
            .order(dsl::enter_at)
            .load::<Visit>(conn)
    }

    /// Delete a visit for a user
    // TODO: this feels a little awkward, maybe there is a better way?
    pub fn delete_for_user(conn: &Connection, user: &User, id: i32) -> Result<usize, Error> {
        use crate::schema::visits::dsl;

        // Returns number of rows deleted.
        let res = diesel::delete(
            dsl::visits
                .filter(dsl::id.eq(id))
                .filter(dsl::user_id.eq(user.id)),
        )
        .execute(conn)?;

        Ok(res)
    }

    /// Finds the next possible visit for some parameters. We'd like to know the next possible
    /// dates for a visit when the visit is some length and we're following the rules of the period
    /// and max-days for that period.
    pub fn next_for_user(
        conn: &Connection,
        user: &User,
        period: i64,
        max_days: i64,
        length: i64,
    ) -> Result<Visit, Error> {
        let visits = Visit::for_user(conn, user)?;
        let today = Utc::now().naive_utc().date();
        let one_day = Duration::days(1);

        // TODO: It might be nice to implement this on a different version of the struct so we're
        // not adding in values we don't care about.
        let mut v = Visit {
            id: 0,
            user_id: user.id,
            enter_at: today,
            exit_at: today + Duration::days(length - 1),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        };

        // Keep incrementing the the days up until we have at least the the number of
        // days left as we want for the length of the visit.
        // TODO: There is likely a nicer way to do this.
        let mut start_at = v.exit_at - Duration::days(period);
        let mut done = false;

        while !done {
            v.enter_at += one_day;
            v.exit_at += one_day;
            start_at += one_day;
            done = max_days - v.sum_all_days_since(start_at, &visits) >= length;
        }

        Ok(v)
    }

    // Count up all the days in a single Visit
    pub fn days(&self) -> i64 {
        self.days_since(self.enter_at)
    }

    // Count up all the days for slice of Visits
    pub fn sum_all_days(vs: &[Visit]) -> i64 {
        vs.iter().fold(0, |acc, v| acc + v.days())
    }

    // Count up all the days in the visit since some start-date
    pub fn days_since(&self, start_at: NaiveDate) -> i64 {
        if start_at > self.exit_at {
            // when we're starting before we left
            0
        } else if start_at > self.enter_at {
            // when we're starting after we entered
            self.exit_at.signed_duration_since(start_at).num_days() + 1
        } else {
            // when we're starting before we entered
            self.exit_at.signed_duration_since(self.enter_at).num_days() + 1
        }
    }

    // Count up all the days for slice of Visits relative to the current Visit since start-date
    pub fn sum_all_days_since(&self, start_at: NaiveDate, vs: &[Visit]) -> i64 {
        vs.iter()
            .filter(|v| v.enter_at < self.exit_at)
            .fold(0, |acc, v| acc + v.days_since(start_at))
    }

    // Calculate the days until today
    pub fn days_until_now(&self) -> i64 {
        let today = Utc::now().naive_utc().date();
        (self.enter_at - today).num_days()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "visits"]
pub struct NewVisit {
    pub user_id: i32,
    pub enter_at: NaiveDate,
    pub exit_at: NaiveDate,
}

impl NewVisit {
    pub fn create(&self, conn: &Connection) -> QueryResult<Visit> {
        use crate::schema::visits::dsl;
        use diesel::insert_into;

        insert_into(dsl::visits).values(self).get_result(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit_days_returns_correct_sum() {
        let v = Visit {
            id: 1,
            user_id: 1,
            enter_at: NaiveDate::from_ymd(2018, 1, 1),
            exit_at: NaiveDate::from_ymd(2018, 1, 10),
            created_at: NaiveDateTime::from_timestamp(0, 42),
            updated_at: NaiveDateTime::from_timestamp(0, 42),
        };
        assert_eq!(v.days(), 10);
    }

    #[test]
    fn visit_sum_all_days_returns_correct_sum() {
        let vs = vec![
            Visit {
                id: 1,
                user_id: 1,
                enter_at: NaiveDate::from_ymd(2018, 1, 1),
                exit_at: NaiveDate::from_ymd(2018, 1, 10),
                created_at: NaiveDateTime::from_timestamp(0, 42),
                updated_at: NaiveDateTime::from_timestamp(0, 42),
            },
            Visit {
                id: 1,
                user_id: 1,
                enter_at: NaiveDate::from_ymd(2018, 1, 1),
                exit_at: NaiveDate::from_ymd(2018, 1, 2),
                created_at: NaiveDateTime::from_timestamp(0, 42),
                updated_at: NaiveDateTime::from_timestamp(0, 42),
            },
        ];
        assert_eq!(Visit::sum_all_days(&vs), 12);
    }

    #[test]
    fn visit_days_since_returns_correct_sum() {
        let v = Visit {
            id: 1,
            user_id: 1,
            enter_at: NaiveDate::from_ymd(2018, 1, 5),
            exit_at: NaiveDate::from_ymd(2018, 1, 10),
            created_at: NaiveDateTime::from_timestamp(0, 42),
            updated_at: NaiveDateTime::from_timestamp(0, 42),
        };

        // when the start date is before the entry
        let start_at = NaiveDate::from_ymd(2017, 1, 1);
        assert_eq!(v.days_since(start_at), 6);

        // when the start date is on the entry
        let start_at = NaiveDate::from_ymd(2018, 1, 5);
        assert_eq!(v.days_since(start_at), 6);

        // when the start date is after the entry
        let start_at = NaiveDate::from_ymd(2018, 1, 7);
        assert_eq!(v.days_since(start_at), 4);

        // when the start date is after the exit
        let start_at = NaiveDate::from_ymd(2018, 1, 11);
        assert_eq!(v.days_since(start_at), 0);
    }

    #[test]
    fn visit_sum_all_days_since_returns_correct_sum() {
        let vs = vec![
            Visit {
                id: 1,
                user_id: 1,
                enter_at: NaiveDate::from_ymd(2018, 1, 1),
                exit_at: NaiveDate::from_ymd(2018, 1, 10),
                created_at: NaiveDateTime::from_timestamp(0, 42),
                updated_at: NaiveDateTime::from_timestamp(0, 42),
            },
            Visit {
                id: 1,
                user_id: 1,
                enter_at: NaiveDate::from_ymd(2018, 2, 1),
                exit_at: NaiveDate::from_ymd(2018, 2, 10),
                created_at: NaiveDateTime::from_timestamp(0, 42),
                updated_at: NaiveDateTime::from_timestamp(0, 42),
            },
        ];

        // when the start date is before all entries
        let start_at = NaiveDate::from_ymd(2017, 1, 1);
        assert_eq!(vs[0].sum_all_days_since(start_at, &vs), 10);
        assert_eq!(vs[1].sum_all_days_since(start_at, &vs), 20);

        // when the start date is on the entry
        let start_at = NaiveDate::from_ymd(2018, 1, 1);
        assert_eq!(vs[0].sum_all_days_since(start_at, &vs), 10);
        assert_eq!(vs[1].sum_all_days_since(start_at, &vs), 20);

        // when the start date is after the first entry
        let start_at = NaiveDate::from_ymd(2018, 1, 5);
        assert_eq!(vs[0].sum_all_days_since(start_at, &vs), 6);
        assert_eq!(vs[1].sum_all_days_since(start_at, &vs), 16);

        // when the start date is after the first exit
        let start_at = NaiveDate::from_ymd(2018, 1, 11);
        assert_eq!(vs[0].sum_all_days_since(start_at, &vs), 0);
        assert_eq!(vs[1].sum_all_days_since(start_at, &vs), 10);

        // when the start date is after the second entry
        let start_at = NaiveDate::from_ymd(2018, 2, 5);
        assert_eq!(vs[0].sum_all_days_since(start_at, &vs), 0);
        assert_eq!(vs[1].sum_all_days_since(start_at, &vs), 6);

        // when the start date is after the second exit
        let start_at = NaiveDate::from_ymd(2018, 2, 11);
        assert_eq!(vs[0].sum_all_days_since(start_at, &vs), 0);
        assert_eq!(vs[1].sum_all_days_since(start_at, &vs), 0);
    }
}
