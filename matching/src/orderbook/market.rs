use crate::{orderbook::orderbook::OrderBook, types::SnapshotData};

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
}
