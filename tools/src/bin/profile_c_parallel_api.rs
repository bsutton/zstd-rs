use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use zstd_safe::{CCtx, CParameter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let input = arguments.next().map(PathBuf::from).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: profile_c_parallel_api INPUT [LEVEL] [RUNS] [JOB_MIB] [WORKERS]",
        )
    })?;
    let level = parse(&mut arguments, 3, "level")?;
    let runs = parse(&mut arguments, 1, "runs")?;
    let job_mib = parse(&mut arguments, 4, "job MiB")?;
    let workers = parse(&mut arguments, 2, "workers")?;
    if workers == 0 || arguments.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "workers must be non-zero and no extra arguments are accepted",
        )
        .into());
    }

    let input = fs::read(input)?;
    let level = i32::try_from(level)?;
    let job_size = u32::try_from(
        job_mib
            .checked_mul(1024 * 1024)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "job size overflow"))?,
    )?;
    let workers = u32::try_from(workers)?;
    let started = Instant::now();
    let mut output_bytes = 0_usize;
    for _ in 0..runs {
        let mut context = CCtx::create();
        set_parameter(&mut context, CParameter::CompressionLevel(level))?;
        set_parameter(&mut context, CParameter::NbWorkers(workers))?;
        set_parameter(&mut context, CParameter::JobSize(job_size))?;
        context
            .set_pledged_src_size(Some(input.len() as u64))
            .map_err(c_api_error)?;
        let mut compressed = Vec::with_capacity(zstd_safe::compress_bound(input.len()));
        context
            .compress2(&mut compressed, black_box(input.as_slice()))
            .map_err(c_api_error)?;
        output_bytes = output_bytes.wrapping_add(compressed.len());
        black_box(compressed);
    }

    eprintln!(
        "C API compressed {} bytes at level {} with {} worker(s), {} MiB jobs, for {} run(s) into {} total output bytes in {:.3}s",
        input.len(),
        level,
        workers,
        job_mib,
        runs,
        output_bytes,
        started.elapsed().as_secs_f64(),
    );
    Ok(())
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
