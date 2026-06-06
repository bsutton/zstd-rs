use alloc::vec::Vec;

use super::{
    greedy::GreedyMatchState,
    opt_match::{bt_get_all_matches_no_dict, BtMatchRequest, OptMatch},
    params::{CompressionParameters, Strategy},
    sequence_store::{OffBase, RepeatCode, RepeatOffsets},
};

fn params_for_min_match(min_match: u32) -> CompressionParameters {
    CompressionParameters {
        window_log: 18,
        chain_log: 15,
        hash_log: 15,
        search_log: 4,
        min_match,
        target_length: 16,
        strategy: Strategy::BtOpt,
    }
}

#[test]
fn opt_match_collector_reports_repcodes_before_tree_matches() {
    let data = b"abcabcabcxyz";
    let params = params_for_min_match(4);
    let mut state = GreedyMatchState::new();
    let mut matches = Vec::new();

    bt_get_all_matches_no_dict(
        &mut matches,
        request(
            data,
            3,
            RepeatOffsets::from_offsets(3, 4, 8),
            false,
            4,
            params,
        ),
        &mut state,
    );

    assert_eq!(
        matches,
        [OptMatch {
            off_base: OffBase::Repeat(RepeatCode::First).to_c_value(),
            len: 6,
        }]
    );
}

#[test]
fn opt_match_collector_reports_increasing_tree_matches() {
    let data = b"xabcdefghijabcdefghij-tail";
    let params = params_for_min_match(4);
    let mut state = GreedyMatchState::new();
    let mut matches = Vec::new();

    bt_get_all_matches_no_dict(
        &mut matches,
        request(data, 11, RepeatOffsets::new(), false, 4, params),
        &mut state,
    );

    assert_eq!(
        matches.last(),
        Some(&OptMatch {
            off_base: OffBase::Offset(10).to_c_value(),
            len: 10,
        })
    );
}

#[test]
fn opt_match_collector_uses_hash3_for_min_match_three() {
    let data = b"xabc---abcXYZ";
    let params = params_for_min_match(3);
    let mut state = GreedyMatchState::new();
    let mut matches = Vec::new();

    bt_get_all_matches_no_dict(
        &mut matches,
        request(data, 7, RepeatOffsets::new(), false, 3, params),
        &mut state,
    );

    assert_eq!(
        matches,
        [OptMatch {
            off_base: OffBase::Offset(6).to_c_value(),
            len: 3,
        }]
    );
}

fn request<'a>(
    src: &'a [u8],
    ip: usize,
    rep: RepeatOffsets,
    ll0: bool,
    length_to_beat: u32,
    params: CompressionParameters,
) -> BtMatchRequest<'a> {
    BtMatchRequest {
        src,
        ip,
        block_end: src.len(),
        rep,
        ll0,
        length_to_beat,
        params,
    }
}
