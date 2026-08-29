#[path = "benchmark_c_port/reference.rs"]
mod reference;
#[path = "benchmark_c_port/report.rs"]
mod report;

use std::{
    collections::BTreeMap,
    env, fs, io,
    path::{Path, PathBuf},
    time::Instant,
};

use reference::{
    compress_rust_reference, process_cpu_seconds, sync_if_requested,
    verify_decoded_matches_with_dictionary, write_c_reference, write_c_reference_timed, CBackend,
    CMode, PreparedDictionaryReferences, ReferenceConfig,
};
use report::{median_sample, write_csv, write_markdown};
use zstd_rs_tools::{benchmark_tmp, has_flag, parse_value, repo_root};

#[derive(Clone)]
struct Args {
    fixtures: PathBuf,
    output_dir: PathBuf,
    zstd_bin: PathBuf,
    c_backend: CBackend,
    c_mode: CMode,
    target_c_block_size: Option<usize>,
    dictionary: Option<PathBuf>,
    prepared_dictionary: bool,
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
        args.target_c_block_size,
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
    let c_backend = parse_c_backend(&parse_value(&raw, "--c-backend", "cli"))?;
    let target_c_block_size = optional_usize(&raw, "--target-c-block-size")?;
    let dictionary = optional_path(&raw, "--dictionary");
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
        c_backend,
        c_mode: parse_c_mode(&parse_value(&raw, "--c-mode", "single-thread"))?,
        target_c_block_size,
        dictionary,
        prepared_dictionary,
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
    [--limit N] [--zstd-bin PATH] [--c-backend BACKEND] [--c-mode MODE] \
    [--target-c-block-size N] [--dictionary PATH] [--prepared-dictionary] [--output-dir DIR] \
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
  --target-c-block-size N
                    Compare targetCBlockSize mode. Requires --c-backend api.
  --dictionary PATH Compare compression with a zstd dictionary.
  --prepared-dictionary
                    Compare Rust's prepared dictionary with C ZSTD_CDict.
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

fn optional_path(args: &[String], name: &str) -> Option<PathBuf> {
    args.windows(2)
        .find_map(|window| (window[0] == name).then(|| PathBuf::from(&window[1])))
}

fn run_benchmarks(args: &Args) -> io::Result<Vec<Row>> {
    fs::create_dir_all(&args.output_dir)?;
    let mut fixtures = collect_fixtures(&args.fixtures)?;
    if let Some(limit) = args.limit {
        fixtures.truncate(limit);
    }
    let dictionary = args.dictionary.as_ref().map(fs::read).transpose()?;
    let prepared_dictionaries = if args.prepared_dictionary {
        let dictionary = dictionary
            .as_deref()
            .expect("argument validation requires a dictionary");
        args.levels
            .iter()
            .map(|&level| {
                PreparedDictionaryReferences::new(dictionary, level)
                    .map(|prepared| (level, prepared))
            })
            .collect::<io::Result<BTreeMap<_, _>>>()?
    } else {
        BTreeMap::new()
    };
    let reference_config = ReferenceConfig {
        zstd_bin: &args.zstd_bin,
        c_backend: args.c_backend,
        c_mode: args.c_mode,
        target_c_block_size: args.target_c_block_size,
        dictionary_path: args.dictionary.as_deref(),
    };

    let mut rows = Vec::new();
    for fixture in fixtures {
        let input = fs::read(&fixture.path)?;
        for level in &args.levels {
            let prepared_dictionary = prepared_dictionaries.get(level);
            let target_suffix = args
                .target_c_block_size
                .map(|target| format!(".t{target}"))
                .unwrap_or_default();
            let rust_output = args
                .output_dir
                .join(format!("{}.l{level}{target_suffix}.rust.zst", fixture.name));
            let c_output = args
                .output_dir
                .join(format!("{}.l{level}{target_suffix}.c.zst", fixture.name));

            let rust = compress_rust_reference(
                &input,
                *level,
                args.target_c_block_size,
                dictionary.as_deref(),
                prepared_dictionary,
            )?;
            fs::write(&rust_output, &rust)?;
            verify_decoded_matches_with_dictionary(
                &args.zstd_bin,
                &rust_output,
                &fixture.path,
                args.dictionary.as_deref(),
            )?;
            remove_output_unless_kept(&rust_output, args.keep_outputs)?;

            write_c_reference(
                &reference_config,
                *level,
                &input,
                dictionary.as_deref(),
                prepared_dictionary,
                &fixture.path,
                &c_output,
            )?;
            verify_decoded_matches_with_dictionary(
                &args.zstd_bin,
                &c_output,
                &fixture.path,
                args.dictionary.as_deref(),
            )?;
            remove_output_unless_kept(&c_output, args.keep_outputs)?;

            let mut rust_walls = Vec::with_capacity(args.runs);
            let mut c_walls = Vec::with_capacity(args.runs);
            let mut rust_cpus = Vec::with_capacity(args.runs);
            let mut c_cpus = Vec::with_capacity(args.runs);
            let mut rust_bytes = rust.len() as u64;
            let mut c_bytes = 0;

            for _ in 0..args.runs {
                sync_if_requested(args.no_sync)?;
                let before_cpu = process_cpu_seconds().unwrap_or(0.0);
                let before = Instant::now();
                let rust = compress_rust_reference(
                    &input,
                    *level,
                    args.target_c_block_size,
                    dictionary.as_deref(),
                    prepared_dictionary,
                )?;
                let rust_wall = before.elapsed().as_secs_f64();
                let rust_cpu = process_cpu_seconds()
                    .map(|seconds| seconds - before_cpu)
                    .unwrap_or(rust_wall);
                fs::write(&rust_output, &rust)?;
                verify_decoded_matches_with_dictionary(
                    &args.zstd_bin,
                    &rust_output,
                    &fixture.path,
                    args.dictionary.as_deref(),
                )?;
                rust_bytes = rust.len() as u64;
                remove_output_unless_kept(&rust_output, args.keep_outputs)?;

                sync_if_requested(args.no_sync)?;
                let (c_wall, c_cpu) = write_c_reference_timed(
                    &reference_config,
                    *level,
                    &input,
                    dictionary.as_deref(),
                    prepared_dictionary,
                    &fixture.path,
                    &c_output,
                )?;
                verify_decoded_matches_with_dictionary(
                    &args.zstd_bin,
                    &c_output,
                    &fixture.path,
                    args.dictionary.as_deref(),
                )?;
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
                rust_wall: median_sample(&mut rust_walls),
                c_wall: median_sample(&mut c_walls),
                rust_cpu: median_sample(&mut rust_cpus),
                c_cpu: median_sample(&mut c_cpus),
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

#[cfg(test)]
mod tests {
    use super::reference::{zstd_cli_level_args, CBackend, CMode};
    use super::{parse_c_backend, parse_c_mode};

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
