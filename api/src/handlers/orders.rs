use std::str::FromStr;

use anchor_client_sdk::{
    predix_program::types::TradeSide,
    utils::{get_match_fills, get_remaining_accounts},
};
use axum::{Extension, Json, extract::State, http::StatusCode};
use matching::types::OrderEntry;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    engine::engine::{EngineMsg, run_market_engine},
    models::{
        auth::AuthUser,
        orders::{
            CancelReq, CancelRes, MergeOrderReq, MergeOrderRes, PlaceOrderReq, PlaceOrderRes, ShareType, SplitOrderReq, SplitOrderRes
        },
    },
    state::state::Shared,
    utils::solana::verify_delegation,
};

pub async fn place_order(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<PlaceOrderReq>,
) -> Result<Json<PlaceOrderRes>, (StatusCode, String)> {
    let order_id = Uuid::new_v4();
    let market_id_str = req.market_id.clone();
    let market_id = market_id_str
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let order = OrderEntry {
        id: order_id,
        user_address: user.solana_address.clone(),
        market_id: market_id,
        price: req.price,
        qty: req.qty,
    };

    let mut markets = state.markets.write().await;
    let tx = if let Some(tx) = markets.get(&market_id) {
        tx.clone()
    } else {
        let (tx, rx) = mpsc::channel::<EngineMsg>(100);
        tokio::spawn(run_market_engine(rx));
        markets.insert(market_id, tx.clone());
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
    let (_id, trades, rem) = resp_rx
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "engine dropped".into()))?;
    let trade_side = match req.share {
        ShareType::Yes => TradeSide::Yes,
        ShareType::No => TradeSide::No,
    };
    dbg!("Trades: {:?}", &trades);
    dbg!("Remaining Qty: {:?}", &rem);
    if trades.is_empty() {
        return Ok(Json(PlaceOrderRes {
            order_id,
            trades,
            remaining_qty: rem,
            message: "Order placed successfully with no matches".into(),
        }));
    }
    verify_delegation(
        &state.rpc_client,
        &user.solana_address,
        &req.collateral_mint,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Verification error: {}", e),
        )
    })?;
    let market_id_str = req.market_id.clone();
    let market_id = market_id_str
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let match_fills = get_match_fills(&trades, trade_side);
    let remaining_accounts = get_remaining_accounts(&trades, trade_side, market_id);
    state
        .predix_sdk
        .place_order(market_id, match_fills, remaining_accounts)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to place order on chain: {}", e),
            )
        })?;
    Ok(Json(PlaceOrderRes {
        order_id,
        trades,
        remaining_qty: rem,
        message: "Order placed successfully".into(),
    }))
}

pub async fn cancel_order(
    State(state): State<Shared>,
    Extension(_user): Extension<AuthUser>,
    Json(req): Json<CancelReq>,
) -> Result<Json<CancelRes>, (StatusCode, String)> {
    let markets = state.markets.read().await;
    let market_id_str = req.market_id.clone();
    let market_id = market_id_str
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let tx = if let Some(tx) = markets.get(&market_id) {
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
    let market_id_str = req.market_id.clone();
    let market_id = market_id_str
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let tx = state
        .predix_sdk
        .split_order(market_id, &user_pubkey, &collateral_mint, req.amount)
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
    let market_id_str = req.market_id.clone();
    let market_id = market_id_str
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let tx = state
        .predix_sdk
        .merge_order(market_id, &user_pubkey, &collateral_mint, req.amount)
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
