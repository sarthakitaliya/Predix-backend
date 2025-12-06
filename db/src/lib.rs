use sqlx::{PgPool, database, postgres::PgPoolOptions};

pub mod models;
pub mod queries;
pub mod utils;
pub struct Db {
    pub pool: PgPool
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }
}