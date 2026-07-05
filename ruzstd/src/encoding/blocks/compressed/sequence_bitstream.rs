use alloc::{rc::Rc, vec::Vec};
use core::convert::TryFrom;

use crate::{
    bit_io::BitWriter, encoding::frame_compressor::OffsetHistory, fse::fse_encoder::FSETable,
};

use super::{
    sequence_codes::{encode_literal_length, encode_match_len, encode_offset},
    sequence_tables::{encode_table, FseTableMode},
    PreparedSequence,
};

pub(super) fn encode_table_count_size(
    mode: &FseTableMode<'_>,
    writer: &mut BitWriter<&mut Vec<u8>>,
) -> usize {
    let start = writer.index();
    encode_table(mode, writer);
    if matches!(mode, FseTableMode::Encoded(_)) {
        byte_size_between(start, writer.index())
    } else {
        0
    }
}

pub(super) fn byte_size_between(start_bits: usize, end_bits: usize) -> usize {
    debug_assert!(start_bits.is_multiple_of(8));
    debug_assert!(end_bits.is_multiple_of(8));
    (end_bits - start_bits) / 8
}

pub(super) fn should_emit_raw_for_legacy_decoder(
    last_count_size: usize,
    bitstream_size: usize,
) -> bool {
    // Mirrors zstd's compatibility guard for decoders <= 1.3.4, which
    // rejected compressed sequence tables when FSE_readNCount saw <4 bytes.
    last_count_size != 0 && last_count_size + bitstream_size < 4
}

pub(super) enum FseTableUpdate {
    Keep,
    Clear,
    Replace(Rc<FSETable>),
}

pub(super) fn fse_table_update(mode: FseTableMode<'_>) -> FseTableUpdate {
    match mode {
        FseTableMode::Encoded(table) => FseTableUpdate::Replace(Rc::new(table)),
        FseTableMode::Predefined(_) | FseTableMode::Rle(_) => FseTableUpdate::Clear,
        FseTableMode::RepeatLast(_) => FseTableUpdate::Keep,
    }
}

pub(super) fn apply_fse_table_update(previous: &mut Option<Rc<FSETable>>, update: FseTableUpdate) {
    match update {
        FseTableUpdate::Keep => {}
        FseTableUpdate::Clear => *previous = None,
        FseTableUpdate::Replace(table) => *previous = Some(table),
    }
}

pub(super) fn encode_sequences_for_history(
    sequences: &[PreparedSequence],
    offset_history: &mut OffsetHistory,
) -> Vec<crate::blocks::sequence_section::Sequence> {
    let mut encoded = Vec::with_capacity(sequences.len());
    encode_sequences_for_history_into(sequences, offset_history, &mut encoded);
    encoded
}

pub(super) fn encode_sequences_for_history_into(
    sequences: &[PreparedSequence],
    offset_history: &mut OffsetHistory,
    encoded: &mut Vec<crate::blocks::sequence_section::Sequence>,
) {
    encoded.clear();
    for sequence in sequences {
        let of = if let Some(offset_value) = sequence.encoded_offset_value {
            offset_history.update_from_offset_value(offset_value, sequence.ll, sequence.raw_offset);
            offset_value
        } else {
            offset_history.encode_offset_value(sequence.raw_offset, sequence.ll)
        };
        encoded.push(crate::blocks::sequence_section::Sequence {
            ll: sequence.ll,
            ml: sequence.ml,
            of,
        });
    }
}

#[inline(always)]
pub(super) fn offset_to_u32(offset: usize) -> u32 {
    match u32::try_from(offset) {
        Ok(offset) => offset,
        Err(_) => unreachable!("match offsets are bounded by the compressor window"),
    }
}

