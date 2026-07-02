use super::{
    cctx_params::{CctxParameters, ParamSwitch},
    params::{CompressionParameters, Strategy},
};

#[test]
fn cctx_params_resolve_row_matchfinder_like_c() {
    let params = CctxParameters::for_level(5, 512 * 1024, 0);

    assert_eq!(params.compression.strategy, Strategy::Greedy);
    assert_eq!(params.use_row_match_finder, ParamSwitch::Enable);

    let tiny = CctxParameters::for_level(5, 16 * 1024, 0);
    assert_eq!(tiny.compression.strategy, Strategy::Lazy);
    assert_eq!(tiny.use_row_match_finder, ParamSwitch::Disable);
}

#[test]
fn cctx_params_resolve_post_block_splitter_like_c() {
    let level16 = CctxParameters::for_level(16, 512 * 1024, 0);
    let level15 = CctxParameters::for_level(15, 512 * 1024, 0);

    assert_eq!(level16.compression.strategy, Strategy::BtOpt);
    assert_eq!(level16.post_block_splitter, ParamSwitch::Enable);
    assert_eq!(level15.compression.strategy, Strategy::BtLazy2);
    assert_eq!(level15.post_block_splitter, ParamSwitch::Disable);
}

#[test]
fn cctx_params_resolve_ldm_for_large_level22_like_c() {
    let level22 = CctxParameters::for_level(22, 256 * 1024 * 1024, 0);

    assert_eq!(level22.compression.window_log, 27);
    assert_eq!(level22.compression.strategy, Strategy::BtUltra2);
    assert_eq!(level22.ldm.enable_ldm, ParamSwitch::Enable);
    assert_eq!(level22.ldm.window_log, 27);
    assert_eq!(level22.ldm.hash_rate_log, 4);
    assert_eq!(level22.ldm.hash_log, 23);
    assert_eq!(level22.ldm.min_match_length, 32);
    assert_eq!(level22.ldm.bucket_size_log, 8);
}

#[test]
fn cctx_params_keep_ldm_disabled_below_c_threshold() {
    let level21 = CctxParameters::for_level(21, 256 * 1024 * 1024, 0);
    let small_level22 = CctxParameters::for_level(22, 128 * 1024, 0);

    assert_eq!(level21.compression.window_log, 26);
    assert_eq!(level21.ldm.enable_ldm, ParamSwitch::Disable);
    assert_eq!(small_level22.compression.window_log, 17);
    assert_eq!(small_level22.ldm.enable_ldm, ParamSwitch::Disable);
}

#[test]
fn cctx_params_external_repcode_search_uses_c_level_gate() {
    assert_eq!(
        CctxParameters::for_level(9, 512 * 1024, 0).search_for_external_repcodes,
        ParamSwitch::Disable
    );
    assert_eq!(
        CctxParameters::for_level(10, 512 * 1024, 0).search_for_external_repcodes,
        ParamSwitch::Enable
    );
}

#[test]
fn cctx_params_can_resolve_from_explicit_compression_params() {
    let compression = CompressionParameters {
        window_log: 27,
        chain_log: 27,
        hash_log: 25,
        search_log: 9,
        min_match: 3,
        target_length: 999,
        strategy: Strategy::BtUltra2,
    };
    let resolved = CctxParameters::from_compression_parameters(22, compression);

    assert_eq!(resolved.ldm.enable_ldm, ParamSwitch::Enable);
    assert_eq!(resolved.post_block_splitter, ParamSwitch::Enable);
}
