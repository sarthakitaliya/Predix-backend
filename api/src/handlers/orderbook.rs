use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};

use matching::types::{MarketSnapshot, OpenOrder};
use tokio::sync::oneshot;

use crate::{engine::engine::EngineMsg, models::auth::AuthUser, state::state::Shared};

pub async fn get_orderbook(
    State(state): State<Shared>,
    Path(market_id): Path<u64>,
) -> Result<Json<MarketSnapshot>, (StatusCode, String)> {
    let markets = state.markets.read().await;
    let tx = if let Some(tx) = markets.get(&market_id) {
        tx.clone()
    } else {
        return Err((StatusCode::NOT_FOUND, "market not found".into()));
    };
    drop(markets);

    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(EngineMsg::Snapshot { resp: resp_tx })
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "engine send failed".into(),
            )
        })?;
    let snapshot = resp_rx
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "engine dropped".into()))?;

    Ok(Json(MarketSnapshot {
        yes: snapshot.0,
        no: snapshot.1,
    }))
}

pub async fn get_open_orders(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Path(market_id): Path<u64>,
) -> Result<Json<Vec<OpenOrder>>, (StatusCode, String)> {
    let markets = state.markets.read().await;
    let tx = if let Some(tx) = markets.get(&market_id) {
        tx.clone()
    } else {
        return Err((StatusCode::NOT_FOUND, "market not found".into()));
    };
    drop(markets);
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(EngineMsg::FindOpenOrders {
        user_address: user.solana_address.clone(),
        market_id: market_id.to_string(),
        resp: resp_tx,
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "engine send failed".into(),
        )
    })?;

    let open_orders = resp_rx
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "engine dropped".into()))?;
    Ok(Json(open_orders))
}
