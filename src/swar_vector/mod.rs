// =============================================================================
// 1. SWAR (SIMD Within A Register) ENGINE - 64-bit Parallel Operations
// =============================================================================

pub struct SwarEngine;

impl SwarEngine {
    /// Detekcija nultog bajta (0x00) unutar 64-bitnog registra (8 bajtova odjednom)
    /// bez ijednog `if` uslova. (Klasika iz `strlen` C biblioteke).
    pub fn has_zero_byte(v: u64) -> bool {
        let mask_sub = v.wrapping_sub(0x01010101_01010101);
        let mask_not = !v;
        let mask_high = 0x80808080_80808080;
        (mask_sub & mask_not & mask_high) != 0
    }

    /// SWAR ASCII To Upper: Prevara 8 ASCII karaktera u velika slova istovremeno
    /// potpunom eliminacijom grananja.
    pub fn ascii_to_upper_u64(v: u64) -> u64 {
        // Proverava koji su bajtovi u opsegu malih slova 'a'..'z' (0x61..0x7A)
        let add_ba = v.wrapping_add(0x7F7F7F7F_7F7F7F7F - 0x60606060_60606060);
        let sub_zb = 0x7F7F7F7F_7F7F7F7F - 0x7A7A7A7A_7A7A7A7A + v;
        let is_lowercase = (add_ba ^ sub_zb) & 0x80808080_80808080;
        
        // Formiramo masku od 0x20 bita za svaki bajt koji je bio malo slovo
        let mask_sub_32 = (is_lowercase >> 2) | (is_lowercase >> 3);
        v ^ mask_sub_32
    }
}

// =============================================================================
// 2. AVX-512 VECTOR SIMULATOR (512-bit ZMM Registers & Opmasks)
// =============================================================================

/// 512-bitni ZMM registar koji sadrži 16 x 32-bitnih elemenata
#[derive(Debug, Clone, Copy)]
pub struct ZmmRegister {
    pub lanes: [u32; 16],
}

impl ZmmRegister {
    pub fn new(val: u32) -> Self {
        Self { lanes: [val; 16] }
    }

    pub fn from_array(arr: [u32; 16]) -> Self {
        Self { lanes: arr }
    }
}

pub struct Avx512Engine;

impl Avx512Engine {
    /// Simulira AVX-512 Poređenje sa kreiranjem opmaske (_mm512_cmp_epi32_mask)
    /// Vraća u16 masku gde svaki bit predstavlja rezultat poređenja za jednu traku (lane).
    pub fn compare_gt_mask(a: ZmmRegister, b: ZmmRegister) -> u16 {
        let mut mask: u16 = 0;
        for i in 0..16 {
            if a.lanes[i] > b.lanes[i] {
                mask |= 1 << i;
            }
        }
        mask
    }

    /// Simulira AVX-512 Maskirano Sabiranje (_mm512_mask_add_epi32)
    /// Sabira elemente samo na trakama gde je maska (k1 registar) postavljena na 1.
    pub fn masked_add(
        src1: ZmmRegister,
        src2: ZmmRegister,
        mask: u16,
        passthrough: ZmmRegister,
    ) -> ZmmRegister {
        let mut result = passthrough;
        for i in 0..16 {
            if (mask & (1 << i)) != 0 {
                result.lanes[i] = src1.lanes[i].wrapping_add(src2.lanes[i]);
            }
        }
        result
    }
}