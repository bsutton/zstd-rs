use std::{
    cmp::Ordering,
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use ruzstd::encoding::compress_slice_c_level;
use zstd_rs_tools::{
    benchmark_tmp, csv_escape, has_flag, parse_value, repo_root, run_command_silent,
    verify_decoded_matches, write_all,
};
use zstd_safe::{CCtx, CParameter};

#[derive(Clone)]
struct Args {
    fixtures: PathBuf,
    output_dir: PathBuf,
    zstd_bin: PathBuf,
    c_backend: CBackend,
    c_mode: CMode,
    levels: Vec<i32>,
    runs: usize,
    limit: Option<usize>,
    csv_output: PathBuf,
    md_output: PathBuf,
    no_sync: bool,
    keep_outputs: bool,
}

struct Fixture {
    name: String,
    path: PathBuf,
    bytes: u64,
}

struct Row {
    fixture: String,
    level: i32,
    input_bytes: u64,
    rust_bytes: u64,
    c_bytes: u64,
    rust_wall: f64,
    c_wall: f64,
    rust_cpu: f64,
    c_cpu: f64,
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

#[derive(Clone, Copy)]
struct CpuSample {
    seconds: f64,
}

fn main() -> io::Result<()> {
    let args = parse_args()?;
    let rows = run_benchmarks(&args)?;
    write_csv(&args.csv_output, &rows)?;
    write_markdown(
        &args.md_output,
        &rows,
        &args.csv_output,
        args.c_backend,
        args.c_mode,
    )?;
    println!("{}", args.csv_output.display());
    println!("{}", args.md_output.display());
    Ok(())
}

fn parse_args() -> io::Result<Args> {
    let raw = env::args().skip(1).collect::<Vec<_>>();
    if has_flag(&raw, "--help") || has_flag(&raw, "-h") {
        print_help();
        std::process::exit(0);
    }

    let repo = repo_root();
    let tmp = benchmark_tmp();
    let default_fixtures = repo
        .join("benchmarks")
        .join("archive")
        .join("tmp")
        .join("realworld-100");

    Ok(Args {
        fixtures: PathBuf::from(parse_value(
            &raw,
            "--fixtures",
            default_fixtures.display().to_string(),
        )),
        output_dir: PathBuf::from(parse_value(
            &raw,
            "--output-dir",
            tmp.join("c-port-benchmark-output").display().to_string(),
        )),
        zstd_bin: PathBuf::from(parse_value(&raw, "--zstd-bin", "/usr/bin/zstd")),
        c_backend: parse_c_backend(&parse_value(&raw, "--c-backend", "cli"))?,
        c_mode: parse_c_mode(&parse_value(&raw, "--c-mode", "single-thread"))?,
        levels: parse_levels(&parse_value(&raw, "--levels", "1,3,5,8,13,16,19,22"))?,
        runs: parse_runs(&parse_value(&raw, "--runs", "3"))?,
        limit: optional_usize(&raw, "--limit")?,
        csv_output: PathBuf::from(parse_value(
            &raw,
            "--csv-output",
            tmp.join("c-port-benchmark.csv").display().to_string(),
        )),
        md_output: PathBuf::from(parse_value(
            &raw,
            "--md-output",
            tmp.join("c-port-benchmark.md").display().to_string(),
        )),
        no_sync: has_flag(&raw, "--no-sync"),
        keep_outputs: has_flag(&raw, "--keep-outputs"),
    })
}

fn print_help() {
    println!(
        "\
Usage: benchmark_c_port [--fixtures DIR] [--levels CSV] [--runs N] \
    [--limit N] [--zstd-bin PATH] [--c-backend BACKEND] [--c-mode MODE] [--output-dir DIR] \
    [--csv-output PATH] [--md-output PATH] [--no-sync] [--keep-outputs]

Options:
  --fixtures DIR    Fixture directory, walked recursively.
  --levels CSV      C compression levels to test, for example -5,0,1,3,9,19.
  --runs N          Timed runs per fixture and level.
  --limit N         Limit fixture count after sorting by path.
  --zstd-bin PATH   Path to the C zstd binary.
  --c-backend MODE  C reference backend: cli or api. Default cli.
  --c-mode MODE     C zstd mode: single-thread or t1. Default single-thread.
                    Only used when --c-backend cli.
  --output-dir DIR  Temporary directory for compressed outputs.
  --csv-output PATH CSV output path.
  --md-output PATH  Markdown output path.
  --no-sync         Skip sync before timed runs.
  --keep-outputs    Keep compressed outputs for inspection.
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

fn parse_runs(raw: &str) -> io::Result<usize> {
    let runs = raw
        .parse()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    if runs == 0 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--runs must be greater than zero",
        ))
    } else {
        Ok(runs)
    }
}

fn parse_levels(raw: &str) -> io::Result<Vec<i32>> {
    let levels = raw
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<i32>()
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
        })
        .collect::<io::Result<Vec<_>>>()?;
    if levels.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--levels must contain at least one level",
        ))
    } else {
        Ok(levels)
    }
}

