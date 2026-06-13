// ! Utilities for displaying a progress monitor to track compression/decompression/whatever else
//!
//! This implementation relies heavily on the `indicatif` crate, see <https://docs.rs/indicatif>cargo hack check --feature-powerset --exclude-features rustc-dep-of-std

use std::{
    fmt::Write,
    io::{IsTerminal, Read},
    time::Duration,
};

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use tracing::info;

/// A generic wrapper around a reader that keeps track of how many bytes have been read
/// from the total.
///
/// This wrapper has a lock on standard out for the lifetime of the monitor
pub struct ProgressMonitor<R: Read> {
    /// The total amount that the reader will read
    pub total: usize,
    /// Amount read so far
    pub read: usize,
    /// The internal reader
    reader: R,
    progress_bar: Option<ProgressBar>,
}

impl<R: Read> ProgressMonitor<R> {
    /// Create a new progress monitor, initialized with zero bytes read
    pub fn new(reader: R, size: usize) -> Self {
        if !std::io::stderr().is_terminal() {
            return Self::without_progress(reader, size);
        }

        Self::with_progress(reader, size)
    }

    fn with_progress(reader: R, size: usize) -> Self {
        // https://docs.rs/indicatif/latest/indicatif/index.html#templates
        let style = ProgressStyle::with_template(
            "{wide_bar} {binary_bytes}/{binary_total_bytes}  \n[est. {eta} remaining]",
        )
        .unwrap();
        let progress_bar = ProgressBar::new(size as u64).with_style(style);
        // The default is 20hz, this reduces rendering overhead
        progress_bar.set_draw_target(ProgressDrawTarget::stderr_with_hz(8));
        Self {
            reader,
            total: size,
            read: 0,
            progress_bar: Some(progress_bar),
        }
    }

    fn without_progress(reader: R, size: usize) -> Self {
        Self {
            reader,
            total: size,
            read: 0,
            progress_bar: None,
        }
    }

    /// This function is called whenever a new read is made, and is responsible for updating the UI
    fn update(&mut self, delta: u64) {
        let Some(progress_bar) = &self.progress_bar else {
            return;
        };

        progress_bar.inc(delta);
        if self.total == self.read && !progress_bar.is_finished() {
            progress_bar.finish_and_clear();
            info!(
                "processed {} in {} ({}/s avg)",
                fmt_size(self.total as f64),
                fmt_duration(progress_bar.elapsed()),
                fmt_size(self.total as f64 / progress_bar.elapsed().as_secs_f64())
            );
        }
    }
}

impl<R: Read> Read for ProgressMonitor<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Fall back on the internally stored reader, but filch the number of bytes read
        // along the way
        let out = self.reader.read(buf)?;
        self.read += out;
        self.update(out as u64);
        Ok(out)
    }
}

/// Converts a quantity in bytes to a human readable size, "GiB, MiB, KiB, etc"
pub fn fmt_size(size_in_bytes: f64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let order_of_magnitude = (size_in_bytes).log10() as usize;
    // Overflow to the next order of magnitude if there are more than `upper_bound` figures
    // before the decimal
    let upper_bound = 3;
    let unit_index = (order_of_magnitude / upper_bound).clamp(0, units.len() - 1);
    let decimal = size_in_bytes / 2_f64.powi((unit_index * 10) as i32);
    // Only use a decimal if displaying a unit larger than a byte
    if unit_index > 0 {
        format!("{:.2}{}", decimal, units[unit_index])
    } else {
        format!("{:.0}{}", decimal, units[unit_index])
    }
}

