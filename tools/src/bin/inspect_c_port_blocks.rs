#[path = "inspect_c_port_blocks/comparison.rs"]
mod comparison;

use std::{
    cmp::Ordering,
    env, fs, io,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use ruzstd::encoding::{
    compress_slice_c_level, compress_slice_c_level_with_dictionary,
    compress_slice_c_level_with_dictionary_and_target_c_block_size,
    compress_slice_c_level_with_prepared_dictionary,
    compress_slice_c_level_with_target_c_block_size, CLevelEncoderDictionary,
};
use zstd_rs_tools::{
    benchmark_tmp,
    block_inspect::{
        inspect_frame_with_decoded_sizes_and_dictionary, BlockInfo, BlockType,
        CompressedSectionInfo, LiteralSectionType, SequenceMode,
    },
    has_flag, parse_value, require_value, run_command_silent, verify_decoded_matches,
};
use zstd_safe::{CCtx, CDict, CParameter};

use comparison::{
    decompressed_size_label, mode_label, print_comparison, source_offset_label,
    write_source_aligned_csv,
};

#[derive(Debug)]
struct Args {
    input: PathBuf,
    level: i32,
    zstd_bin: PathBuf,
    c_backend: CBackend,
    c_mode: CMode,
    output_dir: PathBuf,
    block_limit: usize,
    target_c_block_size: Option<usize>,
    dictionary: Option<PathBuf>,
    prepared_dictionary: bool,
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

struct PreparedDictionaryReferences {
    rust: CLevelEncoderDictionary,
    c: CDict<'static>,
}

impl PreparedDictionaryReferences {
    fn new(dictionary: &[u8], level: i32) -> io::Result<Self> {
        let rust = CLevelEncoderDictionary::copy(dictionary, level).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Rust dictionary preparation failed: {error:?}"),
            )
        })?;
        let c = CDict::try_create(dictionary, level).ok_or_else(|| {
            io::Error::other("C ZSTD_createCDict() failed to allocate or prepare the dictionary")
        })?;
        Ok(Self { rust, c })
    }
}

