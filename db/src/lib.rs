use sqlx::{PgPool, postgres::PgPoolOptions};

pub mod models;
pub mod queries;
pub struct Db {
    pub pool: PgPool
}

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in order to run the application");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }
}