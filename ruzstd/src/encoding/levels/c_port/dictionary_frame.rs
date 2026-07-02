//! Shared setup for C-style dictionary frame encoders.

use alloc::vec::Vec;

use super::{
    c_frame_header::write_frame_header, cctx_params::CctxParameters, dictionary::ParsedDictionary,
    frame_state::FrameBlockState, params::CParamMode,
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
        let original_dict_len = dictionary.content.len();
        let cctx = CctxParameters::for_level_with_mode(
            level,
            src.len() as u64,
            original_dict_len,
            CParamMode::NoAttachDict,
        );
        cctx.assert_resolved();
        let params = cctx.compression;
        let loaded_dictionary = loaded_dictionary_content(params, dictionary.content);
        let dict_len = loaded_dictionary.len();
        let mut combined = Vec::with_capacity(dict_len + src.len());
        combined.extend_from_slice(loaded_dictionary);
        combined.extend_from_slice(src);

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

fn loaded_dictionary_content(params: super::params::CompressionParameters, dict: &[u8]) -> &[u8] {
    let table_limited_size = 1_usize << (params.hash_log + 3).max(params.chain_log + 1).min(31);
    if dict.len() > table_limited_size {
        &dict[dict.len() - table_limited_size..]
    } else {
        dict
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::levels::c_port::dictionary::{parse_dictionary, DictionaryContentType};

    #[test]
    fn large_dictionary_loads_only_suffix_like_c() {
        let dictionary = (0..(256 * 1024 + 1024))
            .map(|value| value as u8)
            .collect::<Vec<_>>();
        let parsed = parse_dictionary(&dictionary, DictionaryContentType::Auto, false)
            .unwrap()
            .expect("raw dictionary");

        let context = DictionaryFrameContext::new(b"payload", 1, parsed);

        assert_eq!(context.dict_len, 128 * 1024);
        assert_eq!(
            &context.combined[..context.dict_len],
            &dictionary[dictionary.len() - context.dict_len..]
        );
        assert_eq!(&context.combined[context.dict_len..], b"payload");
    }
}
