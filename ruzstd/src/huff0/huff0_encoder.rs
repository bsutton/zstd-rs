use alloc::collections::BinaryHeap;
use alloc::vec::Vec;
use core::cmp::{Ordering, Reverse};

use crate::{
    bit_io::BitWriter,
    fse::fse_encoder::{self, FSEEncoder},
};

const MAX_HUFFMAN_BITS: usize = 11;
type ActiveHuffmanNode = Reverse<(usize, Option<usize>, Reverse<usize>, usize)>;

pub(crate) struct HuffmanEncoder<'output, 'table, V: AsMut<Vec<u8>>> {
    table: &'table HuffmanTable,
    writer: &'output mut BitWriter<V>,
}

impl<V: AsMut<Vec<u8>>> HuffmanEncoder<'_, '_, V> {
    pub fn new<'o, 't>(
        table: &'t HuffmanTable,
        writer: &'o mut BitWriter<V>,
    ) -> HuffmanEncoder<'o, 't, V> {
        HuffmanEncoder { table, writer }
    }

    /// Encodes the data using the provided table
    /// Writes
    /// * Table description
    /// * Encoded data
    /// * Padding bits to fill up last byte
    pub fn encode(&mut self, data: &[u8], with_table: bool) {
        if with_table {
            self.write_table();
        }
        Self::encode_stream(self.table, self.writer, data);
    }

    /// Encodes the data using the provided table in 4 concatenated streams
    /// Writes
    /// * Table description
    /// * Jumptable
    /// * Encoded data in 4 streams, each padded to fill the last byte
    pub fn encode4x(&mut self, data: &[u8], with_table: bool) {
        assert!(data.len() >= 4);

        // Split data in 4 equally sized parts (the last one might be a bit smaller than the rest)
        let split_size = data.len().div_ceil(4);
        let src1 = &data[..split_size];
        let src2 = &data[split_size..split_size * 2];
        let src3 = &data[split_size * 2..split_size * 3];
        let src4 = &data[split_size * 3..];

        // Write table description
        if with_table {
            self.write_table();
        }

        // Reserve space for the jump table, will be changed later
        let size_idx = self.writer.index();
        self.writer.write_bits(0u16, 16);
        self.writer.write_bits(0u16, 16);
        self.writer.write_bits(0u16, 16);

        // Write the 4 streams, noting the sizes of the encoded streams
        let index_before = self.writer.index();
        Self::encode_stream(self.table, self.writer, src1);
        let size1 = (self.writer.index() - index_before) / 8;

        let index_before = self.writer.index();
        Self::encode_stream(self.table, self.writer, src2);
        let size2 = (self.writer.index() - index_before) / 8;

        let index_before = self.writer.index();
        Self::encode_stream(self.table, self.writer, src3);
        let size3 = (self.writer.index() - index_before) / 8;

        Self::encode_stream(self.table, self.writer, src4);

        // Sanity check, if this doesn't hold we produce a broken stream
        assert!(size1 <= u16::MAX as usize);
        assert!(size2 <= u16::MAX as usize);
        assert!(size3 <= u16::MAX as usize);

        // Update the jumptable with the real sizes
        self.writer.change_bits(size_idx, size1 as u16, 16);
        self.writer.change_bits(size_idx + 16, size2 as u16, 16);
        self.writer.change_bits(size_idx + 32, size3 as u16, 16);
    }

    /// Encode one stream and pad it to fill the last byte
    fn encode_stream<VV: AsMut<Vec<u8>>>(
        table: &HuffmanTable,
        writer: &mut BitWriter<VV>,
        data: &[u8],
    ) {
        for symbol in data.iter().rev() {
            let (code, num_bits) = table.codes[*symbol as usize];
            debug_assert!(num_bits > 0);
            writer.write_bits(code, num_bits as usize);
        }

        let bits_to_fill = writer.misaligned();
        if bits_to_fill == 0 {
            writer.write_bits(1u32, 8);
        } else {
            writer.write_bits(1u32, bits_to_fill);
        }
    }

