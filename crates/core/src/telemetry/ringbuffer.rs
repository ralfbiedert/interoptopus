use crate::telemetry::MAX_RECORDED_DURATIONS;
use crate::telemetry::report::FunctionReport;
use std::fmt;

/// Pre-allocated fixed-size ring buffer. Avoids all heap allocation on push.
pub(super) struct RingBuffer {
    data: Box<[u64; MAX_RECORDED_DURATIONS]>,
    pos: usize,
    len: usize,
    lifetime_calls: u64,
}

impl fmt::Debug for RingBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RingBuffer")
            .field("len", &self.len)
            .field("lifetime_calls", &self.lifetime_calls)
            .finish_non_exhaustive()
    }
}

impl RingBuffer {
    pub fn new() -> Self {
        Self { data: vec![0u64; MAX_RECORDED_DURATIONS].into_boxed_slice().try_into().unwrap(), pos: 0, len: 0, lifetime_calls: 0 }
    }

    #[inline]
    pub fn push(&mut self, value: u64) {
        self.data[self.pos] = value;
        // MAX_RECORDED_DURATIONS is a power of 2, compiles to a bitwise AND.
        self.pos = (self.pos + 1) % MAX_RECORDED_DURATIONS;
        if self.len < MAX_RECORDED_DURATIONS {
            self.len += 1;
        }
        self.lifetime_calls += 1;
    }

    pub fn snapshot(&self, name: &'static str) -> FunctionReport {
        let mut durations = Vec::with_capacity(self.len);

        let start = if self.len < MAX_RECORDED_DURATIONS { 0 } else { self.pos };
        for i in 0..self.len {
            let d = self.data[(start + i) % MAX_RECORDED_DURATIONS];
            durations.push(d);
        }

        FunctionReport { name, recent_durations_ns: durations, lifetime_calls: self.lifetime_calls }
    }
}
