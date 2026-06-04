#!/usr/bin/env python3
"""Generate deterministic local fixtures for byte-verified zstd benchmarks."""

import argparse
import json
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
BENCHMARK_TMP = REPO_ROOT / "benchmarks" / "tmp"


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=BENCHMARK_TMP / "expanded-fixtures",
        help="Directory to write generated fixtures into.",
    )
    return parser.parse_args()


def xorshift(seed, size):
    state = seed & 0xFFFFFFFF
    data = bytearray()
    for _ in range(size):
        state ^= (state << 13) & 0xFFFFFFFF
        state ^= state >> 17
        state ^= (state << 5) & 0xFFFFFFFF
        data.append(state & 0xFF)
    return bytes(data)


def repeated_text(size):
    parts = [
        b"the quick brown fox jumps over the lazy dog\n",
        b"zstd-rs fastest encoder repeated text fixture\n",
        b"0123456789 abcdefghijklmnopqrstuvwxyz\n",
    ]
    data = bytearray()
    while len(data) < size:
        for part in parts:
            data.extend(part)
            if len(data) >= size:
                break
    return bytes(data[:size])


def json_logs(size):
    data = bytearray()
    idx = 0
    while len(data) < size:
        row = {
            "ts": f"2026-05-29T00:{idx % 60:02d}:{(idx * 7) % 60:02d}Z",
            "level": ["INFO", "WARN", "DEBUG", "ERROR"][idx % 4],
            "service": ["api", "worker", "billing", "search"][idx % 4],
            "tenant": idx % 23,
            "request_id": f"req-{idx:08x}",
            "message": "deterministic synthetic log entry",
            "latency_ms": (idx * 37) % 2000,
        }
        data.extend(json.dumps(row, separators=(",", ":")).encode())
        data.append(0x0A)
        idx += 1
    return bytes(data[:size])


def cross_block_repetition(size):
    prefix = xorshift(0x13579BDF, 4096)
    phrase = b"cross-block-repetition:" + bytes(range(32)) + b"\n"
    block = prefix + phrase * 2048
    data = bytearray()
    while len(data) < size:
        data.extend(block)
        data.extend(xorshift(len(data) + 1, 257))
    return bytes(data[:size])


def write_fixture(output_dir, name, data):
    path = output_dir / name
    path.write_bytes(data)
    return path


def main():
    args = parse_args()
    args.output_dir.mkdir(parents=True, exist_ok=True)

    fixtures = {
        "boundary_000000.bin": b"",
        "boundary_000001.bin": b"a",
        "boundary_000006.bin": b"abcdef",
        "boundary_000063.bin": repeated_text(63),
        "boundary_000128.bin": repeated_text(128),
        "boundary_000256.bin": repeated_text(256),
        "boundary_004096.bin": repeated_text(4096),
        "boundary_131072.bin": repeated_text(128 * 1024),
        "rle_001m.bin": b"Z" * (1024 * 1024),
        "repeated_text_001m.txt": repeated_text(1024 * 1024),
        "json_logs_001m.jsonl": json_logs(1024 * 1024),
        "cross_block_001m.bin": cross_block_repetition(1024 * 1024),
        "xorshift_001m.bin": xorshift(0xC0FFEE, 1024 * 1024),
    }

    for name, data in fixtures.items():
        path = write_fixture(args.output_dir, name, data)
        print(f"{path}\t{len(data)}")


if __name__ == "__main__":
    main()
