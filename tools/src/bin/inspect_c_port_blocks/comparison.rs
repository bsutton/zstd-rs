use std::{fs, io, io::Write, path::Path};

use zstd_rs_tools::block_inspect::{BlockInfo, CompressedSectionInfo, SequenceMode};

#[derive(Clone, Debug)]
struct BlockDelta {
    index: usize,
    delta: i64,
    abs_delta: usize,
    rust: BlockInfo,
    c: BlockInfo,
}

#[derive(Clone, Debug)]
struct SourceGroupDelta {
    source_start: usize,
    source_end: usize,
    rust_index: usize,
    rust_count: usize,
    rust_bytes: usize,
    c_index: usize,
    c_count: usize,
    c_bytes: usize,
}

pub(crate) fn print_comparison(rust: &[BlockInfo], c: &[BlockInfo]) {
    let common = rust.len().min(c.len());
    let common_content_delta = (0..common)
        .map(|idx| rust[idx].content_size as i64 - c[idx].content_size as i64)
        .sum::<i64>();
    let common_abs_content_delta = (0..common)
        .map(|idx| rust[idx].content_size.abs_diff(c[idx].content_size))
        .sum::<usize>();
    let common_source_delta = (0..common)
        .filter_map(|idx| {
            Some(rust[idx].decompressed_size? as i64 - c[idx].decompressed_size? as i64)
        })
        .sum::<i64>();
    let common_abs_source_delta = (0..common)
        .filter_map(|idx| {
            Some(
                rust[idx]
                    .decompressed_size?
                    .abs_diff(c[idx].decompressed_size?),
            )
        })
        .sum::<usize>();
    let type_diffs = (0..common)
        .filter(|&idx| rust[idx].block_type != c[idx].block_type)
        .count();
    let first_diff = (0..common).find(|&idx| {
        rust[idx].block_type != c[idx].block_type
            || rust[idx].content_size != c[idx].content_size
            || rust[idx].source_offset != c[idx].source_offset
            || rust[idx].decompressed_size != c[idx].decompressed_size
    });
    let first_source_diff = (0..common).find(|&idx| {
        rust[idx].source_offset != c[idx].source_offset
            || rust[idx].decompressed_size != c[idx].decompressed_size
    });
    println!(
        "summary: common_blocks={common} block_count_delta={} content_delta={} abs_content_delta={} source_delta={} abs_source_delta={} type_diffs={type_diffs}",
        rust.len() as isize - c.len() as isize,
        common_content_delta,
        common_abs_content_delta,
        common_source_delta,
        common_abs_source_delta,
    );
    match first_diff {
        Some(idx) => println!(
            "first_diff={idx} rust={:?}/{}/{}/{} c={:?}/{}/{}/{}",
            rust[idx].block_type,
            rust[idx].content_size,
            source_offset_label(&rust[idx]),
            decompressed_size_label(&rust[idx]),
            c[idx].block_type,
            c[idx].content_size,
            source_offset_label(&c[idx]),
            decompressed_size_label(&c[idx])
        ),
        None if rust.len() == c.len() => println!("first_diff=none"),
        None => println!("first_diff=block_count rust={} c={}", rust.len(), c.len()),
    }
    match first_source_diff {
        Some(idx) => println!(
            "first_source_diff={idx} rust={}/{} c={}/{}",
            source_offset_label(&rust[idx]),
            decompressed_size_label(&rust[idx]),
            source_offset_label(&c[idx]),
            decompressed_size_label(&c[idx])
        ),
        None if rust.len() == c.len() => println!("first_source_diff=none"),
        None => println!(
            "first_source_diff=block_count rust={} c={}",
            rust.len(),
            c.len()
        ),
    }
    print_largest_block_deltas(rust, c);
    print_source_aligned_deltas(rust, c);
}

