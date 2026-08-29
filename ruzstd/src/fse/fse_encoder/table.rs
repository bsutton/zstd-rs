use crate::bit_io::BitWriter;
use alloc::vec::Vec;

use super::write_normalized_probabilities;

#[derive(Debug, Clone)]
pub struct FSETable {
    state_table: Vec<u16>,
    /// Indexed by symbol.
    symbols: Vec<SymbolTransform>,
    acc_log: u8,
}

impl FSETable {
    #[inline(always)]
    pub(crate) fn encode_symbol(&self, symbol: u8, state: u32) -> (u32, u8, u32) {
        let symbol = usize::from(symbol);
        debug_assert!(symbol < self.symbols.len());
        // SAFETY: Encoding callers only supply symbols accepted by the table.
        // Table selection checks repeat/predefined tables, while newly built
        // tables are derived from the data they encode.
        let transform = unsafe { self.symbols.get_unchecked(symbol) };
        debug_assert_ne!(transform.probability, 0);
        let num_bits = state.wrapping_add(transform.delta_nb_bits) >> 16;
        let table_index = i64::from(state >> num_bits) + i64::from(transform.delta_find_state);
        debug_assert!(table_index >= 0);
        let table_index = table_index as usize;
        debug_assert!(table_index < self.state_table.len());
        // SAFETY: The compression transform maps every valid C state into the
        // compact state table. The exhaustive module test covers every valid
        // symbol/state pair for representative and predefined tables.
        let next_state = u32::from(unsafe { *self.state_table.get_unchecked(table_index) });
        let bits = state & (1u32 << num_bits).wrapping_sub(1);
        (bits, num_bits as u8, next_state)
    }

    pub(crate) fn c_start_state_index(&self, symbol: u8) -> u32 {
        let symbol = usize::from(symbol);
        debug_assert!(symbol < self.symbols.len());
        // SAFETY: Same validated-symbol invariant as `encode_symbol()`.
        let transform = unsafe { self.symbols.get_unchecked(symbol) };
        debug_assert_ne!(transform.probability, 0);
        let num_bits = transform.delta_nb_bits.wrapping_add(1 << 15) >> 16;
        let value = (num_bits << 16).wrapping_sub(transform.delta_nb_bits);
        let table_index = i64::from(value >> num_bits) + i64::from(transform.delta_find_state);
        debug_assert!(table_index >= 0);
        let table_index = table_index as usize;
        debug_assert!(table_index < self.state_table.len());
        // SAFETY: The start-state transform obeys the same compact-table
        // bounds established during construction.
        u32::from(unsafe { *self.state_table.get_unchecked(table_index) })
    }

    #[inline(always)]
    pub(crate) fn state_bits(&self, state: u32) -> u32 {
        state & ((1u32 << self.acc_log).wrapping_sub(1))
    }

    pub(crate) fn can_encode_symbol(&self, symbol: u8) -> bool {
        self.symbols
            .get(symbol as usize)
            .is_some_and(|transform| transform.probability != 0)
    }

    pub(crate) fn bit_cost(&self, symbol: u8, accuracy_log: u8) -> Option<usize> {
        if !self.can_encode_symbol(symbol) {
            return None;
        }

        let table_log = self.acc_log;
        let table_size = 1usize << table_log;
        let delta_nb_bits = self.delta_nb_bits(symbol) as usize;
        let min_nb_bits = delta_nb_bits >> 16;
        let threshold = (min_nb_bits + 1) << 16;
        let delta_from_threshold = threshold.checked_sub(delta_nb_bits + table_size)?;
        let normalized_delta_from_threshold = (delta_from_threshold << accuracy_log) >> table_log;
        let bit_multiplier = 1usize << accuracy_log;

        Some((min_nb_bits + 1) * bit_multiplier - normalized_delta_from_threshold)
    }

    pub fn acc_log(&self) -> u8 {
        self.acc_log
    }

    pub(crate) fn normalized_probability(&self, symbol: u8) -> i32 {
        self.symbols
            .get(symbol as usize)
            .map_or(0, |transform| transform.probability)
    }

