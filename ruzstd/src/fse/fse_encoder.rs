use crate::bit_io::BitWriter;
use alloc::vec::Vec;
use core::convert::TryFrom;

const DIRECT_STATE_LOOKUP_MAX_TABLE_SIZE: usize = 1024;

pub(crate) struct FSEEncoder<'output, V: AsMut<Vec<u8>>> {
    pub(super) table: FSETable,
    writer: &'output mut BitWriter<V>,
}

impl<V: AsMut<Vec<u8>>> FSEEncoder<'_, V> {
    pub fn new(table: FSETable, writer: &mut BitWriter<V>) -> FSEEncoder<'_, V> {
        FSEEncoder { table, writer }
    }

    #[cfg(any(test, feature = "fuzz_exports"))]
    pub fn into_table(self) -> FSETable {
        self.table
    }

    /// Encodes the data using the provided table
    /// Writes
    /// * Table description
    /// * Encoded data
    /// * Last state index
    /// * Padding bits to fill up last byte
    #[cfg(any(test, feature = "fuzz_exports"))]
    pub fn encode(&mut self, data: &[u8]) {
        self.write_table();

        let mut state = self.table.start_state(data[data.len() - 1]);
        for x in data[0..data.len() - 1].iter().rev().copied() {
            let next = self.table.next_state(x, state.index);
            let diff = state.index - next.baseline;
            self.writer.write_bits(diff as u64, next.num_bits as usize);
            state = next;
        }
        self.writer
            .write_bits(state.index as u64, self.acc_log() as usize);

        let bits_to_fill = self.writer.misaligned();
        if bits_to_fill == 0 {
            self.writer.write_bits(1u32, 8);
        } else {
            self.writer.write_bits(1u32, bits_to_fill);
        }
    }

    /// Encodes the data using the provided table but with two interleaved streams
    /// Writes
    /// * Table description
    /// * Encoded data with two interleaved states
    /// * Both Last state indexes
    /// * Padding bits to fill up last byte
    pub fn encode_interleaved(&mut self, data: &[u8]) {
        self.write_table();

        let mut state_1 = self.table.start_state(data[data.len() - 1]);
        let mut state_2 = self.table.start_state(data[data.len() - 2]);

        // The first two symbols are represented by the start states
        // Then encode the state transitions for two symbols at a time
        let mut idx = data.len() - 4;
        loop {
            {
                let state = state_1;
                let x = data[idx + 1];
                let next = self.table.next_state(x, state.index);
                let diff = state.index - next.baseline;
                self.writer.write_bits(diff as u64, next.num_bits as usize);
                state_1 = next;
            }
            {
                let state = state_2;
                let x = data[idx];
                let next = self.table.next_state(x, state.index);
                let diff = state.index - next.baseline;
                self.writer.write_bits(diff as u64, next.num_bits as usize);
                state_2 = next;
            }

            if idx < 2 {
                break;
            }
            idx -= 2;
        }

        // Determine if we have an even or odd number of symbols to encode
        // If odd we need to encode the last states transition and encode the final states in the flipped order
        if idx == 1 {
            let state = state_1;
            let x = data[0];
            let next = self.table.next_state(x, state.index);
            let diff = state.index - next.baseline;
            self.writer.write_bits(diff as u64, next.num_bits as usize);
            state_1 = next;

            self.writer
                .write_bits(state_2.index as u64, self.acc_log() as usize);
            self.writer
                .write_bits(state_1.index as u64, self.acc_log() as usize);
        } else {
            self.writer
                .write_bits(state_1.index as u64, self.acc_log() as usize);
            self.writer
                .write_bits(state_2.index as u64, self.acc_log() as usize);
        }

        let bits_to_fill = self.writer.misaligned();
        if bits_to_fill == 0 {
            self.writer.write_bits(1u32, 8);
        } else {
            self.writer.write_bits(1u32, bits_to_fill);
        }
    }

    fn write_table(&mut self) {
        self.table.write_table(self.writer);
    }

    pub(super) fn acc_log(&self) -> u8 {
        self.table.acc_log()
    }
}

#[derive(Debug, Clone)]
pub struct FSETable {
    /// Indexed by symbol
    pub(super) states: [SymbolStates; 256],
    /// Sum of all states.states.len()
    pub(crate) table_size: usize,
    acc_log: u8,
}

impl FSETable {
    pub(crate) fn next_state(&self, symbol: u8, idx: usize) -> &State {
        let states = &self.states[symbol as usize];
        states.get(idx, self.table_size)
    }

    pub(crate) fn start_state(&self, symbol: u8) -> &State {
        let states = &self.states[symbol as usize];
        &states.states[0]
    }