fn optional_usize(args: &[String], name: &str) -> io::Result<Option<usize>> {
    let value = args
        .windows(2)
        .find_map(|window| (window[0] == name).then(|| window[1].clone()));
    value
        .map(|value| {
            value
                .parse()
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
        })
        .transpose()
}

fn run_benchmarks(args: &Args) -> io::Result<Vec<Row>> {
    fs::create_dir_all(&args.output_dir)?;
    let mut fixtures = collect_fixtures(&args.fixtures)?;
    if let Some(limit) = args.limit {
        fixtures.truncate(limit);
    }

    let mut rows = Vec::new();
    for fixture in fixtures {
        let input = fs::read(&fixture.path)?;
        for level in &args.levels {
            let rust_output = args
                .output_dir
                .join(format!("{}.l{level}.rust.zst", fixture.name));
            let c_output = args
                .output_dir
                .join(format!("{}.l{level}.c.zst", fixture.name));

            let rust = compress_slice_c_level(&input, *level);
            fs::write(&rust_output, &rust)?;
            verify_decoded_matches(&args.zstd_bin, &rust_output, &fixture.path)?;
            remove_output_unless_kept(&rust_output, args.keep_outputs)?;

            write_c_reference(
                args.c_backend,
                &args.zstd_bin,
                args.c_mode,
                *level,
                &input,
                &fixture.path,
                &c_output,
            )?;
            verify_decoded_matches(&args.zstd_bin, &c_output, &fixture.path)?;
            remove_output_unless_kept(&c_output, args.keep_outputs)?;

            let mut rust_walls = Vec::with_capacity(args.runs);
            let mut c_walls = Vec::with_capacity(args.runs);
            let mut rust_cpus = Vec::with_capacity(args.runs);
            let mut c_cpus = Vec::with_capacity(args.runs);
            let mut rust_bytes = rust.len() as u64;
            let mut c_bytes = 0;

            for _ in 0..args.runs {
                sync_if_requested(args.no_sync)?;
                let before_cpu = CpuSample::now();
                let before = Instant::now();
                let rust = compress_slice_c_level(&input, *level);
                let rust_wall = before.elapsed().as_secs_f64();
                let rust_cpu = before_cpu.elapsed().unwrap_or(rust_wall);
                fs::write(&rust_output, &rust)?;
                verify_decoded_matches(&args.zstd_bin, &rust_output, &fixture.path)?;
                rust_bytes = rust.len() as u64;
                remove_output_unless_kept(&rust_output, args.keep_outputs)?;

                sync_if_requested(args.no_sync)?;
                let (c_wall, c_cpu) = write_c_reference_timed(
                    args.c_backend,
                    &args.zstd_bin,
                    args.c_mode,
                    *level,
                    &input,
                    &fixture.path,
                    &c_output,
                )?;
                verify_decoded_matches(&args.zstd_bin, &c_output, &fixture.path)?;
                c_bytes = fs::metadata(&c_output)?.len();
                remove_output_unless_kept(&c_output, args.keep_outputs)?;

                rust_walls.push(rust_wall);
                rust_cpus.push(rust_cpu);
                c_walls.push(c_wall);
                c_cpus.push(c_cpu);
            }

            rows.push(Row {
                fixture: fixture.name.clone(),
                level: *level,
                input_bytes: fixture.bytes,
                rust_bytes,
                c_bytes,
                rust_wall: median(&mut rust_walls),
                c_wall: median(&mut c_walls),
                rust_cpu: median(&mut rust_cpus),
                c_cpu: median(&mut c_cpus),
            });
        }
    }

    Ok(rows)
}

