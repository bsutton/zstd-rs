use std::{
    ffi::OsStr,
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools crate is inside the repository")
        .to_path_buf()
}

pub fn benchmark_tmp() -> PathBuf {
    repo_root().join("benchmarks").join("tmp")
}

pub fn parse_value(args: &[String], name: &str, default: impl Into<String>) -> String {
    args.windows(2)
        .find_map(|window| (window[0] == name).then(|| window[1].clone()))
        .unwrap_or_else(|| default.into())
}

pub fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

pub fn require_value(args: &[String], name: &str) -> io::Result<String> {
    args.windows(2)
        .find_map(|window| (window[0] == name).then(|| window[1].clone()))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("missing {name}")))
}

pub fn ensure_clean_dir(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)?;
    for child in fs::read_dir(path)? {
        let child = child?.path();
        let metadata = fs::symlink_metadata(&child)?;
        if metadata.is_dir() {
            fs::remove_dir_all(child)?;
        } else {
            fs::remove_file(child)?;
        }
    }
    Ok(())
}

pub fn xorshift(seed: u32, size: usize) -> Vec<u8> {
    let mut state = seed;
    let mut data = Vec::with_capacity(size);
    for _ in 0..size {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        data.push((state & 0xff) as u8);
    }
    data
}

pub fn repeated_text(size: usize) -> Vec<u8> {
    let parts: [&[u8]; 3] = [
        b"the quick brown fox jumps over the lazy dog\n",
        b"zstd-rs fastest encoder repeated text fixture\n",
        b"0123456789 abcdefghijklmnopqrstuvwxyz\n",
    ];
    let mut data = Vec::with_capacity(size);
    while data.len() < size {
        for part in parts {
            data.extend_from_slice(part);
            if data.len() >= size {
                break;
            }
        }
    }
    data.truncate(size);
    data
}

pub fn json_logs(size: usize) -> Vec<u8> {
    let levels = ["INFO", "WARN", "DEBUG", "ERROR"];
    let services = ["api", "worker", "billing", "search"];
    let mut data = Vec::with_capacity(size);
    let mut idx = 0usize;
    while data.len() < size {
        let row = format!(
            "{{\"ts\":\"2026-05-29T00:{:02}:{:02}Z\",\"level\":\"{}\",\"service\":\"{}\",\"tenant\":{},\"request_id\":\"req-{idx:08x}\",\"message\":\"deterministic synthetic log entry\",\"latency_ms\":{}}}\n",
            idx % 60,
            (idx * 7) % 60,
            levels[idx % levels.len()],
            services[idx % services.len()],
            idx % 23,
            (idx * 37) % 2000,
        );
        data.extend_from_slice(row.as_bytes());
        idx += 1;
    }
    data.truncate(size);
    data
}

pub fn cross_block_repetition(size: usize) -> Vec<u8> {
    let prefix = xorshift(0x1357_9bdf, 4096);
    let mut phrase = b"cross-block-repetition:".to_vec();
    phrase.extend(0u8..32);
    phrase.push(b'\n');

    let mut block = prefix;
    for _ in 0..2048 {
        block.extend_from_slice(&phrase);
    }

    let mut data = Vec::with_capacity(size);
    while data.len() < size {
        data.extend_from_slice(&block);
        data.extend_from_slice(&xorshift(data.len() as u32 + 1, 257));
    }
    data.truncate(size);
    data
}

pub fn write_fixture(output_dir: &Path, name: &str, data: &[u8]) -> io::Result<PathBuf> {
    fs::create_dir_all(output_dir)?;
    let path = output_dir.join(name);
    fs::write(&path, data)?;
    Ok(path)
}

pub fn repeated_chunks(chunks: &[&str], target_bytes: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(target_bytes);
    let mut index = 0usize;
    while data.len() < target_bytes {
        data.extend_from_slice(chunks[index % chunks.len()].as_bytes());
        index += 1;
    }
    data
}

pub fn run_command_silent(command: &mut Command) -> io::Result<()> {
    let status = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("command failed: {command:?}")))
    }
}

pub fn verify_decoded_matches(
    zstd_bin: &Path,
    compressed: &Path,
    original: &Path,
) -> io::Result<()> {
    let mut process = Command::new(zstd_bin)
        .args(["-q", "-d", "-c"])
        .arg(compressed)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let mut stdout = process
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("missing zstd stdout"))?;
    let mut original_file = fs::File::open(original)?;
    let mut decoded = [0u8; 1024 * 1024];
    let mut expected = vec![0u8; 1024 * 1024];
    loop {
        let decoded_len = stdout.read(&mut decoded)?;
        let expected_len = original_file.read(&mut expected[..decoded_len.max(1)])?;
        if decoded_len == 0 {
            if expected_len != 0 {
                return Err(io::Error::other(format!(
                    "decoded output ended early: {}",
                    compressed.display()
                )));
            }
            break;
        }
        if expected_len != decoded_len {
            let _ = process.kill();
            let _ = process.wait();
            return Err(io::Error::other(format!(
                "decoded output length differed from original: {}",
                compressed.display()
            )));
        }
        if decoded[..decoded_len] != expected[..decoded_len] {
            let _ = process.kill();
            let _ = process.wait();
            return Err(io::Error::other(format!(
                "decoded output did not match original: {}",
                compressed.display()
            )));
        }
    }
    let status = process.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("zstd decode failed: {status}")))
    }
}

pub fn path_has_extension(path: &Path, extension: &str) -> bool {
    path.extension() == Some(OsStr::new(extension))
}

pub fn csv_escape(value: &str) -> String {
    if value.contains([',', '"', '\n']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub fn write_all(path: &Path, text: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(text.as_bytes())
}