    pub(crate) fn can_encode_symbol(&self, symbol: u8) -> bool {
        !self.states[symbol as usize].states.is_empty()
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
        self.states[symbol as usize].probability
    }

    fn delta_nb_bits(&self, symbol: u8) -> u32 {
        let table_log = u32::from(self.acc_log);
        let table_size = 1u32 << table_log;
        let probability = self.normalized_probability(symbol);

        match probability {
            0 => ((table_log + 1) << 16) - table_size,
            -1 | 1 => (table_log << 16) - table_size,
            probability => {
                debug_assert!(probability > 1);
                let probability = probability as u32;
                let max_bits_out = table_log - highbit32(probability - 1);
                let min_state_plus = probability << max_bits_out;
                (max_bits_out << 16) - min_state_plus
            }
        }
    }

    pub(crate) fn write_table<V: AsMut<Vec<u8>>>(&self, writer: &mut BitWriter<V>) {
        let acc_log = self.acc_log();
        writer.write_bits(acc_log - 5, 4);
        let mut probability_counter = 0usize;
        let probability_sum = 1 << acc_log;

        let mut prob_idx = 0;
        while probability_counter < probability_sum {
            let max_remaining_value = probability_sum - probability_counter + 1;
            let bits_to_write = max_remaining_value.ilog2() + 1;
            let low_threshold = ((1 << bits_to_write) - 1) - (max_remaining_value);
            let mask = (1 << (bits_to_write - 1)) - 1;

            let prob = self.states[prob_idx].probability;
            prob_idx += 1;
            let value = (prob + 1) as u32;
            if value < low_threshold as u32 {
                writer.write_bits(value, bits_to_write as usize - 1);
            } else if value > mask {
                writer.write_bits(value + low_threshold as u32, bits_to_write as usize);
            } else {
                writer.write_bits(value, bits_to_write as usize);
            }

            if prob == -1 {
                probability_counter += 1;
            } else if prob > 0 {
                probability_counter += prob as usize;
            } else {
                let mut zeros = 0u8;
                while self.states[prob_idx].probability == 0 {
                    zeros += 1;
                    prob_idx += 1;
                    if zeros == 3 {
                        writer.write_bits(3u8, 2);
                        zeros = 0;
                    }
                }
                writer.write_bits(zeros, 2);
            }
        }
        writer.write_bits(0u8, writer.misaligned());
    }
}

fn highbit32(value: u32) -> u32 {
    u32::BITS - 1 - value.leading_zeros()
}

#[derive(Debug, Clone)]
pub(super) struct SymbolStates {
    /// Sorted by baseline to allow easy lookup using an index
    pub(super) states: Vec<State>,
    lookup: Vec<u16>,
    pub(super) probability: i32,
}

