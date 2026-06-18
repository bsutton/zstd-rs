//! Scalar row-based match finder ported from the no-dictionary row path in
//! `zstd_lazy.c`.

use super::{greedy::GreedyMatchState, params::CompressionParameters, sequence_store::OffBase};

const TAG_BITS: u32 = 8;
const TAG_MASK: u32 = (1 << TAG_BITS) - 1;
const SKIP_THRESHOLD: usize = 384;
const MAX_MATCH_START_POSITIONS_TO_UPDATE: usize = 96;
const MAX_MATCH_END_POSITIONS_TO_UPDATE: usize = 32;
const ROW_HASH_CACHE_SIZE: usize = 8;
const ROW_HASH_CACHE_MASK: usize = ROW_HASH_CACHE_SIZE - 1;

pub(super) fn row_find_best_match(
    src: &[u8],
    ip: usize,
    block_end: usize,
    off_base: &mut u32,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) -> usize {
    match row_log(params) {
        4 => row_find_best_match_impl::<4>(src, ip, block_end, off_base, params, min_match, state),
        5 => row_find_best_match_impl::<5>(src, ip, block_end, off_base, params, min_match, state),
        6 => row_find_best_match_impl::<6>(src, ip, block_end, off_base, params, min_match, state),
        _ => unreachable!("row_log is clamped to 4..=6"),
    }
}

