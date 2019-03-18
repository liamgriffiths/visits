use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::models::User;
use crate::schema::visits;

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
    pub fn for_user(conn: &PgConnection, user: &User) -> QueryResult<Vec<Visit>> {
        Visit::belonging_to(user).load::<Visit>(conn)
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
        if start_at.gt(&self.exit_at) {
            0
        } else {
            // TODO: maybe clean this one up a bit.
            let days_since_start = self.exit_at.signed_duration_since(start_at).num_days() + 1;
            let days_since_enter = self.exit_at.signed_duration_since(self.enter_at).num_days() + 1;
            std::cmp::min(days_since_enter, days_since_start)
        }
    }

    // Count up all the days for slice of Visits relative to the current Visit since start-date
    pub fn sum_all_days_since(&self, start_at: NaiveDate, vs: &[Visit]) -> i64 {
        vs.iter()
            .filter(|v| v.enter_at.lt(&self.exit_at))
            .fold(0, |acc, v| acc + v.days_since(start_at))
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
    pub fn create(&self, conn: &PgConnection) -> QueryResult<Visit> {
        diesel::insert_into(visits::table)
            .values(self)
            .get_result(conn)
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