impl SymbolStates {
    fn get(&self, idx: usize, max_idx: usize) -> &State {
        if !self.lookup.is_empty() {
            let state_idx = self.lookup[idx] as usize;
            debug_assert_ne!(state_idx, u16::MAX as usize);
            return &self.states[state_idx];
        }

        let start_search_at = (idx * self.states.len()) / max_idx;
        self.states[start_search_at..]
            .iter()
            .find(|state| state.contains(idx))
            .unwrap()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct State {
    /// How many bits the range of this state needs to be encoded as
    pub(crate) num_bits: u8,
    /// The first index targeted by this state
    pub(crate) baseline: usize,
    /// The last index targeted by this state (baseline + the maximum number with numbits bits allows)
    pub(crate) last_index: usize,
    /// Index of this state in the decoding table
    pub(crate) index: usize,
}

impl State {
    fn contains(&self, idx: usize) -> bool {
        self.baseline <= idx && self.last_index >= idx
    }
}

pub fn build_table_from_data(
    data: impl Iterator<Item = u8>,
    max_log: u8,
    avoid_0_numbit: bool,
) -> FSETable {
    let mut counts = [0; 256];
    let mut max_symbol = 0;
    for x in data {
        counts[x as usize] += 1;
    }
    for (idx, count) in counts.iter().copied().enumerate() {
        if count > 0 {
            max_symbol = idx;
        }
    }
    build_table_from_counts(&counts[..=max_symbol], max_log, avoid_0_numbit)
}

fn build_table_from_counts(counts: &[usize], max_log: u8, avoid_0_numbit: bool) -> FSETable {
    if max_log <= 6 {
        let (probs, acc_log) = old_normalize_counts(counts, max_log, avoid_0_numbit);
        return build_table_from_probabilities(&probs, acc_log);
    }

    let total = counts.iter().sum::<usize>();
    assert!(total > 0);
    let max_symbol = counts.iter().rposition(|count| *count > 0).unwrap_or(0);
    let acc_log = optimal_table_log(max_log, total, max_symbol);
    let low_prob_count = if total >= 2048 { -1 } else { 1 };
    if let Some(probs) = normalize_counts(counts, acc_log, low_prob_count) {
        build_table_from_probabilities(&probs, acc_log)
    } else {
        let (probs, acc_log) = old_normalize_counts(counts, max_log, avoid_0_numbit);
        build_table_from_probabilities(&probs, acc_log)
    }
}

fn optimal_table_log(max_log: u8, total: usize, max_symbol: usize) -> u8 {
    const MIN_TABLE_LOG: u8 = 5;
    const MAX_TABLE_LOG: u8 = 22;

    let max_bits_src = (total - 1).ilog2().saturating_sub(2);
    let min_bits_src = total.ilog2() + 1;
    let min_bits_symbols = if max_symbol == 0 {
        0
    } else {
        max_symbol.ilog2() + 2
    };
    let min_bits = min_bits_src.min(min_bits_symbols);
    let table_log = u32::from(max_log).min(max_bits_src).max(min_bits);
    table_log.clamp(u32::from(MIN_TABLE_LOG), u32::from(MAX_TABLE_LOG)) as u8
}

fn normalize_counts(counts: &[usize], table_log: u8, low_prob_count: i32) -> Option<Vec<i32>> {
    let total = counts.iter().sum::<usize>();
    let table_size = 1i32 << table_log;
    let low_threshold = total >> table_log;
    let scale = 62 - table_log;
    let step = (1u64 << 62) / total as u64;
    let v_step = 1u64 << (scale - 20);
    let rtb_table = [
        0u64, 473_195, 504_333, 520_860, 550_000, 700_000, 750_000, 830_000,
    ];

    let mut normalized = alloc::vec![0i32; counts.len()];
    let mut still_to_distribute = table_size;
    let mut largest = 0usize;
    let mut largest_probability = 0i32;

    for (symbol, count) in counts.iter().copied().enumerate() {
        if count == 0 {
            continue;
        }
        if count == total {
            normalized[symbol] = table_size;
            return Some(normalized);
        }
        if count <= low_threshold {
            normalized[symbol] = low_prob_count;
            still_to_distribute -= 1;
            continue;
        }

        let scaled = count as u64 * step;
        let mut probability = (scaled >> scale) as i32;
        if probability < 8 {
            let rest_to_beat = v_step * rtb_table[probability as usize];
            if scaled - ((probability as u64) << scale) > rest_to_beat {
                probability += 1;
            }
        }
        if probability > largest_probability {
            largest_probability = probability;
            largest = symbol;
        }
        normalized[symbol] = probability;
        still_to_distribute -= probability;
    }

    if -still_to_distribute >= normalized[largest] >> 1 {
        normalize_counts_slow(counts, table_log, low_prob_count)
    } else {
        normalized[largest] += still_to_distribute;
        Some(normalized)
    }
}

fn normalize_counts_slow(counts: &[usize], table_log: u8, low_prob_count: i32) -> Option<Vec<i32>> {
    const NOT_YET_ASSIGNED: i32 = -2;

    let mut normalized = alloc::vec![0i32; counts.len()];
    let mut distributed = 0usize;
    let mut remaining_total = counts.iter().sum::<usize>();
    let mut low_one = (remaining_total * 3) >> (table_log + 1);
    let low_threshold = remaining_total >> table_log;

    for (symbol, count) in counts.iter().copied().enumerate() {
        if count == 0 {
            continue;
        }
        if count <= low_threshold {
            normalized[symbol] = low_prob_count;
            distributed += 1;
            remaining_total -= count;
            continue;
        }
        if count <= low_one {
            normalized[symbol] = 1;
            distributed += 1;
            remaining_total -= count;
            continue;
        }
        normalized[symbol] = NOT_YET_ASSIGNED;
    }

    let mut to_distribute = (1usize << table_log) - distributed;
    if to_distribute == 0 {
        return Some(normalized);
    }

    if remaining_total / to_distribute > low_one {
        low_one = (remaining_total * 3) / (to_distribute * 2);
        for (symbol, count) in counts.iter().copied().enumerate() {
            if normalized[symbol] == NOT_YET_ASSIGNED && count <= low_one {
                normalized[symbol] = 1;
                distributed += 1;
                remaining_total -= count;
            }
        }
        to_distribute = (1usize << table_log) - distributed;
    }

    if distributed == counts.len() {
        let max_symbol = counts
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .map(|(symbol, _)| symbol)?;
        normalized[max_symbol] += to_distribute as i32;
        return Some(normalized);
    }

    if remaining_total == 0 {
        let mut symbol = 0usize;
        while to_distribute > 0 {
            if normalized[symbol] > 0 {
                normalized[symbol] += 1;
                to_distribute -= 1;
            }
            symbol = (symbol + 1) % counts.len();
        }
        return Some(normalized);
    }

    let v_step_log = 62 - table_log;
    let mid = (1u64 << (v_step_log - 1)) - 1;
    let r_step = (((1u64 << v_step_log) * to_distribute as u64) + mid) / remaining_total as u64;
    let mut tmp_total = mid;
    for (symbol, count) in counts.iter().copied().enumerate() {
        if normalized[symbol] == NOT_YET_ASSIGNED {
            let end = tmp_total + count as u64 * r_step;
            let start = tmp_total >> v_step_log;
            let finish = end >> v_step_log;
            let weight = finish - start;
            if weight < 1 {
                return None;
            }
            normalized[symbol] = weight as i32;
            tmp_total = end;
        }
    }

    Some(normalized)
}

fn old_normalize_counts(counts: &[usize], max_log: u8, avoid_0_numbit: bool) -> (Vec<i32>, u8) {
    let mut probs = alloc::vec![0i32; counts.len()];
    let mut min_count = 0;
    for (idx, count) in counts.iter().copied().enumerate() {
        probs[idx] = count as i32;
        if count > 0 && (count < min_count || min_count == 0) {
            min_count = count;
        }
    }

    min_count -= 1;
    let mut max_prob = 0i32;
    for prob in probs.iter_mut() {
        if *prob > 0 {
            *prob -= min_count as i32;
        }
        max_prob = max_prob.max(*prob);
    }

    if max_prob > 0 && max_prob as usize > probs.len() {
        let divisor = max_prob / (probs.len() as i32);
        for prob in probs.iter_mut() {
            if *prob > 0 {
                *prob = (*prob / divisor).max(1)
            }
        }
    }

    let sum = probs.iter().sum::<i32>();
    assert!(sum > 0);
    let sum = sum as usize;
    let acc_log = (sum.ilog2() as u8 + 1).max(5);
    let acc_log = u8::min(acc_log, max_log);

    if sum < 1 << acc_log {
        let diff = (1 << acc_log) - sum;
        let max = probs.iter_mut().max().unwrap();
        *max += diff as i32;
    } else {
        let mut diff = sum - (1 << acc_log);
        while diff > 0 {
            let min = probs.iter_mut().filter(|prob| **prob > 1).min().unwrap();
            let decrease = usize::min(*min as usize - 1, diff);
            diff -= decrease;
            *min -= decrease as i32;
        }
    }
    let max = probs.iter_mut().max().unwrap();
    if avoid_0_numbit && *max > 1 << (acc_log - 1) {
        let redistribute = *max - (1 << (acc_log - 1));
        *max -= redistribute;
        let max = *max;

        let second_max = *probs.iter_mut().filter(|x| **x != max).max().unwrap();
        let second_max = probs.iter_mut().find(|x| **x == second_max).unwrap();
        *second_max += redistribute;
        assert!(*second_max <= max);
    }

    (probs, acc_log)
}

pub(crate) fn build_table_from_probabilities(probs: &[i32], acc_log: u8) -> FSETable {
    let mut states = core::array::from_fn::<SymbolStates, 256, _>(|_| SymbolStates {
        states: Vec::new(),
        lookup: Vec::new(),
        probability: 0,
    });

    // distribute -1 symbols
    let mut negative_idx = (1 << acc_log) - 1;
    for (symbol, _prob) in probs
        .iter()
        .copied()
        .enumerate()
        .filter(|prob| prob.1 == -1)
    {
        states[symbol].states.push(State {
            num_bits: acc_log,
            baseline: 0,
            last_index: (1 << acc_log) - 1,
            index: negative_idx,
        });
        states[symbol].probability = -1;
        negative_idx -= 1;
    }

    // distribute other symbols

    // Setup all needed states per symbol with their respective index
    let mut idx = 0;
    for (symbol, prob) in probs.iter().copied().enumerate() {
        if prob <= 0 {
            continue;
        }
        states[symbol].probability = prob;
        let states = &mut states[symbol].states;
        for _ in 0..prob {
            states.push(State {
                num_bits: 0,
                baseline: 0,
                last_index: 0,
                index: idx,
            });

            idx = next_position(idx, 1 << acc_log);
            while idx > negative_idx {
                idx = next_position(idx, 1 << acc_log);
            }
        }
        assert_eq!(states.len(), prob as usize);
    }

    // After all states know their index we can determine the numbits and baselines
    for (symbol, prob) in probs.iter().copied().enumerate() {
        if prob <= 0 {
            continue;
        }
        let prob = prob as u32;
        let state = &mut states[symbol];

        // We process the states in their order in the table
        state.states.sort_unstable_by_key(|l| l.index);

        let prob_log = if prob.is_power_of_two() {
            prob.ilog2()
        } else {
            prob.ilog2() + 1
        };
        let rounded_up = 1u32 << prob_log;

        // The lower states target double the amount of indexes -> numbits + 1
        let double_states = rounded_up - prob;
        let single_states = prob - double_states;
        let num_bits = acc_log - prob_log as u8;
        let mut baseline = (single_states as usize * (1 << (num_bits))) % (1 << acc_log);
        for (idx, state) in state.states.iter_mut().enumerate() {
            if (idx as u32) < double_states {
                let num_bits = num_bits + 1;
                state.baseline = baseline;
                state.num_bits = num_bits;
                state.last_index = baseline + ((1 << num_bits) - 1);

                baseline += 1 << num_bits;
                baseline %= 1 << acc_log;
            } else {
                state.baseline = baseline;
                state.num_bits = num_bits;
                state.last_index = baseline + ((1 << num_bits) - 1);
                baseline += 1 << num_bits;
            }
        }

        // For encoding we use the states ordered by the indexes they target
        state.states.sort_unstable_by_key(|l| l.baseline);
        if (1usize << acc_log) <= DIRECT_STATE_LOOKUP_MAX_TABLE_SIZE {
            state.lookup.resize(1usize << acc_log, u16::MAX);
            for (state_idx, fse_state) in state.states.iter().enumerate() {
                let state_idx = match u16::try_from(state_idx) {
                    Ok(state_idx) => state_idx,
                    Err(_) => unreachable!("small FSE direct lookup state indexes fit in u16"),
                };
                for idx in fse_state.baseline..=fse_state.last_index {
                    state.lookup[idx] = state_idx;
                }
            }
        }
    }

    FSETable {
        table_size: 1 << acc_log,
        acc_log,
        states,
    }
}

/// Calculate the position of the next entry of the table given the current
/// position and size of the table.
fn next_position(mut p: usize, table_size: usize) -> usize {
    p += (table_size >> 1) + (table_size >> 3) + 3;
    p &= table_size - 1;
    p
}

const ML_DIST: &[i32] = &[
    1, 4, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1,
];

const LL_DIST: &[i32] = &[
    4, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 1, 1, 1, 1, 1,
    -1, -1, -1, -1,
];

const OF_DIST: &[i32] = &[
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
mod tests {
    use super::{
        build_table_from_data, default_ll_table, default_ml_table, default_of_table,
        normalize_counts, optimal_table_log,
    };

    #[test]
    fn default_tables_cache_their_accuracy_log() {
        for (table, expected_acc_log) in [
            (default_ll_table(), 6),
            (default_ml_table(), 6),
            (default_of_table(), 5),
        ] {
            assert_eq!(table.acc_log(), expected_acc_log);
            assert_eq!(table.table_size, 1 << table.acc_log());
        }
    }

    #[test]
    fn optimal_table_log_matches_c_fast_sequence_shape() {
        assert_eq!(optimal_table_log(9, 3000, 35), 9);
        assert_eq!(optimal_table_log(8, 3000, 24), 8);
        assert_eq!(optimal_table_log(9, 12, 35), 5);
    }

    #[test]
    fn c_style_normalization_sums_to_table_size() {
        let counts = [0, 57, 104, 88, 42, 17, 9, 3, 1, 0, 23, 61];
        let table_log = optimal_table_log(9, counts.iter().sum(), counts.len() - 1);
        let normalized = normalize_counts(&counts, table_log, 1)
            .expect("normalization should represent the distribution");
        let total = normalized
            .iter()
            .map(|probability| probability.unsigned_abs() as usize)
            .sum::<usize>();

        assert_eq!(total, 1usize << table_log);
        for (count, probability) in counts.iter().zip(normalized) {
            assert_eq!(*count == 0, probability == 0);
        }
    }

    #[test]
    fn sequence_table_builder_keeps_large_balanced_tables_precise() {
        let mut data = alloc::vec::Vec::new();
        for _ in 0..100 {
            data.extend(0u8..30);
        }

        let table = build_table_from_data(data.iter().copied(), 9, true);

        assert_eq!(table.acc_log(), 9);
    }
}
