use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use zstd_safe::{CCtx, CParameter};

fn main() -> io::Result<()> {
    let args = Args::parse()?;
    let input = fs::read(&args.input)?;
    let mut total_bytes = 0usize;
    let started = Instant::now();

    for _ in 0..args.runs {
        let compressed = compress_c_api(black_box(input.as_slice()), args.level)?;
        total_bytes = total_bytes.wrapping_add(compressed.len());
        black_box(&compressed);
    }

    eprintln!(
        "C API compressed {} bytes at level {} for {} runs into {} total output bytes in {:.3}s",
        input.len(),
        args.level,
        args.runs,
        total_bytes,
        started.elapsed().as_secs_f64()
    );

    Ok(())
}

fn compress_c_api(input: &[u8], level: i32) -> io::Result<Vec<u8>> {
    let mut context = CCtx::create();
    set_parameter(&mut context, CParameter::CompressionLevel(level))?;
    set_parameter(&mut context, CParameter::ChecksumFlag(false))?;
    context
        .set_pledged_src_size(Some(input.len() as u64))
        .map_err(c_api_error)?;
    let mut output = Vec::with_capacity(zstd_safe::compress_bound(input.len()));
    context.compress2(&mut output, input).map_err(c_api_error)?;
    Ok(output)
}

fn set_parameter(context: &mut CCtx<'_>, parameter: CParameter) -> io::Result<()> {
    context
        .set_parameter(parameter)
        .map(|_| ())
        .map_err(c_api_error)
}

fn c_api_error(code: usize) -> io::Error {
    io::Error::other(format!(
        "zstd C API error {code}: {}",
        zstd_safe::get_error_name(code)
    ))
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
                "usage: profile_c_api INPUT [LEVEL] [RUNS]",
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
