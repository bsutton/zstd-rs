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

        let mut state_index = self.table.c_start_state_index(data[data.len() - 1]);
        for x in data[0..data.len() - 1].iter().rev().copied() {
            let next = self.table.next_state(x, state_index);
            let diff = state_index - next.baseline;
            self.writer.write_bits(diff as u64, next.num_bits as usize);
            state_index = next.index;
        }
        self.writer
            .write_bits(state_index as u64, self.acc_log() as usize);

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

        let mut ip = data.len();
        let mut state_1;
        let mut state_2;

        if data.len() & 1 != 0 {
            ip -= 1;
            state_1 = self.table.c_start_state_index(data[ip]);
            ip -= 1;
            state_2 = self.table.c_start_state_index(data[ip]);
            ip -= 1;
            Self::encode_symbol(&self.table, self.writer, &mut state_1, data[ip]);
        } else {
            ip -= 1;
            state_2 = self.table.c_start_state_index(data[ip]);
            ip -= 1;
            state_1 = self.table.c_start_state_index(data[ip]);
        }

        if (data.len() - 2) & 2 != 0 {
            ip -= 1;
            Self::encode_symbol(&self.table, self.writer, &mut state_2, data[ip]);
            ip -= 1;
            Self::encode_symbol(&self.table, self.writer, &mut state_1, data[ip]);
        }

        while ip > 0 {
            ip -= 1;
            Self::encode_symbol(&self.table, self.writer, &mut state_2, data[ip]);
            ip -= 1;
            Self::encode_symbol(&self.table, self.writer, &mut state_1, data[ip]);

            if ip > 0 {
                ip -= 1;
                Self::encode_symbol(&self.table, self.writer, &mut state_2, data[ip]);
                ip -= 1;
                Self::encode_symbol(&self.table, self.writer, &mut state_1, data[ip]);
            }
        }

        self.writer
            .write_bits(state_2 as u64, self.acc_log() as usize);
        self.writer
            .write_bits(state_1 as u64, self.acc_log() as usize);

        let bits_to_fill = self.writer.misaligned();
        if bits_to_fill == 0 {
            self.writer.write_bits(1u32, 8);
        } else {
            self.writer.write_bits(1u32, bits_to_fill);
        }
    }

    fn encode_symbol<VV: AsMut<Vec<u8>>>(
        table: &FSETable,
        writer: &mut BitWriter<VV>,
        state_index: &mut u32,
        symbol: u8,
    ) {
        let next = table.next_state(symbol, *state_index);
        let diff = *state_index - next.baseline;
        writer.write_bits(u64::from(diff), next.num_bits as usize);
        *state_index = next.index;
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
    #[inline(always)]
    pub(crate) fn next_state(&self, symbol: u8, idx: u32) -> &State {
        let states = &self.states[symbol as usize];
        states.get(idx, self.table_size)
    }

    pub(crate) fn c_start_state_index(&self, symbol: u8) -> u32 {
        let probability = self.normalized_probability(symbol);
        debug_assert_ne!(probability, 0);
        let symbol_states = &self.states[usize::from(symbol)].states;
        if symbol_states.len() == 1 {
            return symbol_states[0].index;
        }

        let mut total = 0usize;
        for current_symbol in 0..usize::from(symbol) {
            total += match self.states[current_symbol].probability {
                -1 => 1,
                probability if probability > 0 => probability as usize,
                _ => 0,
            };
        }

        let delta_nb_bits = self.delta_nb_bits(symbol);
        let nb_bits_out = (delta_nb_bits + (1 << 15)) >> 16;
        let value = (nb_bits_out << 16) - delta_nb_bits;
        let delta_find_state = match probability {
            -1 | 1 => total as isize - 1,
            probability => total as isize - probability as isize,
        };
        let state_table_index = (value >> nb_bits_out) as isize + delta_find_state;
        debug_assert!(state_table_index >= 0);
        let state_table_index = state_table_index as usize;
        let rank = state_table_index - total;
        self.nth_symbol_state_by_decode_index(symbol, rank)
    }

    fn nth_symbol_state_by_decode_index(&self, symbol: u8, rank: usize) -> u32 {
        let states = &self.states[usize::from(symbol)].states;
        let mut lower_bound = None;
        let mut selected = 0;
        for _ in 0..=rank {
            selected = states
                .iter()
                .filter(|state| lower_bound.is_none_or(|lower| state.index > lower))
                .map(|state| state.index)
                .min()
                .expect("symbol state rank must exist");
            lower_bound = Some(selected);
        }
        selected
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
        write_normalized_probabilities(
            (0..self.states.len()).map(|idx| self.states[idx].probability),
            self.acc_log(),
            writer,
        );
    }
}

