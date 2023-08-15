use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_db_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}
