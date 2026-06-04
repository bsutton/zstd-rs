#!/usr/bin/env python3
"""Sweep focused matcher-tuning settings against selected fixture families."""

import argparse
import csv
import hashlib
import itertools
import os
import statistics
import subprocess
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
TMP_ROOT = REPO_ROOT / "benchmarks" / "tmp"

CURRENT_BIN = REPO_ROOT / "target" / "release" / "ruzstd-cli"
C_ZSTD_BIN = Path("/usr/bin/zstd")


FAMILY_PRESETS = {
    "cargo-lock": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_PROBE_STEP": [2, 3, 4],
            "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX": [0, 1, 2],
            "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX": [0, 1, 2],
            "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN": [1, 2, 3],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN": [2, 3],
        },
    },
    "cargo-lock-encoder": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES": [64, 128, 256],
            "RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG": [7, 8],
            "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES": [16, 64, 256, 1024],
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH": ["heuristic", "allsections"],
        },
    },
    "cargo-lock-literal-encoder": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH": ["filetype", "heuristic", "allsections"],
            "RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS": [
                "none",
                "1024",
                "2048",
                "4096",
                "8192",
                "16384",
            ],
            "RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES": [
                "none",
                "64",
                "128",
                "256",
                "512",
            ],
        },
    },
    "cargo-lock-zero-literal-window": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_MAX_MATCH_LEN": [5, 6, 7, 8],
            "RUZSTD_TUNE_LOCKFILE_ZERO_LITERAL_WINDOW_MIN_OFFSET_BITS": [9, 10, 11, 12],
        },
    },
    "cargo-lock-next-position": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN": [5, 6, 7, 8, 9],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT": [6, 8, 10],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD": [1, 2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT": [1, 2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN": [0, 1, 2],
        },
    },
    "cargo-lock-next-position-skip": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS": [1, 2, 3],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN": [7, 8, 9],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT": [6, 8],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD": [1, 2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT": [1, 2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN": [0, 1, 2],
        },
    },
    "cargo-lock-next-position-loss": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS": [2, 3],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN": [7, 8],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX": [0, 1, 2, 3],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT": [6, 8],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD": [2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT": [2, 3],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN": [0, 1, 2],
        },
    },
    "cargo-lock-next-position-wide": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS": [2, 3, 4],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN": [7, 9, 12],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX": [0, 1],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT": [6, 8],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD": [2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT": [3, 4],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN": [0, 1, 2],
        },
    },
    "cargo-lock-splits": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_FASTEST_SPLITS": ["0", "1"],
            "RUZSTD_TUNE_LOCKFILE_COMPARE_WHOLE_TEXT": ["0", "1"],
        },
    },
    "cargo-lock-combined": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_PROBE_STEP": [2, 3],
            "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX": [0, 1, 2],
            "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX": [0, 1, 2],
            "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN": [1, 2, 3],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN": [2, 3],
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH": ["heuristic", "allsections"],
            "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES": [16, 64],
            "RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG": [7, 8],
            "RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES": [64, 256],
        },
    },
    "cargo-lock-combined-lazy": {
        "fixtures": [
            "repo_Cargo.lock",
            "generated_go.sum",
            "generated_poetry.lock",
            "generated_yarn.lock",
        ],
        "grid": {
            "RUZSTD_TUNE_LOCKFILE_REPEAT_KIND_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_LOCKFILE_SAME_END_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_LOCKFILE_SAME_END_BITS_GAIN_MIN": [2, 3],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_SKIP_LITERALS": [2, 3],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MAX_CURRENT_MATCH_LEN": [7, 9],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_LOSS_MAX": [0, 1],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_LITERAL_WEIGHT": [6, 8],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MATCH_REWARD": [2],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_OFFSET_WEIGHT": [3, 4],
            "RUZSTD_TUNE_LOCKFILE_NEXT_POSITION_MARGIN": [0, 1],
        },
    },
    "composer": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_PROBE_STEP": [3, 4],
            "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN": [2, 3],
        },
    },
    "composer-repeat-zero-literals": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX": [0, 1, 2],
            "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_ZERO_LITERALS_ONLY": [
                "0",
                "1",
            ],
        },
    },
    "composer-repeatkind-wide": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_PROBE_STEP": [5, 6],
            "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX": [2, 3, 4],
        },
    },
    "composer-window-disable": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_WINDOW_DISABLE": ["0", "1"],
            "RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG": ["7", "8"],
            "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES": ["16", "64"],
            "RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES": ["64", "256"],
        },
    },
    "composer-zero-literal-repeat-limit": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_ZERO_LITERAL_REPEAT_CANDIDATE_LIMIT": ["1", "2", "3"],
            "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX": ["0", "1", "2"],
        },
    },
    "composer-partitions": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_MAX_PARTITIONS": [1, 2, 3, 4, 5, 6, 7, 8],
        },
    },
    "composer-encoder": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES": [64, 128, 256],
            "RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG": [7, 8],
            "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES": [16, 64, 256, 1024],
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH": ["heuristic", "allsections"],
        },
    },
    "composer-combined": {
        "fixtures": [
            "generated_composer.lock",
            "generated_pipfile.lock",
            "generated_package-lock.json",
            "generated_go.sum",
        ],
        "grid": {
            "RUZSTD_TUNE_COMPOSER_PROBE_STEP": [3, 4],
            "RUZSTD_TUNE_COMPOSER_REPEAT_KIND_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_MATCH_LOSS_MAX": [1, 2],
            "RUZSTD_TUNE_DICTIONARY_SAME_START_BITS_GAIN_MIN": [2, 3],
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH": ["heuristic", "allsections"],
            "RUZSTD_TUNE_OFFSET_PREDEFINED_MAX_SEQUENCES": [16, 64],
            "RUZSTD_TUNE_OFFSET_TABLE_MAX_LOG": [7, 8],
            "RUZSTD_TUNE_REPEAT_TABLE_MAX_SEQUENCES": [64, 256],
        },
    },
    "structured-json": {
        "fixtures": [
            "generated_package.json",
            "generated_turbo.json",
        ],
        "grid": {
            "RUZSTD_TUNE_STRUCTURED_JSON_PROBE_STEP": [1, 2, 3],
        },
    },
    "tsconfig-json-encoder": {
        "fixtures": [
            "generated_tsconfig.json",
            "generated_deno.json",
            "generated_nx.json",
        ],
        "grid": {
            "RUZSTD_TUNE_FILE_TYPE_SMALL_SEQUENCE_PREDEFINED_LLML_MAX_SEQUENCES": [
                "64",
                "128",
                "256",
            ],
            "RUZSTD_TUNE_FILE_TYPE_SINGLE_STREAM_HUFFMAN_MAX_LITERALS": [
                "1024",
                "2048",
                "4096",
                "none",
            ],
            "RUZSTD_TUNE_HUFFMAN_TABLE_SEARCH": ["filetype", "heuristic", "allsections"],
        },
    },
    "tsconfig-json": {
        "fixtures": [
            "generated_tsconfig.json",
            "generated_deno.json",
            "generated_nx.json",
        ],
        "grid": {
            "RUZSTD_TUNE_TSCONFIG_PROBE_STEP": [3, 4, 5, 6],
        },
    },
}


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--family",
        choices=sorted(FAMILY_PRESETS),
        required=True,
        help="Focused family preset to tune.",
    )
    parser.add_argument(
        "--fixtures-root",
        type=Path,
        default=REPO_ROOT / "benchmarks" / "fixtures" / "broad-local",
    )
    parser.add_argument("--level", type=int, default=1)
    parser.add_argument(
        "--runs",
        type=int,
        default=1,
        help="Timed runs per fixture for each candidate.",
    )
    parser.add_argument(
        "--top",
        type=int,
        default=10,
        help="How many top candidates to print and save.",
    )
    parser.add_argument(
        "--csv-output",
        type=Path,
        default=None,
        help="Optional CSV output path.",
    )
    parser.add_argument(
        "--md-output",
        type=Path,
        default=None,
        help="Optional Markdown output path.",
    )
    return parser.parse_args()


