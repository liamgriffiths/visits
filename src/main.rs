#[macro_use]
extern crate diesel;

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

use crate::{
    app::{App, Config},
    cli::Cli,
    cli::Cli::{Add, Remove, Summary},
};

fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let config = Config {
        database_url: env::var("DATABASE_URL")?,
    };

    let app = App::new(config)?;

    match Cli::from_args() {
        Summary { username } => {
            app.session(&username)?.print_summary()?;
        }
        Add {
            username,
            enter,
            exit,
        } => {
            let visit = app.session(&username)?.add_visit(enter, exit)?;
            println!("Added: {} to {}", visit.enter_at, visit.exit_at);
        }
        Remove { username, id } => {
            let visit = app.session(&username)?.remove_visit(id)?;
            println!("Deleted: {}", visit);
        }
    };

    Ok(())
}
