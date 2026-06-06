use super::{CompressionParameters, Strategy, MAX_COMPRESSION_LEVEL, MIN_COMPRESSION_LEVEL};

fn params(
    window_log: u32,
    chain_log: u32,
    hash_log: u32,
    search_log: u32,
    min_match: u32,
    target_length: u32,
    strategy: Strategy,
) -> CompressionParameters {
    CompressionParameters {
        window_log,
        chain_log,
        hash_log,
        search_log,
        min_match,
        target_length,
        strategy,
    }
}

#[test]
fn level_zero_uses_c_default_level_for_unknown_size() {
    assert_eq!(
        CompressionParameters::for_level(0, 0, 0),
        params(21, 16, 17, 1, 5, 0, Strategy::DFast)
    );
}

#[test]
fn unknown_size_uses_large_source_table() {
    assert_eq!(
        CompressionParameters::for_level(1, 0, 0),
        params(19, 13, 14, 1, 7, 0, Strategy::Fast)
    );
    assert_eq!(
        CompressionParameters::for_level(19, 0, 0),
        params(23, 24, 22, 7, 3, 256, Strategy::BtUltra2)
    );
}

#[test]
fn source_size_selects_c_size_tiers() {
    assert_eq!(
        CompressionParameters::for_level(3, 256 * 1024 + 1, 0),
        params(19, 16, 17, 1, 5, 0, Strategy::DFast)
    );
    assert_eq!(
        CompressionParameters::for_level(3, 256 * 1024, 0),
        params(18, 16, 16, 1, 4, 0, Strategy::DFast)
    );
    assert_eq!(
        CompressionParameters::for_level(3, 128 * 1024, 0),
        params(17, 15, 16, 2, 5, 0, Strategy::DFast)
    );
    assert_eq!(
        CompressionParameters::for_level(3, 16 * 1024, 0),
        params(14, 14, 15, 2, 4, 0, Strategy::DFast)
    );
}

#[test]
fn known_tiny_source_clamps_work_tables_like_c() {
    assert_eq!(
        CompressionParameters::for_level(3, 1, 0),
        params(10, 6, 7, 2, 4, 0, Strategy::DFast)
    );
}

#[test]
fn compression_level_is_clamped_to_c_bounds() {
    assert_eq!(
        CompressionParameters::for_level(MAX_COMPRESSION_LEVEL + 100, 0, 0),
        CompressionParameters::for_level(MAX_COMPRESSION_LEVEL, 0, 0)
    );
    assert_eq!(
        CompressionParameters::for_level(-5, 0, 0),
        params(19, 12, 13, 1, 6, 5, Strategy::Fast)
    );
    assert_eq!(
        CompressionParameters::for_level(MIN_COMPRESSION_LEVEL - 1, 0, 0),
        params(19, 12, 13, 1, 6, 128 * 1024, Strategy::Fast)
    );
}