def candidate_envs(grid):
    keys = list(grid)
    values = [grid[key] for key in keys]
    for combo in itertools.product(*values):
        yield dict(zip(keys, combo))


def verify_decoded_matches(compressed: Path, original: Path):
    with original.open("rb") as original_file:
        process = subprocess.Popen(
            [str(C_ZSTD_BIN), "-q", "-d", "-c", str(compressed)],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
        )
        assert process.stdout is not None
        while True:
            decoded = process.stdout.read(1024 * 1024)
            expected = original_file.read(len(decoded) if decoded else 1024 * 1024)
            if decoded != expected:
                process.kill()
                process.wait()
                raise RuntimeError(f"decoded output did not match original: {compressed}")
            if not decoded:
                break
        extra = original_file.read(1)
        returncode = process.wait()
        if returncode != 0:
            raise subprocess.CalledProcessError(returncode, process.args)
        if extra:
            raise RuntimeError(f"decoded output ended early: {compressed}")


def run_timed(command, env, output):
    time_file = output.with_suffix(output.suffix + ".time")
    subprocess.run(
        ["/usr/bin/time", "-f", "%e\t%U\t%S\t%M", "-o", str(time_file), *command],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        env=env,
    )
    elapsed, user, system, rss = time_file.read_text().strip().split("\t")
    time_file.unlink()
    return float(elapsed), float(user) + float(system), int(rss)


