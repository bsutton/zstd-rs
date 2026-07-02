use alloc::vec::Vec;

use super::{
    cctx_params::CctxParameters,
    ldm::{LdmRollingHashState, LDM_BATCH_SIZE},
};

#[test]
fn ldm_gear_initializes_stop_mask_like_c() {
    let params = CctxParameters::for_level(22, 256 * 1024 * 1024, 0).ldm;
    let state = LdmRollingHashState::new(params);

    assert_eq!(state.rolling(), 0xffff_ffff);
    assert_eq!(state.stop_mask(), 0xf000_0000);
}

#[test]
fn ldm_gear_reset_preserves_rolling_like_c_1_5_7() {
    let params = CctxParameters::for_level(22, 256 * 1024 * 1024, 0).ldm;
    let mut state = LdmRollingHashState::new(params);
    let data: Vec<u8> = (0_u16..64).map(|i| (i * 13 + 5) as u8).collect();

    state.reset(&data, params.min_match_length as usize);

    assert_eq!(state.rolling(), 0xffff_ffff);
}

#[test]
fn ldm_gear_feed_finds_c_split_points() {
    let params = CctxParameters::for_level(22, 256 * 1024 * 1024, 0).ldm;
    let mut state = LdmRollingHashState::new(params);
    let data: Vec<u8> = (0_u16..512).map(|i| ((i * 37 + 11) & 0xff) as u8).collect();
    let mut splits = [0_usize; LDM_BATCH_SIZE];
    let mut num_splits = 0;

    let processed = state.feed(&data, &mut splits, &mut num_splits);

    assert_eq!(processed, 512);
    assert_eq!(num_splits, 25);
    assert_eq!(state.rolling(), 0x9e16_056e_bb3e_cce6);
    assert_eq!(
        &splits[..num_splits],
        &[
            7, 10, 69, 95, 120, 126, 130, 146, 176, 178, 192, 229, 252, 266, 325, 351, 376, 382,
            386, 402, 432, 434, 448, 485, 508,
        ]
    );
}
