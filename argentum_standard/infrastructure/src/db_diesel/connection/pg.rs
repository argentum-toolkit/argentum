use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(thiserror::Error, Debug)]
pub enum ConnectionError {
    #[error("Can't get a connection")]
    CantGetConnection(#[from] Box<dyn Error>),
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection(env_name: &str) -> PgPool {
    dotenv().ok();

    //TODO: move it to configuration
    let database_url = env::var(env_name).expect(format!("{} must be set", env_name).as_str());
    init_pool(&database_url).expect("Failed to create pool")
}

pub struct ConnectionPoolManager {
    pool: PgPool,
}

impl ConnectionPoolManager {
    pub fn new(env_name: &str) -> ConnectionPoolManager {
        let pool = establish_connection(env_name);

        ConnectionPoolManager { pool }
    }

    pub fn get_pooled_connection(&self) -> Result<PgPooledConnection, ConnectionError> {
        self.pool
            .get()
            .map_err(|e| ConnectionError::CantGetConnection(Box::new(e)))
    }
}
