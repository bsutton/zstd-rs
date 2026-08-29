use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use ruzstd::encoding::{encode_all, CompressionLevel, EncoderOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let input = arguments.next().map(PathBuf::from).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: profile_bounded_api INPUT [LEVEL] [RUNS] [CHUNK_MIB] [MEMORY_MIB]",
        )
    })?;
    let level = parse(&mut arguments, 3, "level")?;
    let runs = parse(&mut arguments, 1, "runs")?;
    let chunk_mib = parse(&mut arguments, 8, "chunk MiB")?;
    let memory_mib = parse(&mut arguments, 96, "memory MiB")?;
    if arguments.next().is_some() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many arguments").into());
    }

    let input = fs::read(input)?;
    let level = CompressionLevel::try_from(i32::try_from(level)?)?;
    let options = EncoderOptions::new(level)
        .with_frame_chunk_size(chunk_mib * 1024 * 1024)
        .with_memory_limit(memory_mib * 1024 * 1024);
    let started = Instant::now();
    let mut output_bytes = 0_usize;
    for _ in 0..runs {
        let compressed = encode_all(black_box(input.as_slice()), options.clone())?;
        output_bytes = output_bytes.wrapping_add(compressed.len());
        black_box(compressed);
    }
    eprintln!(
        "bounded API compressed {} bytes at level {} for {} run(s) into {} total output bytes in {:.3}s",
        input.len(),
        level.get(),
        runs,
        output_bytes,
        started.elapsed().as_secs_f64()
    );
    Ok(())
}

fn parse(
    arguments: &mut impl Iterator<Item = std::ffi::OsString>,
    default: usize,
    name: &str,
) -> io::Result<usize> {
    arguments.next().map_or(Ok(default), |raw| {
        raw.to_string_lossy()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("invalid {name}")))
    })
}
