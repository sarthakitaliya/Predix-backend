use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::user::User;

pub async fn create_user(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    email: &str,
    solana_address: &str,
) -> Result<User, Error> {
    let rec = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (id, name, email, solana_address) VALUES ($1, $2, $3, $4)  
        RETURNING id, name, email, solana_address, created_at"#,
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .bind(solana_address)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, Error> {
    let rec = sqlx::query_as::<_, User>(
        r#"SELECT id, name, email, solana_address, created_at FROM users WHERE email = $1"#,
    )
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}