fn main() -> io::Result<()> {
    let args = parse_args()?;
    fs::create_dir_all(&args.output_dir)?;

    let stem = args
        .input
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("input");
    let rust_output = args.output_dir.join(format!(
        "{stem}.l{}{}.rust.zst",
        args.level,
        mode_suffix(
            args.target_c_block_size,
            args.dictionary.as_deref(),
            args.prepared_dictionary,
        )
    ));
    let c_output = args.output_dir.join(format!(
        "{stem}.l{}{}.c.zst",
        args.level,
        mode_suffix(
            args.target_c_block_size,
            args.dictionary.as_deref(),
            args.prepared_dictionary,
        )
    ));

    let input = fs::read(&args.input)?;
    let dictionary = args.dictionary.as_ref().map(fs::read).transpose()?;
    let prepared_dictionary = if args.prepared_dictionary {
        Some(PreparedDictionaryReferences::new(
            dictionary
                .as_deref()
                .expect("argument validation requires a dictionary"),
            args.level,
        )?)
    } else {
        None
    };
    let rust_compressed = compress_rust_reference(
        &input,
        args.level,
        args.target_c_block_size,
        dictionary.as_deref(),
        prepared_dictionary.as_ref(),
    )?;
    fs::write(&rust_output, rust_compressed)?;
    verify_decoded_matches_with_dictionary(
        &args.zstd_bin,
        &rust_output,
        &args.input,
        args.dictionary.as_deref(),
    )?;

    write_c_reference(
        &args,
        &input,
        dictionary.as_deref(),
        prepared_dictionary.as_ref(),
        &c_output,
    )?;
    verify_decoded_matches_with_dictionary(
        &args.zstd_bin,
        &c_output,
        &args.input,
        args.dictionary.as_deref(),
    )?;

    let rust = inspect_frame_with_decoded_sizes_and_dictionary(
        &fs::read(&rust_output)?,
        dictionary.as_deref(),
    )?;
    let c = inspect_frame_with_decoded_sizes_and_dictionary(
        &fs::read(&c_output)?,
        dictionary.as_deref(),
    )?;

    println!(
        "input={} level={} c_backend={} rust_bytes={} c_bytes={}",
        args.input.display(),
        args.level,
        args.c_backend
            .description(args.c_mode, args.prepared_dictionary),
        fs::metadata(&rust_output)?.len(),
        fs::metadata(&c_output)?.len()
    );
    print_blocks("rust", &rust, args.block_limit);
    print_blocks("c", &c, args.block_limit);
    print_comparison(&rust, &c);
    write_blocks_csv(&args.output_dir.join("rust.blocks.csv"), &rust)?;
    write_blocks_csv(&args.output_dir.join("c.blocks.csv"), &c)?;
    write_source_aligned_csv(&args.output_dir.join("source-aligned.csv"), &rust, &c)?;

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
    let c_backend = parse_c_backend(&parse_value(&raw, "--c-backend", "cli"))?;
    let target_c_block_size = parse_optional_usize(&raw, "--target-c-block-size")?;
    let dictionary = parse_optional_path(&raw, "--dictionary");
    let prepared_dictionary = has_flag(&raw, "--prepared-dictionary");
    if target_c_block_size.is_some() && c_backend != CBackend::Api {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--target-c-block-size requires --c-backend api",
        ));
    }
    if prepared_dictionary && c_backend != CBackend::Api {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--prepared-dictionary requires --c-backend api",
        ));
    }
    if prepared_dictionary && dictionary.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--prepared-dictionary requires --dictionary",
        ));
    }
    if prepared_dictionary && target_c_block_size.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--prepared-dictionary cannot be combined with --target-c-block-size",
        ));
    }

    Ok(Args {
        input,
        level,
        zstd_bin: PathBuf::from(parse_value(&raw, "--zstd-bin", "/usr/bin/zstd")),
        c_backend,
        c_mode: parse_c_mode(&parse_value(&raw, "--c-mode", "single-thread"))?,
        target_c_block_size,
        dictionary,
        prepared_dictionary,
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
  --target-c-block-size N
                    Set C targetCBlockSize. Requires --c-backend api.
  --dictionary PATH Compress and decode with a zstd dictionary.
  --prepared-dictionary
                    Compare Rust's prepared dictionary with C ZSTD_CDict.
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

fn parse_optional_usize(raw: &[String], name: &str) -> io::Result<Option<usize>> {
    if raw.iter().any(|arg| arg == name) {
        parse_value(raw, name, "")
            .parse::<usize>()
            .map(Some)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
    } else {
        Ok(None)
    }
}

fn parse_optional_path(raw: &[String], name: &str) -> Option<PathBuf> {
    raw.iter()
        .position(|arg| arg == name)
        .and_then(|index| raw.get(index + 1))
        .map(PathBuf::from)
}

fn mode_suffix(
    target_c_block_size: Option<usize>,
    dictionary: Option<&Path>,
    prepared_dictionary: bool,
) -> String {
    let mut suffix = String::new();
    if let Some(target) = target_c_block_size {
        suffix.push_str(&format!(".target{target}"));
    }
    if dictionary.is_some() {
        suffix.push_str(".dict");
    }
    if prepared_dictionary {
        suffix.push_str(".prepared");
    }
    suffix
}

fn compress_rust_reference(
    input: &[u8],
    level: i32,
    target_c_block_size: Option<usize>,
    dictionary: Option<&[u8]>,
    prepared_dictionary: Option<&PreparedDictionaryReferences>,
) -> io::Result<Vec<u8>> {
    if let Some(prepared) = prepared_dictionary {
        return Ok(compress_slice_c_level_with_prepared_dictionary(
            input,
            &prepared.rust,
        ));
    }
    if let Some(dictionary) = dictionary {
        if let Some(target_c_block_size) = target_c_block_size {
            return compress_slice_c_level_with_dictionary_and_target_c_block_size(
                input,
                level,
                dictionary,
                target_c_block_size,
            )
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Rust dictionary target compression failed: {err:?}"),
                )
            })?
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "dictionary targetCBlockSize is outside C's accepted range",
                )
            });
        }
        return compress_slice_c_level_with_dictionary(input, level, dictionary).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Rust dictionary compression failed: {err:?}"),
            )
        });
    }

    if let Some(target_c_block_size) = target_c_block_size {
        return compress_slice_c_level_with_target_c_block_size(input, level, target_c_block_size)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "targetCBlockSize is outside C's accepted range",
                )
            });
    }

    Ok(compress_slice_c_level(input, level))
}

fn write_c_reference(
    args: &Args,
    input: &[u8],
    dictionary: Option<&[u8]>,
    prepared_dictionary: Option<&PreparedDictionaryReferences>,
    output: &Path,
) -> io::Result<()> {
    match args.c_backend {
        CBackend::Cli => {
            if args.target_c_block_size.is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "--target-c-block-size requires --c-backend api",
                ));
            }
            run_c_zstd(
                &args.zstd_bin,
                args.c_mode,
                args.level,
                args.dictionary.as_deref(),
                &args.input,
                output,
            )
        }
        CBackend::Api => {
            let compressed = compress_c_api(
                input,
                args.level,
                args.target_c_block_size,
                dictionary,
                prepared_dictionary,
            )?;
            fs::write(output, compressed)
        }
    }
}

