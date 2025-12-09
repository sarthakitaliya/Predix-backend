use std::collections::{BTreeMap, VecDeque};

use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::types::{OrderEntry, Side, Trade};

#[derive(Default, Serialize, Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<Decimal, VecDeque<OrderEntry>>,
    pub asks: BTreeMap<Decimal, VecDeque<OrderEntry>>,
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
    pub fn place_order(
        &mut self,
        mut order: OrderEntry,
        side: Side,
    ) -> (Uuid, Vec<Trade>, Decimal) {
        let mut trades: Vec<Trade> = Vec::new();
        match side {
            Side::Bid => {
                while order.qty > Decimal::ZERO {
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
                                buyer_address: order.user_address.clone(),
                                seller_address: maker.user_address.clone(),
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
                }
                if order.qty > Decimal::ZERO {
                    let entry = OrderEntry {
                        id: order.id,
                        user_address: order.user_address.clone(),
                        market_id: order.market_id.clone(),
                        side: Side::Bid,
                        price: order.price,
                        qty: order.qty,
                    };
                    self.bids.entry(order.price).or_default().push_back(entry);
                }
            }
            Side::Ask => {
                while order.qty > Decimal::ZERO {
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
                                buyer_address: maker.user_address.clone(),
                                seller_address: order.user_address.clone(),
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
                    }
                }
                if order.qty > Decimal::ZERO {
                    let entry = OrderEntry {
                        id: order.id,
                        user_address: order.user_address.clone(),
                        market_id: order.market_id.clone(),
                        side: Side::Ask,
                        price: order.price.clone(),
                        qty: order.qty.clone(),
                    };
                    self.asks
                        .entry(order.price.clone())
                        .or_default()
                        .push_back(entry);
                }
            }
        }
        (order.id, trades, order.qty)
    }

    pub fn cancel_order(&mut self, side: Side, price: Decimal, order_id: Uuid) -> (bool, String) {
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
                return (true, "done".to_string());
            }
        } else {
            return (false, "Not found".to_string());
        }
        (false, "something went wrong".to_string())
    }
}
