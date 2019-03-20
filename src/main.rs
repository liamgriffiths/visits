#[macro_use]
extern crate diesel;

use chrono::Utc;
use dotenv::dotenv;
use exitfailure::ExitFailure;
use prettytable::{cell, row, Table};
use std::env;
use structopt::StructOpt;
use time::Duration;

mod app;
mod cli;
mod models;
mod pg;
mod schema;
mod session;

use crate::{
    app::{App, Config},
    cli::Cli,
    cli::Cli::{Add, List, Next, Remove},
};

fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let config = Config {
        database_url: env::var("DATABASE_URL")?,
    };

    let app = App::new(config)?;

    match Cli::from_args() {
        List {
            username,
            period,
            max_days,
        } => match app.session(&username)?.all_visits() {
            Ok(visits) => {
                let today = Utc::now().naive_utc().date();
                let mut table = Table::new();
                table.set_titles(row![b => "Id", "Entry", "Exit", "Length", "Days leftover"]);

                for v in &visits {
                    let start_at = v.exit_at - Duration::days(period);
                    let used_days = v.sum_all_days_since(start_at, &visits);
                    let days_left = max_days - used_days;

                    if days_left < 0 {
                        // If we've gone over our days, color the row red
                        table.add_row(row![Fr => v.id, v.enter_at, v.exit_at, v.days(), days_left]);
                    } else if v.exit_at < today - Duration::days(period) {
                        // If the visit is older than our period make it italics
                        table.add_row(row![i => v.id, v.enter_at, v.exit_at, v.days(), days_left]);
                    } else {
                        // Otherwise print it normally
                        table.add_row(row![v.id, v.enter_at, v.exit_at, v.days(), days_left]);
                    }
                }

                println!(
                    "OK: {} visits found. ({}/{} days per period)",
                    visits.len(),
                    max_days,
                    period
                );
                table.printstd();
            }
            Err(err) => println!("ERROR: {}", err),
        },

        Next {
            username,
            length,
            period,
            max_days,
        } => match app.session(&username)?.next_visit(period, max_days, length) {
            Ok(visit) => {
                let mut table = Table::new();
                table.set_titles(row![b => "Entry", "Exit", "Length", "Days from now"]);
                table.add_row(row![Fy =>
                    visit.enter_at,
                    visit.exit_at,
                    visit.days(),
                    visit.days_until_now()
                ]);

                println!("OK: Next possible visit found!");
                table.printstd();
            }
            Err(err) => println!("ERROR: {}", err),
        },

        Add {
            username,
            enter,
            exit,
        } => match app.session(&username)?.add_visit(enter, exit) {
            Ok(visit) => {
                let mut table = Table::new();
                table.set_titles(row![b => "Id", "Entry", "Exit", "Length"]);
                table.add_row(row![Fgb => visit.id, visit.enter_at, visit.exit_at, visit.days()]);

                println!("OK: Added!");
                table.printstd();
            }
            Err(err) => println!("ERROR: {}", err),
        },

        // TODO: Make interactive - for example, display the record and ask y/n to confirm.
        Remove { username, id } => match app.session(&username)?.remove_visit(id) {
            Ok(n) if n > 0 => println!("OK: Deleted {}.", id),
            Ok(_) => println!("ERROR: No visit with id {}.", id),
            Err(e) => panic!(e),
        },
    };

    Ok(())
}
