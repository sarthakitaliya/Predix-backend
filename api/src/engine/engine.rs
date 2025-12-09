

use matching::{orderbook::market::MarketBooks, types::{OpenOrder, OrderEntry, Side, SnapshotData, Trade}};
use rust_decimal::Decimal;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::models::orders::ShareType;


pub enum EngineMsg {
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
        resp: oneshot::Sender<(bool, String)>,
    },
    Snapshot {
        resp: oneshot::Sender<(
            (Vec<SnapshotData>, Vec<SnapshotData>),
            (Vec<SnapshotData>, Vec<SnapshotData>),
        )>,
    },
    FindOpenOrders {
        user_address: String,
        market_id: String,
        resp: oneshot::Sender<Vec<OpenOrder>>,
    },
}



pub async fn run_market_engine(mut rx: mpsc::Receiver<EngineMsg>) {
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
                let snapshot = book.snapshot();
                let _ = resp.send(snapshot);
            },
            EngineMsg::FindOpenOrders { user_address, market_id, resp } => {
                let open_orders = book.find_open_orders(&user_address, &market_id);
                let _ = resp.send(open_orders);
            }
        }
    }
}
