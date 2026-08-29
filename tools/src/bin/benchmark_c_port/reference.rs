use std::{cmp::Ordering, fs, io, path::Path, process::Command, time::Instant};

use ruzstd::encoding::{
    compress_slice_c_level, compress_slice_c_level_with_dictionary,
    compress_slice_c_level_with_dictionary_and_target_c_block_size,
    compress_slice_c_level_with_prepared_dictionary,
    compress_slice_c_level_with_target_c_block_size, CLevelEncoderDictionary,
};
use zstd_rs_tools::{run_command_silent, verify_decoded_matches};
use zstd_safe::{CCtx, CDict, CParameter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CMode {
    SingleThread,
    T1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CBackend {
    Cli,
    Api,
}

pub(crate) struct ReferenceConfig<'a> {
    pub(crate) zstd_bin: &'a Path,
    pub(crate) c_backend: CBackend,
    pub(crate) c_mode: CMode,
    pub(crate) target_c_block_size: Option<usize>,
    pub(crate) dictionary_path: Option<&'a Path>,
}

pub(crate) struct PreparedDictionaryReferences {
    rust: CLevelEncoderDictionary,
    c: CDict<'static>,
}

impl PreparedDictionaryReferences {
    pub(crate) fn new(dictionary: &[u8], level: i32) -> io::Result<Self> {
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

#[derive(Clone, Copy)]
struct CpuSample {
    seconds: f64,
}

pub(crate) fn write_c_reference(
    config: &ReferenceConfig<'_>,
    level: i32,
    input: &[u8],
    dictionary: Option<&[u8]>,
    prepared_dictionary: Option<&PreparedDictionaryReferences>,
    input_path: &Path,
    output: &Path,
) -> io::Result<()> {
    match config.c_backend {
        CBackend::Cli => run_c_zstd(
            config.zstd_bin,
            config.c_mode,
            level,
            input_path,
            config.dictionary_path,
            output,
        ),
        CBackend::Api => {
            let compressed = compress_c_api(
                input,
                level,
                config.target_c_block_size,
                dictionary,
                prepared_dictionary,
            )?;
            fs::write(output, compressed)
        }
    }
}

pub(crate) fn write_c_reference_timed(
    config: &ReferenceConfig<'_>,
    level: i32,
    input: &[u8],
    dictionary: Option<&[u8]>,
    prepared_dictionary: Option<&PreparedDictionaryReferences>,
    input_path: &Path,
    output: &Path,
) -> io::Result<(f64, f64)> {
    match config.c_backend {
        CBackend::Cli => run_c_zstd_timed(
            config.zstd_bin,
            config.c_mode,
            level,
            input_path,
            config.dictionary_path,
            output,
        ),
        CBackend::Api => {
            let before_cpu = CpuSample::now();
            let before = Instant::now();
            let compressed = compress_c_api(
                input,
                level,
                config.target_c_block_size,
                dictionary,
                prepared_dictionary,
            )?;
            let wall = before.elapsed().as_secs_f64();
            let cpu = before_cpu.elapsed().unwrap_or(wall);
            fs::write(output, compressed)?;
            Ok((wall, cpu))
        }
    }
}

pub(crate) fn compress_rust_reference(
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
        compress_slice_c_level_with_dictionary(input, level, dictionary).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Rust dictionary compression failed: {err:?}"),
            )
        })
    } else if let Some(target_c_block_size) = target_c_block_size {
        compress_slice_c_level_with_target_c_block_size(input, level, target_c_block_size)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "targetCBlockSize is outside C's accepted range",
                )
            })
    } else {
        Ok(compress_slice_c_level(input, level))
    }
}

pub(crate) fn verify_decoded_matches_with_dictionary(
    zstd_bin: &Path,
    compressed: &Path,
    original: &Path,
    dictionary: Option<&Path>,
) -> io::Result<()> {
    if let Some(dictionary) = dictionary {
        verify_decoded_matches_with_zstd_dictionary(zstd_bin, compressed, original, dictionary)
    } else {
        verify_decoded_matches(zstd_bin, compressed, original)
    }
}

pub(crate) fn sync_if_requested(no_sync: bool) -> io::Result<()> {
    if no_sync {
        return Ok(());
    }
    let mut sync = Command::new("sync");
    run_command_silent(&mut sync)
}

