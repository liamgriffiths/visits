use crate::{pg::ConnectionPool, session::Session};

pub struct Config {
    pub database_url: String,
}

/// App holds on to any shared state across sessions
pub struct App {
    pg_pool: ConnectionPool,
}

impl App {
    pub fn new(config: Config) -> App {
        App {
            pg_pool: ConnectionPool::new(&config.database_url),
        }
    }

    /// Create a new session for a user
    pub fn session(&self, username: &String) -> Session {
        Session::new(self.pg_pool.conn(), username)
    }
}