fn write_normalized_probabilities<V: AsMut<Vec<u8>>>(
    mut probabilities: impl Iterator<Item = i32>,
    acc_log: u8,
    writer: &mut BitWriter<V>,
) {
    writer.write_bits(acc_log - 5, 4);
    let mut probability_counter = 0usize;
    let probability_sum = 1 << acc_log;
    let mut pending_zero_probability = None;

    while probability_counter < probability_sum {
        let prob = pending_zero_probability.take().unwrap_or_else(|| {
            probabilities
                .next()
                .expect("normalized probabilities must sum to the table size")
        });
        let max_remaining_value = probability_sum - probability_counter + 1;
        let bits_to_write = max_remaining_value.ilog2() + 1;
        let low_threshold = ((1 << bits_to_write) - 1) - max_remaining_value;
        let mask = (1 << (bits_to_write - 1)) - 1;

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
            loop {
                let next = probabilities
                    .next()
                    .expect("normalized probabilities must contain the next non-zero symbol");
                if next != 0 {
                    pending_zero_probability = Some(next);
                    break;
                }
                zeros += 1;
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

pub(crate) fn ncount_cost_from_probabilities(probs: &[i32], acc_log: u8) -> usize {
    let mut probabilities = probs.iter().copied();
    let mut bit_count = 4usize;
    let mut probability_counter = 0usize;
    let probability_sum = 1 << acc_log;
    let mut pending_zero_probability = None;

    while probability_counter < probability_sum {
        let prob = pending_zero_probability.take().unwrap_or_else(|| {
            probabilities
                .next()
                .expect("normalized probabilities must sum to the table size")
        });
        let max_remaining_value = probability_sum - probability_counter + 1;
        let bits_to_write = max_remaining_value.ilog2() as usize + 1;
        let low_threshold = ((1 << bits_to_write) - 1) - max_remaining_value;

        let value = (prob + 1) as u32;
        bit_count += if value < low_threshold as u32 {
            bits_to_write - 1
        } else {
            bits_to_write
        };

        if prob == -1 {
            probability_counter += 1;
        } else if prob > 0 {
            probability_counter += prob as usize;
        } else {
            let mut zeros = 0u8;
            loop {
                let next = probabilities
                    .next()
                    .expect("normalized probabilities must contain the next non-zero symbol");
                if next != 0 {
                    pending_zero_probability = Some(next);
                    break;
                }
                zeros += 1;
                if zeros == 3 {
                    bit_count += 2;
                    zeros = 0;
                }
            }
            bit_count += 2;
        }
    }

    bit_count.next_multiple_of(8)
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
    #[inline(always)]
    fn get(&self, idx: u32, max_idx: usize) -> &State {
        let idx_usize = idx as usize;
        if !self.lookup.is_empty() {
            let state_idx = self.lookup[idx_usize] as usize;
            debug_assert_ne!(state_idx, u16::MAX as usize);
            return &self.states[state_idx];
        }
        if self.states.len() == 1 {
            return &self.states[0];
        }

        let start_search_at = (idx_usize * self.states.len()) / max_idx;
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
    pub(crate) baseline: u32,
    /// The last index targeted by this state (baseline + the maximum number with numbits bits allows)
    pub(crate) last_index: u32,
    /// Index of this state in the decoding table
    pub(crate) index: u32,
}

impl State {
    #[inline(always)]
    fn contains(&self, idx: u32) -> bool {
        self.baseline <= idx && self.last_index >= idx
    }
}

#[cfg(debug_assertions)]
fn uninitialized_lookup(table_size: usize) -> Vec<u16> {
    alloc::vec![u16::MAX; table_size]
}

#[cfg(not(debug_assertions))]
fn uninitialized_lookup(table_size: usize) -> Vec<u16> {
    let mut lookup = Vec::with_capacity(table_size);
    // SAFETY: callers immediately write every entry before the table is used.
    unsafe {
        lookup.set_len(table_size);
    }
    lookup
}

#[cfg(debug_assertions)]
fn write_lookup(lookup: &mut [u16], idx: usize, state_idx: u16) {
    lookup[idx] = state_idx;
}

#[cfg(not(debug_assertions))]
fn write_lookup(lookup: &mut [u16], idx: usize, state_idx: u16) {
    // SAFETY: idx is produced from FSE state ranges bounded by table_size.
    unsafe {
        lookup.as_mut_ptr().add(idx).write(state_idx);
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
    let (probs, acc_log) = normalized_probabilities_from_counts(counts, max_log, avoid_0_numbit);
    build_table_from_probabilities(&probs, acc_log)
}

pub(crate) fn normalized_probabilities_from_counts(
    counts: &[usize],
    max_log: u8,
    avoid_0_numbit: bool,
) -> (Vec<i32>, u8) {
    if max_log <= 6 {
        return old_normalize_counts(counts, max_log, avoid_0_numbit);
    }

    let total = counts.iter().sum::<usize>();
    assert!(total > 0);
    let max_symbol = counts.iter().rposition(|count| *count > 0).unwrap_or(0);
    let acc_log = optimal_table_log(max_log, total, max_symbol);
    let low_prob_count = if total >= 2048 { -1 } else { 1 };
    if let Some(probs) = normalize_counts(counts, acc_log, low_prob_count) {
        (probs, acc_log)
    } else {
        old_normalize_counts(counts, max_log, avoid_0_numbit)
    }
}

pub(crate) fn normalize_counts_with_table_log(
    counts: &[usize],
    table_log: u8,
    max_log: u8,
    low_prob_count: i32,
    avoid_0_numbit: bool,
) -> (Vec<i32>, u8) {
    normalize_counts(counts, table_log, low_prob_count)
        .map(|probs| (probs, table_log))
        .unwrap_or_else(|| old_normalize_counts(counts, max_log, avoid_0_numbit))
}

/// Build the FSE table used for Huffman weight descriptions.
///
/// zstd's `HUF_compressWeights()` normalizes low-probability symbols as `1`
/// instead of `-1`, which can produce a larger NCount header but a shorter
/// compressed weight stream.
pub(crate) fn build_huffman_weight_table_from_data(data: &[u8], max_log: u8) -> FSETable {
    let mut counts = [0; 256];
    let mut max_symbol = 0;
    for symbol in data {
        counts[usize::from(*symbol)] += 1;
    }
    for (idx, count) in counts.iter().copied().enumerate() {
        if count > 0 {
            max_symbol = idx;
        }
    }

    let counts = &counts[..=max_symbol];
    let total = counts.iter().sum::<usize>();
    let acc_log = optimal_table_log(max_log, total, max_symbol);
    let (probs, acc_log) = normalize_counts(counts, acc_log, 1)
        .map(|probs| (probs, acc_log))
        .unwrap_or_else(|| old_normalize_counts(counts, max_log, false));
    build_table_from_probabilities(&probs, acc_log)
}

pub(crate) fn optimal_table_log(max_log: u8, total: usize, max_symbol: usize) -> u8 {
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
    let table_size = 1usize << acc_log;
    let mut spread_symbols = alloc::vec![0u8; table_size];

    // distribute -1 symbols
    let mut negative_idx = table_size - 1;
    for (symbol, _prob) in probs
        .iter()
        .copied()
        .enumerate()
        .filter(|prob| prob.1 == -1)
    {
        let symbol_states = &mut states[symbol];
        symbol_states.states = Vec::with_capacity(1);
        symbol_states.states.push(State {
            num_bits: acc_log,
            baseline: 0,
            last_index: ((1 << acc_log) - 1) as u32,
            index: negative_idx as u32,
        });
        symbol_states.probability = -1;
        negative_idx -= 1;
    }

    // distribute other symbols

    // First spread symbols by table index. We then walk the spread table in
    // ascending index order so each per-symbol state list is already sorted by
    // index, avoiding a hot per-symbol sort.
    let mut idx = 0;
    for (symbol, prob) in probs.iter().copied().enumerate() {
        if prob <= 0 {
            continue;
        }
        let symbol_states = &mut states[symbol];
        symbol_states.probability = prob;
        symbol_states.states = Vec::with_capacity(prob as usize);
        for _ in 0..prob {
            spread_symbols[idx] = symbol as u8;
            idx = next_position(idx, table_size);
            while idx > negative_idx {
                idx = next_position(idx, table_size);
            }
        }
    }

    for (idx, symbol) in spread_symbols[..=negative_idx].iter().copied().enumerate() {
        states[usize::from(symbol)].states.push(State {
            num_bits: 0,
            baseline: 0,
            last_index: 0,
            index: idx as u32,
        });
    }

    // After all states know their index we can determine the numbits and baselines
    for (symbol, prob) in probs.iter().copied().enumerate() {
        if prob <= 0 {
            continue;
        }
        let prob = prob as u32;
        let state = &mut states[symbol];

        debug_assert_eq!(state.states.len(), prob as usize);

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
        let mut start_state_idx = 0usize;
        for (idx, state) in state.states.iter_mut().enumerate() {
            if (idx as u32) < double_states {
                let num_bits = num_bits + 1;
                state.baseline = baseline as u32;
                state.num_bits = num_bits;
                state.last_index = (baseline + ((1 << num_bits) - 1)) as u32;
                if state.baseline == 0 {
                    start_state_idx = idx;
                }

                baseline += 1 << num_bits;
                baseline %= 1 << acc_log;
            } else {
                state.baseline = baseline as u32;
                state.num_bits = num_bits;
                state.last_index = (baseline + ((1 << num_bits) - 1)) as u32;
                if state.baseline == 0 {
                    start_state_idx = idx;
                }
                baseline += 1 << num_bits;
            }
        }

        if state.states.len() == 1 {
            continue;
        }

        if (1usize << acc_log) <= DIRECT_STATE_LOOKUP_MAX_TABLE_SIZE {
            state.states.swap(0, start_state_idx);
            state.lookup = uninitialized_lookup(1usize << acc_log);
            for (state_idx, fse_state) in state.states.iter().enumerate() {
                let state_idx = match u16::try_from(state_idx) {
                    Ok(state_idx) => state_idx,
                    Err(_) => unreachable!("small FSE direct lookup state indexes fit in u16"),
                };
                for idx in fse_state.baseline..=fse_state.last_index {
                    write_lookup(&mut state.lookup, idx as usize, state_idx);
                }
            }
            debug_assert!(state.lookup.iter().all(|idx| *idx != u16::MAX));
        } else {
            // The fallback lookup searches by baseline, so keep larger tables
            // ordered by the indexes they target.
            state.states.sort_unstable_by_key(|state| state.baseline);
        }
    }

    FSETable {
        table_size,
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
        build_huffman_weight_table_from_data, build_table_from_data,
        build_table_from_probabilities, default_ll_table, default_ml_table, default_of_table,
        ncount_cost_from_probabilities, normalize_counts, optimal_table_log,
    };
    use crate::bit_io::BitWriter;
    use alloc::vec::Vec;

    #[test]
    fn ncount_cost_matches_written_table_size() {
        fn assert_ncount_cost_matches_writer(probs: &[i32], acc_log: u8) {
            let table = build_table_from_probabilities(probs, acc_log);
            let mut bytes = Vec::new();
            let mut writer = BitWriter::from(&mut bytes);
            table.write_table(&mut writer);
            writer.flush();

            assert_eq!(
                ncount_cost_from_probabilities(probs, acc_log),
                bytes.len() * 8
            );
        }

        assert_ncount_cost_matches_writer(super::LL_DIST, 6);
        assert_ncount_cost_matches_writer(super::ML_DIST, 6);
        assert_ncount_cost_matches_writer(super::OF_DIST, 5);

        let cases = [
            &[128, 64, 32, 16, 8, 4, 2, 1][..],
            &[9, 0, 7, 0, 5, 3, 0, 2, 1][..],
            &[4000, 31, 29, 23, 17, 11, 7, 5, 3, 1][..],
        ];
        for counts in cases {
            let (probs, acc_log) = super::normalized_probabilities_from_counts(counts, 9, true);
            assert_ncount_cost_matches_writer(&probs, acc_log);
        }
    }

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
    fn huffman_weight_table_uses_c_low_probability_policy() {
        let mut weights = Vec::new();
        weights.extend(alloc::vec![1; 70]);
        weights.extend(alloc::vec![2; 96]);
        weights.extend(alloc::vec![3; 42]);
        weights.extend(alloc::vec![4; 26]);
        weights.extend(alloc::vec![5; 18]);
        weights.extend(alloc::vec![6; 3]);

        let table = build_huffman_weight_table_from_data(&weights, 6);
        let probabilities = (0..=6)
            .map(|symbol| table.normalized_probability(symbol))
            .collect::<Vec<_>>();

        assert_eq!(table.acc_log(), 5);
        assert_eq!(probabilities, [0, 8, 13, 5, 3, 2, 1]);
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