pub(crate) fn write_source_aligned_csv(
    path: &Path,
    rust: &[BlockInfo],
    c: &[BlockInfo],
) -> io::Result<()> {
    let mut output = fs::File::create(path)?;
    writeln!(
        output,
        "source_start,source_end,delta,abs_delta,rust_bytes,c_bytes,rust_index,rust_count,c_index,c_count,source_size"
    )?;
    for delta in source_aligned_deltas(rust, c) {
        let byte_delta = delta.rust_bytes as i64 - delta.c_bytes as i64;
        writeln!(
            output,
            "{},{},{},{},{},{},{},{},{},{},{}",
            delta.source_start,
            delta.source_end,
            byte_delta,
            delta.rust_bytes.abs_diff(delta.c_bytes),
            delta.rust_bytes,
            delta.c_bytes,
            delta.rust_index,
            delta.rust_count,
            delta.c_index,
            delta.c_count,
            delta.source_end - delta.source_start,
        )?;
    }
    Ok(())
}

fn print_largest_block_deltas(rust: &[BlockInfo], c: &[BlockInfo]) {
    let common = rust.len().min(c.len());
    let mut deltas = (0..common)
        .filter_map(|index| {
            let rust_block = rust[index].clone();
            let c_block = c[index].clone();
            let delta = rust_block.content_size as i64 - c_block.content_size as i64;
            let source_delta = match (rust_block.decompressed_size, c_block.decompressed_size) {
                (Some(rust_size), Some(c_size)) => rust_size as i64 - c_size as i64,
                _ => 0,
            };
            let type_changed = rust_block.block_type != c_block.block_type;
            (delta != 0 || source_delta != 0 || type_changed).then(|| BlockDelta {
                index,
                delta,
                abs_delta: rust_block.content_size.abs_diff(c_block.content_size),
                rust: rust_block,
                c: c_block,
            })
        })
        .collect::<Vec<_>>();
    deltas.sort_by(|left, right| {
        right
            .abs_delta
            .cmp(&left.abs_delta)
            .then_with(|| left.index.cmp(&right.index))
    });

    println!("largest_deltas:");
    if deltas.is_empty() {
        println!("delta,none");
        return;
    }
    for delta in deltas.into_iter().take(12) {
        println!(
            "delta,{},{},{},{:?},{},{},{},{},{:?},{},{},{},{}{}",
            delta.index,
            delta.delta,
            source_delta(&delta.rust, &delta.c),
            delta.rust.block_type,
            delta.rust.content_size,
            source_offset_label(&delta.rust),
            decompressed_size_label(&delta.rust),
            describe_section(delta.rust.section_info.as_ref()),
            delta.c.block_type,
            delta.c.content_size,
            source_offset_label(&delta.c),
            decompressed_size_label(&delta.c),
            describe_section(delta.c.section_info.as_ref()),
            if delta.rust.block_type == delta.c.block_type {
                ""
            } else {
                ",type_changed"
            }
        );
    }
}

fn print_source_aligned_deltas(rust: &[BlockInfo], c: &[BlockInfo]) {
    let mut deltas = source_aligned_deltas(rust, c);
    deltas.sort_by(|left, right| {
        right
            .rust_bytes
            .abs_diff(right.c_bytes)
            .cmp(&left.rust_bytes.abs_diff(left.c_bytes))
            .then_with(|| left.source_start.cmp(&right.source_start))
    });

    println!("source_aligned_deltas:");
    if deltas.is_empty() {
        println!(
            "source_aligned_summary,groups=0,total_delta=0,abs_delta=0,rust_groups=0,c_groups=0"
        );
        println!("source_delta,none");
        return;
    }

    let total_delta = deltas
        .iter()
        .map(|delta| delta.rust_bytes as i64 - delta.c_bytes as i64)
        .sum::<i64>();
    let abs_delta = deltas
        .iter()
        .map(|delta| delta.rust_bytes.abs_diff(delta.c_bytes))
        .sum::<usize>();
    let rust_groups = deltas.iter().map(|delta| delta.rust_count).sum::<usize>();
    let c_groups = deltas.iter().map(|delta| delta.c_count).sum::<usize>();
    println!(
        "source_aligned_summary,groups={},total_delta={},abs_delta={},rust_groups={},c_groups={}",
        deltas.len(),
        total_delta,
        abs_delta,
        rust_groups,
        c_groups
    );

    for delta in deltas.into_iter().take(12) {
        println!(
            "source_delta,{},{},{},{},{},{},{},{},{},{}",
            delta.source_start,
            delta.source_end,
            delta.rust_bytes as i64 - delta.c_bytes as i64,
            delta.rust_bytes,
            delta.c_bytes,
            delta.rust_index,
            delta.rust_count,
            delta.c_index,
            delta.c_count,
            delta.source_end - delta.source_start,
        );
    }
}