    pub(super) fn weights(&self) -> Vec<u8> {
        let max = self.table.max_num_bits;
        let weights = self
            .table
            .codes
            .iter()
            .copied()
            .map(|(_, nb)| if nb == 0 { 0 } else { max - nb + 1 })
            .collect::<Vec<u8>>();

        weights
    }

    fn write_table(&mut self) {
        // TODO strategy for determining this?
        let weights = self.weights();
        let weights = &weights[..weights.len() - 1]; // dont encode last weight
        if weights.len() > 16 {
            let size_idx = self.writer.index();
            self.writer.write_bits(0u8, 8);
            let idx_before = self.writer.index();
            let mut encoder = FSEEncoder::new(
                fse_encoder::build_table_from_data(weights.iter().copied(), 6, true),
                self.writer,
            );
            encoder.encode_interleaved(weights);
            let encoded_len = (self.writer.index() - idx_before) / 8;
            assert!(encoded_len < 128);
            self.writer.change_bits(size_idx, encoded_len as u8, 8);
        } else {
            self.writer.write_bits(weights.len() as u8 + 127, 8);
            let pairs = weights.chunks_exact(2);
            let remainder = pairs.remainder();
            for pair in pairs.into_iter() {
                let weight1 = pair[0];
                let weight2 = pair[1];
                assert!(weight1 < 16);
                assert!(weight2 < 16);
                self.writer.write_bits(weight2, 4);
                self.writer.write_bits(weight1, 4);
            }
            if !remainder.is_empty() {
                let weight = remainder[0];
                assert!(weight < 16);
                self.writer.write_bits(weight << 4, 8);
            }
        }
    }
}

pub struct HuffmanTable {
    /// Index is the symbol, values are the bitstring in the lower bits of the u32 and the amount of bits in the u8
    codes: Vec<(u32, u8)>,
    max_num_bits: u8,
}

impl HuffmanTable {
    #[cfg(any(test, feature = "fuzz_exports"))]
    pub fn build_from_data(data: &[u8]) -> Self {
        let mut counts = [0; 256];
        let mut max = 0;
        for x in data {
            counts[*x as usize] += 1;
            max = max.max(*x);
        }

        Self::build_from_counts(&counts[..=max as usize])
    }

    pub fn build_from_counts(counts: &[usize]) -> Self {
        assert!(counts.len() <= 256);
        let weights = if is_flat_distribution(counts) {
            rank_limited_weights(counts)
        } else if let Some(lengths) = length_limited_code_lengths(counts, MAX_HUFFMAN_BITS) {
            code_lengths_to_weights(&lengths, MAX_HUFFMAN_BITS)
        } else {
            rank_limited_weights(counts)
        };
        Self::build_from_weights(&weights)
    }

    pub(crate) fn build_smallest_from_counts(
        counts: &[usize],
        data: &[u8],
        four_streams: bool,
    ) -> Self {
        let mut best = Self::build_from_counts(counts);
        let mut best_len = best.encoded_len(data, true, four_streams);

        if is_flat_distribution(counts) {
            return best;
        }

        let min_bits = counts
            .iter()
            .filter(|count| **count > 0)
            .count()
            .next_power_of_two()
            .ilog2() as usize;

        for max_bits in min_bits.max(1)..MAX_HUFFMAN_BITS {
            let Some(candidate) = Self::build_from_counts_with_max_bits(counts, max_bits) else {
                continue;
            };
            let candidate_len = candidate.encoded_len(data, true, four_streams);
            if candidate_len < best_len {
                best = candidate;
                best_len = candidate_len;
            }
        }

        best
    }

