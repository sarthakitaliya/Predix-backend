use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use matching::{MarketBooks, MarketSnapshot, OrderBook, OrderEntry, Side, SnapshotData, Trade};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc, oneshot};
use uuid::Uuid;

enum EngineMsg {
    PlaceOrder {
        side: Side,
        share: ShareType,
        trades: OrderEntry,
        resp: oneshot::Sender<(Uuid, Vec<Trade>, Decimal)>,
    },
    CloseOrder {
        side: Side,
        share: ShareType,
        price: Decimal,
        order_id: Uuid,
        resp: oneshot::Sender<bool>,
    },
    Snapshot {  resp: oneshot::Sender<((Vec<SnapshotData>, Vec<SnapshotData>), (Vec<SnapshotData>, Vec<SnapshotData>))> }
}

#[derive(Deserialize)]
enum ShareType {
    Yes,
    No,
}


async fn run_market_engine(mut rx: mpsc::Receiver<EngineMsg>) {
    let mut book = MarketBooks::new();
    while let Some(msg) = rx.recv().await {
        match msg {
            EngineMsg::PlaceOrder {
                side,
                share,
                trades,
                resp,
            } => match share {
                ShareType::Yes => {
                    dbg!(&trades);
                    let result = book.yes.place_order(trades, side);
                    let _ = resp.send(result);
                }
                ShareType::No => {
                    let result = book.no.place_order(trades, side);
                    let _ = resp.send(result);
                }
            },
            EngineMsg::CloseOrder {
                side,
                share,
                price,
                order_id,
                resp,
            } => match share {
                ShareType::Yes => {
                    let result = book.yes.cancel_order(side, price, order_id);
                    let _ = resp.send(result);
                }
                ShareType::No => {
                    let result = book.no.cancel_order(side, price, order_id);
                    let _ = resp.send(result);
                }
            },
             EngineMsg::Snapshot { resp } => {
                print!("{:?}", book.yes);
                print!("{:?}", book.no);
                // build yes snapshot
                let yes_snapshot = book.snapshot();
                let _ = resp.send(yes_snapshot);
            }
        }
    }
}
struct AppState {
    markets: RwLock<HashMap<String, mpsc::Sender<EngineMsg>>>,
}

type Shared = Arc<AppState>;

#[derive(Deserialize)]
struct PlaceOrderReq {
    user_id: String,
    market_id: String,
    side: String, // "bid" or "ask"
    share: ShareType,
    price: String,
    qty: String,
}

#[derive(Serialize)]
struct PlaceOrderRes {
    order_id: Uuid,
    trades: Vec<Trade>,
    remaining_qty: Decimal,
}

async fn place_order(
    State(state): State<Shared>,
    Json(req): Json<PlaceOrderReq>,
) -> Result<Json<PlaceOrderRes>, (StatusCode, String)> {
    let side = match req.side.to_lowercase().as_str() {
        "bid" => Side::Bid,
        "ask" => Side::Ask,
        _ => return Err((StatusCode::BAD_REQUEST, "INVALID".into())),
    };
    let price =
        Decimal::from_str(&req.price).map_err(|_| (StatusCode::BAD_REQUEST, "bad price".into()))?;
    let qty =
        Decimal::from_str(&req.qty).map_err(|_| (StatusCode::BAD_REQUEST, "bad qty".into()))?;

        let order_id = Uuid::new_v4();
    let order = OrderEntry {
        id: order_id,
        user_id: "user_id".to_string(),
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
        side,
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

async fn snapshot(
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

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        markets: RwLock::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/", get(|| async { "hello" }))
        .route("/orderbook", post(place_order))
        .route("/snapshot/{market_id}", get(snapshot))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
