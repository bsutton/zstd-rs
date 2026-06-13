use std::{
    cmp::Ordering,
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use zstd_rs_tools::{
    benchmark_tmp, csv_escape, has_flag, parse_value, repo_root, run_command_silent,
    verify_decoded_matches, write_all,
};

#[derive(Clone)]
struct Args {
    fixtures: PathBuf,
    output_dir: PathBuf,
    current_bin: PathBuf,
    upstream_bin: PathBuf,
    c_zstd_bin: PathBuf,
    level: u8,
    runs: usize,
    csv_output: PathBuf,
    md_output: PathBuf,
    commentary: String,
    no_sync: bool,
}

#[derive(Clone)]
struct Row {
    fixture: String,
    encoder: &'static str,
    bytes: u64,
    elapsed: f64,
    cpu: f64,
    rss: u64,
}

fn main() -> io::Result<()> {
    let args = parse_args()?;
    let rows = run_benchmarks(&args)?;
    write_csv(&args.csv_output, &rows)?;
    write_markdown(&args.md_output, &rows, &args.csv_output, &args.commentary)?;
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
    Ok(Args {
        fixtures: PathBuf::from(parse_value(
            &raw,
            "--fixtures",
            repo.join("benchmarks")
                .join("fixtures")
                .join("broad-local")
                .display()
                .to_string(),
        )),
        output_dir: PathBuf::from(parse_value(
            &raw,
            "--output-dir",
            tmp.join("benchmark-output").display().to_string(),
        )),
        current_bin: PathBuf::from(parse_value(
            &raw,
            "--current-bin",
            repo.join("target")
                .join("release")
                .join("ruzstd-cli")
                .display()
                .to_string(),
        )),
        upstream_bin: PathBuf::from(parse_value(
            &raw,
            "--upstream-bin",
            tmp.join("ruzstd-cli-baseline").display().to_string(),
        )),
        c_zstd_bin: PathBuf::from(parse_value(&raw, "--c-zstd-bin", "/usr/bin/zstd")),
        level: parse_value(&raw, "--level", parse_value(&raw, "-l", "1"))
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?,
        runs: parse_value(&raw, "--runs", "3")
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?,
        csv_output: PathBuf::from(parse_value(
            &raw,
            "--csv-output",
            tmp.join("zstd-rs-benchmark.csv").display().to_string(),
        )),
        md_output: PathBuf::from(parse_value(
            &raw,
            "--md-output",
            tmp.join("zstd-rs-benchmark.md").display().to_string(),
        )),
        commentary: parse_value(&raw, "--commentary", ""),
        no_sync: has_flag(&raw, "--no-sync"),
    })
}

fn print_help() {
    println!(
        "Usage: benchmark_zstd [--fixtures DIR] [--output-dir DIR] [--current-bin PATH] \\\n    [--upstream-bin PATH] [--c-zstd-bin PATH] [-l LEVEL] [--runs N] \\\n    [--csv-output PATH] [--md-output PATH] [--commentary TEXT] [--no-sync]\n\nOptions:\n  --fixtures DIR       Directory containing input fixture files.\n  --output-dir DIR     Temporary directory for compressed outputs.\n  --current-bin PATH   Path to the current ruzstd-cli binary.\n  --upstream-bin PATH  Path to the upstream/baseline ruzstd-cli binary.\n  --c-zstd-bin PATH    Path to the C zstd binary.\n  -l, --level LEVEL    Compression level.\n  --runs N             Timed runs per fixture.\n  --csv-output PATH    CSV output path.\n  --md-output PATH     Markdown output path.\n  --commentary TEXT    Short note for the Markdown output.\n  --no-sync            Skip sync before timed runs.\n  -h, --help           Show this help message."
    );
}

fn encoder_commands(args: &Args, input: &Path, output: &Path) -> Vec<(&'static str, Command)> {
    let mut upstream = Command::new(&args.upstream_bin);
    upstream
        .arg("compress")
        .arg(input)
        .arg(output)
        .arg("-l")
        .arg(args.level.to_string());

    let mut current = Command::new(&args.current_bin);
    current
        .arg("compress")
        .arg(input)
        .arg(output)
        .arg("-l")
        .arg(args.level.to_string());

    let mut c_zstd = Command::new(&args.c_zstd_bin);
    c_zstd
        .args(["-q", "-f"])
        .arg(format!("-{}", args.level))
        .arg(input)
        .arg("-o")
        .arg(output);

    vec![
        ("upstream", upstream),
        ("current", current),
        ("c_zstd", c_zstd),
    ]
}

fn run_benchmarks(args: &Args) -> io::Result<Vec<Row>> {
    fs::create_dir_all(&args.output_dir)?;
    let mut fixtures = fs::read_dir(&args.fixtures)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    fixtures.sort();

    let mut rows = Vec::new();
    for fixture in fixtures {
        let fixture_name = fixture
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| io::Error::other("fixture name is not UTF-8"))?
            .to_string();
        for (encoder, mut command) in encoder_commands(args, &fixture, Path::new("{output}")) {
            let output = args
                .output_dir
                .join(format!("{fixture_name}.{encoder}.zst"));
            replace_output_arg(&mut command, &output);
            run_command_silent(&mut command)?;
            verify_decoded_matches(&args.c_zstd_bin, &output, &fixture)?;
            fs::remove_file(&output)?;

            let mut elapsed_runs = Vec::with_capacity(args.runs);
            let mut cpu_runs = Vec::with_capacity(args.runs);
            let mut rss_runs = Vec::with_capacity(args.runs);
            let mut size = 0;
            for _ in 0..args.runs {
                if !args.no_sync {
                    let mut sync = Command::new("sync");
                    run_command_silent(&mut sync)?;
                }
                let (elapsed, cpu, rss) = run_timed(&command, &output)?;
                verify_decoded_matches(&args.c_zstd_bin, &output, &fixture)?;
                size = fs::metadata(&output)?.len();
                fs::remove_file(&output)?;
                elapsed_runs.push(elapsed);
                cpu_runs.push(cpu);
                rss_runs.push(rss);
            }

            rows.push(Row {
                fixture: fixture_name.clone(),
                encoder,
                bytes: size,
                elapsed: median(&mut elapsed_runs),
                cpu: median(&mut cpu_runs),
                rss: rss_runs.into_iter().max().unwrap_or(0),
            });
        }
    }
    Ok(rows)
}

