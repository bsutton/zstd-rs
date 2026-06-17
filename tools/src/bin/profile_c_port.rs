use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use ruzstd::encoding::compress_to_vec_c_level;

fn main() -> io::Result<()> {
    let args = Args::parse()?;
    let input = fs::read(&args.input)?;
    let mut total_bytes = 0usize;
    let started = Instant::now();

    for _ in 0..args.runs {
        let compressed = compress_to_vec_c_level(black_box(input.as_slice()), args.level);
        total_bytes = total_bytes.wrapping_add(compressed.len());
        black_box(&compressed);
    }

    eprintln!(
        "compressed {} bytes at level {} for {} runs into {} total output bytes in {:.3}s",
        input.len(),
        args.level,
        args.runs,
        total_bytes,
        started.elapsed().as_secs_f64()
    );

    Ok(())
}

struct Args {
    input: PathBuf,
    level: i32,
    runs: usize,
}

impl Args {
    fn parse() -> io::Result<Self> {
        let mut raw = env::args().skip(1);
        let input = raw.next().map(PathBuf::from).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: profile_c_port INPUT [LEVEL] [RUNS]",
            )
        })?;
        let level = raw
            .next()
            .map(|value| value.parse::<i32>())
            .transpose()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "LEVEL must be an i32"))?
            .unwrap_or(3);
        let runs = raw
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

        Ok(Self { input, level, runs })
    }
}
