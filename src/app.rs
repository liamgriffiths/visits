use failure::Error;

use crate::{pg::ConnectionPool, session::Session};

pub struct Config {
    pub database_url: String,
}

/// App holds on to any shared state across sessions
pub struct App {
    pg_pool: ConnectionPool,
}

impl App {
    pub fn new(config: Config) -> Result<App, Error> {
        let pg_pool = ConnectionPool::new(&config.database_url)?;
        Ok(App { pg_pool })
    }

    /// Create a new session for a user
    pub fn session(&self, username: &str) -> Result<Session, Error> {
        let conn = self.pg_pool.get()?;
        Ok(Session::new(conn, username))
    }
}
