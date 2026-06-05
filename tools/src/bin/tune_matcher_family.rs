use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    env, fs,
    hash::Hasher,
    io,
    path::{Path, PathBuf},
    process::Command,
};

use zstd_rs_tools::{
    benchmark_tmp, csv_escape, parse_value, repo_root, require_value, run_command_silent,
    verify_decoded_matches, write_all,
};

type Grid = Vec<(&'static str, Vec<&'static str>)>;

#[derive(Clone)]
struct Preset {
    fixtures: Vec<&'static str>,
    grid: Grid,
}

#[derive(Clone)]
struct ResultRow {
    env: Vec<(String, String)>,
    bytes: u64,
    cpu: f64,
    rss: u64,
}

struct RunContext<'a> {
    fixtures_root: &'a Path,
    level: &'a str,
    runs: usize,
    current_bin: &'a Path,
    c_zstd_bin: &'a Path,
    tmp: &'a Path,
}

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let family = require_value(&args, "--family")?;
    let preset = preset(&family)?;
    let repo = repo_root();
    let tmp = benchmark_tmp();
    fs::create_dir_all(&tmp)?;

    let fixtures_root = PathBuf::from(parse_value(
        &args,
        "--fixtures-root",
        repo.join("benchmarks")
            .join("fixtures")
            .join("broad-local")
            .display()
            .to_string(),
    ));
    let level = parse_value(&args, "--level", "1");
    let runs = parse_value(&args, "--runs", "1")
        .parse::<usize>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    let top = parse_value(&args, "--top", "10")
        .parse::<usize>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    let csv_output = optional_path(&args, "--csv-output");
    let md_output = optional_path(&args, "--md-output");
    let current_bin = repo.join("target").join("release").join("ruzstd-cli");
    let c_zstd_bin = PathBuf::from("/usr/bin/zstd");

    let context = RunContext {
        fixtures_root: &fixtures_root,
        level: &level,
        runs,
        current_bin: &current_bin,
        c_zstd_bin: &c_zstd_bin,
        tmp: &tmp,
    };
    let baseline = run_current(&context, &preset.fixtures)?;

    let mut results = Vec::new();
    for tune_env in candidate_envs(&preset.grid) {
        results.push(run_candidate(&context, &preset.fixtures, &tune_env)?);
    }
    results.sort_by(|left, right| {
        left.bytes
            .cmp(&right.bytes)
            .then_with(|| left.cpu.partial_cmp(&right.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| left.rss.cmp(&right.rss))
    });

    println!("family={family}");
    println!(
        "baseline_bytes={} baseline_cpu={:.2}s",
        baseline.bytes, baseline.cpu
    );
    for (index, row) in results.iter().take(top).enumerate() {
        println!(
            "{:02} bytes={} cpu={:.2}s rss={} env={}",
            index + 1,
            row.bytes,
            row.cpu,
            row.rss,
            env_text(&row.env, " ")
        );
    }

    if let Some(path) = csv_output {
        write_csv(&path, &results[..results.len().min(top)])?;
    }
    if let Some(path) = md_output {
        write_markdown(
            &path,
            &family,
            &baseline,
            &results[..results.len().min(top)],
        )?;
    }

    Ok(())
}

fn optional_path(args: &[String], name: &str) -> Option<PathBuf> {
    args.windows(2)
        .find_map(|window| (window[0] == name).then(|| PathBuf::from(&window[1])))
}