    #[cfg(test)]
    pub(crate) fn probabilities(&self) -> Vec<i32> {
        self.symbols
            .iter()
            .map(|symbol| symbol.probability)
            .collect()
    }

    fn delta_nb_bits(&self, symbol: u8) -> u32 {
        let symbol = usize::from(symbol);
        debug_assert!(symbol < self.symbols.len());
        // SAFETY: The only caller first verifies `can_encode_symbol()`.
        unsafe { self.symbols.get_unchecked(symbol).delta_nb_bits }
    }

    pub(crate) fn write_table<V: AsMut<Vec<u8>>>(&self, writer: &mut BitWriter<V>) {
        write_normalized_probabilities(
            self.symbols.iter().map(|symbol| symbol.probability),
            self.acc_log(),
            writer,
        );
    }

    #[cfg(test)]
    pub(super) fn state_table_len(&self) -> usize {
        self.state_table.len()
    }
}

#[derive(Debug, Clone, Copy)]
struct SymbolTransform {
    probability: i32,
    delta_nb_bits: u32,
    delta_find_state: i32,
}

/// Reusable workspace for C-style FSE compression-table construction.
///
/// `FSE_buildCTable_wksp()` receives these two temporary regions from its
/// caller. Keeping them outside the returned table lets a compression context
/// retain their allocations across LL, OF, and ML construction and then across
/// source blocks.
#[derive(Debug, Default)]
pub(crate) struct FSETableBuildScratch {
    cumulative: Vec<usize>,
    table_symbols: Vec<u8>,
    recycled_tables: Vec<FSETable>,
}

impl FSETableBuildScratch {
    pub(crate) const fn new() -> Self {
        Self {
            cumulative: Vec::new(),
            table_symbols: Vec::new(),
            recycled_tables: Vec::new(),
        }
    }

    pub(crate) fn recycle_table(&mut self, table: FSETable) {
        if self.recycled_tables.len() < 3 {
            self.recycled_tables.push(table);
        }
    }

    fn take_recycled_table(&mut self, state_len: usize, symbol_len: usize) -> Option<FSETable> {
        let best = self
            .recycled_tables
            .iter()
            .enumerate()
            .filter(|(_, table)| {
                table.state_table.capacity() >= state_len && table.symbols.capacity() >= symbol_len
            })
            .min_by_key(|(_, table)| table.state_table.capacity() + table.symbols.capacity())
            .map(|(index, _)| index)
            .or_else(|| (!self.recycled_tables.is_empty()).then_some(0))?;
        Some(self.recycled_tables.swap_remove(best))
    }

    #[cfg(test)]
    pub(crate) fn retained_capacity(&self) -> (usize, usize) {
        (self.cumulative.capacity(), self.table_symbols.capacity())
    }

    #[cfg(test)]
    pub(crate) fn recycled_table_count(&self) -> usize {
        self.recycled_tables.len()
    }

    #[cfg(test)]
    pub(crate) fn recycled_table_allocation_addresses(&self) -> Vec<(usize, usize)> {
        self.recycled_tables
            .iter()
            .map(|table| {
                (
                    table.state_table.as_ptr() as usize,
                    table.symbols.as_ptr() as usize,
                )
            })
            .collect()
    }
}

pub(crate) fn build_rle_table(symbol: u8) -> FSETable {
    let mut probabilities = Vec::with_capacity(usize::from(symbol) + 1);
    probabilities.resize(usize::from(symbol), 0);
    probabilities.push(1);
    build_table_from_probabilities(&probabilities, 0)
}

#[cfg_attr(target_vendor = "apple", link_section = "__TEXT,__rz_fset")]
#[cfg_attr(target_family = "windows", link_section = ".text$043.rz.fset")]
#[cfg_attr(
    all(
        not(target_vendor = "apple"),
        not(target_family = "windows"),
        not(target_family = "wasm")
    ),
    link_section = ".text.sorted.043.ruzstd.fse.table"
)]
pub(crate) fn build_table_from_probabilities(probs: &[i32], acc_log: u8) -> FSETable {
    build_table_from_probabilities_with_scratch(probs, acc_log, &mut FSETableBuildScratch::new())
}