fn remove_output_unless_kept(path: &Path, keep_outputs: bool) -> io::Result<()> {
    if keep_outputs {
        Ok(())
    } else {
        fs::remove_file(path)
    }
}

fn collect_fixtures(root: &Path) -> io::Result<Vec<Fixture>> {
    let mut paths = Vec::new();
    collect_fixture_paths(root, &mut paths)?;
    paths.sort();
    let fixtures = paths
        .into_iter()
        .map(|path| {
            let mut name = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace(['/', '\\'], "_");
            if name.is_empty() {
                name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("fixture")
                    .to_string();
            }
            let bytes = fs::metadata(&path)?.len();
            Ok(Fixture { name, path, bytes })
        })
        .collect::<io::Result<Vec<_>>>()?;
    if fixtures.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("no readable fixture files found under {}", root.display()),
        ))
    } else {
        Ok(fixtures)
    }
}

fn collect_fixture_paths(path: &Path, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    let Ok(metadata) = fs::metadata(path) else {
        return Ok(());
    };
    if metadata.is_file() {
        paths.push(path.to_path_buf());
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            collect_fixture_paths(&entry?.path(), paths)?;
        }
    }
    Ok(())
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

fn write_c_reference_timed(
    backend: CBackend,
    zstd_bin: &Path,
    mode: CMode,
    level: i32,
    input: &[u8],
    input_path: &Path,
    output: &Path,
) -> io::Result<(f64, f64)> {
    match backend {
        CBackend::Cli => run_c_zstd_timed(zstd_bin, mode, level, input_path, output),
        CBackend::Api => {
            let before_cpu = CpuSample::now();
            let before = Instant::now();
            let compressed = compress_c_api(input, level)?;
            let wall = before.elapsed().as_secs_f64();
            let cpu = before_cpu.elapsed().unwrap_or(wall);
            fs::write(output, compressed)?;
            Ok((wall, cpu))
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

fn run_c_zstd_timed(
    zstd_bin: &Path,
    mode: CMode,
    level: i32,
    input: &Path,
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
    fn description(self, mode: CMode) -> String {
        match self {
            Self::Cli => format!("C zstd CLI {}", mode.description()),
            Self::Api => "C ZSTD_compress2() API".to_string(),
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

fn sync_if_requested(no_sync: bool) -> io::Result<()> {
    if no_sync {
        return Ok(());
    }
    let mut sync = Command::new("sync");
    run_command_silent(&mut sync)
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

fn process_cpu_seconds() -> Option<f64> {
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

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    values[values.len() / 2]
}

fn write_csv(path: &Path, rows: &[Row]) -> io::Result<()> {
    let mut csv = String::from(
        "fixture,level,input_bytes,c_bytes,rust_bytes,rust_vs_c_bytes_pct,c_cpu,rust_cpu,cpu_improvement_pct,c_wall,rust_wall\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{:+.2},{:.4},{:.4},{:+.2},{:.4},{:.4}\n",
            csv_escape(&row.fixture),
            row.level,
            row.input_bytes,
            row.c_bytes,
            row.rust_bytes,
            pct_delta(row.rust_bytes as f64, row.c_bytes as f64),
            row.c_cpu,
            row.rust_cpu,
            pct_improvement(row.rust_cpu, row.c_cpu),
            row.c_wall,
            row.rust_wall,
        ));
    }
    write_all(path, &csv)
}

fn write_markdown(
    path: &Path,
    rows: &[Row],
    csv_path: &Path,
    c_backend: CBackend,
    c_mode: CMode,
) -> io::Result<()> {
    let headers = [
        "Fixture",
        "Lvl",
        "Input",
        "C bytes",
        "Rust bytes",
        "Gap",
        "C CPU",
        "Rust CPU",
        "CPU Improvement",
    ];
    let table_rows = rows
        .iter()
        .map(|row| {
            vec![
                row.fixture.clone(),
                row.level.to_string(),
                format_number(row.input_bytes),
                format_number(row.c_bytes),
                format_number(row.rust_bytes),
                format!(
                    "{:+.1}%",
                    pct_delta(row.rust_bytes as f64, row.c_bytes as f64)
                ),
                format!("{:.4}s", row.c_cpu),
                format!("{:.4}s", row.rust_cpu),
                format!("{:+.1}%", pct_improvement(row.rust_cpu, row.c_cpu)),
            ]
        })
        .collect::<Vec<_>>();

    let widths = (0..headers.len())
        .map(|column| {
            table_rows
                .iter()
                .map(|row| row[column].len())
                .chain([headers[column].len()])
                .max()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    let mut lines = vec![
        "# C Port vs C zstd Benchmark".to_string(),
        String::new(),
        format!("Source CSV: `{}`", csv_path.display()),
        String::new(),
        format!(
            "Gap is Rust compressed size versus {} with frame checksums disabled; positive means Rust is larger. CPU Improvement is positive when Rust uses less CPU than the C reference. Every output is decoded with C zstd and byte-compared against the original fixture.",
            c_backend.description(c_mode)
        ),
        String::new(),
        "```text".to_string(),
        format_row(&headers, &widths),
    ];
    let separators = widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect::<Vec<_>>();
    let separator_refs = separators.iter().map(String::as_str).collect::<Vec<_>>();
    lines.push(format_row(&separator_refs, &widths));
    for row in &table_rows {
        let row_refs = row.iter().map(String::as_str).collect::<Vec<_>>();
        lines.push(format_row(&row_refs, &widths));
    }
    lines.push("```".to_string());
    lines.push(String::new());
    write_all(path, &lines.join("\n"))
}

fn pct_improvement(value: f64, baseline: f64) -> f64 {
    if baseline == 0.0 {
        0.0
    } else {
        (baseline - value) * 100.0 / baseline
    }
}

fn pct_delta(value: f64, baseline: f64) -> f64 {
    if baseline == 0.0 {
        0.0
    } else {
        (value - baseline) * 100.0 / baseline
    }
}

fn format_row(row: &[&str], widths: &[usize]) -> String {
    row.iter()
        .enumerate()
        .map(|(idx, value)| format!("{value:<width$}", width = widths[idx]))
        .collect::<Vec<_>>()
        .join("  ")
}

fn format_number(value: u64) -> String {
    let text = value.to_string();
    let mut out = String::new();
    for (idx, ch) in text.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_c_backend, parse_c_mode, zstd_cli_level_args, CBackend, CMode};

    #[test]
    fn c_level_zero_uses_cli_default() {
        assert!(zstd_cli_level_args(0).is_empty());
    }

    #[test]
    fn positive_c_levels_use_dash_level() {
        assert_eq!(zstd_cli_level_args(16), vec!["-16".to_string()]);
    }

    #[test]
    fn ultra_c_levels_enable_ultra_mode() {
        assert_eq!(
            zstd_cli_level_args(22),
            vec!["--ultra".to_string(), "-22".to_string()]
        );
    }

    #[test]
    fn negative_c_levels_use_fast_mode() {
        assert_eq!(zstd_cli_level_args(-5), vec!["--fast=5".to_string()]);
    }

    #[test]
    fn parses_c_modes() {
        assert_eq!(parse_c_mode("single-thread").unwrap(), CMode::SingleThread);
        assert_eq!(parse_c_mode("t1").unwrap(), CMode::T1);
        assert!(parse_c_mode("threads").is_err());
    }

    #[test]
    fn parses_c_backends() {
        assert_eq!(parse_c_backend("cli").unwrap(), CBackend::Cli);
        assert_eq!(parse_c_backend("api").unwrap(), CBackend::Api);
        assert!(parse_c_backend("library").is_err());
    }
}
