pub mod orderbook;

use orderbook::{LimitOrderBook, OrderSide};
use std::time::Instant;
use std::time::SystemTime;

pub struct QuantumHftEngine {
    pub book: LimitOrderBook,
    pub execution_latency_ns: u64,
    pub bot_enabled: bool,
    pub total_volume_traded: u64,
    pub logs: Vec<String>,
}

impl QuantumHftEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            book: LimitOrderBook::new("QTC/USD"),
            execution_latency_ns: 120, // Simuliranih 120 nanosekundi
            bot_enabled: true,
            total_volume_traded: 0,
            logs: Vec::new(),
        };

        engine.logs.push("Quantum HFT Engine sa ultra-low latency sklopa inicijalizovan.".into());
        engine.seed_initial_liquidity();
        engine
    }

    pub fn seed_initial_liquidity(&mut self) {
        // Generisanje početne ponude i potražnje na berzi
        self.book.add_order(OrderSide::Buy, 100.5, 10);
        self.book.add_order(OrderSide::Buy, 100.2, 25);
        self.book.add_order(OrderSide::Buy, 99.8, 50);

        self.book.add_order(OrderSide::Sell, 101.0, 15);
        self.book.add_order(OrderSide::Sell, 101.4, 30);
        self.book.add_order(OrderSide::Sell, 102.0, 100);

        self.logs.push("Početni Liquidity Pool za QTC/USD uspešno unesen u Order Book.".into());
    }

    pub fn submit_order(&mut self, side: OrderSide, price: f64, quantity: u32) -> u64 {
        let start = Instant::now();

        let id = self.book.add_order(side, price, quantity);

        let elapsed = start.elapsed().as_nanos() as u64;
        self.execution_latency_ns = if elapsed == 0 { 85 } else { elapsed }; // Minimalna simulirana brzina

        self.total_volume_traded += quantity as u64;
        self.logs.push(format!(
            "⚡ HFT Nalog #{}: {:?} {}x {:.2} | Latencija: {}ns",
            id, side, quantity, price, self.execution_latency_ns
        ));

        id
    }

    /// Takt HFT Algoritmatskog Bota koji održava spread i stalno kupuje/prodaje
    pub fn tick_bot(&mut self) {
        if !self.bot_enabled {
            return;
        }

        // Botić generiše slučajne mikro-transakcije oko trenutne cene
        let base_price = 100.0;
        let random_offset = ((SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 20) as f64) / 10.0;

        let buy_p = base_price + random_offset;
        let sell_p = buy_p + 0.4;

        self.submit_order(OrderSide::Buy, buy_p, 5);
        self.submit_order(OrderSide::Sell, sell_p, 5);
    }
}