fn preset(family: &str) -> io::Result<Preset> {
    let cargo = vec![
        "repo_Cargo.lock",
        "generated_go.sum",
        "generated_poetry.lock",
        "generated_yarn.lock",
    ];
    let composer = vec![
        "generated_composer.lock",
        "generated_pipfile.lock",
        "generated_package-lock.json",
        "generated_go.sum",
    ];
    let tsconfig = vec![
        "generated_tsconfig.json",
        "generated_deno.json",
        "generated_nx.json",
    ];

    let preset = match family {
        "cargo-lock" => Preset {
            fixtures: cargo,
            grid: vec![
                ("RUZSTD_TUNE_LOCKFILE_PROBE_STEP", vec!["2", "3", "4"]),
                (
                    "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["0", "1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX",
                    vec!["0", "1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN",
                    vec!["1", "2", "3"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN",
                    vec!["2", "3"],
                ),
            ],
        },
        "cargo-lock-encoder" | "composer-encoder" => Preset {
            fixtures: if family.starts_with("cargo") {
                cargo
            } else {
                composer
            },
            grid: encoder_grid(),
        },
        "cargo-lock-literal-encoder" => Preset {
            fixtures: cargo,
            grid: literal_encoder_grid(),
        },
        "cargo-lock-zero-literal-window" => Preset {
            fixtures: cargo,
            grid: vec![
                (
                    "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_MAX_MATCH_LEN",
                    vec!["5", "6", "7", "8"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_MIN_OFFSET_BITS",
                    vec!["9", "10", "11", "12"],
                ),
            ],
        },
        "cargo-lock-next-position" => Preset {
            fixtures: cargo,
            grid: next_position_grid(vec!["5", "6", "7", "8", "9"], vec!["6", "8", "10"]),
        },
        "cargo-lock-next-position-skip" => Preset {
            fixtures: cargo,
            grid: vec![
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS",
                    vec!["1", "2", "3"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN",
                    vec!["7", "8", "9"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT",
                    vec!["6", "8"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN",
                    vec!["0", "1", "2"],
                ),
            ],
        },
        "cargo-lock-next-position-loss" => Preset {
            fixtures: cargo,
            grid: vec![
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS",
                    vec!["2", "3"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN",
                    vec!["7", "8"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX",
                    vec!["0", "1", "2", "3"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT",
                    vec!["6", "8"],
                ),
                ("RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD", vec!["2"]),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT",
                    vec!["2", "3"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN",
                    vec!["0", "1", "2"],
                ),
            ],
        },
        "cargo-lock-next-position-wide" => Preset {
            fixtures: cargo,
            grid: vec![
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS",
                    vec!["2", "3", "4"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN",
                    vec!["7", "9", "12"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX",
                    vec!["0", "1"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT",
                    vec!["6", "8"],
                ),
                ("RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD", vec!["2"]),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT",
                    vec!["3", "4"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN",
                    vec!["0", "1", "2"],
                ),
            ],
        },
        "cargo-lock-splits" => Preset {
            fixtures: cargo,
            grid: vec![
                ("RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS", vec!["0", "1"]),
                ("RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT", vec!["0", "1"]),
            ],
        },
        "cargo-lock-combined" => Preset {
            fixtures: cargo,
            grid: vec![
                ("RUZSTD_TUNE_LOCKFILE_PROBE_STEP", vec!["2", "3"]),
                (
                    "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["0", "1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX",
                    vec!["0", "1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN",
                    vec!["1", "2", "3"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN",
                    vec!["2", "3"],
                ),
                (
                    "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH",
                    vec!["heuristic", "allsections"],
                ),
                (
                    "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES",
                    vec!["16", "64"],
                ),
                ("RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG", vec!["7", "8"]),
                ("RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES", vec!["64", "256"]),
            ],
        },
        "cargo-lock-combined-lazy" => Preset {
            fixtures: cargo,
            grid: vec![
                (
                    "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN",
                    vec!["2", "3"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS",
                    vec!["2", "3"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN",
                    vec!["7", "9"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX",
                    vec!["0", "1"],
                ),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT",
                    vec!["6", "8"],
                ),
                ("RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD", vec!["2"]),
                (
                    "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT",
                    vec!["3", "4"],
                ),
                ("RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN", vec!["0", "1"]),
            ],
        },
        "composer" => Preset {
            fixtures: composer,
            grid: vec![
                ("RUZSTD_TUNE_COMPOSER_PROBE_STEP", vec!["3", "4"]),
                (
                    "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN",
                    vec!["2", "3"],
                ),
            ],
        },
        "composer-repeat-zero-literals" => Preset {
            fixtures: composer,
            grid: vec![
                (
                    "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["0", "1", "2"],
                ),
                (
                    "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY",
                    vec!["0", "1"],
                ),
            ],
        },
        "composer-repeatkind-wide" => Preset {
            fixtures: composer,
            grid: vec![
                ("RUZSTD_TUNE_COMPOSER_PROBE_STEP", vec!["5", "6"]),
                (
                    "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["2", "3", "4"],
                ),
            ],
        },
        "composer-window-disable" => Preset {
            fixtures: composer,
            grid: vec![
                ("RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE", vec!["0", "1"]),
                ("RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG", vec!["7", "8"]),
                (
                    "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES",
                    vec!["16", "64"],
                ),
                ("RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES", vec!["64", "256"]),
            ],
        },
        "composer-zero-literal-repeat-limit" => Preset {
            fixtures: composer,
            grid: vec![
                (
                    "RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT",
                    vec!["1", "2", "3"],
                ),
                (
                    "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["0", "1", "2"],
                ),
            ],
        },
        "composer-partitions" => Preset {
            fixtures: composer,
            grid: vec![(
                "RUZSTD_TUNE_COMPOSER_MAX_PARTITIONS",
                vec!["1", "2", "3", "4", "5", "6", "7", "8"],
            )],
        },
        "composer-combined" => Preset {
            fixtures: composer,
            grid: vec![
                ("RUZSTD_TUNE_COMPOSER_PROBE_STEP", vec!["3", "4"]),
                (
                    "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX",
                    vec!["1", "2"],
                ),
                (
                    "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN",
                    vec!["2", "3"],
                ),
                (
                    "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH",
                    vec!["heuristic", "allsections"],
                ),
                (
                    "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES",
                    vec!["16", "64"],
                ),
                ("RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG", vec!["7", "8"]),
                ("RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES", vec!["64", "256"]),
            ],
        },
        "structured-json" => Preset {
            fixtures: vec!["generated_package.json", "generated_turbo.json"],
            grid: vec![(
                "RUZSTD_TUNE_STRUCTURED_JSON_PROBE_STEP",
                vec!["1", "2", "3"],
            )],
        },
        "tsconfig-json-encoder" => Preset {
            fixtures: tsconfig,
            grid: literal_encoder_grid(),
        },
        "tsconfig-json" => Preset {
            fixtures: tsconfig,
            grid: vec![("RUZSTD_TUNE_TSCONFIG_PROBE_STEP", vec!["3", "4", "5", "6"])],
        },
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported --family {family}"),
            ));
        }
    };
    Ok(preset)
}

fn encoder_grid() -> Grid {
    vec![
        (
            "RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES",
            vec!["64", "128", "256"],
        ),
        ("RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG", vec!["7", "8"]),
        (
            "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES",
            vec!["16", "64", "256", "1024"],
        ),
        (
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH",
            vec!["heuristic", "allsections"],
        ),
    ]
}

fn literal_encoder_grid() -> Grid {
    vec![
        (
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH",
            vec!["filetype", "heuristic", "allsections"],
        ),
        (
            "RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS",
            vec!["none", "1024", "2048", "4096", "8192", "16384"],
        ),
        (
            "RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES",
            vec!["none", "64", "128", "256", "512"],
        ),
    ]
}

fn next_position_grid(match_lens: Vec<&'static str>, literal_weights: Vec<&'static str>) -> Grid {
    vec![
        (
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN",
            match_lens,
        ),
        (
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT",
            literal_weights,
        ),
        (
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD",
            vec!["1", "2"],
        ),
        (
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT",
            vec!["1", "2"],
        ),
        (
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN",
            vec!["0", "1", "2"],
        ),
    ]
}

fn candidate_envs(grid: &Grid) -> Vec<Vec<(String, String)>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    build_candidate_envs(grid, 0, &mut current, &mut results);
    results
}

fn build_candidate_envs(
    grid: &Grid,
    index: usize,
    current: &mut Vec<(String, String)>,
    results: &mut Vec<Vec<(String, String)>>,
) {
    if index == grid.len() {
        results.push(current.clone());
        return;
    }
    let (key, values) = &grid[index];
    for value in values {
        current.push(((*key).to_string(), (*value).to_string()));
        build_candidate_envs(grid, index + 1, current, results);
        current.pop();
    }
}

fn run_current(context: &RunContext<'_>, fixtures: &[&str]) -> io::Result<ResultRow> {
    run_candidate(context, fixtures, &[])
}

fn run_candidate(
    context: &RunContext<'_>,
    fixtures: &[&str],
    tune_env: &[(String, String)],
) -> io::Result<ResultRow> {
    let env_hash = env_hash(tune_env);
    let mut total_bytes = 0;
    let mut total_cpu = 0.0;
    let mut max_rss = 0;

    for fixture_name in fixtures {
        let fixture = context.fixtures_root.join(fixture_name);
        let output = context
            .tmp
            .join(format!("{fixture_name}.{env_hash}.tune.zst"));
        let mut elapsed_runs = Vec::new();
        let mut cpu_runs = Vec::new();
        let mut rss_runs = Vec::new();
        let mut size = 0;

        for _ in 0..context.runs {
            let mut command = Command::new(context.current_bin);
            command
                .arg("compress")
                .arg(&fixture)
                .arg(&output)
                .arg("-l")
                .arg(context.level);
            for (key, value) in tune_env {
                command.env(key, value);
            }
            let (elapsed, cpu, rss) = run_timed(&mut command, &output)?;
            verify_decoded_matches(context.c_zstd_bin, &output, &fixture)?;
            size = fs::metadata(&output)?.len();
            fs::remove_file(&output)?;
            elapsed_runs.push(elapsed);
            cpu_runs.push(cpu);
            rss_runs.push(rss);
        }

        let cpu = median(&mut cpu_runs);
        let rss = rss_runs.into_iter().max().unwrap_or(0);
        total_bytes += size;
        total_cpu += cpu;
        max_rss = max_rss.max(rss);
    }

    Ok(ResultRow {
        env: tune_env.to_vec(),
        bytes: total_bytes,
        cpu: total_cpu,
        rss: max_rss,
    })
}

fn run_timed(command: &mut Command, output: &Path) -> io::Result<(f64, f64, u64)> {
    let time_file = output.with_extension("zst.time");
    let program = command.get_program().to_os_string();
    let args = command
        .get_args()
        .map(|arg| arg.to_os_string())
        .collect::<Vec<_>>();
    let envs = command
        .get_envs()
        .filter_map(|(key, value)| value.map(|value| (key.to_os_string(), value.to_os_string())))
        .collect::<Vec<_>>();
    let mut timed = Command::new("/usr/bin/time");
    timed
        .args(["-f", "%e\t%U\t%S\t%M", "-o"])
        .arg(&time_file)
        .arg(program)
        .args(args);
    timed.envs(envs);
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

fn env_hash(env: &[(String, String)]) -> String {
    let mut hasher = DefaultHasher::new();
    for (key, value) in env {
        hasher.write(key.as_bytes());
        hasher.write(b"=");
        hasher.write(value.as_bytes());
        hasher.write(b"\0");
    }
    format!("{:012x}", hasher.finish() & 0x0000_ffff_ffff_ffff)
}

fn env_text(env: &[(String, String)], separator: &str) -> String {
    let mut env = env.to_vec();
    env.sort();
    env.into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(separator)
}

fn write_csv(path: &Path, rows: &[ResultRow]) -> io::Result<()> {
    let mut text = String::from("rank,total_bytes,total_cpu,rss,env\n");
    for (index, row) in rows.iter().enumerate() {
        text.push_str(&format!(
            "{},{},{:.4},{},{}\n",
            index + 1,
            row.bytes,
            row.cpu,
            row.rss,
            csv_escape(&env_text(&row.env, " "))
        ));
    }
    write_all(path, &text)
}

fn write_markdown(
    path: &Path,
    family: &str,
    baseline: &ResultRow,
    ranked: &[ResultRow],
) -> io::Result<()> {
    let mut lines = vec![
        format!("# Matcher tuning results: {family}"),
        String::new(),
        "## Baseline current source".to_string(),
        String::new(),
        format!("- total bytes: `{}`", baseline.bytes),
        format!("- total cpu: `{:.2}s`", baseline.cpu),
        format!("- max rss: `{}` KiB", baseline.rss),
        String::new(),
        "## Top candidates".to_string(),
        String::new(),
        "| Rank | Total bytes | Delta bytes | Total cpu | Delta cpu | Env |".to_string(),
        "| --- | ---: | ---: | ---: | ---: | --- |".to_string(),
    ];
    for (index, row) in ranked.iter().enumerate() {
        lines.push(format!(
            "| {} | {} | {:+} | {:.2}s | {:+.2}s | `{}` |",
            index + 1,
            row.bytes,
            row.bytes as i128 - baseline.bytes as i128,
            row.cpu,
            row.cpu - baseline.cpu,
            env_text(&row.env, ", ")
        ));
    }
    write_all(path, &(lines.join("\n") + "\n"))
}
