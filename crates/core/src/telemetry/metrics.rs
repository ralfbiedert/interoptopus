use crate::telemetry::ringbuffer::RingBuffer;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Performance report covering all instrumented functions.
#[derive(Debug)]
pub struct Report {
    pub functions: Vec<FunctionReport>,
}

/// Report for a single instrumented function.
#[derive(Debug)]
pub struct FunctionReport {
    pub name: &'static str,
    /// Recent durations in chronological order (last N calls).
    pub recent_durations_ns: Vec<u64>,
    /// Total number of recorded calls (may exceed window size).
    pub lifetime_calls: u64,
    /// Sum of durations in the recent window.
    pub recent_sum_ns: u64,
    /// Minimum duration in the recent window (0 if empty).
    pub recent_min_ns: u64,
    /// Maximum duration in the recent window.
    pub recent_max_ns: u64,
}

#[derive(Debug)]
pub struct Function {
    name: &'static str,
    durations: Mutex<RingBuffer>,
}

#[doc(hidden)]
pub struct MetricsRecorder {
    enabled: AtomicBool,
    epoch: Instant,
    functions: Vec<Function>,
}

impl MetricsRecorder {
    #[must_use]
    pub fn from(functions: &[&'static str]) -> Self {
        Self {
            enabled: AtomicBool::new(false),
            epoch: Instant::now(),
            functions: functions.iter().map(|name| Function { name, durations: Mutex::new(RingBuffer::new()) }).collect(),
        }
    }

    /// Returns nanoseconds since this instrumentor was created, or `0` if disabled.
    #[inline]
    pub fn time_ns(&self) -> u64 {
        if !self.enabled.load(Ordering::Relaxed) {
            return 0;
        }
        u64::try_from(self.epoch.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }

    /// Records a function call duration. Does nothing if disabled or if
    /// `start_ns` and `stop_ns` are both zero (i.e., timestamps taken while disabled).
    #[inline]
    pub fn record_call(&self, index: usize, start_ns: u64, stop_ns: u64) {
        if start_ns == 0 || stop_ns == 0 {
            return;
        }

        if let Some(function) = self.functions.get(index) {
            function.durations.lock().unwrap().push(stop_ns.saturating_sub(start_ns));
        }
    }

    #[inline]
    pub fn record(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    /// Returns a performance report for all instrumented functions.
    ///
    /// Each entry contains the function name, the last N recorded durations
    /// in chronological order, and pre-computed summary statistics.
    pub fn report(&self) -> Report {
        Report { functions: self.functions.iter().map(|f| f.durations.lock().unwrap().snapshot(f.name)).collect() }
    }
}
