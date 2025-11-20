use std::str::FromStr;

use axum::{
    Extension, Json,
    extract::{Request, State},
    http::StatusCode,
};

use matching::types::OrderEntry;
use privy_rs::{AuthorizationContext, JwtUser, PrivateKey};
use rust_decimal::Decimal;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::{
    auth::{claims::AuthUser, privy::PClient},
    engine::engine::{EngineMsg, run_market_engine},
    models::order::{CancelReq, CancelRes, PlaceOrderReq, PlaceOrderRes},
    state::Shared,
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

pub async fn health_check(
    Extension(user): Extension<AuthUser>,
    State(state): State<Shared>,
) -> &'static str {
    // let solana_service = state.privy_client.wallets().solana();
    // let authorization_key = std::env::var("PRIVY_SIGNER_PRIVATE_KEY")
    //     .expect("PRIVY_SIGNER_PRIVATE_KEY environment variable not set");
    // let ctx = AuthorizationContext::new()
    // .push(JwtUser((*state.privy_client).clone(), user.access_token.clone()))
    // .push(PrivateKey(authorization_key.to_string()));
    // let sign = solana_service.sign_message(&user.wallet_id, "asdw",&ctx , None).await.unwrap();

    // dbg!(sign);
    // dbg!(_wallets);
    dbg!("Health check called");
    "OK"
}