fn compress_c_api(
    input: &[u8],
    level: i32,
    target_c_block_size: Option<usize>,
    dictionary: Option<&[u8]>,
    prepared_dictionary: Option<&PreparedDictionaryReferences>,
) -> io::Result<Vec<u8>> {
    let mut context = CCtx::create();
    if let Some(prepared) = prepared_dictionary {
        let mut output = Vec::with_capacity(zstd_safe::compress_bound(input.len()));
        context
            .compress_using_cdict(&mut output, input, &prepared.c)
            .map_err(c_api_error)?;
        return Ok(output);
    }
    set_c_api_parameter(&mut context, CParameter::CompressionLevel(level))?;
    if let Some(dictionary) = dictionary {
        context.load_dictionary(dictionary).map_err(c_api_error)?;
    }
    if let Some(target_c_block_size) = target_c_block_size {
        let target_c_block_size = target_c_block_size.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "targetCBlockSize must fit in a u32",
            )
        })?;
        set_c_api_parameter(
            &mut context,
            CParameter::TargetCBlockSize(target_c_block_size),
        )?;
    }
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
    dictionary: Option<&Path>,
    input: &Path,
    output: &Path,
) -> io::Result<()> {
    let mut command = Command::new(zstd_bin);
    command.args(["-q", "-f"]);
    command.args(mode.zstd_args());
    command.arg("--no-check");
    command.args(zstd_cli_level_args(level));
    if let Some(dictionary) = dictionary {
        command.arg("-D").arg(dictionary);
    }
    command.arg(input).arg("-o").arg(output);
    run_command_silent(&mut command)
}

fn verify_decoded_matches_with_dictionary(
    zstd_bin: &Path,
    compressed: &Path,
    original: &Path,
    dictionary: Option<&Path>,
) -> io::Result<()> {
    if let Some(dictionary) = dictionary {
        let output = Command::new(zstd_bin)
            .arg("-q")
            .arg("-d")
            .arg("-c")
            .arg("-D")
            .arg(dictionary)
            .arg(compressed)
            .output()?;
        if !output.status.success() {
            return Err(io::Error::other(format!(
                "zstd decode with dictionary failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        let expected = fs::read(original)?;
        if output.stdout != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "decoded dictionary output does not match original",
            ));
        }
        return Ok(());
    }

    verify_decoded_matches(zstd_bin, compressed, original)
}

impl CBackend {
    fn description(self, mode: CMode, prepared_dictionary: bool) -> String {
        match self {
            Self::Cli => format!("C zstd CLI {}", mode.description()),
            Self::Api if prepared_dictionary => "C ZSTD_compress_usingCDict() API".to_string(),
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

fn write_blocks_csv(path: &Path, blocks: &[BlockInfo]) -> io::Result<()> {
    let mut output = fs::File::create(path)?;
    writeln!(
        output,
        "index,frame_offset,block_type,last,content_size,source_offset,decompressed_size,literal_type,literal_regenerated_size,literal_payload_size,literal_table_size,literal_streams,sequences,ll_mode,of_mode,ml_mode"
    )?;
    for block in blocks {
        let section = block.section_info.as_ref();
        writeln!(
            output,
            "{},{},{:?},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            block.index,
            block.offset,
            block.block_type,
            block.last,
            block.content_size,
            source_offset_label(block),
            decompressed_size_label(block),
            section
                .map(|info| format!("{:?}", info.literal_type))
                .unwrap_or_else(|| "-".to_string()),
            section
                .map(|info| info.literal_regenerated_size.to_string())
                .unwrap_or_else(|| "-".to_string()),
            section
                .map(|info| info.literal_payload_size.to_string())
                .unwrap_or_else(|| "-".to_string()),
            section
                .and_then(|info| info.literal_table_size)
                .map(|size| size.to_string())
                .unwrap_or_else(|| "-".to_string()),
            section
                .and_then(|info| info.literal_streams)
                .map(|streams| streams.to_string())
                .unwrap_or_else(|| "-".to_string()),
            section
                .map(|info| info.sequences.to_string())
                .unwrap_or_else(|| "-".to_string()),
            mode_label(section.and_then(|info| info.ll_mode)),
            mode_label(section.and_then(|info| info.of_mode)),
            mode_label(section.and_then(|info| info.ml_mode)),
        )?;
    }
    Ok(())
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
        ",lit={:?}/regen:{}/payload:{}/table:{}/streams:{},seqs={},modes={}/{}/{}",
        info.literal_type,
        info.literal_regenerated_size,
        info.literal_payload_size,
        info.literal_table_size
            .map(|size| size.to_string())
            .unwrap_or_else(|| "-".to_string()),
        info.literal_streams
            .map(|streams| streams.to_string())
            .unwrap_or_else(|| "-".to_string()),
        info.sequences,
        mode_label(info.ll_mode),
        mode_label(info.of_mode),
        mode_label(info.ml_mode),
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
