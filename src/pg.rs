use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use failure::Error;
use std::time::Duration;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// Creates and manages a connection pool to Postgres.
pub struct ConnectionPool(Pool<ConnectionManager<PgConnection>>);

impl ConnectionPool {
    pub fn new(url: &str) -> Result<ConnectionPool, Error> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        // See: https://docs.rs/r2d2/0.7.4/r2d2/struct.Config.html
        // TODO: statement-timeout is configured through a `CustomizeConnection`
        let pool = Pool::builder()
            .max_size(10)
            .min_idle(None)
            .test_on_check_out(true)
            .connection_timeout(Duration::from_secs(10))
            .build(manager)?;

        Ok(ConnectionPool(pool))
    }

    pub fn get(&self) -> Result<Connection, Error> {
        let conn = self.0.get()?;
        Ok(conn)
    }
}
