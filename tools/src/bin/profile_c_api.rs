use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use zstd_safe::{CCtx, CParameter};

fn main() -> io::Result<()> {
    let args = Args::parse()?;
    let input = fs::read(&args.input)?;
    let dictionary = args.dictionary.as_ref().map(fs::read).transpose()?;
    let mut total_bytes = 0usize;
    let started = Instant::now();

    for _ in 0..args.runs {
        let compressed = compress_c_api(
            black_box(input.as_slice()),
            args.level,
            args.target_c_block_size,
            dictionary.as_deref().map(black_box),
        )?;
        total_bytes = total_bytes.wrapping_add(compressed.len());
        black_box(&compressed);
    }

    eprintln!(
        "C API compressed {} bytes at level {}{} for {} runs into {} total output bytes in {:.3}s",
        input.len(),
        args.level,
        ModeDisplay {
            target_c_block_size: args.target_c_block_size,
            dictionary: args.dictionary.as_ref(),
        },
        args.runs,
        total_bytes,
        started.elapsed().as_secs_f64()
    );

    Ok(())
}

fn compress_c_api(
    input: &[u8],
    level: i32,
    target_c_block_size: Option<usize>,
    dictionary: Option<&[u8]>,
) -> io::Result<Vec<u8>> {
    let mut context = CCtx::create();
    set_parameter(&mut context, CParameter::CompressionLevel(level))?;
    if let Some(dictionary) = dictionary {
        context.load_dictionary(dictionary).map_err(c_api_error)?;
    }
    if let Some(target_c_block_size) = target_c_block_size {
        let target_c_block_size = target_c_block_size.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "TARGET_C_BLOCK_SIZE must fit in a u32",
            )
        })?;
        set_parameter(
            &mut context,
            CParameter::TargetCBlockSize(target_c_block_size),
        )?;
    }
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
    target_c_block_size: Option<usize>,
    dictionary: Option<PathBuf>,
}

impl Args {
    fn parse() -> io::Result<Self> {
        let mut raw = env::args().skip(1);
        let input = raw.next().map(PathBuf::from).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: profile_c_api INPUT [LEVEL] [RUNS] [TARGET_C_BLOCK_SIZE|DICTIONARY_PATH] [DICTIONARY_PATH]",
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
        let (target_c_block_size, dictionary) = parse_mode_args(raw.next(), raw.next())?;

        Ok(Self {
            input,
            level,
            runs,
            target_c_block_size,
            dictionary,
        })
    }
}

fn parse_mode_args(
    fourth: Option<String>,
    fifth: Option<String>,
) -> io::Result<(Option<usize>, Option<PathBuf>)> {
    let Some(fourth) = fourth else {
        return Ok((None, None));
    };

    if let Ok(target_c_block_size) = fourth.parse::<usize>() {
        return Ok((Some(target_c_block_size), fifth.map(PathBuf::from)));
    }

    if fifth.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unexpected fifth argument without targetCBlockSize",
        ));
    }

    Ok((None, Some(PathBuf::from(fourth))))
}

struct ModeDisplay<'a> {
    target_c_block_size: Option<usize>,
    dictionary: Option<&'a PathBuf>,
}

impl std::fmt::Display for ModeDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.target_c_block_size, self.dictionary) {
            (Some(target), Some(dictionary)) => {
                write!(
                    f,
                    " targetCBlockSize {target} dictionary {}",
                    dictionary.display()
                )
            }
            (Some(target), None) => write!(f, " targetCBlockSize {target}"),
            (None, Some(dictionary)) => write!(f, " dictionary {}", dictionary.display()),
            (None, None) => Ok(()),
        }
    }
}
