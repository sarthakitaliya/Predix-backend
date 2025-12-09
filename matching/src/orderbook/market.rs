
use crate::{orderbook::orderbook::OrderBook, types::{ OpenOrder, SnapshotData}};
pub struct MarketBooks {
    pub yes: OrderBook,
    pub no: OrderBook,
}

impl MarketBooks {
    pub fn new() -> Self {
        Self {
            yes: OrderBook::new(),
            no: OrderBook::new(),
        }
    }
    pub fn snapshot(
        &self,
    ) -> (
        (Vec<SnapshotData>, Vec<SnapshotData>),
        (Vec<SnapshotData>, Vec<SnapshotData>),
    ) {
        let yes_bids = self
            .yes
            .bids
            .iter()
            .rev()
            .map(|(p, q)| SnapshotData {
                price: *p,
                quantity: q.iter().map(|o| o.qty).sum(),
                total: q.iter().map(|o| o.qty * p).sum(),
            })
            .collect::<Vec<SnapshotData>>();
        let yes_asks = self
            .yes
            .asks
            .iter()
            .rev()
            .map(|(p, q)| SnapshotData {
                price: *p,
                quantity: q.iter().map(|o| o.qty).sum(),
                total: q.iter().map(|o| o.qty * p).sum(),
            })
            .collect::<Vec<SnapshotData>>();
        let no_bids = self
            .no
            .bids
            .iter()
            .rev()
            .map(|(p, q)| SnapshotData {
                price: *p,
                quantity: q.iter().map(|o| o.qty).sum(),
                total: q.iter().map(|o| o.qty * p).sum(),
            })
            .collect::<Vec<SnapshotData>>();
        let no_asks = self
            .no
            .asks
            .iter()
            .rev()
            .map(|(p, q)| SnapshotData {
                price: *p,
                quantity: q.iter().map(|o| o.qty).sum(),
                total: q.iter().map(|o| o.qty * p).sum(),
            })
            .collect::<Vec<SnapshotData>>();
        let yes = (yes_bids, yes_asks);
        let no = (no_bids, no_asks);
        (yes, no)
    }

    pub fn find_open_orders(&self, user_address: &String, market_id: &String) -> Vec<OpenOrder> {
        let mut open_orders = Vec::new();

        for orders in self.yes.bids.values().chain(self.yes.asks.values())
        {
            for order in orders {
                if &order.user_address == user_address {
                    let order = OpenOrder {
                        id: order.id.clone(),
                        market_id: market_id.clone(),
                        outcome: "Yes".to_string(),
                        side: order.side.clone(),
                        price: order.price,
                        quantity: order.qty,
                    };
                    open_orders.push(order);
                }
            }
        }
        for orders in self.no.bids.values().chain(self.no.asks.values())
        {
            for order in orders {
                if &order.user_address == user_address {
                    let order = OpenOrder {
                        id: order.id.clone(),
                        market_id: market_id.clone(),
                        outcome: "No".to_string(),
                        side: order.side.clone(),
                        price: order.price,
                        quantity: order.qty,
                    };
                    open_orders.push(order);
                }
            }
        }

        open_orders
    }
}
