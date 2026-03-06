use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use eyre::Context;
use r2d2;

pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Clone)]
pub struct DbPool {
    read_pool: Pool<ConnectionManager<SqliteConnection>>,
    write_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DbPool {
    pub fn new(path: &str) -> Result<Self, eyre::Error> {
        let manager = ConnectionManager::new(path);

        let read_pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .wrap_err_with(|| {
                format!("sqlite_repo: failed to build read_pool for path='{}'", path)
            })?;

        let manager = ConnectionManager::new(path);

        let write_pool = Pool::builder()
            .max_size(1)
            .build(manager)
            .wrap_err_with(|| {
                format!(
                    "sqlite_repo: failed to build write_pool for path='{}'",
                    path
                )
            })?;

        Ok(Self {
            read_pool,
            write_pool,
        })
    }

    pub fn read(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, r2d2::Error> {
        self.read_pool.get()
    }

    pub fn write(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, r2d2::Error> {
        self.write_pool.get()
    }
}
