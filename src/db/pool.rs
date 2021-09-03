use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    // TODO: pass the connection URL into this function rather than extracting
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL"); // TODO: handle errors
    let mgr = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .build(mgr)
        .expect("could not build connection pool") // TODO: handle errors
}