pub(super) fn encode_sequences(
    sequences: &[crate::blocks::sequence_section::Sequence],
    writer: &mut BitWriter<&mut Vec<u8>>,
    ll_mode: &FseTableMode<'_>,
    ml_mode: &FseTableMode<'_>,
    of_mode: &FseTableMode<'_>,
) {
    if let (
        FseTableMode::Rle(ll_symbol),
        FseTableMode::Rle(ml_symbol),
        FseTableMode::Rle(of_symbol),
    ) = (ll_mode, ml_mode, of_mode)
    {
        encode_rle_sequences(sequences, writer, *ll_symbol, *ml_symbol, *of_symbol);
        return;
    }

    let sequence = sequences[sequences.len() - 1];
    let ll_table = ll_mode.table();
    let ml_table = ml_mode.table();
    let of_table = of_mode.table();
    if let (Some(ll_table), Some(ml_table), Some(of_table)) = (ll_table, ml_table, of_table) {
        encode_table_sequences(sequences, writer, ll_table, ml_table, of_table);
        return;
    }

    let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
    let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
    let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);
    let mut ll_state = init_fse_state(ll_mode, ll_code);
    let mut ml_state = init_fse_state(ml_mode, ml_code);
    let mut of_state = init_fse_state(of_mode, of_code);

    writer.write_bits(ll_add_bits, ll_num_bits);
    writer.write_bits(ml_add_bits, ml_num_bits);
    writer.write_bits(of_add_bits, of_num_bits);

    // Encode backwards so the decoder reads the first sequence first.
    let mut sequence_idx = sequences.len() - 1;
    while sequence_idx > 0 {
        sequence_idx -= 1;
        let sequence = sequences[sequence_idx];
        let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
        let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
        let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);

        {
            update_fse_state(of_table, &mut of_state, of_code, writer);
        }
        {
            update_fse_state(ml_table, &mut ml_state, ml_code, writer);
        }
        {
            update_fse_state(ll_table, &mut ll_state, ll_code, writer);
        }

        writer.write_bits(ll_add_bits, ll_num_bits);
        writer.write_bits(ml_add_bits, ml_num_bits);
        writer.write_bits(of_add_bits, of_num_bits);
    }
    flush_fse_state(ml_table, ml_state, writer);
    flush_fse_state(of_table, of_state, writer);
    flush_fse_state(ll_table, ll_state, writer);

    let bits_to_fill = writer.misaligned();
    if bits_to_fill == 0 {
        writer.write_bits(1u32, 8);
    } else {
        writer.write_bits(1u32, bits_to_fill);
    }
}

fn encode_table_sequences(
    sequences: &[crate::blocks::sequence_section::Sequence],
    writer: &mut BitWriter<&mut Vec<u8>>,
    ll_table: &FSETable,
    ml_table: &FSETable,
    of_table: &FSETable,
) {
    let sequence = sequences[sequences.len() - 1];
    let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
    let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
    let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);
    let mut ll_state = ll_table.c_start_state_index(ll_code);
    let mut ml_state = ml_table.c_start_state_index(ml_code);
    let mut of_state = of_table.c_start_state_index(of_code);

    writer.write_bits(ll_add_bits, ll_num_bits);
    writer.write_bits(ml_add_bits, ml_num_bits);
    writer.write_bits(of_add_bits, of_num_bits);

    let mut sequence_idx = sequences.len() - 1;
    while sequence_idx > 0 {
        sequence_idx -= 1;
        let sequence = sequences[sequence_idx];
        let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
        let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
        let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);

        update_table_fse_state(of_table, &mut of_state, of_code, writer);
        update_table_fse_state(ml_table, &mut ml_state, ml_code, writer);
        update_table_fse_state(ll_table, &mut ll_state, ll_code, writer);

        writer.write_bits(ll_add_bits, ll_num_bits);
        writer.write_bits(ml_add_bits, ml_num_bits);
        writer.write_bits(of_add_bits, of_num_bits);
    }
    writer.write_bits(u64::from(ml_state), ml_table.acc_log() as usize);
    writer.write_bits(u64::from(of_state), of_table.acc_log() as usize);
    writer.write_bits(u64::from(ll_state), ll_table.acc_log() as usize);

    let bits_to_fill = writer.misaligned();
    if bits_to_fill == 0 {
        writer.write_bits(1u32, 8);
    } else {
        writer.write_bits(1u32, bits_to_fill);
    }
}

