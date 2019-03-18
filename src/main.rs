#[macro_use]
extern crate diesel;

// TODO: these macros clobber diesel it seems :(
// #[macro_use]
extern crate prettytable;

extern crate exitfailure;
extern crate failure;
extern crate structopt;

use dotenv::dotenv;
use exitfailure::ExitFailure;
use std::env;
use structopt::StructOpt;

mod app;
mod cli;
mod models;
mod pg;
mod schema;
mod session;

use self::cli::Cli;
use self::cli::Cli::{Add, Summary};
use app::{App, Config};

fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let app = App::new(Config {
        database_url: env::var("DATABASE_URL")?,
    });

    match Cli::from_args() {
        Summary { username } => {
            let ses = app.session(username);
            ses.print_summary();
        }
        Add {
            username,
            enter,
            exit,
        } => {
            let ses = app.session(username);
            let visit = ses.add_visit(enter, exit);
            println!("Added: {} to {}", visit.enter_at, visit.exit_at);
        }
    };

    Ok(())
}
