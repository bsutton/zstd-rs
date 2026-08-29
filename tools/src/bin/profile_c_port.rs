use std::{env, fs, hint::black_box, io, path::PathBuf, time::Instant};

use ruzstd::encoding::{
    compress_slice_c_level, compress_slice_c_level_with_dictionary,
    compress_slice_c_level_with_dictionary_and_target_c_block_size,
    compress_slice_c_level_with_target_c_block_size,
};

fn main() -> io::Result<()> {
    let args = Args::parse()?;
    let input = fs::read(&args.input)?;
    let dictionary = args.dictionary.as_ref().map(fs::read).transpose()?;
    let mut total_bytes = 0usize;
    let started = Instant::now();

    for _ in 0..args.runs {
        let compressed = if let Some(dictionary) = dictionary.as_deref() {
            if let Some(target_c_block_size) = args.target_c_block_size {
                compress_slice_c_level_with_dictionary_and_target_c_block_size(
                    black_box(input.as_slice()),
                    args.level,
                    black_box(dictionary),
                    target_c_block_size,
                )
                .map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("dictionary target compression failed: {err:?}"),
                    )
                })?
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "dictionary targetCBlockSize is outside C's accepted range",
                    )
                })?
            } else {
                compress_slice_c_level_with_dictionary(
                    black_box(input.as_slice()),
                    args.level,
                    black_box(dictionary),
                )
                .map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("dictionary compression failed: {err:?}"),
                    )
                })?
            }
        } else if let Some(target_c_block_size) = args.target_c_block_size {
            compress_slice_c_level_with_target_c_block_size(
                black_box(input.as_slice()),
                args.level,
                target_c_block_size,
            )
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "targetCBlockSize is outside C's accepted range",
                )
            })?
        } else {
            compress_slice_c_level(black_box(input.as_slice()), args.level)
        };
        total_bytes = total_bytes.wrapping_add(compressed.len());
        black_box(&compressed);
    }

    eprintln!(
        "compressed {} bytes at level {}{} for {} runs into {} total output bytes in {:.3}s",
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
                "usage: profile_c_port INPUT [LEVEL] [RUNS] [TARGET_C_BLOCK_SIZE|DICTIONARY_PATH] [DICTIONARY_PATH]",
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