fn encode_rle_sequences(
    sequences: &[crate::blocks::sequence_section::Sequence],
    writer: &mut BitWriter<&mut Vec<u8>>,
    ll_symbol: u8,
    ml_symbol: u8,
    of_symbol: u8,
) {
    for sequence in sequences.iter().rev() {
        let (ll_code, ll_add_bits, ll_num_bits) = encode_literal_length(sequence.ll);
        let (of_code, of_add_bits, of_num_bits) = encode_offset(sequence.of);
        let (ml_code, ml_add_bits, ml_num_bits) = encode_match_len(sequence.ml);
        debug_assert_eq!(ll_code, ll_symbol);
        debug_assert_eq!(ml_code, ml_symbol);
        debug_assert_eq!(of_code, of_symbol);

        writer.write_bits(ll_add_bits, ll_num_bits);
        writer.write_bits(ml_add_bits, ml_num_bits);
        writer.write_bits(of_add_bits, of_num_bits);
    }

    let bits_to_fill = writer.misaligned();
    if bits_to_fill == 0 {
        writer.write_bits(1u32, 8);
    } else {
        writer.write_bits(1u32, bits_to_fill);
    }
}

fn init_fse_state(mode: &FseTableMode<'_>, symbol: u8) -> Option<u32> {
    match mode {
        FseTableMode::Rle(rle_symbol) => {
            debug_assert_eq!(*rle_symbol, symbol);
            None
        }
        _ => mode.table().map(|table| table.c_start_state_index(symbol)),
    }
}

fn update_fse_state(
    table: Option<&FSETable>,
    state: &mut Option<u32>,
    symbol: u8,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if let Some(table) = table {
        if let Some(current) = *state {
            let next = table.next_state(symbol, current);
            let diff = current - next.baseline;
            writer.write_bits(u64::from(diff), next.num_bits as usize);
            *state = Some(next.index);
        } else {
            unreachable!("non-RLE FSE mode must have a state");
        }
    }
}

#[inline(always)]
fn update_table_fse_state(
    table: &FSETable,
    state: &mut u32,
    symbol: u8,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    let next = table.next_state(symbol, *state);
    let diff = *state - next.baseline;
    writer.write_bits(u64::from(diff), next.num_bits as usize);
    *state = next.index;
}

fn flush_fse_state(
    table: Option<&FSETable>,
    state: Option<u32>,
    writer: &mut BitWriter<&mut Vec<u8>>,
) {
    if let Some(table) = table {
        if let Some(state) = state {
            writer.write_bits(u64::from(state), table.acc_log() as usize);
        } else {
            unreachable!("non-RLE FSE mode must have a state");
        }
    }
}

pub(super) fn encode_seqnum(seqnum: usize, writer: &mut BitWriter<impl AsMut<Vec<u8>>>) {
    const UPPER_LIMIT: usize = 0xFFFF + 0x7F00;
    match seqnum {
        1..=127 => writer.write_bits(seqnum as u32, 8),
        128..=0x7FFF => {
            let upper = ((seqnum >> 8) | 0x80) as u8;
            let lower = seqnum as u8;
            writer.write_bits(upper, 8);
            writer.write_bits(lower, 8);
        }
        0x8000..=UPPER_LIMIT => {
            let encode = seqnum - 0x7F00;
            let upper = (encode >> 8) as u8;
            let lower = encode as u8;
            writer.write_bits(255u8, 8);
            writer.write_bits(upper, 8);
            writer.write_bits(lower, 8);
        }
        _ => unreachable!(),
    }
}
