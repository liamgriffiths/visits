use crate::pg::{pool, Connection, ConnectionPool};
use crate::session::Session;

pub struct Config {
    pub database_url: String,
}

pub struct App {
    pg_pool: ConnectionPool,
}

impl App {
    pub fn new(config: Config) -> App {
        App {
            pg_pool: pool(&config.database_url),
        }
    }

    pub fn session(&self, username: String) -> Session {
        Session::new(self.conn(), username)
    }

    fn conn(&self) -> Connection {
        self.pg_pool.get().unwrap()
    }
}