pub(crate) fn build_table_from_probabilities_with_scratch(
    probs: &[i32],
    acc_log: u8,
    scratch: &mut FSETableBuildScratch,
) -> FSETable {
    debug_assert!(acc_log <= 12);
    let table_size = 1usize << acc_log;
    scratch.cumulative.clear();
    scratch.cumulative.resize(probs.len() + 1, 0);
    scratch.table_symbols.clear();
    scratch.table_symbols.resize(table_size, 0);
    let mut table = scratch
        .take_recycled_table(table_size, probs.len())
        .unwrap_or_else(|| FSETable {
            state_table: Vec::new(),
            symbols: Vec::new(),
            acc_log,
        });
    let cumulative = &mut scratch.cumulative;
    let table_symbols = &mut scratch.table_symbols;
    let mut high_threshold = table_size - 1;

    for symbol in 1..=probs.len() {
        let probability = probs[symbol - 1];
        if probability == -1 {
            cumulative[symbol] = cumulative[symbol - 1] + 1;
            table_symbols[high_threshold] = (symbol - 1) as u8;
            high_threshold = high_threshold.saturating_sub(1);
        } else {
            cumulative[symbol] = cumulative[symbol - 1] + probability as usize;
        }
    }

    let mut position = 0usize;
    for (symbol, probability) in probs.iter().copied().enumerate() {
        for _ in 0..probability.max(0) {
            table_symbols[position] = symbol as u8;
            position = next_position(position, table_size);
            while position > high_threshold {
                position = next_position(position, table_size);
            }
        }
    }

    table.state_table.clear();
    table.state_table.resize(table_size, 0);
    for (decode_index, symbol) in table_symbols.iter().copied().enumerate() {
        let cumulative_entry = &mut cumulative[usize::from(symbol)];
        table.state_table[*cumulative_entry] = (table_size + decode_index) as u16;
        *cumulative_entry += 1;
    }
    fill_symbol_transforms(probs, acc_log, &mut table.symbols);
    table.acc_log = acc_log;
    table
}

pub(crate) fn build_probability_table_for_estimate(probs: &[i32], acc_log: u8) -> FSETable {
    FSETable {
        state_table: Vec::new(),
        symbols: symbol_transforms(probs, acc_log),
        acc_log,
    }
}

fn symbol_transforms(probs: &[i32], acc_log: u8) -> Vec<SymbolTransform> {
    let mut symbols = Vec::with_capacity(probs.len());
    fill_symbol_transforms(probs, acc_log, &mut symbols);
    symbols
}

fn fill_symbol_transforms(probs: &[i32], acc_log: u8, symbols: &mut Vec<SymbolTransform>) {
    let table_log = u32::from(acc_log);
    let table_size = 1u32 << table_log;
    let mut total = 0u32;
    symbols.clear();
    symbols.reserve(probs.len());

    for probability in probs.iter().copied() {
        let (delta_nb_bits, delta_find_state) = match probability {
            0 => (((table_log + 1) << 16) - table_size, 0),
            -1 | 1 => {
                let transform = ((table_log << 16).wrapping_sub(table_size), total as i32 - 1);
                total += 1;
                transform
            }
            probability => {
                debug_assert!(probability > 1);
                let probability = probability as u32;
                let max_bits_out = table_log - highbit32(probability - 1);
                let min_state_plus = probability << max_bits_out;
                let transform = (
                    (max_bits_out << 16) - min_state_plus,
                    total as i32 - probability as i32,
                );
                total += probability;
                transform
            }
        };
        symbols.push(SymbolTransform {
            probability,
            delta_nb_bits,
            delta_find_state,
        });
    }
}

fn highbit32(value: u32) -> u32 {
    u32::BITS - 1 - value.leading_zeros()
}

/// Calculate the position of the next entry of the table.
fn next_position(mut position: usize, table_size: usize) -> usize {
    position += (table_size >> 1) + (table_size >> 3) + 3;
    position &= table_size - 1;
    position
}

pub(super) const ML_DIST: &[i32] = &[
    1, 4, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1,
];

pub(super) const LL_DIST: &[i32] = &[
    4, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 1, 1, 1, 1, 1,
    -1, -1, -1, -1,
];

