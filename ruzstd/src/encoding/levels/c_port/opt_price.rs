//! Optimal-parser price model ported from the no-dictionary path in
//! `zstd_opt.c`.

use super::hash_chain_match::highbit32;
use crate::encoding::blocks::{literal_length_code, match_length_code};

const BITCOST_ACCURACY: u32 = 8;
pub(super) const BITCOST_MULTIPLIER: u32 = 1 << BITCOST_ACCURACY;
pub(super) const ZSTD_MAX_PRICE: i32 = 1 << 30;
const ZSTD_BLOCKSIZE_MAX: u32 = 128 * 1024;
const ZSTD_PREDEF_THRESHOLD: usize = 8;
const ZSTD_LITFREQ_ADD: u32 = 2;
const MINMATCH: u32 = 3;

const MAX_LL: usize = 35;
const MAX_ML: usize = 52;
const MAX_OFF: usize = 31;
const MAX_LIT: usize = 255;

const LL_BITS: [u8; MAX_LL + 1] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];

const ML_BITS: [u8; MAX_ML + 1] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];

const BASE_LL_FREQS: [u32; MAX_LL + 1] = [
    4, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1,
];

const BASE_OFF_FREQS: [u32; MAX_OFF + 1] = [
    6, 2, 1, 1, 2, 3, 4, 4, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OptLevel {
    BtOpt,
    BtUltra,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PriceType {
    Predefined,
    Dynamic,
}

#[derive(Clone, Debug)]
pub(super) struct OptPriceState {
    lit_freq: [u32; MAX_LIT + 1],
    lit_length_freq: [u32; MAX_LL + 1],
    match_length_freq: [u32; MAX_ML + 1],
    off_code_freq: [u32; MAX_OFF + 1],
    lit_sum: u32,
    lit_length_sum: u32,
    match_length_sum: u32,
    off_code_sum: u32,
    lit_sum_base_price: u32,
    lit_length_sum_base_price: u32,
    match_length_sum_base_price: u32,
    off_code_sum_base_price: u32,
    price_type: PriceType,
    compressed_literals: bool,
}

impl OptPriceState {
    pub(super) fn new() -> Self {
        Self {
            lit_freq: [0; MAX_LIT + 1],
            lit_length_freq: [0; MAX_LL + 1],
            match_length_freq: [0; MAX_ML + 1],
            off_code_freq: [0; MAX_OFF + 1],
            lit_sum: 0,
            lit_length_sum: 0,
            match_length_sum: 0,
            off_code_sum: 0,
            lit_sum_base_price: 0,
            lit_length_sum_base_price: 0,
            match_length_sum_base_price: 0,
            off_code_sum_base_price: 0,
            price_type: PriceType::Dynamic,
            compressed_literals: true,
        }
    }

    pub(super) fn rescale_freqs(&mut self, src: &[u8], opt_level: OptLevel) {
        self.price_type = PriceType::Dynamic;

        if self.lit_length_sum == 0 {
            if src.len() <= ZSTD_PREDEF_THRESHOLD {
                self.price_type = PriceType::Predefined;
            }

            if self.compressed_literals {
                self.lit_freq = [0; MAX_LIT + 1];
                for &literal in src {
                    self.lit_freq[literal as usize] += 1;
                }
                self.lit_sum = downscale_stats(&mut self.lit_freq, 8, false);
            }

            self.lit_length_freq = BASE_LL_FREQS;
            self.lit_length_sum = sum(&self.lit_length_freq);
            self.match_length_freq = [1; MAX_ML + 1];
            self.match_length_sum = (MAX_ML + 1) as u32;
            self.off_code_freq = BASE_OFF_FREQS;
            self.off_code_sum = sum(&self.off_code_freq);
        } else {
            if self.compressed_literals {
                self.lit_sum = scale_stats(&mut self.lit_freq, 12);
            }
            self.lit_length_sum = scale_stats(&mut self.lit_length_freq, 11);
            self.match_length_sum = scale_stats(&mut self.match_length_freq, 11);
            self.off_code_sum = scale_stats(&mut self.off_code_freq, 11);
        }

        self.set_base_prices(opt_level);
    }

    pub(super) fn raw_literals_cost(&self, literals: &[u8], opt_level: OptLevel) -> u32 {
        if literals.is_empty() {
            return 0;
        }

        if !self.compressed_literals {
            return ((literals.len() as u32) << 3) * BITCOST_MULTIPLIER;
        }

        if self.price_type == PriceType::Predefined {
            return (literals.len() as u32 * 6) * BITCOST_MULTIPLIER;
        }

        let mut price = self.lit_sum_base_price * literals.len() as u32;
        let lit_price_max = self.lit_sum_base_price - BITCOST_MULTIPLIER;
        for &literal in literals {
            let lit_price = weight(self.lit_freq[literal as usize], opt_level).min(lit_price_max);
            price -= lit_price;
        }
        price
    }

    pub(super) fn lit_length_price(&self, lit_length: u32, opt_level: OptLevel) -> u32 {
        debug_assert!(lit_length <= ZSTD_BLOCKSIZE_MAX);
        if self.price_type == PriceType::Predefined {
            return weight(lit_length, opt_level);
        }

        if lit_length == ZSTD_BLOCKSIZE_MAX {
            return BITCOST_MULTIPLIER + self.lit_length_price(ZSTD_BLOCKSIZE_MAX - 1, opt_level);
        }

        let ll_code = literal_length_code(lit_length) as usize;
        u32::from(LL_BITS[ll_code]) * BITCOST_MULTIPLIER + self.lit_length_sum_base_price
            - weight(self.lit_length_freq[ll_code], opt_level)
    }

    pub(super) fn match_price(&self, off_base: u32, match_length: u32, opt_level: OptLevel) -> u32 {
        debug_assert!(match_length >= MINMATCH);
        let off_code = highbit32(off_base);
        let ml_base = match_length - MINMATCH;

        if self.price_type == PriceType::Predefined {
            return weight(ml_base, opt_level) + ((16 + off_code) * BITCOST_MULTIPLIER);
        }

        let mut price = off_code * BITCOST_MULTIPLIER + self.off_code_sum_base_price
            - weight(self.off_code_freq[off_code as usize], opt_level);
        if opt_level.favors_small_offsets() && off_code >= 20 {
            price += (off_code - 19) * 2 * BITCOST_MULTIPLIER;
        }

        let ml_code = match_length_code(match_length) as usize;
        price += u32::from(ML_BITS[ml_code]) * BITCOST_MULTIPLIER
            + self.match_length_sum_base_price
            - weight(self.match_length_freq[ml_code], opt_level);
        price + BITCOST_MULTIPLIER / 5
    }

    pub(super) fn update_stats(
        &mut self,
        lit_length: u32,
        literals: &[u8],
        off_base: u32,
        match_length: u32,
    ) {
        debug_assert!(literals.len() >= lit_length as usize);

        if self.compressed_literals {
            for &literal in &literals[..lit_length as usize] {
                self.lit_freq[literal as usize] += ZSTD_LITFREQ_ADD;
            }
            self.lit_sum += lit_length * ZSTD_LITFREQ_ADD;
        }

        let ll_code = literal_length_code(lit_length) as usize;
        self.lit_length_freq[ll_code] += 1;
        self.lit_length_sum += 1;

        let off_code = highbit32(off_base) as usize;
        self.off_code_freq[off_code] += 1;
        self.off_code_sum += 1;

        let ml_code = match_length_code(match_length) as usize;
        self.match_length_freq[ml_code] += 1;
        self.match_length_sum += 1;
    }

    pub(super) fn refresh_base_prices(&mut self, opt_level: OptLevel) {
        self.set_base_prices(opt_level);
    }

    #[cfg(test)]
    pub(super) fn frequency_snapshot(
        &self,
        ll_code: usize,
        ml_code: usize,
        off_code: usize,
    ) -> (u32, u32, u32) {
        (
            self.lit_length_freq[ll_code],
            self.match_length_freq[ml_code],
            self.off_code_freq[off_code],
        )
    }

    fn set_base_prices(&mut self, opt_level: OptLevel) {
        if self.compressed_literals {
            self.lit_sum_base_price = weight(self.lit_sum, opt_level);
        }
        self.lit_length_sum_base_price = weight(self.lit_length_sum, opt_level);
        self.match_length_sum_base_price = weight(self.match_length_sum, opt_level);
        self.off_code_sum_base_price = weight(self.off_code_sum, opt_level);
    }
}

impl OptLevel {
    fn accurate_weights(self) -> bool {
        matches!(self, Self::BtUltra)
    }

    fn favors_small_offsets(self) -> bool {
        matches!(self, Self::BtOpt)
    }
}

fn weight(stat: u32, opt_level: OptLevel) -> u32 {
    if opt_level.accurate_weights() {
        frac_weight(stat)
    } else {
        bit_weight(stat)
    }
}

fn bit_weight(stat: u32) -> u32 {
    highbit32(stat + 1) * BITCOST_MULTIPLIER
}

fn frac_weight(raw_stat: u32) -> u32 {
    let stat = raw_stat + 1;
    let high_bit = highbit32(stat);
    let base_weight = high_bit * BITCOST_MULTIPLIER;
    let frac_weight = (stat << BITCOST_ACCURACY) >> high_bit;
    base_weight + frac_weight
}

fn scale_stats<const N: usize>(table: &mut [u32; N], log_target: u32) -> u32 {
    let previous_sum = sum(table);
    let factor = previous_sum >> log_target;
    if factor <= 1 {
        return previous_sum;
    }
    downscale_stats(table, highbit32(factor), true)
}

fn downscale_stats<const N: usize>(table: &mut [u32; N], shift: u32, base_one: bool) -> u32 {
    let mut sum = 0;
    for stat in table {
        let base = u32::from(base_one || *stat > 0);
        let new_stat = base + (*stat >> shift);
        *stat = new_stat;
        sum += new_stat;
    }
    sum
}

fn sum(table: &[u32]) -> u32 {
    table.iter().sum()
}