/// Converts a [`std::time::Duration`] to a human readable format
fn fmt_duration(duration: Duration) -> String {
    let as_secs = duration.as_secs_f64();
    let as_min = (as_secs / 60.0).floor() as usize;
    // When displayed in long form, the value shown
    let secs_portion: f64 = as_secs % 60.0;
    let min_portion: usize = ((as_secs - secs_portion) as usize / 60) % 60;
    let hr_portion: usize = ((as_min - min_portion) / 60) % 60;

    let mut output = String::with_capacity(8);
    if hr_portion > 0 {
        write!(&mut output, "{hr_portion}h ").unwrap();
    }
    if min_portion > 0 {
        write!(&mut output, "{min_portion}m ").unwrap();
    }
    // Formatting for seconds is fairly manual
    // to provide a "useful" level of precision
    if as_secs > 60.0 && secs_portion != 0.0 {
        // Zero points of precision
        write!(&mut output, "{:.0}s", secs_portion.round()).unwrap();
    } else if secs_portion > 4.0 {
        // One point of precision
        write!(&mut output, "{secs_portion:.1}s").unwrap();
    } else if secs_portion > 1.0 {
        // Two points of precision
        write!(&mut output, "{secs_portion:.2}s").unwrap();
    } else if secs_portion > 0.0 {
        // Display as ms with two units of precision
        write!(&mut output, "{:.2}ms", secs_portion * 1000.0).unwrap();
    }
    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        fs::File,
        io::BufReader,
        io::{Cursor, Read},
        time::Duration,
    };

    use super::{fmt_duration, fmt_size};
    use crate::progress::ProgressMonitor;

    #[test]
    fn human_readable_filesize() {
        // Bytes
        assert_eq!(&fmt_size(100.0), "100B");
        // Kibibytes
        assert_eq!(&fmt_size(12.0 * 2.0_f64.powi(10)), "12.00KiB");
        // Mebibytes
        assert_eq!(&fmt_size(7.0 * 2.0_f64.powi(20)), "7.00MiB");
        // Gibibytes
        assert_eq!(&fmt_size(123.0 * 2.0_f64.powi(30)), "123.00GiB");
    }

    #[test]
    fn human_readable_duration() {
        assert_eq!(&fmt_duration(Duration::from_millis(7)), "7.00ms");
        assert_eq!(&fmt_duration(Duration::from_millis(1500)), "1.50s");
        assert_eq!(&fmt_duration(Duration::from_secs(30)), "30.0s");
        assert_eq!(&fmt_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(&fmt_duration(Duration::from_secs(5 * 60)), "5m");
        assert_eq!(&fmt_duration(Duration::from_secs(3 * 60 * 60)), "3h");
        assert_eq!(
            &fmt_duration(Duration::from_secs(60 * 60 + 20 * 60 + 30)),
            "1h 20m 30s"
        );
    }

    #[test]
    fn hidden_progress_monitor_reads_input() {
        let input = b"progress-free benchmark input";
        let mut monitor = ProgressMonitor::without_progress(Cursor::new(input), input.len());
        let mut output = Vec::new();

        monitor.read_to_end(&mut output).unwrap();

        assert_eq!(output, input);
        assert_eq!(monitor.read, input.len());
    }

    #[test]
    #[ignore]
    fn best_level_progress_monitor_round_trips_external_fixture_from_env() {
        let fixture = std::env::var("RUZSTD_BEST_FIXTURE")
            .expect("set RUZSTD_BEST_FIXTURE to a fixture path");
        let data = fs::read(&fixture).expect("fixture must be readable");
        let source_file = File::open(&fixture).expect("fixture must reopen");
        let reader = BufReader::new(source_file);
        let monitor = ProgressMonitor::without_progress(reader, data.len());
        let mut compressed = Vec::new();

        ruzstd::encoding::compress(
            monitor,
            &mut compressed,
            ruzstd::encoding::CompressionLevel::Best,
        );

        let mut decoded = Vec::with_capacity(data.len());
        zstd::stream::copy_decode(compressed.as_slice(), &mut decoded)
            .expect("progress monitor output should decode with C zstd");
        assert_eq!(decoded, data);

        let temp_output = std::env::temp_dir().join("ruzstd-progress-monitor-best.zst");
        let source_file = File::open(&fixture).expect("fixture must reopen");
        let reader = BufReader::new(source_file);
        let monitor = ProgressMonitor::without_progress(reader, data.len());
        let mut output_file = File::create(&temp_output).expect("temp output must be creatable");
        ruzstd::encoding::compress(
            monitor,
            &mut output_file,
            ruzstd::encoding::CompressionLevel::Best,
        );
        drop(output_file);

        let decode = std::process::Command::new("/usr/bin/zstd")
            .args(["-q", "-d", "-c", temp_output.to_str().expect("utf8 path")])
            .output()
            .expect("external zstd must run");
        assert!(
            decode.status.success(),
            "external zstd failed: {}",
            String::from_utf8_lossy(&decode.stderr)
        );
        assert_eq!(decode.stdout, data);
        let _ = fs::remove_file(&temp_output);
    }
}