pub(super) const OF_DIST: &[i32] = &[
    1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1,
];

pub(crate) fn default_ml_table() -> FSETable {
    build_table_from_probabilities(ML_DIST, 6)
}

pub(crate) fn default_ll_table() -> FSETable {
    build_table_from_probabilities(LL_DIST, 6)
}

pub(crate) fn default_of_table() -> FSETable {
    build_table_from_probabilities(OF_DIST, 5)
}

#[cfg(test)]
mod access_tests {
    use super::{
        build_table_from_probabilities, build_table_from_probabilities_with_scratch,
        default_ll_table, default_ml_table, default_of_table, FSETable, FSETableBuildScratch,
    };

    #[test]
    fn reusable_build_workspace_matches_fresh_tables_across_alphabets() {
        let cases: &[(&[i32], u8)] = &[
            (&[1, 4, 3, 2, 2, 2, -1, -1], 4),
            (&[9, 0, 0, 3, 1, 1, 1, 1], 4),
            (&[8, 6, 4, 3, 2, 1, 1, 1, 1, 1, -1, -1, -1, -1, 0, 0], 5),
        ];
        let mut scratch = FSETableBuildScratch::new();

        for _ in 0..2 {
            for &(probabilities, table_log) in cases {
                let fresh = build_table_from_probabilities(probabilities, table_log);
                let reused = build_table_from_probabilities_with_scratch(
                    probabilities,
                    table_log,
                    &mut scratch,
                );
                assert_eq!(reused.acc_log, fresh.acc_log);
                assert_eq!(reused.state_table, fresh.state_table);
                assert_eq!(reused.symbols.len(), fresh.symbols.len());
                for (actual, expected) in reused.symbols.iter().zip(&fresh.symbols) {
                    assert_eq!(actual.probability, expected.probability);
                    assert_eq!(actual.delta_nb_bits, expected.delta_nb_bits);
                    assert_eq!(actual.delta_find_state, expected.delta_find_state);
                }
            }
        }

        let (cumulative_capacity, spread_capacity) = scratch.retained_capacity();
        assert!(cumulative_capacity >= 17);
        assert!(spread_capacity >= 32);
    }

    #[test]
    fn recycled_output_table_reuses_both_owning_allocations() {
        let probabilities = &[8, 6, 4, 3, 2, 1, 1, 1, 1, 1, -1, -1, -1, -1, 0, 0];
        let mut scratch = FSETableBuildScratch::new();
        let first = build_table_from_probabilities_with_scratch(probabilities, 5, &mut scratch);
        let state_allocation = first.state_table.as_ptr();
        let symbol_allocation = first.symbols.as_ptr();

        scratch.recycle_table(first);
        assert_eq!(scratch.recycled_table_count(), 1);
        let second = build_table_from_probabilities_with_scratch(probabilities, 5, &mut scratch);

        assert_eq!(scratch.recycled_table_count(), 0);
        assert_eq!(second.state_table.as_ptr(), state_allocation);
        assert_eq!(second.symbols.as_ptr(), symbol_allocation);
    }

    #[test]
    fn compression_transforms_cover_every_valid_symbol_and_state() {
        let tables = [
            default_ll_table(),
            default_ml_table(),
            default_of_table(),
            build_table_from_probabilities(&[3, 2, 1, 1, -1], 3),
        ];

        for table in &tables {
            assert_table_accesses(table);
        }
    }

    fn assert_table_accesses(table: &FSETable) {
        let table_size = 1u32 << table.acc_log;
        for symbol in 0..table.symbols.len() {
            if table.symbols[symbol].probability == 0 {
                continue;
            }

            let symbol = symbol as u8;
            let start = table.c_start_state_index(symbol);
            assert!((table_size..2 * table_size).contains(&start));

            for state in table_size..2 * table_size {
                let (_, _, next_state) = table.encode_symbol(symbol, state);
                assert!(
                    (table_size..2 * table_size).contains(&next_state),
                    "log={} symbol={symbol} state={state} next={next_state}",
                    table.acc_log
                );
            }
        }
    }
}