    fn build_from_counts_with_max_bits(counts: &[usize], max_bits: usize) -> Option<Self> {
        assert!(counts.len() <= 256);
        if max_bits == 0 || max_bits > MAX_HUFFMAN_BITS || is_flat_distribution(counts) {
            return None;
        }

        length_limited_code_lengths(counts, max_bits)
            .map(|lengths| Self::build_from_weights(&code_lengths_to_weights(&lengths, max_bits)))
    }

    pub fn build_from_weights(weights: &[usize]) -> Self {
        let mut sorted = Vec::with_capacity(weights.len());
        struct SortEntry {
            symbol: u8,
            weight: usize,
        }

        // TODO this doesn't need to be a temporary Vec, it could be done in a [_; 264]
        // only non-zero weights are interesting here
        for (symbol, weight) in weights.iter().copied().enumerate() {
            if weight > 0 {
                sorted.push(SortEntry {
                    symbol: symbol as u8,
                    weight,
                });
            }
        }
        // We process symbols ordered by weight and then ordered by symbol
        sorted.sort_unstable_by(|left, right| match left.weight.cmp(&right.weight) {
            Ordering::Equal => left.symbol.cmp(&right.symbol),
            other => other,
        });

        // Prepare huffman table with placeholders
        let mut table = HuffmanTable {
            codes: Vec::with_capacity(weights.len()),
            max_num_bits: 0,
        };
        for _ in 0..weights.len() {
            table.codes.push((0, 0));
        }

        // Determine the number of bits needed for codes with the lowest weight
        let weight_sum = sorted.iter().map(|e| 1 << (e.weight - 1)).sum::<usize>();
        if !weight_sum.is_power_of_two() {
            panic!("This is an internal error");
        }
        let max_num_bits = highest_bit_set(weight_sum) - 1; // this is a log_2 of a clean power of two

        // Starting at the symbols with the lowest weight we update the placeholders in the table
        let mut current_code = 0;
        let mut current_weight = 0;
        let mut current_num_bits = 0;
        for entry in sorted.iter() {
            // If the entry isn't the same weight as the last one we need to change a few things
            if current_weight != entry.weight {
                // The code shifts by the difference of the weights to allow for enough unique values
                current_code >>= entry.weight - current_weight;
                // Encoding a symbol of this weight will take less bits than the previous weight
                current_num_bits = max_num_bits - entry.weight + 1;
                // Run the next update when the weight changes again
                current_weight = entry.weight;
            }
            table.codes[entry.symbol as usize] = (current_code as u32, current_num_bits as u8);
            table.max_num_bits = table.max_num_bits.max(current_num_bits as u8);
            current_code += 1;
        }

        table
    }

    pub(crate) fn can_encode_counts(&self, counts: &[usize]) -> bool {
        if counts.len() > self.codes.len() {
            return false;
        }

        counts
            .iter()
            .copied()
            .zip(self.codes.iter().copied())
            .all(|(count, (_, num_bits))| count == 0 || num_bits != 0)
    }

    pub(crate) fn encoded_len(&self, data: &[u8], with_table: bool, four_streams: bool) -> usize {
        let table_len = if with_table {
            self.table_description_len()
        } else {
            0
        };
        let data_len = if four_streams {
            let split_size = data.len().div_ceil(4);
            6 + self.stream_encoded_len(&data[..split_size])
                + self.stream_encoded_len(&data[split_size..split_size * 2])
                + self.stream_encoded_len(&data[split_size * 2..split_size * 3])
                + self.stream_encoded_len(&data[split_size * 3..])
        } else {
            self.stream_encoded_len(data)
        };
        table_len + data_len
    }

    fn table_description_len(&self) -> usize {
        let mut encoded = Vec::new();
        let mut writer = BitWriter::from(&mut encoded);
        HuffmanEncoder::new(self, &mut writer).write_table();
        writer.flush();
        encoded.len()
    }

    fn stream_encoded_len(&self, data: &[u8]) -> usize {
        let mut bit_len = 0usize;
        for symbol in data {
            bit_len += self.codes[*symbol as usize].1 as usize;
        }
        bit_len / 8 + 1
    }
}

