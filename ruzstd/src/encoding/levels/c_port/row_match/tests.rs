use super::super::hash_chain_match::MatchSearchConfig;
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
fn row_match_mask_matches_scalar_for_all_row_widths() {
    for width in [16usize, 32, 64] {
        let mut row = vec![0u8; width];
        for (idx, byte) in row.iter_mut().enumerate() {
            *byte = (idx.wrapping_mul(37).wrapping_add(width) & 0xff) as u8;
        }
        for idx in (3..width).step_by(7) {
            row[idx] = 0x5a;
        }

        for head in [0usize, 1, width / 3, width - 1] {
            assert_eq!(
                row_match_mask(&row, 0x5a, head),
                row_match_mask_scalar(&row, 0x5a, head)
            );
        }
    }
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

    let match_len = row_find_best_match(
        data,
        8,
        data.len(),
        &mut off_base,
        &mut state,
        MatchSearchConfig::new(params, 4, 0),
    );

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

    let match_len = row_find_best_match(
        &data,
        500,
        data.len(),
        &mut off_base,
        &mut state,
        MatchSearchConfig::new(params, 4, 0),
    );

    assert_eq!(match_len, 3);
    assert_eq!(off_base, 0);
}