pub(crate) fn process_cpu_seconds() -> Option<f64> {
    let stat = fs::read_to_string("/proc/self/stat").ok()?;
    let close_paren = stat.rfind(')')?;
    let fields = stat[close_paren + 2..]
        .split_whitespace()
        .collect::<Vec<_>>();
    let user_ticks = fields.get(11)?.parse::<f64>().ok()?;
    let system_ticks = fields.get(12)?.parse::<f64>().ok()?;
    let ticks_per_second = ticks_per_second()?;
    Some((user_ticks + system_ticks) / ticks_per_second)
}

pub(crate) fn zstd_cli_level_args(level: i32) -> Vec<String> {
    match level.cmp(&0) {
        Ordering::Less => vec![format!("--fast={}", level.unsigned_abs())],
        Ordering::Equal => Vec::new(),
        Ordering::Greater if level > 19 => vec!["--ultra".to_string(), format!("-{level}")],
        Ordering::Greater => vec![format!("-{level}")],
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

impl CBackend {
    pub(crate) fn description(self, mode: CMode) -> String {
        match self {
            Self::Cli => format!("C zstd CLI {}", mode.description()),
            Self::Api => "C ZSTD_compress2() API".to_string(),
        }
    }
}

impl CpuSample {
    fn now() -> Self {
        Self {
            seconds: process_cpu_seconds().unwrap_or(0.0),
        }
    }

    fn elapsed(self) -> Option<f64> {
        process_cpu_seconds().map(|seconds| seconds - self.seconds)
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
    set_c_api_parameter(&mut context, CParameter::ChecksumFlag(false))?;
    if let Some(dictionary) = dictionary {
        context.load_dictionary(dictionary).map_err(c_api_error)?;
    }
    if let Some(target_c_block_size) = target_c_block_size {
        let target_c_block_size = target_c_block_size.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "targetCBlockSize is out of range for the C API",
            )
        })?;
        set_c_api_parameter(
            &mut context,
            CParameter::TargetCBlockSize(target_c_block_size),
        )?;
    }
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
    dictionary: Option<&Path>,
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

fn run_c_zstd_timed(
    zstd_bin: &Path,
    mode: CMode,
    level: i32,
    input: &Path,
    dictionary: Option<&Path>,
    output: &Path,
) -> io::Result<(f64, f64)> {
    let time_file = output.with_extension("zst.time");
    let mut timed = Command::new("/usr/bin/time");
    timed
        .args(["-f", "%e\t%U\t%S", "-o"])
        .arg(&time_file)
        .arg(zstd_bin)
        .args(["-q", "-f"]);
    timed.args(mode.zstd_args());
    timed.arg("--no-check");
    timed.args(zstd_cli_level_args(level));
    if let Some(dictionary) = dictionary {
        timed.arg("-D").arg(dictionary);
    }
    timed.arg(input).arg("-o").arg(output);
    run_command_silent(&mut timed)?;
    let text = fs::read_to_string(&time_file)?;
    fs::remove_file(&time_file)?;
    let fields = text.trim().split('\t').collect::<Vec<_>>();
    if fields.len() != 3 {
        return Err(io::Error::other(format!("unexpected time output: {text}")));
    }
    let wall = fields[0].parse::<f64>().unwrap_or(0.0);
    let user = fields[1].parse::<f64>().unwrap_or(0.0);
    let system = fields[2].parse::<f64>().unwrap_or(0.0);
    Ok((wall, user + system))
}

fn verify_decoded_matches_with_zstd_dictionary(
    zstd_bin: &Path,
    compressed: &Path,
    original: &Path,
    dictionary: &Path,
) -> io::Result<()> {
    let decoded = Command::new(zstd_bin)
        .args(["-q", "-d", "-c", "-D"])
        .arg(dictionary)
        .arg(compressed)
        .output()?;
    if !decoded.status.success() {
        return Err(io::Error::other(format!(
            "zstd decode with dictionary failed: {}",
            compressed.display()
        )));
    }
    let expected = fs::read(original)?;
    if decoded.stdout == expected {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "decoded output did not match original: {}",
            compressed.display()
        )))
    }
}

fn ticks_per_second() -> Option<f64> {
    let output = Command::new("getconf").arg("CLK_TCK").output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()?
        .trim()
        .parse::<f64>()
        .ok()
}