fn row_find_best_match_impl<const ROW_LOG: u32>(
    src: &[u8],
    ip: usize,
    block_end: usize,
    off_base: &mut u32,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) -> usize {
    let row_entries = 1usize << ROW_LOG;
    let row_mask = row_entries - 1;
    let row_hash_log = params.hash_log - ROW_LOG;
    let max_attempts = 1usize << params.search_log.min(ROW_LOG);
    let curr = ip;
    let max_distance = 1usize << params.window_log;
    let low_limit = curr.saturating_sub(max_distance);

    let hash = if state.lazy_skipping {
        state.next_to_update = curr;
        hash_ptr_salted(
            src,
            curr,
            row_hash_log + TAG_BITS,
            min_match,
            state.hash_salt,
        )
    } else {
        update_rows::<ROW_LOG>(src, curr, params, min_match, state);
        next_cached_hash(src, curr, row_hash_log, ROW_LOG, min_match, state)
    };

    state.hash_salt_entropy = state.hash_salt_entropy.wrapping_add(hash);

    let row_start = ((hash >> TAG_BITS) as usize) << ROW_LOG;
    let tag = (hash & TAG_MASK) as u8;
    let head = usize::from(state.tag_table[row_start] & row_mask as u8);
    let mut best_len = 3usize;
    let mut matches = row_match_mask(
        &state.tag_table[row_start..row_start + row_entries],
        tag,
        head,
    );

    #[cfg(target_arch = "x86_64")]
    {
        let mut attempts = 0usize;
        let mut match_buffer = [const { core::mem::MaybeUninit::<usize>::uninit() }; 1 << 6];
        let mut match_count = 0usize;

        while matches != 0 && attempts < max_attempts {
            let step = matches.trailing_zeros() as usize;
            matches &= matches - 1;
            let pos = (head + step) & row_mask;
            if pos == 0 {
                continue;
            }

            let match_index = state.hash_table[row_start + pos] as usize;
            if match_index < low_limit {
                break;
            }
            if match_index >= curr {
                continue;
            }
            debug_assert!(match_count < match_buffer.len());
            prefetch_read(src.as_ptr().wrapping_add(match_index));
            match_buffer[match_count].write(match_index);
            match_count += 1;
            attempts += 1;
        }

        for match_index in match_buffer[..match_count]
            .iter()
            // SAFETY: entries below match_count are written before match_count is incremented.
            .map(|entry| unsafe { entry.assume_init() })
        {
            let mut current_len = 0usize;
            if read32(src, match_index + best_len - 3) == read32(src, ip + best_len - 3) {
                current_len = super::hash_chain_match::count_match(src, ip, match_index, block_end);
            }

            if current_len > best_len {
                best_len = current_len;
                *off_base = OffBase::from_offset((curr - match_index) as u32)
                    .expect("row match has non-zero offset")
                    .to_c_value();
                if ip + current_len == block_end {
                    break;
                }
            }
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let mut attempts = 0usize;
        while matches != 0 && attempts < max_attempts {
            let step = matches.trailing_zeros() as usize;
            matches &= matches - 1;
            let pos = (head + step) & row_mask;
            if pos == 0 {
                continue;
            }

            let match_index = state.hash_table[row_start + pos] as usize;
            if match_index < low_limit {
                break;
            }
            if match_index >= curr {
                continue;
            }
            attempts += 1;

            let mut current_len = 0usize;
            if read32(src, match_index + best_len - 3) == read32(src, ip + best_len - 3) {
                current_len = super::hash_chain_match::count_match(src, ip, match_index, block_end);
            }

            if current_len > best_len {
                best_len = current_len;
                *off_base = OffBase::from_offset((curr - match_index) as u32)
                    .expect("row match has non-zero offset")
                    .to_c_value();
                if ip + current_len == block_end {
                    break;
                }
            }
        }
    }

    let insert_pos = next_row_index(&mut state.tag_table[row_start], row_mask);
    state.tag_table[row_start + insert_pos] = tag;
    state.hash_table[row_start + insert_pos] = state.next_to_update as u32;
    state.next_to_update += 1;

    best_len
}

fn update_rows<const ROW_LOG: u32>(
    src: &[u8],
    target: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    update_rows_internal::<ROW_LOG>(src, target, params, min_match, state, true);
}

pub(super) fn fill_hash_cache(
    src: &[u8],
    idx: usize,
    limit: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    match row_log(params) {
        4 => fill_hash_cache_impl::<4>(src, idx, limit, params, min_match, state),
        5 => fill_hash_cache_impl::<5>(src, idx, limit, params, min_match, state),
        6 => fill_hash_cache_impl::<6>(src, idx, limit, params, min_match, state),
        _ => unreachable!("row_log is clamped to 4..=6"),
    }
}

fn fill_hash_cache_impl<const ROW_LOG: u32>(
    src: &[u8],
    mut idx: usize,
    limit: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    let row_hash_log = params.hash_log - ROW_LOG;
    let end = idx
        .saturating_add(ROW_HASH_CACHE_SIZE)
        .min(limit.saturating_add(1));
    while idx < end {
        let hash = hash_ptr_salted(
            src,
            idx,
            row_hash_log + TAG_BITS,
            min_match,
            state.hash_salt,
        );
        #[cfg(target_arch = "x86_64")]
        prefetch_row(state, ((hash >> TAG_BITS) as usize) << ROW_LOG);
        state.row_hash_cache[idx & ROW_HASH_CACHE_MASK] = hash;
        idx += 1;
    }
}

pub(super) fn load_dictionary_rows(
    src: &[u8],
    target: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
) {
    match row_log(params) {
        4 => update_rows_internal::<4>(src, target, params, min_match, state, false),
        5 => update_rows_internal::<5>(src, target, params, min_match, state, false),
        6 => update_rows_internal::<6>(src, target, params, min_match, state, false),
        _ => unreachable!("row_log is clamped to 4..=6"),
    }
}

fn update_rows_internal<const ROW_LOG: u32>(
    src: &[u8],
    target: usize,
    params: CompressionParameters,
    min_match: u32,
    state: &mut GreedyMatchState,
    use_cache_skip: bool,
) {
    let row_mask = (1usize << ROW_LOG) - 1;
    let row_hash_log = params.hash_log - ROW_LOG;
    let mut idx = state.next_to_update;

    if use_cache_skip && target.saturating_sub(idx) > SKIP_THRESHOLD {
        let start_bound = idx + MAX_MATCH_START_POSITIONS_TO_UPDATE;
        update_rows_range(
            src,
            idx,
            start_bound,
            row_hash_log,
            ROW_LOG,
            row_mask,
            min_match,
            state,
            use_cache_skip,
        );
        idx = target - MAX_MATCH_END_POSITIONS_TO_UPDATE;
        fill_hash_cache(src, idx, target, params, min_match, state);
    }

    update_rows_range(
        src,
        idx,
        target,
        row_hash_log,
        ROW_LOG,
        row_mask,
        min_match,
        state,
        use_cache_skip,
    );
    state.next_to_update = target;
}

#[allow(clippy::too_many_arguments)]
fn update_rows_range(
    src: &[u8],
    mut idx: usize,
    target: usize,
    row_hash_log: u32,
    row_log: u32,
    row_mask: usize,
    min_match: u32,
    state: &mut GreedyMatchState,
    use_cache: bool,
) {
    while idx < target {
        let hash = if use_cache {
            next_cached_hash(src, idx, row_hash_log, row_log, min_match, state)
        } else {
            hash_ptr_salted(
                src,
                idx,
                row_hash_log + TAG_BITS,
                min_match,
                state.hash_salt,
            )
        };
        let row_start = ((hash >> TAG_BITS) as usize) << row_log;
        let pos = next_row_index(&mut state.tag_table[row_start], row_mask);
        state.tag_table[row_start + pos] = (hash & TAG_MASK) as u8;
        state.hash_table[row_start + pos] = idx as u32;
        idx += 1;
    }
}

fn next_cached_hash(
    src: &[u8],
    idx: usize,
    row_hash_log: u32,
    row_log: u32,
    min_match: u32,
    state: &mut GreedyMatchState,
) -> u32 {
    let new_hash = hash_ptr_salted(
        src,
        idx + ROW_HASH_CACHE_SIZE,
        row_hash_log + TAG_BITS,
        min_match,
        state.hash_salt,
    );
    #[cfg(target_arch = "x86_64")]
    prefetch_row(state, ((new_hash >> TAG_BITS) as usize) << row_log);
    let cached = state.row_hash_cache[idx & ROW_HASH_CACHE_MASK];
    state.row_hash_cache[idx & ROW_HASH_CACHE_MASK] = new_hash;
    cached
}

#[cfg(target_arch = "x86_64")]
fn prefetch_row(state: &GreedyMatchState, row_start: usize) {
    prefetch_read(state.hash_table.as_ptr().wrapping_add(row_start));
    prefetch_read(state.tag_table.as_ptr().wrapping_add(row_start));
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn prefetch_read<T>(ptr: *const T) {
    unsafe {
        use core::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
        _mm_prefetch(ptr.cast::<i8>(), _MM_HINT_T0);
    }
}

fn next_row_index(head: &mut u8, row_mask: usize) -> usize {
    let mut next = usize::from(head.wrapping_sub(1)) & row_mask;
    if next == 0 {
        next = row_mask;
    }
    *head = next as u8;
    next
}

fn row_match_mask(tag_row: &[u8], tag: u8, head: usize) -> u64 {
    debug_assert!(matches!(tag_row.len(), 16 | 32 | 64));
    debug_assert!(head < tag_row.len());

    let mut matches = 0u64;
    let splat = u64::from_le_bytes([tag; 8]);
    let mut chunk_start = 0usize;
    while chunk_start < tag_row.len() {
        let chunk = read64(tag_row, chunk_start) ^ splat;
        let mut zero_bytes =
            chunk.wrapping_sub(0x0101_0101_0101_0101) & !chunk & 0x8080_8080_8080_8080;
        while zero_bytes != 0 {
            let byte = (zero_bytes.trailing_zeros() >> 3) as usize;
            matches |= 1u64 << (chunk_start + byte);
            zero_bytes &= zero_bytes - 1;
        }
        chunk_start += 8;
    }

    rotate_right_within(matches, head, tag_row.len())
}

fn rotate_right_within(value: u64, shift: usize, width: usize) -> u64 {
    debug_assert!((1..=64).contains(&width));
    debug_assert!(shift < width);
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };
    if shift == 0 {
        value & mask
    } else {
        ((value >> shift) | (value << (width - shift))) & mask
    }
}

pub(super) fn row_log(params: CompressionParameters) -> u32 {
    params.search_log.clamp(4, 6)
}

pub(super) fn row_match_finder_enabled(params: CompressionParameters) -> bool {
    matches!(
        params.strategy,
        super::params::Strategy::Greedy
            | super::params::Strategy::Lazy
            | super::params::Strategy::Lazy2
    ) && params.window_log > 14
}

fn hash_ptr_salted(src: &[u8], pos: usize, h_bits: u32, min_match: u32, salt: u64) -> u32 {
    match min_match {
        5 => hash5(read64(src, pos), h_bits, salt),
        6 => hash6(read64(src, pos), h_bits, salt),
        _ => hash4(read32(src, pos), h_bits, salt as u32),
    }
}

fn hash4(value: u32, h_bits: u32, salt: u32) -> u32 {
    const PRIME_4_BYTES: u32 = 2_654_435_761;
    (value.wrapping_mul(PRIME_4_BYTES) ^ salt).wrapping_shr(32 - h_bits)
}

fn hash5(value: u64, h_bits: u32, salt: u64) -> u32 {
    const PRIME_5_BYTES: u64 = 889_523_592_379;
    (((value << (64 - 40)).wrapping_mul(PRIME_5_BYTES) ^ salt) >> (64 - h_bits)) as u32
}

fn hash6(value: u64, h_bits: u32, salt: u64) -> u32 {
    const PRIME_6_BYTES: u64 = 227_718_039_650_203;
    (((value << (64 - 48)).wrapping_mul(PRIME_6_BYTES) ^ salt) >> (64 - h_bits)) as u32
}

fn read32(src: &[u8], pos: usize) -> u32 {
    debug_assert!(pos + 4 <= src.len());
    // SAFETY: row hashing and match probes bound positions before reading.
    // Unaligned loads mirror zstd's MEM_read32() hot path.
    unsafe {
        u32::from_le(core::ptr::read_unaligned(
            src.as_ptr().add(pos).cast::<u32>(),
        ))
    }
}

fn read64(src: &[u8], pos: usize) -> u64 {
    debug_assert!(pos + 8 <= src.len());
    // SAFETY: row hashing bounds positions before reading. Unaligned loads
    // mirror zstd's MEM_readST/MEM_read64() hot path.
    unsafe {
        u64::from_le(core::ptr::read_unaligned(
            src.as_ptr().add(pos).cast::<u64>(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::levels::c_port::params::Strategy;
    use alloc::vec;

    fn params() -> CompressionParameters {
        CompressionParameters {
            window_log: 18,
            chain_log: 16,
            hash_log: 16,
            search_log: 5,
            min_match: 4,
            target_length: 0,
            strategy: Strategy::Greedy,
        }
    }

    #[test]
    fn row_next_index_cycles_backwards_and_skips_zero() {
        let mut head = 0u8;

        assert_eq!(next_row_index(&mut head, 15), 15);
        assert_eq!(head, 15);
        assert_eq!(next_row_index(&mut head, 15), 14);
    }

    #[test]
    fn row_match_mask_preserves_circular_scan_order() {
        let mut row = [0u8; 16];
        row[0] = 5;
        row[2] = 7;
        row[5] = 7;
        row[9] = 7;
        row[15] = 7;

        let mut matches = row_match_mask(&row, 7, usize::from(row[0]));
        let mut positions = vec![];
        while matches != 0 {
            let step = matches.trailing_zeros() as usize;
            matches &= matches - 1;
            positions.push((usize::from(row[0]) + step) & 15);
        }

        assert_eq!(positions, vec![5, 9, 15, 2]);
    }

    #[test]
    fn row_finder_reports_previous_match() {
        let data = b"abcdefghabcdefgh-tail";
        let mut state = GreedyMatchState::new();
        let params = params();
        state.ensure_tables(params);
        let mut off_base = 0;
        fill_hash_cache(
            data,
            state.next_to_update,
            data.len() - 16,
            params,
            4,
            &mut state,
        );

        let match_len =
            row_find_best_match(data, 8, data.len(), &mut off_base, params, 4, &mut state);

        assert!(match_len >= 8);
        assert_eq!(off_base, 11);
    }

    #[test]
    fn row_finder_uses_c_userspace_window_gate() {
        let mut params = params();
        assert!(row_match_finder_enabled(params));
        params.window_log = 15;
        assert!(row_match_finder_enabled(params));
        params.window_log = 14;
        assert!(!row_match_finder_enabled(params));
    }

    #[test]
    fn row_update_skips_middle_of_large_gaps_like_c() {
        let mut data = vec![0u8; 540];
        for (idx, byte) in data.iter_mut().enumerate() {
            *byte = (idx.wrapping_mul(37) & 0xFF) as u8;
        }
        let pattern = b"abcdefghijklmnopqrstuvwxyz";
        data[200..200 + pattern.len()].copy_from_slice(pattern);
        data[500..500 + pattern.len()].copy_from_slice(pattern);

        let mut state = GreedyMatchState::new();
        let params = params();
        state.ensure_tables(params);
        let mut off_base = 0;
        fill_hash_cache(
            &data,
            state.next_to_update,
            data.len() - 16,
            params,
            4,
            &mut state,
        );

        let match_len =
            row_find_best_match(&data, 500, data.len(), &mut off_base, params, 4, &mut state);

        assert_eq!(match_len, 3);
        assert_eq!(off_base, 0);
    }
}
