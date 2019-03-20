use chrono::{format::ParseError, NaiveDate};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "visits", about = "counting days")]
pub enum Cli {
    /// Adds a visit to your log
    #[structopt(name = "add")]
    Add {
        #[structopt(long = "username", short = "u")]
        username: String,

        /// When you came (Format: "yyyy-mm-dd")
        #[structopt(parse(try_from_str = "parse_date"))]
        enter: NaiveDate,

        /// When you left (Format: "yyyy-mm-dd")
        #[structopt(parse(try_from_str = "parse_date"))]
        exit: NaiveDate,
    },

    /// Remove a visit to your log
    #[structopt(name = "rm")]
    Remove {
        #[structopt(long = "username", short = "u")]
        username: String,

        /// Visit Id to remove.
        #[structopt(name = "id")]
        id: i32,
    },

    /// List out your visits
    #[structopt(name = "ls")]
    List {
        #[structopt(long = "username", short = "u")]
        username: String,

        /// The number of days in a period
        #[structopt(long = "period", short = "p", default_value = "180")]
        period: i64,

        /// The max number of days per period.
        #[structopt(long = "days", short = "d", default_value = "90")]
        max_days: i64,
    },

    /// Find the next date available for a visit with optional length
    #[structopt(name = "next")]
    Next {
        #[structopt(long = "username", short = "u")]
        username: String,

        /// The number of days in a period
        #[structopt(long = "period", short = "p", default_value = "180")]
        period: i64,

        /// The max number of days per period.
        #[structopt(long = "days", short = "d", default_value = "90")]
        max_days: i64,

        /// The number of days in a period
        #[structopt(long = "length", short = "l", default_value = "1")]
        length: i64,
    },
    // TODO: Add `check` feature - to test whether potential enter/exit would be OK
}

fn parse_date(s: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}