def run_candidate(fixtures_root: Path, fixtures: list[str], level: int, tune_env: dict[str, int], runs: int):
    env = os.environ.copy()
    env.update({key: str(value) for key, value in tune_env.items()})
    env_key = " ".join(f"{key}={tune_env[key]}" for key in sorted(tune_env))
    env_hash = hashlib.sha256(env_key.encode("utf-8")).hexdigest()[:12]

    total_bytes = 0
    total_cpu = 0.0
    max_rss = 0
    per_fixture = {}

    for fixture_name in fixtures:
        fixture = fixtures_root / fixture_name
        output = TMP_ROOT / f"{fixture_name}.{env_hash}.tune.zst"
        command = [
            str(CURRENT_BIN),
            "compress",
            str(fixture),
            str(output),
            "-l",
            str(level),
        ]

        elapsed_runs = []
        cpu_runs = []
        rss_runs = []
        size = 0
        for _ in range(runs):
            elapsed, cpu, rss = run_timed(command, env, output)
            verify_decoded_matches(output, fixture)
            size = output.stat().st_size
            output.unlink()
            elapsed_runs.append(elapsed)
            cpu_runs.append(cpu)
            rss_runs.append(rss)

        total_bytes += size
        total_cpu += statistics.median(cpu_runs)
        max_rss = max(max_rss, max(rss_runs))
        per_fixture[fixture_name] = {
            "bytes": size,
            "elapsed": statistics.median(elapsed_runs),
            "cpu": statistics.median(cpu_runs),
            "rss": max(rss_runs),
        }

    return {
        "env": tune_env,
        "bytes": total_bytes,
        "cpu": total_cpu,
        "rss": max_rss,
        "per_fixture": per_fixture,
    }


def baseline_env(grid):
    return {key: min(values) for key, values in grid.items()}


