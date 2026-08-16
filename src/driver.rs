use crate::domain::QuquatVal;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct WinQuantumDriver {
    hardware_register: Arc<AtomicU64>,
}

impl WinQuantumDriver {
    pub fn new() -> Self {
        Self {
            hardware_register: Arc::new(AtomicU64::new(0x5555_5555_5555_5555)),
        }
    }

    pub fn read_register(&self) -> u64 {
        self.hardware_register.load(Ordering::Relaxed)
    }

    pub fn write_register(&self, val: u64) {
        self.hardware_register.store(val, Ordering::Relaxed);
    }

    pub fn set_ququat(&self, idx: usize, val: QuquatVal) {
        let shift = (idx & 0x1F) * 2;
        let mask = !(0b11u64 << shift);
        let new_bits = (val as u64) << shift;

        self.hardware_register
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                Some((old & mask) | new_bits)
            })
            .ok();
    }

    pub fn get_ququat(&self, idx: usize) -> QuquatVal {
        let current = self.read_register();
        let shift = (idx & 0x1F) * 2;
        match ((current >> shift) & 0b11) as u8 {
            0b00 => QuquatVal::Q00,
            0b01 => QuquatVal::Q01,
            0b10 => QuquatVal::Q10,
            0b11 => QuquatVal::Q11,
            _ => unreachable!(),
        }
    }
}

//Ako neko pocne da spaja
//Samo da mu se ovde neuron silicijum i fotonika ne svadjaju