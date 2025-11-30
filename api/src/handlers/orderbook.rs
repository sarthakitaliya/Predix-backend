use std::env;
use std::str::FromStr;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};

use anyhow::Result;
use matching::types::{MarketSnapshot, OrderEntry};
use rust_decimal::Decimal;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    engine::engine::{EngineMsg, run_market_engine},
    models::{
        auth::AuthUser,
        orderbook::{
            CancelReq, CancelRes, MergeOrderReq, MergeOrderRes, PlaceOrderReq, PlaceOrderRes,
            SplitOrderReq, SplitOrderRes,
        },
    },
    state::state::Shared,
};

pub async fn place_order(
    State(state): State<Shared>,
    Json(req): Json<PlaceOrderReq>,
) -> Result<Json<PlaceOrderRes>, (StatusCode, String)> {
    let price =
        Decimal::from_str(&req.price).map_err(|_| (StatusCode::BAD_REQUEST, "bad price".into()))?;
    let qty =
        Decimal::from_str(&req.qty).map_err(|_| (StatusCode::BAD_REQUEST, "bad qty".into()))?;

    let order_id = Uuid::new_v4();
    let order = OrderEntry {
        id: order_id,
        user_id: req.user_id.clone(),
        market_id: req.market_id.clone(),
        price,
        qty,
    };

    let mut markets = state.markets.write().await;
    let tx = if let Some(tx) = markets.get(&req.market_id) {
        tx.clone()
    } else {
        let (tx, rx) = mpsc::channel::<EngineMsg>(100);
        tokio::spawn(run_market_engine(rx));
        markets.insert(req.market_id, tx.clone());
        tx
    };
    drop(markets);

    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(EngineMsg::PlaceOrder {
        side: req.side,
        share: req.share,
        trades: order,
        resp: resp_tx,
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "engine send failed".into(),
        )
    })?;
    let (id, trades, rem) = resp_rx
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "engine dropped".into()))?;

    Ok(Json(PlaceOrderRes {
        order_id,
        trades,
        remaining_qty: rem,
    }))
}

pub async fn cancel_order(
    State(state): State<Shared>,
    Json(req): Json<CancelReq>,
) -> Result<Json<CancelRes>, (StatusCode, String)> {
    let markets = state.markets.read().await;
    let tx = if let Some(tx) = markets.get(&req.market_id) {
        tx.clone()
    } else {
        return Err((StatusCode::NOT_FOUND, "market not found".into()));
    };
    let (resp_tx, resp_rx) = oneshot::channel();

    tx.send(EngineMsg::CloseOrder {
        side: req.side,
        share: req.share,
        price: req.price,
        order_id: req.order_id,
        resp: resp_tx,
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "engine send failed".into(),
        )
    })?;
    let (res, message) = resp_rx
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "engine dropped".into()))?;
    if res {
        Ok(Json(CancelRes {
            success: res,
            message,
        }))
    } else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, message.into()));
    }
}

pub async fn split_order(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<SplitOrderReq>,
) -> Result<Json<SplitOrderRes>, (StatusCode, String)> {
    let collateral_mint = Pubkey::from_str(&req.collateral_mint).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid collateral mint address: {}", e),
        )
    })?;
    let user_pubkey = Pubkey::from_str(&user.solana_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid user pubkey address: {}", e),
        )
    })?;
    // let payer_private_key =
    //     env::var("FEE_PAYER_PRIVATE_KEY").expect("FEE_PAYER_PRIVATE_KEY must be set");
    // let key_pair = Keypair::from_base58_string(&payer_private_key);

    let tx = state
        .predix_sdk
        .split_order(req.market_id, &user_pubkey, &collateral_mint, req.amount)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create split order instruction: {}", e),
            )
        })?;
    dbg!("Split order tx: {}", &tx);
    Ok(Json(SplitOrderRes {
        tx_message: tx,
        message: "Split order instruction created successfully".into(),
    }))
}

pub async fn merge_order(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<MergeOrderReq>,
) -> Result<Json<MergeOrderRes>, (StatusCode, String)> {
    let collateral_mint = Pubkey::from_str(&req.collateral_mint).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid collateral mint address: {}", e),
        )
    })?;
    let user_pubkey = Pubkey::from_str(&user.solana_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid user pubkey address: {}", e),
        )
    })?;

    let tx = state
        .predix_sdk
        .merge_order(req.market_id, &user_pubkey, &collateral_mint, req.amount)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create merge order instruction: {}", e),
            )
        })?;
    dbg!("Merge order tx: {}", &tx);
    Ok(Json(MergeOrderRes {
        tx_message: tx,
        message: "Merge order instruction created successfully".into(),
    }))
}
pub async fn snapshot(
    State(state): State<Shared>,
    Path(market_id): Path<String>,
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
