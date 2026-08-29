#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate ruzstd;
use ruzstd::encoding::{encode_all, CompressionLevel, EncoderOptions};

fuzz_target!(|data: &[u8]| {
    for level in [
        CompressionLevel::UNCOMPRESSED,
        CompressionLevel::FASTEST,
        CompressionLevel::DEFAULT,
    ] {
        let options = EncoderOptions::new(level)
            .with_frame_chunk_size(1024)
            .with_memory_limit(usize::MAX);
        let output = encode_all(data, options).unwrap();

        let mut decoded = Vec::with_capacity(data.len());
        let mut decoder = ruzstd::decoding::FrameDecoder::new();
        decoder.decode_all_to_vec(&output, &mut decoded).unwrap();
        assert_eq!(data, decoded);

        let decoded_by_c = zstd::decode_all(output.as_slice()).unwrap();
        assert_eq!(data, decoded_by_c);
    }
});