fn is_flat_distribution(counts: &[usize]) -> bool {
    let mut nonzero = 0usize;
    let mut min = usize::MAX;
    let mut max = 0usize;
    for count in counts.iter().copied().filter(|count| *count > 0) {
        nonzero += 1;
        min = min.min(count);
        max = max.max(count);
    }

    nonzero > 128 && max <= min.saturating_mul(2)
}

fn length_limited_code_lengths(counts: &[usize], max_bits: usize) -> Option<Vec<usize>> {
    let mut nodes = counts
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(symbol, count)| {
            (count > 0).then_some(HuffmanNode {
                count,
                symbol: Some(symbol),
                parent: None,
            })
        })
        .collect::<Vec<_>>();

    if nodes.len() <= 1 {
        return None;
    }

    let active_key =
        |idx, node: &HuffmanNode| Reverse((node.count, node.symbol, Reverse(idx), idx));

    let mut active = BinaryHeap::with_capacity(nodes.len());
    for (idx, node) in nodes.iter().enumerate() {
        active.push(active_key(idx, node));
    }

    while active.len() > 1 {
        let Reverse((_, _, _, left)) = pop_active_huffman_node(&mut active);
        let Reverse((_, _, _, right)) = pop_active_huffman_node(&mut active);
        let parent = nodes.len();
        nodes[left].parent = Some(parent);
        nodes[right].parent = Some(parent);
        nodes.push(HuffmanNode {
            count: nodes[left].count + nodes[right].count,
            symbol: None,
            parent: None,
        });
        active.push(active_key(parent, &nodes[parent]));
    }

    let mut lengths = alloc::vec![0; counts.len()];
    for idx in 0..counts.len() {
        if counts[idx] == 0 {
            continue;
        }
        let mut node_idx = match nodes.iter().position(|node| node.symbol == Some(idx)) {
            Some(node_idx) => node_idx,
            None => invalid_huffman_tree(),
        };
        let mut len = 0;
        while let Some(parent) = nodes[node_idx].parent {
            len += 1;
            node_idx = parent;
        }
        lengths[idx] = len;
    }

    limit_code_lengths(&mut lengths, counts, max_bits).then_some(lengths)
}

struct HuffmanNode {
    count: usize,
    symbol: Option<usize>,
    parent: Option<usize>,
}

fn limit_code_lengths(lengths: &mut [usize], counts: &[usize], max_bits: usize) -> bool {
    let mut bl_count = alloc::vec![0usize; max_bits + 1];
    let mut overflow = 0usize;
    for len in lengths.iter_mut().filter(|len| **len > 0) {
        if *len > max_bits {
            *len = max_bits;
            overflow += 1;
        }
        bl_count[*len] += 1;
    }

    if !overflow.is_multiple_of(2) {
        return false;
    }

    while overflow > 0 {
        let mut bits = max_bits - 1;
        while bl_count[bits] == 0 {
            if bits == 0 {
                return false;
            }
            bits -= 1;
        }
        bl_count[bits] -= 1;
        bl_count[bits + 1] += 2;
        bl_count[max_bits] -= 1;
        overflow -= 2;
    }

    let mut symbols = counts
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, count)| *count > 0)
        .collect::<Vec<_>>();
    symbols.sort_unstable_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0)));

    lengths.fill(0);
    let mut symbol_idx = 0;
    for bits in (1..=max_bits).rev() {
        for _ in 0..bl_count[bits] {
            lengths[symbols[symbol_idx].0] = bits;
            symbol_idx += 1;
        }
    }

    symbol_idx == symbols.len() && length_units(lengths, max_bits) == 1usize << max_bits
}

fn length_units(lengths: &[usize], max_bits: usize) -> usize {
    lengths
        .iter()
        .copied()
        .filter(|len| *len > 0)
        .map(|len| 1usize << (max_bits - len))
        .sum()
}

