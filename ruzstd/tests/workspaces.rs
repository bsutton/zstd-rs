use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
};

use zstd_complete::{
    decoding::{DecoderWorkspace, StaticDecoderWorkspace},
    encoding::{CompressionLevel, EncoderWorkspace, StaticEncoderWorkspace},
};

struct CountingAllocator;

thread_local! {
    static COUNT_ALLOCATIONS: Cell<bool> = const { Cell::new(false) };
    static ALLOCATION_COUNT: Cell<usize> = const { Cell::new(0) };
}

static RECORDED_ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);
static RECORDED_ALLOCATION_SIZES: [AtomicUsize; 32] = [const { AtomicUsize::new(0) }; 32];

fn record_allocation(size: usize) {
    let index = RECORDED_ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
    if let Some(slot) = RECORDED_ALLOCATION_SIZES.get(index) {
        slot.store(size, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        COUNT_ALLOCATIONS.with(|enabled| {
            if enabled.get() {
                ALLOCATION_COUNT.with(|count| count.set(count.get() + 1));
                record_allocation(layout.size());
            }
        });
        // SAFETY: this allocator delegates the original request unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: the allocation was obtained from `System` above.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        COUNT_ALLOCATIONS.with(|enabled| {
            if enabled.get() {
                ALLOCATION_COUNT.with(|count| count.set(count.get() + 1));
                record_allocation(new_size);
            }
        });
        // SAFETY: this allocator delegates the original request unchanged.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn allocation_count(operation: impl FnOnce()) -> usize {
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(false));
    ALLOCATION_COUNT.with(|count| count.set(0));
    RECORDED_ALLOCATION_COUNT.store(0, Ordering::Relaxed);
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(true));
    operation();
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(false));
    ALLOCATION_COUNT.with(Cell::get)
}

fn recorded_allocation_sizes() -> Vec<usize> {
    let count = RECORDED_ALLOCATION_COUNT.load(Ordering::Relaxed);
    RECORDED_ALLOCATION_SIZES[..count.min(RECORDED_ALLOCATION_SIZES.len())]
        .iter()
        .map(|size| size.load(Ordering::Relaxed))
        .collect()
}

fn representative_input() -> Vec<u8> {
    let mut input = Vec::with_capacity(384 * 1024);
    let mut random = 0x9e37_79b9_u32;
    for index in 0..384 * 1024 {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        let structured = ((index / 97) % 251) as u8;
        input.push(if index % 19 == 0 {
            random as u8
        } else {
            structured
        });
    }
    input
}

#[test]
fn prepared_encoder_operations_do_not_allocate_at_representative_levels() {
    let input = representative_input();
    for level in [
        CompressionLevel::UNCOMPRESSED,
        CompressionLevel::fast(64).unwrap(),
        CompressionLevel::new(1).unwrap(),
        CompressionLevel::new(3).unwrap(),
        CompressionLevel::new(5).unwrap(),
        CompressionLevel::new(8).unwrap(),
        CompressionLevel::new(16).unwrap(),
        CompressionLevel::new(22).unwrap(),
    ] {
        let mut workspace = EncoderWorkspace::new(level, input.len()).unwrap();
        let mut output = vec![0_u8; EncoderWorkspace::required_output_size(input.len()).unwrap()];
        let mut encoded_len = 0;
        let allocations = allocation_count(|| {
            for _ in 0..2 {
                encoded_len = workspace.encode_into(&input, &mut output).unwrap().len();
            }
        });
        assert_eq!(
            allocations,
            0,
            "level {} allocated {:?}",
            level.get(),
            recorded_allocation_sizes()
        );

        let decoded = zstd::bulk::decompress(&output[..encoded_len], input.len()).unwrap();
        assert_eq!(decoded, input);
    }
}

