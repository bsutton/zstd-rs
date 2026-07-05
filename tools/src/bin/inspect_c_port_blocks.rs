use std::{
    cmp::Ordering,
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use ruzstd::encoding::compress_slice_c_level;
use zstd_rs_tools::{
    benchmark_tmp,
    block_inspect::{
        inspect_frame_with_decoded_sizes, BlockInfo, BlockType, CompressedSectionInfo,
        LiteralSectionType, SequenceMode,
    },
    has_flag, parse_value, require_value, run_command_silent, verify_decoded_matches,
};
use zstd_safe::{CCtx, CParameter};

#[derive(Debug)]
struct Args {
    input: PathBuf,
    level: i32,
    zstd_bin: PathBuf,
    c_backend: CBackend,
    c_mode: CMode,
    output_dir: PathBuf,
    block_limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CMode {
    SingleThread,
    T1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CBackend {
    Cli,
    Api,
}

#[derive(Clone, Debug)]
struct BlockDelta {
    index: usize,
    delta: i64,
    abs_delta: usize,
    rust: BlockInfo,
    c: BlockInfo,
}

fn main() -> io::Result<()> {
    let args = parse_args()?;
    fs::create_dir_all(&args.output_dir)?;

    let stem = args
        .input
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("input");
    let rust_output = args
        .output_dir
        .join(format!("{stem}.l{}.rust.zst", args.level));
    let c_output = args
        .output_dir
        .join(format!("{stem}.l{}.c.zst", args.level));

    let input = fs::read(&args.input)?;
    fs::write(&rust_output, compress_slice_c_level(&input, args.level))?;
    verify_decoded_matches(&args.zstd_bin, &rust_output, &args.input)?;

    write_c_reference(
        args.c_backend,
        &args.zstd_bin,
        args.c_mode,
        args.level,
        &input,
        &args.input,
        &c_output,
    )?;
    verify_decoded_matches(&args.zstd_bin, &c_output, &args.input)?;

    let rust = inspect_frame_with_decoded_sizes(&fs::read(&rust_output)?)?;
    let c = inspect_frame_with_decoded_sizes(&fs::read(&c_output)?)?;

    println!(
        "input={} level={} c_backend={} rust_bytes={} c_bytes={}",
        args.input.display(),
        args.level,
        args.c_backend.description(args.c_mode),
        fs::metadata(&rust_output)?.len(),
        fs::metadata(&c_output)?.len()
    );
    print_blocks("rust", &rust, args.block_limit);
    print_blocks("c", &c, args.block_limit);
    print_comparison(&rust, &c);

    Ok(())
}

fn parse_args() -> io::Result<Args> {
    let raw = env::args().skip(1).collect::<Vec<_>>();
    if has_flag(&raw, "--help") || has_flag(&raw, "-h") {
        print_help();
        std::process::exit(0);
    }

    let input = PathBuf::from(require_value(&raw, "--input")?);
    let level = parse_value(&raw, "--level", "5")
        .parse::<i32>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    Ok(Args {
        input,
        level,
        zstd_bin: PathBuf::from(parse_value(&raw, "--zstd-bin", "/usr/bin/zstd")),
        c_backend: parse_c_backend(&parse_value(&raw, "--c-backend", "cli"))?,
        c_mode: parse_c_mode(&parse_value(&raw, "--c-mode", "single-thread"))?,
        output_dir: PathBuf::from(parse_value(
            &raw,
            "--output-dir",
            benchmark_tmp()
                .join("c-port-block-inspect")
                .display()
                .to_string(),
        )),
        block_limit: parse_usize(&parse_value(&raw, "--block-limit", "24"))?,
    })
}

fn print_help() {
    println!(
        "\
Usage: inspect_c_port_blocks --input FILE [--level N] [--zstd-bin PATH] [--output-dir DIR]

Options:
  --input FILE      Input fixture to compress and inspect.
  --level N         Compression level, default 5.
  --zstd-bin PATH   Path to the C zstd binary.
  --c-backend MODE  C reference backend: cli or api. Default cli.
  --c-mode MODE     C zstd mode: single-thread or t1. Default single-thread.
                    Only used when --c-backend cli.
  --output-dir DIR  Directory for generated .zst files.
  --block-limit N   Number of leading blocks to print for each output, default 24.
  -h, --help        Show this help message."
    );
}

fn parse_c_backend(raw: &str) -> io::Result<CBackend> {
    match raw {
        "cli" => Ok(CBackend::Cli),
        "api" => Ok(CBackend::Api),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported --c-backend {raw:?}; expected cli or api"),
        )),
    }
}

fn parse_c_mode(raw: &str) -> io::Result<CMode> {
    match raw {
        "single-thread" => Ok(CMode::SingleThread),
        "t1" | "T1" => Ok(CMode::T1),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported --c-mode {raw:?}; expected single-thread or t1"),
        )),
    }
}