fn code_lengths_to_weights(lengths: &[usize], max_bits: usize) -> Vec<usize> {
    lengths
        .iter()
        .copied()
        .map(|len| if len == 0 { 0 } else { max_bits - len + 1 })
        .collect()
}

fn rank_limited_weights(counts: &[usize]) -> Vec<usize> {
    let zeros = counts.iter().filter(|x| **x == 0).count();
    let mut weights = distribute_weights(counts.len() - zeros);
    let limit = weights.len().ilog2() as usize + 2;
    redistribute_weights(&mut weights, limit);

    weights.reverse();
    let mut counts_sorted = counts.iter().enumerate().collect::<Vec<_>>();
    counts_sorted.sort_by_key(|(_, c1)| *c1);

    let mut weights_distributed = alloc::vec![0; counts.len()];
    for (idx, count) in counts_sorted {
        if *count == 0 {
            weights_distributed[idx] = 0;
        } else {
            weights_distributed[idx] = match weights.pop() {
                Some(weight) => weight,
                None => invalid_huffman_tree(),
            };
        }
    }

    weights_distributed
}

#[inline(always)]
fn pop_active_huffman_node(active: &mut BinaryHeap<ActiveHuffmanNode>) -> ActiveHuffmanNode {
    match active.pop() {
        Some(node) => node,
        None => invalid_huffman_tree(),
    }
}

#[cold]
#[inline(never)]
fn invalid_huffman_tree() -> ! {
    panic!("huffman tree construction invariant failed")
}

/// Assert that the provided value is greater than zero, and returns index of the first set bit
fn highest_bit_set(x: usize) -> usize {
    assert!(x > 0);
    usize::BITS as usize - x.leading_zeros() as usize
}

#[test]
fn huffman() {
    let table = HuffmanTable::build_from_weights(&[2, 2, 2, 1, 1]);
    assert_eq!(table.codes[0], (1, 2));
    assert_eq!(table.codes[1], (2, 2));
    assert_eq!(table.codes[2], (3, 2));
    assert_eq!(table.codes[3], (0, 3));
    assert_eq!(table.codes[4], (1, 3));

    let table = HuffmanTable::build_from_weights(&[4, 3, 2, 0, 1, 1]);
    assert_eq!(table.codes[0], (1, 1));
    assert_eq!(table.codes[1], (1, 2));
    assert_eq!(table.codes[2], (1, 3));
    assert_eq!(table.codes[3], (0, 0));
    assert_eq!(table.codes[4], (0, 4));
    assert_eq!(table.codes[5], (1, 4));
}

/// Distributes weights that add up to a clean power of two
fn distribute_weights(amount: usize) -> Vec<usize> {
    assert!(amount >= 2);
    assert!(amount <= 256);
    let mut weights = Vec::new();

    // This is the trivial power of two we always need
    weights.push(1);
    weights.push(1);

    // This is the weight we are adding right now
    let mut target_weight = 1;
    // Counts how many times we have added weights
    let mut weight_counter = 2;

    // We always add a power of 2 new weights so that the weights that we add equal
    // the weights are already in the vec if raised to the power of two.
    // This means we double the weights in the vec -> results in a new power of two
    //
    // Example: [1, 1]      -> [1,1,2]       (2^1 + 2^1 == 2^2)
    //
    // Example: [1, 1]      -> [1,1,1,1]     (2^1 + 2^1 == 2^1 + 2^1)
    //          [1,1,1,1]   -> [1,1,1,1,3]   (2^1 + 2^1 + 2^1 + 2^1 == 2^3)
    while weights.len() < amount {
        let mut add_new = 1 << (weight_counter - target_weight);
        let available_space = amount - weights.len();

        // If the amount of new weights needed to get to the next power of two would exceed amount
        // We instead add 1 of a bigger weight and start the cycle again
        if add_new > available_space {
            // TODO we could maybe instead do this until add_new <= available_space?
            //  target_weight += 1
            //  add_new /= 2
            target_weight = weight_counter;
            add_new = 1;
        }

        for _ in 0..add_new {
            weights.push(target_weight);
        }
        weight_counter += 1;
    }

    assert_eq!(amount, weights.len());

    weights
}

