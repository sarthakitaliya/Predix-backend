use std::collections::{BTreeMap, VecDeque};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum Side {
    Bid,
    Ask,
}

#[derive(Clone, Serialize, Debug)]
pub struct OrderEntry {
    pub id: Uuid,
    pub user_id: String,
    pub market_id: String,
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    market_id: String,
    buyer_id: String,
    seller_id: String,
    price: Decimal,
    quantity: Decimal,
}
#[derive(Default, Serialize, Debug)]
pub struct OrderBook {
    bids: BTreeMap<Decimal, VecDeque<OrderEntry>>,
    asks: BTreeMap<Decimal, VecDeque<OrderEntry>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    // Get the highest bid price
    pub fn best_bid(&self) -> Option<Decimal> {
        self.bids.keys().next_back().copied()
    }
    // Get the lowest ask price
    pub fn best_ask(&self) -> Option<Decimal> {
        self.asks.keys().next().copied()
    }
    pub fn place_order(&mut self, mut order: OrderEntry, side: Side) -> (Uuid, Vec<Trade>, Decimal) {
        let mut trades: Vec<Trade> = Vec::new();
        match side {
            Side::Bid => {
                dbg!(&order);
                while order.qty > Decimal::ZERO {
                    print!("Bid order matching in progress");
                    // get the best ask price
                    let Some(best_ask_price) = self.best_ask() else {
                        break;
                    };
                    // if the order price is less than the best ask price, stop matching
                    if order.price < best_ask_price || order.qty == Decimal::ZERO {
                        break;
                    }
                    // get the queue of orders at the best ask price
                    let queue = self.asks.get_mut(&best_ask_price).unwrap();
                    while order.qty > Decimal::ZERO {
                        // maker is the order at the front of the queue
                        if let Some(maker) = queue.front_mut() {
                            let take: Decimal = order.qty.min(maker.qty);
                            maker.qty -= take;
                            order.qty -= take;

                            // record the trade
                            trades.push(Trade {
                                buyer_id: order.user_id.clone(),
                                seller_id: maker.user_id.clone(),
                                price: order.price,
                                quantity: take,
                                market_id: maker.market_id.clone(),
                            });

                            // remove the maker if fully filled
                            if maker.qty == Decimal::ZERO {
                                queue.pop_front();
                            }

                            // if the queue is empty, remove the price level
                            if queue.is_empty() {
                                drop(queue);
                                self.asks.remove(&best_ask_price);
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if order.qty > Decimal::ZERO {
                        print!("Adding remaining bid order to book");
                        let order_id = Uuid::new_v4();
                        let entry = OrderEntry {
                            id: order_id,
                            user_id: order.user_id.clone(),
                            market_id: order.market_id.clone(),
                            price: order.price,
                            qty: order.qty,
                        };
                        self.bids.entry(order.price).or_default().push_back(entry);
                    }
                    print!("Bid order matching in progress");
                }
                print!("Bid order processed: {:?} and {:?}", self.bids.len(), self.asks.len());
                dbg!(&self.bids);
                dbg!(&self.asks);
                dbg!(&trades);

            }
            Side::Ask => {
                while order.qty > Decimal::ZERO {
                    print!("Ask order matching in progress");
                    let Some(best_bid_price) = self.best_bid() else {
                        break;
                    };

                    if order.price > best_bid_price || order.qty == Decimal::ZERO {
                        break;
                    }

                    let queue = self.bids.get_mut(&best_bid_price).unwrap();
                    while order.qty > Decimal::ZERO {
                        if let Some(maker) = queue.front_mut() {
                            let take: Decimal = maker.qty.min(order.qty);
                            maker.qty -= take;
                            order.qty -= take;

                            trades.push(Trade {
                                buyer_id: maker.user_id.clone(),
                                seller_id: order.user_id.clone(),
                                price: order.price,
                                quantity: take,
                                market_id: maker.market_id.clone(),
                            });

                            if maker.qty == Decimal::ZERO {
                                queue.pop_front();
                            }

                            if queue.is_empty() {
                                drop(queue);
                                self.bids.remove(&best_bid_price);
                                break;
                            }
                        } else {
                            break;
                        }
                        print!("Ask order matching in progress");
                    }
                    if order.qty > Decimal::ZERO {
                        print!("Adding remaining ask order to book");
                        let order_id = Uuid::new_v4();
                        let entry = OrderEntry {
                            id: order_id,
                            user_id: order.user_id.clone(),
                            market_id: order.market_id.clone(),
                            price: order.price.clone(),
                            qty: order.qty.clone(),
                        };
                        self.asks
                            .entry(order.price.clone())
                            .or_default()
                            .push_back(entry);
                        dbg!(&self.asks);
                    }
                }
                print!("Bid order processed: {:?} and {:?}", self.bids.len(), self.asks.len());
            }
        }
        (order.id, trades, order.qty)
    }

    pub fn cancel_order(&mut self, side: Side, price: Decimal, order_id: Uuid) -> bool {
        let map = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        if let Some(q) = map.get_mut(&price) {
            if let Some(pos) = q.iter().position(|o| o.id == order_id) {
                q.remove(pos);
                if q.is_empty() {
                    map.remove(&price);
                }
                return true;
            }
        }
        false
    }
}

pub struct MarketBooks {
    pub yes: OrderBook,
    pub no: OrderBook,
}
#[derive(Serialize, Debug)]
pub struct SnapshotData {
    pub price: Decimal,
    pub quantity: Decimal,
    pub total: Decimal,
}
#[derive(Serialize)]
pub struct MarketSnapshot {
    pub yes: (Vec<SnapshotData>, Vec<SnapshotData>),
    pub no: (Vec<SnapshotData>, Vec<SnapshotData>),
}
impl MarketBooks {
    pub fn new() -> Self {
        Self {
            yes: OrderBook::new(),
            no: OrderBook::new(),
        }
    }
    pub fn snapshot(&self) -> ((Vec<SnapshotData>, Vec<SnapshotData>), (Vec<SnapshotData>, Vec<SnapshotData>)) {
        let yes_bids = self.yes.bids.iter().rev().map(|(p, q)| SnapshotData {
            price: *p,
            quantity: q.iter().map(|o| o.qty).sum(),
            total: q.iter().map(|o| o.qty * o.price).sum(),
        }).collect::<Vec<SnapshotData>>();
        let yes_asks = self.yes.asks.iter().rev().map(|(p, q)| SnapshotData {
            price: *p,
            quantity: q.iter().map(|o| o.qty).sum(),
            total: q.iter().map(|o| o.qty * o.price).sum(),
        }).collect::<Vec<SnapshotData>>();
        let no_bids = self.no.bids.iter().rev().map(|(p, q)| SnapshotData {
            price: *p,
            quantity: q.iter().map(|o| o.qty).sum(),
            total: q.iter().map(|o| o.qty * o.price).sum(),
        }).collect::<Vec<SnapshotData>>();
        let no_asks = self.no.asks.iter().rev().map(|(p, q)| SnapshotData {
            price: *p,
            quantity: q.iter().map(|o| o.qty).sum(),
            total: q.iter().map(|o| o.qty * o.price).sum(),
        }).collect::<Vec<SnapshotData>>();
        let yes = (yes_bids, yes_asks);
        let no = (no_bids, no_asks);
        print!("Snapshot generated: {:?}, {:?}", yes, no);
        (yes, no)
    }
}