fn source_aligned_deltas(rust: &[BlockInfo], c: &[BlockInfo]) -> Vec<SourceGroupDelta> {
    let mut rust_index = 0usize;
    let mut c_index = 0usize;
    let mut deltas = Vec::new();

    while rust_index < rust.len() && c_index < c.len() {
        let group_rust_index = rust_index;
        let group_c_index = c_index;
        let Some(source_start) = rust[rust_index].source_offset else {
            break;
        };
        let Some(c_source_start) = c[c_index].source_offset else {
            break;
        };
        if source_start != c_source_start {
            break;
        }

        let mut rust_bytes = 0usize;
        let mut c_bytes = 0usize;
        let mut rust_count = 0usize;
        let mut c_count = 0usize;
        let Some(mut rust_end) =
            add_source_group_block(rust, &mut rust_index, &mut rust_count, &mut rust_bytes)
        else {
            break;
        };
        let Some(mut c_end) = add_source_group_block(c, &mut c_index, &mut c_count, &mut c_bytes)
        else {
            break;
        };

        while rust_end != c_end {
            if rust_end < c_end {
                let Some(next_end) =
                    add_source_group_block(rust, &mut rust_index, &mut rust_count, &mut rust_bytes)
                else {
                    break;
                };
                rust_end = next_end;
            } else {
                let Some(next_end) =
                    add_source_group_block(c, &mut c_index, &mut c_count, &mut c_bytes)
                else {
                    break;
                };
                c_end = next_end;
            }
        }

        if rust_bytes != c_bytes || rust_count != c_count {
            deltas.push(SourceGroupDelta {
                source_start,
                source_end: rust_end.max(c_end),
                rust_index: group_rust_index,
                rust_count,
                rust_bytes,
                c_index: group_c_index,
                c_count,
                c_bytes,
            });
        }
    }

    deltas
}

fn add_source_group_block(
    blocks: &[BlockInfo],
    index: &mut usize,
    count: &mut usize,
    bytes: &mut usize,
) -> Option<usize> {
    let block = blocks.get(*index)?;
    let source_end = block.source_offset? + block.decompressed_size?;
    *bytes += block.content_size + 3;
    *count += 1;
    *index += 1;
    Some(source_end)
}

pub(crate) fn source_offset_label(block: &BlockInfo) -> String {
    block
        .source_offset
        .map(|offset| offset.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub(crate) fn decompressed_size_label(block: &BlockInfo) -> String {
    block
        .decompressed_size
        .map(|size| size.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn source_delta(rust: &BlockInfo, c: &BlockInfo) -> i64 {
    match (rust.decompressed_size, c.decompressed_size) {
        (Some(rust_size), Some(c_size)) => rust_size as i64 - c_size as i64,
        _ => 0,
    }
}

fn describe_section(info: Option<&CompressedSectionInfo>) -> String {
    let Some(info) = info else {
        return "-".to_string();
    };
    format!(
        "{:?}/regen:{}/payload:{}/seqs:{}/modes:{}/{}/{}",
        info.literal_type,
        info.literal_regenerated_size,
        info.literal_payload_size,
        info.sequences,
        mode_label(info.ll_mode),
        mode_label(info.of_mode),
        mode_label(info.ml_mode)
    )
}

pub(crate) fn mode_label(mode: Option<SequenceMode>) -> &'static str {
    match mode {
        Some(SequenceMode::Predefined) => "pre",
        Some(SequenceMode::Rle) => "rle",
        Some(SequenceMode::FseCompressed) => "fse",
        Some(SequenceMode::Repeat) => "rep",
        None => "-",
    }
}
