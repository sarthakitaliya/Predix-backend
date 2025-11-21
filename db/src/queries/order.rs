use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::close_order::{CloseOrder, OrderStatus, ShareType};

pub async fn create_close_order(
    pool: &PgPool,
    id: Uuid,
    market_id: Uuid,
    user_id: Uuid,
    share_type: ShareType,
    price: f64,
    quantity: f64,
    filled_quantity: f64,
    status: OrderStatus,
    closed_at: DateTime<Utc>,
) -> Result<CloseOrder, Error> {
    let rec = sqlx::query_as::<_, CloseOrder>(
        r#"INSERT INTO orders (id, market_id, user_id, type, price, qty, filled_qty, status, closed_at) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id, market_id, user_id, type, price, qty, filled_qty, status, closed_at"#,
    )
    .bind(id)
    .bind(market_id)
    .bind(user_id)
    .bind(share_type)
    .bind(price)
    .bind(quantity)
    .bind(filled_quantity)
    .bind(status)
    .bind(closed_at)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_close_order_by_id(pool: &PgPool, id: Uuid) -> Result<CloseOrder, Error> {
    let rec = sqlx::query_as::<_, CloseOrder>(
        r#"SELECT id, market_id, user_id, type, price, qty, filled_qty, status, closed_at 
        FROM orders WHERE id = $1"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn list_close_orders_by_market_id(
    pool: &PgPool,
    market_id: Uuid,
) -> Result<Vec<CloseOrder>, Error> {
    let recs = sqlx::query_as::<_, CloseOrder>(
        r#"SELECT id, market_id, user_id, type, price, qty, filled_qty, status, closed_at 
        FROM orders WHERE market_id = $1"#,
    )
    .bind(market_id)
    .fetch_all(pool)
    .await?;

    Ok(recs)
}

pub async fn list_closed_orders_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CloseOrder>, Error> {
    let recs = sqlx::query_as::<_, CloseOrder>(
        r#"SELECT id, market_id, user_id, type, price, qty, filled_qty, status, closed_at 
        FROM orders WHERE user_id = $1"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(recs)
}
