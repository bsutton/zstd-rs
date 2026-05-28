#!/usr/bin/env python3
"""Benchmark zstd encoders and verify decoded bytes against each fixture."""

import argparse
import csv
import statistics
import subprocess
import sys
from pathlib import Path


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--fixtures",
        type=Path,
        default=Path("/tmp/zstd-bench/fixtures"),
        help="Directory containing input fixture files.",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("/tmp/zstd-current-branch-bench"),
        help="Temporary directory for compressed outputs.",
    )
    parser.add_argument(
        "--current-bin",
        default="/tmp/ruzstd-cli-huffman-maxheight",
        help="Path to the current ruzstd-cli binary.",
    )
    parser.add_argument(
        "--upstream-bin",
        default="/tmp/ruzstd-cli-baseline",
        help="Path to the upstream/baseline ruzstd-cli binary.",
    )
    parser.add_argument(
        "--c-zstd-bin",
        default="/usr/bin/zstd",
        help="Path to the C zstd binary.",
    )
    parser.add_argument("-l", "--level", type=int, default=1)
    parser.add_argument("--runs", type=int, default=3)
    parser.add_argument(
        "--csv-output",
        type=Path,
        default=Path("/tmp/zstd-rs-benchmark.csv"),
    )
    parser.add_argument(
        "--md-output",
        type=Path,
        default=Path("/tmp/zstd-rs-benchmark.md"),
    )
    parser.add_argument(
        "--no-sync",
        action="store_true",
        help="Skip sync before timed runs.",
    )
    return parser.parse_args()


def encoder_commands(args):
    return [
        (
            "upstream",
            [
                args.upstream_bin,
                "compress",
                "{input}",
                "{output}",
                "-l",
                str(args.level),
            ],
        ),
        (
            "current",
            [
                args.current_bin,
                "compress",
                "{input}",
                "{output}",
                "-l",
                str(args.level),
            ],
        ),
        (
            "c_zstd",
            [
                args.c_zstd_bin,
                "-q",
                "-f",
                f"-{args.level}",
                "{input}",
                "-o",
                "{output}",
            ],
        ),
    ]


def render(command, fixture, output):
    return [part.format(input=str(fixture), output=str(output)) for part in command]


def run_timed(command, output):
    time_file = output.with_suffix(output.suffix + ".time")
    subprocess.run(
        ["/usr/bin/time", "-f", "%e\t%U\t%S\t%M", "-o", str(time_file), *command],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    elapsed, user, system, rss = time_file.read_text().strip().split("\t")
    time_file.unlink()
    return float(elapsed), float(user) + float(system), int(rss)


def verify_decoded_matches(zstd_bin, compressed, original):
    with original.open("rb") as original_file:
        process = subprocess.Popen(
            [zstd_bin, "-q", "-d", "-c", str(compressed)],
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
                raise RuntimeError(
                    f"decoded output did not match original: {compressed}"
                )
            if not decoded:
                break
        extra = original_file.read(1)
        returncode = process.wait()
        if returncode != 0:
            raise subprocess.CalledProcessError(returncode, process.args)
        if extra:
            raise RuntimeError(f"decoded output ended early: {compressed}")


def run_benchmarks(args):
    args.output_dir.mkdir(exist_ok=True)
    rows = []
    for fixture in sorted(args.fixtures.iterdir()):
        if not fixture.is_file():
            continue
        for encoder, command_template in encoder_commands(args):
            output = args.output_dir / f"{fixture.name}.{encoder}.zst"
            command = render(command_template, fixture, output)
            subprocess.run(
                command,
                check=True,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            verify_decoded_matches(args.c_zstd_bin, output, fixture)
            output.unlink()

            elapsed_runs = []
            cpu_runs = []
            rss_runs = []
            size = 0
            for _ in range(args.runs):
                if not args.no_sync:
                    subprocess.run(["sync"], check=True, stdout=subprocess.DEVNULL)
                elapsed, cpu, rss = run_timed(command, output)
                verify_decoded_matches(args.c_zstd_bin, output, fixture)
                size = output.stat().st_size
                output.unlink()
                elapsed_runs.append(elapsed)
                cpu_runs.append(cpu)
                rss_runs.append(rss)

            rows.append(
                {
                    "fixture": fixture.name,
                    "encoder": encoder,
                    "bytes": str(size),
                    "elapsed": f"{statistics.median(elapsed_runs):.2f}",
                    "cpu": f"{statistics.median(cpu_runs):.2f}",
                    "rss": str(max(rss_runs)),
                }
            )
    return rows


def write_csv(path, rows):
    with path.open("w", newline="") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=["fixture", "encoder", "bytes", "elapsed", "cpu", "rss"],
        )
        writer.writeheader()
        writer.writerows(rows)


def pct_improvement(before, after):
    if before == 0:
        return 0.0
    return (before - after) * 100.0 / before


def write_markdown(path, rows, csv_path):
    by_fixture = {}
    for row in rows:
        by_fixture.setdefault(row["fixture"], {})[row["encoder"]] = row

    headers = [
        "Fixture",
        "Upstream bytes",
        "C bytes",
        "New bytes",
        "% Improvement",
        "Upstream CPU",
        "C CPU",
        "New CPU",
        "% Improvement",
    ]
    table_rows = []
    for fixture in sorted(by_fixture):
        encoders = by_fixture[fixture]
        upstream = encoders["upstream"]
        current = encoders["current"]
        c_zstd = encoders["c_zstd"]
        upstream_bytes = int(upstream["bytes"])
        current_bytes = int(current["bytes"])
        c_bytes = int(c_zstd["bytes"])
        upstream_cpu = float(upstream["cpu"])
        current_cpu = float(current["cpu"])
        c_cpu = float(c_zstd["cpu"])
        table_rows.append(
            [
                fixture,
                f"{upstream_bytes:,}",
                f"{c_bytes:,}",
                f"{current_bytes:,}",
                f"{pct_improvement(upstream_bytes, current_bytes):+.1f}%",
                f"{upstream_cpu:.2f}s",
                f"{c_cpu:.2f}s",
                f"{current_cpu:.2f}s",
                f"{pct_improvement(upstream_cpu, current_cpu):+.1f}%",
            ]
        )

    widths = [max(len(str(item)) for item in column) for column in zip(headers, *table_rows)]

    def fmt(row):
        return "  ".join(str(value).ljust(widths[idx]) for idx, value in enumerate(row))

    lines = [
        "# zstd-rs Benchmark",
        "",
        f"Source CSV: `{csv_path}`",
        "",
        "Percent improvements compare new/current against upstream. Each compressed output is decoded with C zstd and byte-compared against the original fixture.",
        "",
        "```text",
        fmt(headers),
        fmt(["-" * width for width in widths]),
    ]
    lines.extend(fmt(row) for row in table_rows)
    lines.extend(["```", ""])
    path.write_text("\n".join(lines))


def main():
    args = parse_args()
    rows = run_benchmarks(args)
    write_csv(args.csv_output, rows)
    write_markdown(args.md_output, rows, args.csv_output)
    print(args.csv_output)
    print(args.md_output)


if __name__ == "__main__":
    try:
        main()
    except BrokenPipeError:
        sys.exit(1)
