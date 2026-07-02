use alloc::vec::Vec;

use super::{
    cctx_params::{CctxParameters, LdmParameters, ParamSwitch},
    ldm::{LdmEntry, LdmHashTable, LdmRollingHashState, LDM_BATCH_SIZE},
};

fn small_ldm_params() -> LdmParameters {
    LdmParameters {
        enable_ldm: ParamSwitch::Enable,
        window_log: 10,
        hash_log: 6,
        min_match_length: 16,
        bucket_size_log: 3,
        hash_rate_log: 4,
    }
}

#[test]
fn ldm_hash_table_sizes_like_c() {
    let table = LdmHashTable::new(small_ldm_params());

    assert_eq!(table.table_len(), 64);
    assert_eq!(table.bucket_count(), 8);
    assert_eq!(table.bucket(5).len(), 8);
}

#[test]
fn ldm_hash_table_inserts_into_bucket_slots_like_c() {
    let mut table = LdmHashTable::new(small_ldm_params());

    table.insert_entry(
        5,
        LdmEntry {
            offset: 11,
            checksum: 101,
        },
    );
    table.insert_entry(
        5,
        LdmEntry {
            offset: 12,
            checksum: 102,
        },
    );
    table.insert_entry(
        2,
        LdmEntry {
            offset: 21,
            checksum: 201,
        },
    );

    assert_eq!(table.bucket_offset(5), 2);
    assert_eq!(table.bucket_offset(2), 1);
    assert_eq!(
        &table.bucket(5)[..3],
        &[
            LdmEntry {
                offset: 11,
                checksum: 101,
            },
            LdmEntry {
                offset: 12,
                checksum: 102,
            },
            LdmEntry::default(),
        ]
    );
    assert_eq!(
        table.bucket(2)[0],
        LdmEntry {
            offset: 21,
            checksum: 201,
        }
    );
}

#[test]
fn ldm_hash_table_wraps_bucket_offsets_like_c() {
    let mut table = LdmHashTable::new(small_ldm_params());

    for i in 0..10 {
        table.insert_entry(
            3,
            LdmEntry {
                offset: i,
                checksum: 100 + i,
            },
        );
    }

    assert_eq!(table.bucket_offset(3), 2);
    assert_eq!(
        &table.bucket(3)[..4],
        &[
            LdmEntry {
                offset: 8,
                checksum: 108,
            },
            LdmEntry {
                offset: 9,
                checksum: 109,
            },
            LdmEntry {
                offset: 2,
                checksum: 102,
            },
            LdmEntry {
                offset: 3,
                checksum: 103,
            },
        ]
    );
}

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
