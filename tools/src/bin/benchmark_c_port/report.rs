use std::{cmp::Ordering, io, path::Path};

use zstd_rs_tools::{csv_escape, write_all};

use super::reference::{CBackend, CMode};
use super::Row;

pub(crate) fn write_csv(path: &Path, rows: &[Row]) -> io::Result<()> {
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

pub(crate) fn write_markdown(
    path: &Path,
    rows: &[Row],
    csv_path: &Path,
    c_backend: CBackend,
    c_mode: CMode,
    target_c_block_size: Option<usize>,
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
            "Gap is Rust compressed size versus {}{} with frame checksums disabled; positive means Rust is larger. CPU Improvement is positive when Rust uses less CPU than the C reference. Every output is decoded with C zstd and byte-compared against the original fixture.",
            c_backend.description(c_mode),
            TargetDescription(target_c_block_size),
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

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    values[values.len() / 2]
}

pub(crate) fn median_sample(values: &mut [f64]) -> f64 {
    median(values)
}

struct TargetDescription(Option<usize>);

impl std::fmt::Display for TargetDescription {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(target) => write!(formatter, " with targetCBlockSize {target}"),
            None => Ok(()),
        }
    }
}
