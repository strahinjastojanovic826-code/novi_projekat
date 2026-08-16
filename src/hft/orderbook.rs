use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub price: f64,
    pub quantity: u32,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub trade_id: u64,
    pub symbol: String,
    pub price: f64,
    pub quantity: u32,
    pub buy_order_id: u64,
    pub sell_order_id: u64,
    pub timestamp_ns: u64,
}

pub struct LimitOrderBook {
    pub symbol: String,
    pub bids: Vec<Order>, // Kupovni nalozi (Sortirani opadajuće po ceni)
    pub asks: Vec<Order>, // Prodajni nalozi (Sortirani rastuće po ceni)
    pub trades: Vec<Trade>,
    next_order_id: u64,
    next_trade_id: u64,
}

impl LimitOrderBook {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            bids: Vec::new(),
            asks: Vec::new(),
            trades: Vec::new(),
            next_order_id: 1,
            next_trade_id: 1,
        }
    }

    fn current_time_ns() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }

    /// Dodaj novi limit nalog i pokreni HFT Matching Engine
    pub fn add_order(&mut self, side: OrderSide, price: f64, mut quantity: u32) -> u64 {
        let order_id = self.next_order_id;
        self.next_order_id += 1;

        let now = Self::current_time_ns();

        if side == OrderSide::Buy {
            // Uparivanje sa najjeftinijim Asks
            let mut i = 0;
            while i < self.asks.len() && quantity > 0 {
                if self.asks[i].price <= price {
                    let fill_qty = quantity.min(self.asks[i].quantity);
                    let match_price = self.asks[i].price;

                    quantity -= fill_qty;
                    self.asks[i].quantity -= fill_qty;

                    self.trades.push(Trade {
                        trade_id: self.next_trade_id,
                        symbol: self.symbol.clone(),
                        price: match_price,
                        quantity: fill_qty,
                        buy_order_id: order_id,
                        sell_order_id: self.asks[i].id,
                        timestamp_ns: now,
                    });
                    self.next_trade_id += 1;

                    if self.asks[i].quantity == 0 {
                        self.asks.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    break;
                }
            }

            // Ako ostane neispunjen deo, ubaci u knjigu
            if quantity > 0 {
                self.bids.push(Order {
                    id: order_id,
                    symbol: self.symbol.clone(),
                    side,
                    price,
                    quantity,
                    timestamp_ns: now,
                });
                self.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            }
        } else {
            // Sell nalog: Uparivanje sa najskupljim Bids
            let mut i = 0;
            while i < self.bids.len() && quantity > 0 {
                if self.bids[i].price >= price {
                    let fill_qty = quantity.min(self.bids[i].quantity);
                    let match_price = self.bids[i].price;

                    quantity -= fill_qty;
                    self.bids[i].quantity -= fill_qty;

                    self.trades.push(Trade {
                        trade_id: self.next_trade_id,
                        symbol: self.symbol.clone(),
                        price: match_price,
                        quantity: fill_qty,
                        buy_order_id: self.bids[i].id,
                        sell_order_id: order_id,
                        timestamp_ns: now,
                    });
                    self.next_trade_id += 1;

                    if self.bids[i].quantity == 0 {
                        self.bids.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    break;
                }
            }

            if quantity > 0 {
                self.asks.push(Order {
                    id: order_id,
                    symbol: self.symbol.clone(),
                    side,
                    price,
                    quantity,
                    timestamp_ns: now,
                });
                self.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            }
        }

        order_id
    }

    pub fn spread(&self) -> Option<f64> {
        if let (Some(best_bid), Some(best_ask)) = (self.bids.first(), self.asks.first()) {
            Some(best_ask.price - best_bid.price)
        } else {
            None
        }
    }
}