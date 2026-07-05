use std::{
    cmp::Ordering,
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use ruzstd::encoding::compress_slice_c_level;
use zstd_rs_tools::{
    benchmark_tmp, has_flag, parse_value, require_value, run_command_silent, verify_decoded_matches,
};

const ZSTD_MAGIC: u32 = 0xfd2f_b528;

#[derive(Debug)]
struct Args {
    input: PathBuf,
    level: i32,
    zstd_bin: PathBuf,
    output_dir: PathBuf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BlockType {
    Raw,
    Rle,
    Compressed,
    Reserved,
}

#[derive(Clone, Debug)]
struct BlockInfo {
    index: usize,
    offset: usize,
    block_type: BlockType,
    last: bool,
    content_size: usize,
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

fn inspect_frame(encoded: &[u8]) -> io::Result<Vec<BlockInfo>> {
    let mut offset = frame_header_size(encoded)?;
    let mut blocks = Vec::new();
    loop {
        if offset + 3 > encoded.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "missing block header",
            ));
        }
        let header_offset = offset;
        let raw = u32::from(encoded[offset])
            | (u32::from(encoded[offset + 1]) << 8)
            | (u32::from(encoded[offset + 2]) << 16);
        offset += 3;

        let last = (raw & 1) != 0;
        let block_type = match (raw >> 1) & 0b11 {
            0 => BlockType::Raw,
            1 => BlockType::Rle,
            2 => BlockType::Compressed,
            _ => BlockType::Reserved,
        };
        let block_size = (raw >> 3) as usize;
        let content_size = match block_type {
            BlockType::Raw | BlockType::Compressed => block_size,
            BlockType::Rle => 1,
            BlockType::Reserved => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "reserved block type",
                ))
            }
        };
        if offset + content_size > encoded.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "truncated block payload",
            ));
        }
        blocks.push(BlockInfo {
            index: blocks.len(),
            offset: header_offset,
            block_type,
            last,
            content_size,
        });
        offset += content_size;
        if last {
            break;
        }
    }
    Ok(blocks)
}

fn frame_header_size(encoded: &[u8]) -> io::Result<usize> {
    if encoded.len() < 5 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "missing frame header",
        ));
    }
    let magic = u32::from_le_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]);
    if magic != ZSTD_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected zstd magic: {magic:#x}"),
        ));
    }

    let descriptor = encoded[4];
    let single_segment = (descriptor & 0b0010_0000) != 0;
    let dict_id_len = match descriptor & 0b0000_0011 {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 4,
    };
    let fcs_len = match descriptor >> 6 {
        0 if single_segment => 1,
        0 => 0,
        1 => 2,
        2 => 4,
        _ => 8,
    };
    let header_size = 5 + usize::from(!single_segment) + dict_id_len + fcs_len;
    if header_size > encoded.len() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "truncated frame header",
        ));
    }
    Ok(header_size)
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
    for block in blocks.iter().take(24) {
        println!(
            "{label},{},{},{:?},{},{}",
            block.index, block.offset, block.block_type, block.last, block.content_size
        );
    }
    if blocks.len() > 24 {
        println!("{label}: ... {} more blocks", blocks.len() - 24);
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