fn parse_usize(raw: &str) -> io::Result<usize> {
    raw.parse()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
}

fn write_c_reference(
    backend: CBackend,
    zstd_bin: &Path,
    mode: CMode,
    level: i32,
    input: &[u8],
    input_path: &Path,
    output: &Path,
) -> io::Result<()> {
    match backend {
        CBackend::Cli => run_c_zstd(zstd_bin, mode, level, input_path, output),
        CBackend::Api => {
            let compressed = compress_c_api(input, level)?;
            fs::write(output, compressed)
        }
    }
}

fn compress_c_api(input: &[u8], level: i32) -> io::Result<Vec<u8>> {
    let mut context = CCtx::create();
    set_c_api_parameter(&mut context, CParameter::CompressionLevel(level))?;
    set_c_api_parameter(&mut context, CParameter::ChecksumFlag(false))?;
    context
        .set_pledged_src_size(Some(input.len() as u64))
        .map_err(c_api_error)?;
    let mut output = Vec::with_capacity(zstd_safe::compress_bound(input.len()));
    context.compress2(&mut output, input).map_err(c_api_error)?;
    Ok(output)
}

fn set_c_api_parameter(context: &mut CCtx<'_>, parameter: CParameter) -> io::Result<()> {
    context
        .set_parameter(parameter)
        .map(|_| ())
        .map_err(c_api_error)
}

fn c_api_error(code: usize) -> io::Error {
    io::Error::other(format!(
        "zstd C API error {code}: {}",
        zstd_safe::get_error_name(code)
    ))
}

fn run_c_zstd(
    zstd_bin: &Path,
    mode: CMode,
    level: i32,
    input: &Path,
    output: &Path,
) -> io::Result<()> {
    let mut command = Command::new(zstd_bin);
    command.args(["-q", "-f"]);
    command.args(mode.zstd_args());
    command.arg("--no-check");
    command.args(zstd_cli_level_args(level));
    command.arg(input).arg("-o").arg(output);
    run_command_silent(&mut command)
}

impl CBackend {
    fn description(self, mode: CMode) -> String {
        match self {
            Self::Cli => format!("C zstd CLI {}", mode.description()),
            Self::Api => "C ZSTD_compress2() API".to_string(),
        }
    }
}

impl CMode {
    fn zstd_args(self) -> &'static [&'static str] {
        match self {
            Self::SingleThread => &["--single-thread"],
            Self::T1 => &["-T1"],
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::SingleThread => "--single-thread",
            Self::T1 => "-T1",
        }
    }
}

fn zstd_cli_level_args(level: i32) -> Vec<String> {
    match level.cmp(&0) {
        Ordering::Less => vec![format!("--fast={}", level.unsigned_abs())],
        Ordering::Equal => Vec::new(),
        Ordering::Greater if level > 19 => vec!["--ultra".to_string(), format!("-{level}")],
        Ordering::Greater => vec![format!("-{level}")],
    }
}

fn print_blocks(label: &str, blocks: &[BlockInfo], block_limit: usize) {
    let compressed_bytes = blocks
        .iter()
        .map(|block| block.content_size + 3)
        .sum::<usize>();
    let (raw, rle, compressed) = block_type_counts(blocks);
    println!(
        "{label}: blocks={} block_bytes={compressed_bytes} raw={raw} rle={rle} compressed={compressed}",
        blocks.len(),
    );
    print_section_summary(label, blocks);
    for block in blocks.iter().take(block_limit) {
        println!(
            "{label},{},{},{:?},{},{},{},{}{}",
            block.index,
            block.offset,
            block.block_type,
            block.last,
            block.content_size,
            source_offset_label(block),
            decompressed_size_label(block),
            section_suffix(block.section_info.as_ref())
        );
    }
    if blocks.len() > block_limit {
        println!("{label}: ... {} more blocks", blocks.len() - block_limit);
    }
}