/// Sometimes distribute_weights generates weights that require too many bits to encode
/// This redistributes the weights to have less variance by raising the lower weights while still maintaining the
/// required attributes of the weight distribution
fn redistribute_weights(weights: &mut [usize], max_num_bits: usize) {
    let weight_sum_log = weights
        .iter()
        .copied()
        .map(|x| 1 << x)
        .sum::<usize>()
        .ilog2() as usize;

    // Nothing needs to be done, this is already fine
    if weight_sum_log < max_num_bits {
        return;
    }

    // We need to decrease the weight difference by the difference between weight_sum_log and max_num_bits
    let decrease_weights_by = weight_sum_log - max_num_bits + 1;

    // To do that we raise the lower weights up by that difference, recording how much weight we added in the process
    let mut added_weights = 0;
    for weight in weights.iter_mut() {
        if *weight < decrease_weights_by {
            for add in *weight..decrease_weights_by {
                added_weights += 1 << add;
            }
            *weight = decrease_weights_by;
        }
    }

    // Then we reduce weights until the added weights are equaled out
    while added_weights > 0 {
        // Find the highest weight that is still lower or equal to the added weight
        let mut current_idx = 0;
        let mut current_weight = 0;
        for (idx, weight) in weights.iter().copied().enumerate() {
            if 1 << (weight - 1) > added_weights {
                break;
            }
            if weight > current_weight {
                current_weight = weight;
                current_idx = idx;
            }
        }

        // Reduce that weight by 1
        added_weights -= 1 << (current_weight - 1);
        weights[current_idx] -= 1;
    }

    // At the end we normalize the weights so that they start at 1 again
    if weights[0] > 1 {
        let offset = weights[0] - 1;
        for weight in weights.iter_mut() {
            *weight -= offset;
        }
    }
}

