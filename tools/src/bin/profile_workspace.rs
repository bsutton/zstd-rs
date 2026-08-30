use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use ruzstd::encoding::{CompressionLevel, EncoderWorkspace};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let input_path = args.next().map(PathBuf::from).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: profile_workspace INPUT [LEVEL] [RUNS]",
        )
    })?;
    let level = args
        .next()
        .map(|value| value.parse::<i32>())
        .transpose()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "LEVEL must be an i32"))?
        .unwrap_or(3);
    let runs = args
        .next()
        .map(|value| value.parse::<usize>())
        .transpose()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "RUNS must be a usize"))?
        .unwrap_or(10);
    if runs == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "RUNS must be greater than zero",
        ));
    }
    let level = CompressionLevel::try_from(level)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    let input = fs::read(input_path)?;
    let mut workspace = EncoderWorkspace::new(level, input.len())
        .map_err(|error| io::Error::other(error.to_string()))?;
    let output_size = EncoderWorkspace::required_output_size(input.len())
        .map_err(|error| io::Error::other(error.to_string()))?;
    let mut output = vec![0_u8; output_size];
    let mut total_bytes = 0usize;
    let started = Instant::now();

    for _ in 0..runs {
        let encoded = workspace
            .encode_into(black_box(&input), &mut output)
            .map_err(|error| io::Error::other(error.to_string()))?;
        total_bytes = total_bytes.wrapping_add(encoded.len());
        black_box(encoded);
    }

    eprintln!(
        "workspace compressed {} bytes at level {} for {} runs into {} total output bytes in {:.3}s",
        input.len(),
        level.get(),
        runs,
        total_bytes,
        started.elapsed().as_secs_f64()
    );
    Ok(())
}