fn print_section_summary(label: &str, blocks: &[BlockInfo]) {
    let mut compressed_blocks = 0usize;
    let mut total_sequences = 0usize;
    let mut regenerated_literals = 0usize;
    let mut literal_payload = 0usize;
    let mut literal_raw = 0usize;
    let mut literal_rle = 0usize;
    let mut literal_compressed = 0usize;
    let mut literal_treeless = 0usize;
    let mut ll_modes = [0usize; 4];
    let mut of_modes = [0usize; 4];
    let mut ml_modes = [0usize; 4];

    for info in blocks
        .iter()
        .filter_map(|block| block.section_info.as_ref())
    {
        compressed_blocks += 1;
        total_sequences += info.sequences;
        regenerated_literals += info.literal_regenerated_size;
        literal_payload += info.literal_payload_size;
        match info.literal_type {
            LiteralSectionType::Raw => literal_raw += 1,
            LiteralSectionType::Rle => literal_rle += 1,
            LiteralSectionType::Compressed => literal_compressed += 1,
            LiteralSectionType::Treeless => literal_treeless += 1,
        }
        if let Some(mode) = info.ll_mode {
            ll_modes[mode_index(mode)] += 1;
        }
        if let Some(mode) = info.of_mode {
            of_modes[mode_index(mode)] += 1;
        }
        if let Some(mode) = info.ml_mode {
            ml_modes[mode_index(mode)] += 1;
        }
    }

    if compressed_blocks == 0 {
        return;
    }
    println!(
        "{label}: compressed_sections={compressed_blocks} total_sequences={total_sequences} regenerated_literals={regenerated_literals} literal_payload={literal_payload} literal_types=raw:{literal_raw}/rle:{literal_rle}/compressed:{literal_compressed}/treeless:{literal_treeless} ll_modes={} of_modes={} ml_modes={}",
        mode_counts(ll_modes),
        mode_counts(of_modes),
        mode_counts(ml_modes),
    );
}

fn mode_index(mode: SequenceMode) -> usize {
    match mode {
        SequenceMode::Predefined => 0,
        SequenceMode::Rle => 1,
        SequenceMode::FseCompressed => 2,
        SequenceMode::Repeat => 3,
    }
}

fn mode_counts(counts: [usize; 4]) -> String {
    format!(
        "pre:{}/rle:{}/fse:{}/rep:{}",
        counts[0], counts[1], counts[2], counts[3]
    )
}

fn section_suffix(info: Option<&CompressedSectionInfo>) -> String {
    let Some(info) = info else {
        return String::new();
    };
    format!(
        ",lit={:?}/regen:{}/payload:{}/streams:{},seqs={},modes={}/{}/{}",
        info.literal_type,
        info.literal_regenerated_size,
        info.literal_payload_size,
        info.literal_streams
            .map(|streams| streams.to_string())
            .unwrap_or_else(|| "-".to_string()),
        info.sequences,
        mode_label(info.ll_mode),
        mode_label(info.of_mode),
        mode_label(info.ml_mode),
    )
}

fn mode_label(mode: Option<SequenceMode>) -> &'static str {
    match mode {
        Some(SequenceMode::Predefined) => "pre",
        Some(SequenceMode::Rle) => "rle",
        Some(SequenceMode::FseCompressed) => "fse",
        Some(SequenceMode::Repeat) => "rep",
        None => "-",
    }
}

fn print_comparison(rust: &[BlockInfo], c: &[BlockInfo]) {
    let common = rust.len().min(c.len());
    let common_content_delta = (0..common)
        .map(|idx| rust[idx].content_size as i64 - c[idx].content_size as i64)
        .sum::<i64>();
    let common_abs_content_delta = (0..common)
        .map(|idx| rust[idx].content_size.abs_diff(c[idx].content_size))
        .sum::<usize>();
    let common_source_delta = (0..common)
        .filter_map(|idx| {
            Some(rust[idx].decompressed_size? as i64 - c[idx].decompressed_size? as i64)
        })
        .sum::<i64>();
    let common_abs_source_delta = (0..common)
        .filter_map(|idx| {
            Some(
                rust[idx]
                    .decompressed_size?
                    .abs_diff(c[idx].decompressed_size?),
            )
        })
        .sum::<usize>();
    let type_diffs = (0..common)
        .filter(|&idx| rust[idx].block_type != c[idx].block_type)
        .count();
    let first_diff = (0..common).find(|&idx| {
        rust[idx].block_type != c[idx].block_type
            || rust[idx].content_size != c[idx].content_size
            || rust[idx].source_offset != c[idx].source_offset
            || rust[idx].decompressed_size != c[idx].decompressed_size
    });
    let first_source_diff = (0..common).find(|&idx| {
        rust[idx].source_offset != c[idx].source_offset
            || rust[idx].decompressed_size != c[idx].decompressed_size
    });
    println!(
        "summary: common_blocks={common} block_count_delta={} content_delta={} abs_content_delta={} source_delta={} abs_source_delta={} type_diffs={type_diffs}",
        rust.len() as isize - c.len() as isize,
        common_content_delta,
        common_abs_content_delta,
        common_source_delta,
        common_abs_source_delta,
    );
    match first_diff {
        Some(idx) => println!(
            "first_diff={idx} rust={:?}/{}/{}/{} c={:?}/{}/{}/{}",
            rust[idx].block_type,
            rust[idx].content_size,
            source_offset_label(&rust[idx]),
            decompressed_size_label(&rust[idx]),
            c[idx].block_type,
            c[idx].content_size,
            source_offset_label(&c[idx]),
            decompressed_size_label(&c[idx])
        ),
        None if rust.len() == c.len() => println!("first_diff=none"),
        None => println!("first_diff=block_count rust={} c={}", rust.len(), c.len()),
    }
    match first_source_diff {
        Some(idx) => println!(
            "first_source_diff={idx} rust={}/{} c={}/{}",
            source_offset_label(&rust[idx]),
            decompressed_size_label(&rust[idx]),
            source_offset_label(&c[idx]),
            decompressed_size_label(&c[idx])
        ),
        None if rust.len() == c.len() => println!("first_source_diff=none"),
        None => println!(
            "first_source_diff=block_count rust={} c={}",
            rust.len(),
            c.len()
        ),
    }
    print_largest_block_deltas(rust, c);
}

