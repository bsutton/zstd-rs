use std::{env, fs, io, io::Write, path::PathBuf};

const MIB: usize = 1024 * 1024;
const DEFAULT_SIZE_MIB: usize = 64;

fn main() -> io::Result<()> {
    let mut arguments = env::args_os().skip(1);
    let output = arguments.next().map(PathBuf::from).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: generate_release_corpus OUTPUT_DIRECTORY [SIZE_MIB]",
        )
    })?;
    let size_mib = arguments.next().map_or(Ok(DEFAULT_SIZE_MIB), |raw| {
        raw.to_string_lossy()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "SIZE_MIB must be an integer"))
    })?;
    if size_mib == 0 || arguments.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "SIZE_MIB must be positive and no extra arguments are accepted",
        ));
    }
    let size = size_mib.checked_mul(MIB).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "requested corpus is too large")
    })?;

    fs::create_dir_all(&output)?;
    write_repeated(output.join("zeros.bin"), size, &[0])?;
    write_repeated(
        output.join("repeated-records.bin"),
        size,
        b"2026-08-29T12:34:56Z INFO tenant=42 object=archive-000123 completed=true\n",
    )?;
    write_random(
        output.join("deterministic-random.bin"),
        size,
        0x243f_6a88_85a3_08d3,
    )?;
    write_structured_json(output.join("structured-json.jsonl"), size)?;
    write_long_distance(output.join("long-distance.bin"), size)?;

    eprintln!(
        "generated five deterministic {} MiB fixtures in {}",
        size_mib,
        output.display()
    );
    Ok(())
}

fn write_repeated(path: PathBuf, size: usize, pattern: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    let mut block = vec![0_u8; MIB];
    for (index, byte) in block.iter_mut().enumerate() {
        *byte = pattern[index % pattern.len()];
    }
    write_to_size(&mut file, size, &block)
}

fn write_random(path: PathBuf, size: usize, mut state: u64) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    let mut block = vec![0_u8; MIB];
    let mut remaining = size;
    while remaining != 0 {
        for chunk in block.chunks_exact_mut(8) {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            chunk.copy_from_slice(&state.to_le_bytes());
        }
        let count = remaining.min(block.len());
        file.write_all(&block[..count])?;
        remaining -= count;
    }
    Ok(())
}

fn write_structured_json(path: PathBuf, size: usize) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    let mut written = 0;
    let mut record = 0_u64;
    while written < size {
        let line = format!(
            "{{\"timestamp\":\"2026-08-29T12:{:02}:{:02}Z\",\"tenant\":{},\"object\":\"archive-{record:08}\",\"status\":\"complete\",\"bytes\":{}}}\n",
            (record / 60) % 60,
            record % 60,
            record % 127,
            4096 + (record % 65536),
        );
        let count = (size - written).min(line.len());
        file.write_all(&line.as_bytes()[..count])?;
        written += count;
        record += 1;
    }
    Ok(())
}

fn write_long_distance(path: PathBuf, size: usize) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    let segment_size = (size / 4).clamp(1, 8 * MIB);
    let mut segment = vec![0_u8; segment_size];
    let mut state = 0x1319_8a2e_0370_7344_u64;
    for byte in &mut segment {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        *byte = state as u8;
    }

    let mut written = 0;
    while written < size {
        let count = (size - written).min(segment.len());
        file.write_all(&segment[..count])?;
        written += count;
        if written < size {
            let gap = (size - written).min(segment_size * 2);
            write_pattern(&mut file, gap, b"long-distance-gap-")?;
            written += gap;
        }
    }
    Ok(())
}

fn write_to_size(file: &mut fs::File, size: usize, block: &[u8]) -> io::Result<()> {
    let mut written = 0;
    while written < size {
        let count = (size - written).min(block.len());
        file.write_all(&block[..count])?;
        written += count;
    }
    Ok(())
}

fn write_pattern(file: &mut fs::File, size: usize, pattern: &[u8]) -> io::Result<()> {
    let mut block = [0_u8; 64 * 1024];
    for (index, byte) in block.iter_mut().enumerate() {
        *byte = pattern[index % pattern.len()];
    }
    write_to_size(file, size, &block)
}