#[test]
#[ignore = "large level-22 workspace exercises automatic long-distance matching"]
fn large_level22_ldm_workspace_does_not_allocate() {
    let maximum_input = (1 << 26) + 1;
    let level = CompressionLevel::new(22).unwrap();
    let mut input = Vec::with_capacity(512 * 1024);
    let mut random = 0x6a09_e667_u32;
    for _ in 0..256 * 1024 {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        input.push(random as u8);
    }
    input.extend_from_within(..);

    let mut workspace = EncoderWorkspace::new(level, maximum_input).unwrap();
    let mut output = vec![0_u8; EncoderWorkspace::required_output_size(input.len()).unwrap()];
    let mut encoded_len = 0;
    let allocations = allocation_count(|| {
        for _ in 0..2 {
            encoded_len = workspace.encode_into(&input, &mut output).unwrap().len();
        }
    });
    assert_eq!(
        allocations,
        0,
        "level-22 LDM allocated {:?}",
        recorded_allocation_sizes()
    );
    let decoded = zstd::bulk::decompress(&output[..encoded_len], input.len()).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn caller_byte_buffers_support_allocation_free_roundtrip() {
    let input = representative_input();
    let level = CompressionLevel::DEFAULT;
    let encoder_size = EncoderWorkspace::required_size(level, input.len()).unwrap();
    let mut encoder_storage = vec![0_u8; encoder_size];
    let mut encoded = vec![0_u8; EncoderWorkspace::required_output_size(input.len()).unwrap()];
    let mut encoder =
        StaticEncoderWorkspace::new(&mut encoder_storage, level, input.len()).unwrap();
    let mut encoded_len = 0;
    let allocations = allocation_count(|| {
        encoded_len = encoder.encode_into(&input, &mut encoded).unwrap().len();
    });
    assert_eq!(
        allocations,
        0,
        "allocated {:?}",
        recorded_allocation_sizes()
    );

    let window_size = 1 << 20;
    let decoder_size = StaticDecoderWorkspace::required_size(window_size, 0).unwrap();
    let mut decoder_storage = vec![0_u8; decoder_size];
    let mut decoded = vec![0_u8; input.len()];
    let mut decoder = StaticDecoderWorkspace::new(&mut decoder_storage, window_size, 0).unwrap();
    let mut decoded_len = 0;
    assert_eq!(
        allocation_count(|| {
            decoded_len = decoder
                .decode_into(&encoded[..encoded_len], &mut decoded)
                .unwrap();
        }),
        0
    );
    assert_eq!(&decoded[..decoded_len], input);
}

#[test]
fn owned_decoder_operation_does_not_allocate() {
    let input = representative_input();
    let encoded = zstd::bulk::compress(&input, 8).unwrap();
    let mut output = vec![0_u8; input.len()];
    let mut decoder = DecoderWorkspace::new(1 << 20, 0).unwrap();
    let mut decoded_len = 0;
    assert_eq!(
        allocation_count(|| {
            decoded_len = decoder.decode_into(&encoded, &mut output).unwrap();
        }),
        0
    );
    assert_eq!(&output[..decoded_len], input);
}

#[test]
fn prepared_decoder_dictionaries_do_not_allocate() {
    let dictionary = include_bytes!("../dict_tests/dictionary");
    let encoded = include_bytes!("../dict_tests/files/debug-shell.service.zst");
    let expected = include_bytes!("../dict_tests/files/debug-shell.service");
    let window_size = 1 << 20;
    let workspace_size =
        StaticDecoderWorkspace::required_size(window_size, dictionary.len()).unwrap();
    let mut storage = vec![0_u8; workspace_size];
    let mut output = vec![0_u8; expected.len()];
    let mut decoder =
        StaticDecoderWorkspace::new(&mut storage, window_size, dictionary.len()).unwrap();
    let mut decoded_len = 0;
    let allocations = allocation_count(|| {
        for _ in 0..2 {
            decoded_len = decoder
                .decode_into_with_dictionary(encoded, dictionary, &mut output)
                .unwrap();
        }
    });
    assert_eq!(
        allocations,
        0,
        "formatted dictionary allocated {:?}",
        recorded_allocation_sizes()
    );
    assert_eq!(&output[..decoded_len], expected);

    let raw_dictionary = b"alpha beta gamma delta repeated record prefix";
    let raw_input = b"alpha beta gamma delta repeated record prefix: one; alpha beta gamma delta repeated record prefix: two";
    let mut compressor = zstd::bulk::Compressor::with_dictionary(3, raw_dictionary).unwrap();
    let raw_encoded = compressor.compress(raw_input).unwrap();
    let raw_workspace_size =
        StaticDecoderWorkspace::required_size(window_size, raw_dictionary.len()).unwrap();
    let mut raw_storage = vec![0_u8; raw_workspace_size];
    let mut raw_output = vec![0_u8; raw_input.len()];
    let mut raw_decoder =
        StaticDecoderWorkspace::new(&mut raw_storage, window_size, raw_dictionary.len()).unwrap();
    let mut raw_decoded_len = 0;
    let allocations = allocation_count(|| {
        for _ in 0..2 {
            raw_decoded_len = raw_decoder
                .decode_into_with_raw_dictionary(&raw_encoded, raw_dictionary, &mut raw_output)
                .unwrap();
        }
    });
    assert_eq!(
        allocations,
        0,
        "raw dictionary allocated {:?}",
        recorded_allocation_sizes()
    );
    assert_eq!(&raw_output[..raw_decoded_len], raw_input);
}

#[test]
fn capacity_queries_and_errors_are_deterministic() {
    let level = CompressionLevel::DEFAULT;
    let required = EncoderWorkspace::required_size(level, 4096).unwrap();
    let mut too_small = vec![0_u8; required - 1];
    assert!(StaticEncoderWorkspace::new(&mut too_small, level, 4096).is_err());
    assert_eq!(
        EncoderWorkspace::required_output_size(usize::MAX),
        Err(zstd_complete::encoding::EncoderWorkspaceError::SizeOverflow)
    );

    let decoder_required = StaticDecoderWorkspace::required_size(1 << 18, 0).unwrap();
    let mut decoder_too_small = vec![0_u8; decoder_required - 1];
    assert!(StaticDecoderWorkspace::new(&mut decoder_too_small, 1 << 18, 0).is_err());
}