#[test]
fn weights() {
    // assert_eq!(distribute_weights(5).as_slice(), &[1, 1, 2, 3, 4]);
    for amount in 2..=256 {
        let mut weights = distribute_weights(amount);
        assert_eq!(weights.len(), amount);
        let sum = weights
            .iter()
            .copied()
            .map(|weight| 1 << weight)
            .sum::<usize>();
        assert!(sum.is_power_of_two());

        for num_bit_limit in (amount.ilog2() as usize + 1)..=11 {
            redistribute_weights(&mut weights, num_bit_limit);
            let sum = weights
                .iter()
                .copied()
                .map(|weight| 1 << weight)
                .sum::<usize>();
            assert!(sum.is_power_of_two());
            assert!(
                sum.ilog2() <= 11,
                "Max bits too big: sum: {} {weights:?}",
                sum
            );

            let codes = HuffmanTable::build_from_weights(&weights).codes;
            for (code, num_bits) in codes.iter().copied() {
                for (code2, num_bits2) in codes.iter().copied() {
                    if num_bits == 0 || num_bits2 == 0 || (code, num_bits) == (code2, num_bits2) {
                        continue;
                    }
                    if num_bits <= num_bits2 {
                        let code2_shifted = code2 >> (num_bits2 - num_bits);
                        assert_ne!(
                            code, code2_shifted,
                            "{code:b},{num_bits:} is prefix of {code2:b},{num_bits2:}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn counts() {
    let counts = &[3, 0, 4, 1, 5];
    let table = HuffmanTable::build_from_counts(counts).codes;

    assert_eq!(table[1].1, 0);
    assert!(table[3].1 >= table[0].1);
    assert!(table[0].1 >= table[2].1);
    assert!(table[2].1 >= table[4].1);

    let counts = &[3, 0, 4, 0, 7, 2, 2, 2, 0, 2, 2, 1, 5];
    let table = HuffmanTable::build_from_counts(counts).codes;

    assert_eq!(table[1].1, 0);
    assert_eq!(table[3].1, 0);
    assert_eq!(table[8].1, 0);
    assert!(table[11].1 >= table[5].1);
    assert!(table[5].1 >= table[6].1);
    assert!(table[6].1 >= table[7].1);
    assert!(table[7].1 >= table[9].1);
    assert!(table[9].1 >= table[10].1);
    assert!(table[10].1 >= table[0].1);
    assert!(table[0].1 >= table[2].1);
    assert!(table[2].1 >= table[12].1);
    assert!(table[12].1 >= table[4].1);
}

#[test]
fn cached_max_num_bits_matches_codes() {
    let cases: &[&[usize]] = &[
        &[3, 0, 4, 1, 5],
        &[16, 16, 16, 16, 16, 16, 16, 16],
        &[1, 1, 2, 3, 5, 8, 13, 21],
        &[0, 7, 7, 7, 7, 7, 0],
    ];

    for counts in cases {
        let table = HuffmanTable::build_from_counts(counts);
        let max_num_bits = table.codes.iter().map(|(_, bits)| *bits).max().unwrap_or(0);
        assert_eq!(table.max_num_bits, max_num_bits);
    }
}

#[test]
fn build_from_counts_produces_bounded_prefix_free_codes() {
    let flat_counts = [1usize; 256];
    let sparse_skewed_counts = [
        3, 0, 4, 0, 7, 2, 2, 2, 0, 2, 2, 1, 5, 144, 89, 55, 34, 21, 13, 8,
    ];
    let tied_counts = [8, 8, 4, 4, 2, 2, 1, 1, 0, 16, 16, 32, 32];
    let cases: &[&[usize]] = &[&flat_counts, &sparse_skewed_counts, &tied_counts];

    for counts in cases {
        let table = HuffmanTable::build_from_counts(counts);

        assert_eq!(table.codes.len(), counts.len());
        assert!(table.max_num_bits <= MAX_HUFFMAN_BITS as u8);
        for (symbol, (code, num_bits)) in table.codes.iter().copied().enumerate() {
            assert_eq!(
                num_bits == 0,
                counts[symbol] == 0,
                "symbol {symbol} has count {} and code ({code:b}, {num_bits})",
                counts[symbol]
            );
            assert!(num_bits <= MAX_HUFFMAN_BITS as u8);
        }
        assert_prefix_free(&table.codes);
    }
}

#[test]
fn build_smallest_from_counts_reduces_small_repeated_text_literals() {
    let data = b"the quick brown fox jumps over the lazy dog\n\
zstd-rs fastest encoder repeated text fixture\n\
0123456789 abcdefghijklmnopqrstuvwxyz\n\
the quick brown fox";
    let mut counts = [0usize; 256];
    let mut max_symbol = 0usize;
    for symbol in data {
        let symbol = *symbol as usize;
        counts[symbol] += 1;
        max_symbol = max_symbol.max(symbol);
    }
    let counts = &counts[..=max_symbol];

    let baseline = HuffmanTable::build_from_counts(counts);
    let smallest = HuffmanTable::build_smallest_from_counts(counts, data, false);

    assert!(smallest.encoded_len(data, true, false) < baseline.encoded_len(data, true, false));
    assert!(smallest.max_num_bits <= MAX_HUFFMAN_BITS as u8);
    assert_prefix_free(&smallest.codes);
}

#[test]
fn can_encode_counts_checks_symbols_without_building_table() {
    let table = HuffmanTable::build_from_counts(&[4, 3, 0, 1]);

    assert!(table.can_encode_counts(&[1, 2, 0, 1]));
    assert!(!table.can_encode_counts(&[1, 2, 1, 1]));
    assert!(!table.can_encode_counts(&[1, 2, 0, 1, 1]));
}

#[test]
fn rank_limited_weights_preserve_symbol_order_for_equal_counts() {
    let mut expected_nonzero = distribute_weights(5);
    let limit = expected_nonzero.len().ilog2() as usize + 2;
    redistribute_weights(&mut expected_nonzero, limit);

    let weights = rank_limited_weights(&[0, 7, 7, 7, 7, 7, 0]);

    assert_eq!(weights[0], 0);
    assert_eq!(weights[6], 0);
    assert_eq!(&weights[1..6], expected_nonzero.as_slice());
}

#[test]
fn length_limited_code_lengths_are_stable_for_tied_counts() {
    let counts = &[8, 8, 4, 4, 2, 2, 1, 1, 0, 16, 16, 32, 32];
    let lengths = length_limited_code_lengths(counts, MAX_HUFFMAN_BITS)
        .expect("length-limited table should be valid");

    for _ in 0..8 {
        assert_eq!(
            length_limited_code_lengths(counts, MAX_HUFFMAN_BITS),
            Some(lengths.clone())
        );
    }

    assert_eq!(
        length_units(&lengths, MAX_HUFFMAN_BITS),
        1 << MAX_HUFFMAN_BITS
    );
}

#[test]
fn from_data() {
    let data = &[0, 2, 4, 4, 0, 3, 2, 2, 0, 2];
    let table = HuffmanTable::build_from_data(data).codes;

    assert_eq!(table[1].1, 0);
    for symbol in [0, 2, 3, 4] {
        assert!(table[symbol].1 > 0);
        assert!(table[symbol].1 <= MAX_HUFFMAN_BITS as u8);
    }
}

#[test]
fn encoded_len_matches_single_stream_encoder() {
    let data = b"abbcccddddeeeee";
    let table = HuffmanTable::build_from_data(data);

    assert_eq!(
        table.encoded_len(data, true, false),
        actual_encoded_len(&table, data, true, false)
    );
    assert_eq!(
        table.encoded_len(data, false, false),
        actual_encoded_len(&table, data, false, false)
    );
}

#[test]
fn encoded_len_matches_four_stream_encoder() {
    let data = b"tenant=alpha path=/v1/archive status=200 tenant=beta path=/v1/search status=404";
    let table = HuffmanTable::build_from_data(data);

    assert_eq!(
        table.encoded_len(data, true, true),
        actual_encoded_len(&table, data, true, true)
    );
    assert_eq!(
        table.encoded_len(data, false, true),
        actual_encoded_len(&table, data, false, true)
    );
}

#[cfg(test)]
fn actual_encoded_len(
    table: &HuffmanTable,
    data: &[u8],
    with_table: bool,
    four_streams: bool,
) -> usize {
    let mut encoded = Vec::new();
    let mut writer = BitWriter::from(&mut encoded);
    let mut encoder = HuffmanEncoder::new(table, &mut writer);
    if four_streams {
        encoder.encode4x(data, with_table);
    } else {
        encoder.encode(data, with_table);
    }
    writer.flush();
    encoded.len()
}

#[cfg(test)]
fn assert_prefix_free(codes: &[(u32, u8)]) {
    for (idx, (code, num_bits)) in codes.iter().copied().enumerate() {
        if num_bits == 0 {
            continue;
        }
        for (other_idx, (other_code, other_num_bits)) in codes.iter().copied().enumerate() {
            if idx == other_idx || other_num_bits == 0 {
                continue;
            }
            if num_bits <= other_num_bits {
                let other_code_prefix = other_code >> (other_num_bits - num_bits);
                assert_ne!(
                    code, other_code_prefix,
                    "symbol {idx}'s {num_bits}-bit code is a prefix of symbol {other_idx}'s {other_num_bits}-bit code"
                );
            }
        }
    }
}
