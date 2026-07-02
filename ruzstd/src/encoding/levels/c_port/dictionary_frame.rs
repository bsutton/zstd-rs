//! Shared setup for C-style dictionary frame encoders.

use alloc::vec::Vec;

use super::{
    c_frame_header::write_frame_header, cctx_params::CctxParameters, dictionary::ParsedDictionary,
    frame_state::FrameBlockState,
};

pub(crate) struct DictionaryFrameContext {
    pub(crate) combined: Vec<u8>,
    pub(crate) dict_len: usize,
    pub(crate) cctx: CctxParameters,
    pub(crate) output: Vec<u8>,
    pub(crate) frame_state: FrameBlockState,
}

impl DictionaryFrameContext {
    pub(crate) fn new(src: &[u8], level: i32, dictionary: ParsedDictionary<'_>) -> Self {
        let mut combined = Vec::with_capacity(dictionary.content.len() + src.len());
        combined.extend_from_slice(dictionary.content);
        combined.extend_from_slice(src);

        let dict_len = dictionary.content.len();
        let cctx = CctxParameters::for_level(level, src.len() as u64, dict_len);
        cctx.assert_resolved();
        let params = cctx.compression;
        let dictionary_id = (dictionary.dict_id != 0).then_some(dictionary.dict_id);
        let mut output = Vec::new();
        write_frame_header(&mut output, src.len(), params, dictionary_id);
        let frame_state = FrameBlockState::with_dictionary(params, output.len(), &dictionary);

        Self {
            combined,
            dict_len,
            cctx,
            output,
            frame_state,
        }
    }

    pub(crate) fn src_end(&self) -> usize {
        self.combined.len()
    }
}
