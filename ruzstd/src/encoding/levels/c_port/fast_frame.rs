//! Frame-level adapter for the C fast no-dictionary path.

use alloc::vec::Vec;

use super::{
    fast_block::{encode_block_fast_no_dict, FastBlockEncodeContext},
    params::CompressionParameters,
    sequence_store::RepeatOffsets,
};
use crate::encoding::{
    blocks::BlockCompressionConfig,
    frame_compressor::{FseTables, OffsetHistory},
    frame_header::FrameHeader,
    CompressionLevel,
};

pub(crate) fn encode_single_block_frame_fast_no_dict(src: &[u8], level: i32) -> Vec<u8> {
    let mut output = Vec::new();
    FrameHeader {
        frame_content_size: Some(src.len() as u64),
        single_segment: true,
        content_checksum: false,
        dictionary_id: None,
        window_size: None,
    }
    .serialize(&mut output);

    let mut fse_tables = FseTables::new();
    let mut offset_history = OffsetHistory::new();
    let encoded_block = encode_block_fast_no_dict(
        src,
        true,
        CompressionParameters::for_level(level, src.len() as u64, 0),
        BlockCompressionConfig::for_level(CompressionLevel::Fastest),
        RepeatOffsets::new(),
        FastBlockEncodeContext {
            previous_huff_table: None,
            fse_tables: &mut fse_tables,
            offset_history: &mut offset_history,
        },
    );
    output.extend_from_slice(&encoded_block.bytes);
    output
}
