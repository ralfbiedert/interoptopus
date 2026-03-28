/// Performance report covering all instrumented functions.
#[derive(Debug)]
pub struct Report {
    pub functions: Vec<FunctionReport>,
}

impl Report {
    /// Prints a table of per-function metrics to stdout.
    ///
    /// Columns: name, avg, p50, p95, p99, p99.9, total calls.
    /// Per-row time unit is chosen so the smallest value stays ≥ 0.01; unit is color-coded.
    pub fn print_stdout(&self) {
        let col = self.functions.iter().map(|f| f.name.len()).max().unwrap_or(4).max(4);
        println!("\x1b[1m{:<col$}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}  {:>12}\x1b[0m", "name", "avg", "p50", "p95", "p99", "p99.9", "total calls");
        println!("{}", "-".repeat(col + 74));
        for f in &self.functions {
            let avg = if f.recent_durations_ns.is_empty() { 0 } else { f.recent_sum_ns / f.recent_durations_ns.len() as u64 };
            let p50 = percentile(&f.recent_durations_ns, 50, 100);
            let p95 = percentile(&f.recent_durations_ns, 95, 100);
            let p99 = percentile(&f.recent_durations_ns, 99, 100);
            let p999 = percentile(&f.recent_durations_ns, 999, 1000);
            let unit = Unit::pick(&[avg, p50, p95, p99, p999]);
            let name = format!("\x1b[1m{:<col$}\x1b[0m", f.name);
            println!("{}  {}  {}  {}  {}  {}  {:>12}", name, unit.fmt(avg), unit.fmt(p50), unit.fmt(p95), unit.fmt(p99), unit.fmt(p999), f.lifetime_calls);
        }
        if let Some(f) = self.functions.iter().find(|f| f.lifetime_calls_saturated) {
            println!("* Timings reflect last {} calls only.", f.recent_durations_ns.len());
        }
    }
}

/// Report for a single instrumented function.
#[derive(Debug)]
pub struct FunctionReport {
    pub name: &'static str,
    /// Recent durations in chronological order (last N calls).
    pub recent_durations_ns: Vec<u64>,
    /// Total number of recorded calls (may exceed window size).
    pub lifetime_calls: u64,
    /// True if `lifetime_calls` reached the recorder's window capacity.
    pub lifetime_calls_saturated: bool,
    /// Sum of durations in the recent window.
    pub recent_sum_ns: u64,
    /// Minimum duration in the recent window (0 if empty).
    pub recent_min_ns: u64,
    /// Maximum duration in the recent window.
    pub recent_max_ns: u64,
}

const RESET: &str = "\x1b[0m";

#[derive(Clone, Copy)]
enum Unit { Ns, Us, Ms, S }

impl Unit {
    /// Picks the smallest unit where the largest value stays ≤ 999.
    fn pick(values: &[u64]) -> Self {
        let max = values.iter().copied().max().unwrap_or(0);
        if max <= 999 { Self::Ns }
        else if max <= 999_000 { Self::Us }
        else if max <= 999_000_000 { Self::Ms }
        else { Self::S }
    }

    fn factor(self) -> f64 { match self { Self::Ns => 1.0, Self::Us => 1e3, Self::Ms => 1e6, Self::S => 1e9 } }

    /// Suffix padded to 2 visual chars.
    fn suffix(self) -> &'static str { match self { Self::Ns => "ns", Self::Us => "µs", Self::Ms => "ms", Self::S => " s" } }

    fn color(self) -> &'static str {
        match self {
            Self::Ns => "\x1b[32m",   // green
            Self::Us => "\x1b[93m",   // bright yellow
            Self::Ms => "\x1b[33m",   // dark yellow
            Self::S  => "\x1b[31m",   // red
        }
    }

    /// Returns a fixed 10-visual-char cell: `"NNN.NN Xu"` with colored unit suffix.
    fn fmt(self, ns: u64) -> String {
        format!("{:7.2} {}{}{}", ns as f64 / self.factor(), self.color(), self.suffix(), RESET)
    }
}

/// Computes a percentile given numerator/denominator (e.g. 50/100 for p50, 999/1000 for p99.9).
fn percentile(data: &[u64], num: usize, denom: usize) -> u64 {
    if data.is_empty() { return 0; }
    let mut sorted = data.to_vec();
    sorted.sort_unstable();
    sorted[((num * sorted.len()).saturating_sub(1)) / denom]
}
