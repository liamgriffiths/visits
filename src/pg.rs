use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::time::Duration;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// Creates and manages a connection pool to Postgres.
pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
    pub fn new(url: &str) -> ConnectionPool {
        let manager = ConnectionManager::<PgConnection>::new(url);
        // See: https://docs.rs/r2d2/0.7.4/r2d2/struct.Config.html
        // TODO: statement-timeout is configured through a `CustomizeConnection`
        let pool = Pool::builder()
            .max_size(10)
            .min_idle(None)
            .test_on_check_out(true)
            .connection_timeout(Duration::from_secs(10))
            .build(manager)
            .unwrap();

        ConnectionPool { pool }
    }

    pub fn conn(&self) -> Connection {
        self.pool.get().unwrap()
    }
}
