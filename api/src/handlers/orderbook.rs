use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use matching::types::MarketSnapshot;
use tokio::sync::oneshot;

use crate::{engine::engine::EngineMsg, state::state::Shared};

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