fn print_largest_block_deltas(rust: &[BlockInfo], c: &[BlockInfo]) {
    let common = rust.len().min(c.len());
    let mut deltas = (0..common)
        .filter_map(|index| {
            let rust_block = rust[index].clone();
            let c_block = c[index].clone();
            let delta = rust_block.content_size as i64 - c_block.content_size as i64;
            let source_delta = match (rust_block.decompressed_size, c_block.decompressed_size) {
                (Some(rust_size), Some(c_size)) => rust_size as i64 - c_size as i64,
                _ => 0,
            };
            let type_changed = rust_block.block_type != c_block.block_type;
            (delta != 0 || source_delta != 0 || type_changed).then(|| BlockDelta {
                index,
                delta,
                abs_delta: rust_block.content_size.abs_diff(c_block.content_size),
                rust: rust_block,
                c: c_block,
            })
        })
        .collect::<Vec<_>>();
    deltas.sort_by(|left, right| {
        right
            .abs_delta
            .cmp(&left.abs_delta)
            .then_with(|| left.index.cmp(&right.index))
    });

    println!("largest_deltas:");
    if deltas.is_empty() {
        println!("delta,none");
        return;
    }
    for delta in deltas.into_iter().take(12) {
        println!(
            "delta,{},{},{},{:?},{},{},{},{},{:?},{},{},{},{}{}",
            delta.index,
            delta.delta,
            source_delta(&delta.rust, &delta.c),
            delta.rust.block_type,
            delta.rust.content_size,
            source_offset_label(&delta.rust),
            decompressed_size_label(&delta.rust),
            describe_section(delta.rust.section_info.as_ref()),
            delta.c.block_type,
            delta.c.content_size,
            source_offset_label(&delta.c),
            decompressed_size_label(&delta.c),
            describe_section(delta.c.section_info.as_ref()),
            if delta.rust.block_type == delta.c.block_type {
                ""
            } else {
                ",type_changed"
            }
        );
    }
}

fn source_offset_label(block: &BlockInfo) -> String {
    block
        .source_offset
        .map(|offset| offset.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn decompressed_size_label(block: &BlockInfo) -> String {
    block
        .decompressed_size
        .map(|size| size.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn source_delta(rust: &BlockInfo, c: &BlockInfo) -> i64 {
    match (rust.decompressed_size, c.decompressed_size) {
        (Some(rust_size), Some(c_size)) => rust_size as i64 - c_size as i64,
        _ => 0,
    }
}

fn describe_section(info: Option<&CompressedSectionInfo>) -> String {
    let Some(info) = info else {
        return "-".to_string();
    };
    format!(
        "{:?}/regen:{}/payload:{}/seqs:{}/modes:{}/{}/{}",
        info.literal_type,
        info.literal_regenerated_size,
        info.literal_payload_size,
        info.sequences,
        mode_label(info.ll_mode),
        mode_label(info.of_mode),
        mode_label(info.ml_mode)
    )
}

fn block_type_counts(blocks: &[BlockInfo]) -> (usize, usize, usize) {
    let mut raw = 0;
    let mut rle = 0;
    let mut compressed = 0;
    for block in blocks {
        match block.block_type {
            BlockType::Raw => raw += 1,
            BlockType::Rle => rle += 1,
            BlockType::Compressed => compressed += 1,
            BlockType::Reserved => {}
        }
    }
    (raw, rle, compressed)
}
