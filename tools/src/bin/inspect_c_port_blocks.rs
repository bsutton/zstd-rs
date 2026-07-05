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
        inspect_frame, BlockInfo, BlockType, CompressedSectionInfo, LiteralSectionType,
        SequenceMode,
    },
    has_flag, parse_value, require_value, run_command_silent, verify_decoded_matches,
};

#[derive(Debug)]
struct Args {
    input: PathBuf,
    level: i32,
    zstd_bin: PathBuf,
    output_dir: PathBuf,
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

    run_c_zstd(&args.zstd_bin, args.level, &args.input, &c_output)?;
    verify_decoded_matches(&args.zstd_bin, &c_output, &args.input)?;

    let rust = inspect_frame(&fs::read(&rust_output)?)?;
    let c = inspect_frame(&fs::read(&c_output)?)?;

    println!(
        "input={} level={} rust_bytes={} c_bytes={}",
        args.input.display(),
        args.level,
        fs::metadata(&rust_output)?.len(),
        fs::metadata(&c_output)?.len()
    );
    print_blocks("rust", &rust);
    print_blocks("c", &c);
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
        output_dir: PathBuf::from(parse_value(
            &raw,
            "--output-dir",
            benchmark_tmp()
                .join("c-port-block-inspect")
                .display()
                .to_string(),
        )),
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
  --output-dir DIR  Directory for generated .zst files.
  -h, --help        Show this help message."
    );
}

fn run_c_zstd(zstd_bin: &Path, level: i32, input: &Path, output: &Path) -> io::Result<()> {
    let mut command = Command::new(zstd_bin);
    command.args(["-q", "-f", "--single-thread", "--no-check"]);
    if let Some(level_arg) = zstd_cli_level_arg(level) {
        command.arg(level_arg);
    }
    command.arg(input).arg("-o").arg(output);
    run_command_silent(&mut command)
}

fn zstd_cli_level_arg(level: i32) -> Option<String> {
    match level.cmp(&0) {
        Ordering::Less => Some(format!("--fast={}", level.unsigned_abs())),
        Ordering::Equal => None,
        Ordering::Greater => Some(format!("-{level}")),
    }
}

fn print_blocks(label: &str, blocks: &[BlockInfo]) {
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
    for block in blocks.iter().take(24) {
        println!(
            "{label},{},{},{:?},{},{}{}",
            block.index,
            block.offset,
            block.block_type,
            block.last,
            block.content_size,
            section_suffix(block.section_info.as_ref())
        );
    }
    if blocks.len() > 24 {
        println!("{label}: ... {} more blocks", blocks.len() - 24);
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
    let type_diffs = (0..common)
        .filter(|&idx| rust[idx].block_type != c[idx].block_type)
        .count();
    let first_diff = (0..common).find(|&idx| {
        rust[idx].block_type != c[idx].block_type || rust[idx].content_size != c[idx].content_size
    });
    println!(
        "summary: common_blocks={common} block_count_delta={} content_delta={} abs_content_delta={} type_diffs={type_diffs}",
        rust.len() as isize - c.len() as isize,
        common_content_delta,
        common_abs_content_delta,
    );
    match first_diff {
        Some(idx) => println!(
            "first_diff={idx} rust={:?}/{} c={:?}/{}",
            rust[idx].block_type, rust[idx].content_size, c[idx].block_type, c[idx].content_size
        ),
        None if rust.len() == c.len() => println!("first_diff=none"),
        None => println!("first_diff=block_count rust={} c={}", rust.len(), c.len()),
    }
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