def baseline_current(fixtures_root: Path, fixtures: list[str], level: int, runs: int):
    total_bytes = 0
    total_cpu = 0.0
    max_rss = 0
    per_fixture = {}

    for fixture_name in fixtures:
        fixture = fixtures_root / fixture_name
        output = TMP_ROOT / f"{fixture_name}.baseline-current.zst"
        command = [
            str(CURRENT_BIN),
            "compress",
            str(fixture),
            str(output),
            "-l",
            str(level),
        ]

        elapsed_runs = []
        cpu_runs = []
        rss_runs = []
        size = 0
        for _ in range(runs):
            elapsed, cpu, rss = run_timed(command, os.environ.copy(), output)
            verify_decoded_matches(output, fixture)
            size = output.stat().st_size
            output.unlink()
            elapsed_runs.append(elapsed)
            cpu_runs.append(cpu)
            rss_runs.append(rss)

        total_bytes += size
        total_cpu += statistics.median(cpu_runs)
        max_rss = max(max_rss, max(rss_runs))
        per_fixture[fixture_name] = {
            "bytes": size,
            "elapsed": statistics.median(elapsed_runs),
            "cpu": statistics.median(cpu_runs),
            "rss": max(rss_runs),
        }

    return {
        "env": {},
        "bytes": total_bytes,
        "cpu": total_cpu,
        "rss": max_rss,
        "per_fixture": per_fixture,
    }


def write_csv(path: Path, rows):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=["rank", "total_bytes", "total_cpu", "rss", "env"],
        )
        writer.writeheader()
        writer.writerows(rows)


def write_markdown(path: Path, family: str, baseline: dict, ranked: list[dict]):
    path.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        f"# Matcher tuning results: {family}",
        "",
        "## Baseline current source",
        "",
        f"- total bytes: `{baseline['bytes']}`",
        f"- total cpu: `{baseline['cpu']:.2f}s`",
        f"- max rss: `{baseline['rss']}` KiB",
        "",
        "## Top candidates",
        "",
        "| Rank | Total bytes | Delta bytes | Total cpu | Delta cpu | Env |",
        "| --- | ---: | ---: | ---: | ---: | --- |",
    ]
    for index, row in enumerate(ranked, start=1):
        delta_bytes = row["bytes"] - baseline["bytes"]
        delta_cpu = row["cpu"] - baseline["cpu"]
        env_text = ", ".join(f"{k}={v}" for k, v in sorted(row["env"].items()))
        lines.append(
            f"| {index} | {row['bytes']} | {delta_bytes:+} | {row['cpu']:.2f}s | {delta_cpu:+.2f}s | `{env_text}` |"
        )
    path.write_text("\n".join(lines) + "\n")


def main():
    args = parse_args()
    TMP_ROOT.mkdir(parents=True, exist_ok=True)

    preset = FAMILY_PRESETS[args.family]
    baseline = baseline_current(args.fixtures_root, preset["fixtures"], args.level, args.runs)

    results = []
    for env in candidate_envs(preset["grid"]):
        result = run_candidate(args.fixtures_root, preset["fixtures"], args.level, env, args.runs)
        results.append(result)

    ranked = sorted(results, key=lambda row: (row["bytes"], row["cpu"], row["rss"]))

    print(f"family={args.family}")
    print(f"baseline_bytes={baseline['bytes']} baseline_cpu={baseline['cpu']:.2f}s")
    for index, row in enumerate(ranked[: args.top], start=1):
        env_text = " ".join(f"{k}={v}" for k, v in sorted(row["env"].items()))
        print(
            f"{index:02d} bytes={row['bytes']} cpu={row['cpu']:.2f}s rss={row['rss']} env={env_text}"
        )

    if args.csv_output:
        rows = [
            {
                "rank": index,
                "total_bytes": row["bytes"],
                "total_cpu": f"{row['cpu']:.4f}",
                "rss": row["rss"],
                "env": " ".join(f"{k}={v}" for k, v in sorted(row["env"].items())),
            }
            for index, row in enumerate(ranked[: args.top], start=1)
        ]
        write_csv(args.csv_output, rows)

    if args.md_output:
        write_markdown(args.md_output, args.family, baseline, ranked[: args.top])


if __name__ == "__main__":
    main()
