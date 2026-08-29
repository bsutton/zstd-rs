use std::{env, fs::File, io, path::PathBuf, time::Instant};

use ruzstd::encoding::{encode, CompressionLevel, EncoderOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let input = arguments.next().map(PathBuf::from).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: profile_streaming_api INPUT [LEVEL] [RUNS] [CHUNK_MIB] [MEMORY_MIB] [OUTPUT]",
        )
    })?;
    let level = parse(&mut arguments, 3, "level")?;
    let runs = parse(&mut arguments, 1, "runs")?;
    let chunk_mib = parse(&mut arguments, 8, "chunk MiB")?;
    let memory_mib = parse(&mut arguments, 96, "memory MiB")?;
    let output = arguments.next().map(PathBuf::from);
    if arguments.next().is_some() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many arguments").into());
    }

    let level = CompressionLevel::try_from(i32::try_from(level)?)?;
    let options = EncoderOptions::new(level)
        .with_frame_chunk_size(chunk_mib * 1024 * 1024)
        .with_memory_limit(memory_mib * 1024 * 1024);
    let started = Instant::now();
    let mut output_bytes = 0_u64;
    for _ in 0..runs {
        let source = File::open(&input)?;
        if let Some(output) = output.as_ref() {
            let mut file = File::create(output)?;
            encode(source, &mut file, options.clone())?;
            output_bytes += file.metadata()?.len();
        } else {
            let mut sink = CountingSink::default();
            encode(source, &mut sink, options.clone())?;
            output_bytes += sink.bytes;
        }
    }
    eprintln!(
        "streamed level {} for {} run(s) into {} bytes in {:.3}s",
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

#[derive(Default)]
struct CountingSink {
    bytes: u64,
}

impl io::Write for CountingSink {
    fn write(&mut self, source: &[u8]) -> io::Result<usize> {
        self.bytes += source.len() as u64;
        Ok(source.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
