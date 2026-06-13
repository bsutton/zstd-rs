use std::{env, io, path::PathBuf};

use zstd_rs_tools::{
    benchmark_tmp, cross_block_repetition, has_flag, json_logs, parse_value, repeated_text,
    write_fixture, xorshift,
};

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if has_flag(&args, "--help") || has_flag(&args, "-h") {
        print_help();
        return Ok(());
    }
    let output_dir = PathBuf::from(parse_value(
        &args,
        "--output-dir",
        benchmark_tmp()
            .join("expanded-fixtures")
            .display()
            .to_string(),
    ));

    let fixtures: Vec<(&str, Vec<u8>)> = vec![
        ("boundary_000000.bin", Vec::new()),
        ("boundary_000001.bin", b"a".to_vec()),
        ("boundary_000006.bin", b"abcdef".to_vec()),
        ("boundary_000063.bin", repeated_text(63)),
        ("boundary_000128.bin", repeated_text(128)),
        ("boundary_000256.bin", repeated_text(256)),
        ("boundary_004096.bin", repeated_text(4096)),
        ("boundary_131072.bin", repeated_text(128 * 1024)),
        ("rle_001m.bin", vec![b'Z'; 1024 * 1024]),
        ("repeated_text_001m.txt", repeated_text(1024 * 1024)),
        ("json_logs_001m.jsonl", json_logs(1024 * 1024)),
        ("cross_block_001m.bin", cross_block_repetition(1024 * 1024)),
        ("xorshift_001m.bin", xorshift(0x00c0_ffee, 1024 * 1024)),
    ];

    for (name, data) in fixtures {
        let path = write_fixture(&output_dir, name, &data)?;
        println!("{}\t{}", path.display(), data.len());
    }

    Ok(())
}

fn print_help() {
    println!(
        "Usage: generate_zstd_fixtures [--output-dir DIR]\n\nOptions:\n  --output-dir DIR  Directory to write generated fixtures into.\n  -h, --help        Show this help message."
    );
}