fn replace_output_arg(command: &mut Command, output: &Path) {
    let args = command
        .get_args()
        .map(|arg| {
            if arg == "{output}" {
                output.as_os_str().to_os_string()
            } else {
                arg.to_os_string()
            }
        })
        .collect::<Vec<_>>();
    let program = command.get_program().to_os_string();
    *command = Command::new(program);
    command.args(args);
}

fn run_timed(command: &Command, output: &Path) -> io::Result<(f64, f64, u64)> {
    let time_file = output.with_extension("zst.time");
    let mut timed = Command::new("/usr/bin/time");
    timed
        .args(["-f", "%e\t%U\t%S\t%M", "-o"])
        .arg(&time_file)
        .arg(command.get_program())
        .args(command.get_args());
    run_command_silent(&mut timed)?;
    let text = fs::read_to_string(&time_file)?;
    fs::remove_file(&time_file)?;
    let fields = text.trim().split('\t').collect::<Vec<_>>();
    if fields.len() != 4 {
        return Err(io::Error::other(format!("unexpected time output: {text}")));
    }
    let elapsed = fields[0].parse::<f64>().unwrap_or(0.0);
    let user = fields[1].parse::<f64>().unwrap_or(0.0);
    let system = fields[2].parse::<f64>().unwrap_or(0.0);
    let rss = fields[3].parse::<u64>().unwrap_or(0);
    Ok((elapsed, user + system, rss))
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    values[values.len() / 2]
}

fn write_csv(path: &Path, rows: &[Row]) -> io::Result<()> {
    let mut csv = String::from("fixture,encoder,bytes,elapsed,cpu,rss\n");
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{:.2},{:.2},{}\n",
            csv_escape(&row.fixture),
            row.encoder,
            row.bytes,
            row.elapsed,
            row.cpu,
            row.rss
        ));
    }
    write_all(path, &csv)
}

fn pct_improvement(before: f64, after: f64) -> f64 {
    if before == 0.0 {
        0.0
    } else {
        (before - after) * 100.0 / before
    }
}

fn write_markdown(path: &Path, rows: &[Row], csv_path: &Path, commentary: &str) -> io::Result<()> {
    let mut fixtures = rows
        .iter()
        .map(|row| row.fixture.as_str())
        .collect::<Vec<_>>();
    fixtures.sort_unstable();
    fixtures.dedup();

    let headers = [
        "Fixture",
        "Upstream bytes",
        "C bytes",
        "New bytes",
        "% Improvement",
        "Upstream CPU",
        "C CPU",
        "New CPU",
        "% Improvement",
    ];
    let mut table_rows = Vec::new();
    for fixture in fixtures {
        let upstream = find_row(rows, fixture, "upstream")?;
        let current = find_row(rows, fixture, "current")?;
        let c_zstd = find_row(rows, fixture, "c_zstd")?;
        table_rows.push(vec![
            fixture.to_string(),
            format_number(upstream.bytes),
            format_number(c_zstd.bytes),
            format_number(current.bytes),
            format!(
                "{:+.1}%",
                pct_improvement(upstream.bytes as f64, current.bytes as f64)
            ),
            format!("{:.2}s", upstream.cpu),
            format!("{:.2}s", c_zstd.cpu),
            format!("{:.2}s", current.cpu),
            format!("{:+.1}%", pct_improvement(upstream.cpu, current.cpu)),
        ]);
    }

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
        "# zstd-rs Benchmark".to_string(),
        String::new(),
        format!("Source CSV: `{}`", csv_path.display()),
        String::new(),
    ];
    if !commentary.trim().is_empty() {
        lines.push(format!("Commentary: {}", commentary.trim()));
        lines.push(String::new());
    }
    lines.push("Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.".to_string());
    lines.push(String::new());
    lines.push("```text".to_string());
    lines.push(format_row(&headers, &widths));
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

fn find_row<'a>(rows: &'a [Row], fixture: &str, encoder: &str) -> io::Result<&'a Row> {
    rows.iter()
        .find(|row| row.fixture == fixture && row.encoder == encoder)
        .ok_or_else(|| io::Error::other(format!("missing {encoder} result for {fixture}")))
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
