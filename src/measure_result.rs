use std::time::Duration;

/// The result of measurement.
#[derive(Debug, Clone)]
pub struct MeasureResult {
    pub(crate) times:         u128,
    pub(crate) total_elapsed: Duration,
}

impl MeasureResult {
    #[inline]
    pub(crate) const fn new(elapsed: Duration) -> MeasureResult {
        MeasureResult {
            times: 1, total_elapsed: elapsed
        }
    }

    #[inline]
    pub(crate) const fn empty() -> MeasureResult {
        MeasureResult {
            times: 0, total_elapsed: Duration::from_secs(0)
        }
    }

    #[inline]
    /// Determine how long does an iteration take on average.
    ///
    /// The result must contain at least one measurement. If `times() == 0`, for example
    /// when every benchmark invocation has been passed, calling this method casuses a panic.
    pub const fn elapsed(&self) -> Duration {
        let nano_secs = self.total_elapsed.as_nanos() / self.times;

        let secs = (nano_secs / 1_000_000_000) as u64;

        let nano_secs = (nano_secs % 1_000_000_000) as u32;

        Duration::new(secs, nano_secs)
    }

    #[inline]
    /// Determine how many iterations can be executed within one second.
    ///
    /// The result should contain at least one measurement. If `times() == 0`, for example
    /// when every benchmark invocation has been passed, the returned value is not meaningful.
    pub const fn speed(&self) -> f64 {
        (self.times as f64 / self.total_elapsed.as_nanos() as f64) * 1_000_000_000.0
    }

    #[inline]
    /// Get how many times the measurements has been executed.
    pub const fn times(&self) -> u128 {
        self.times
    }

    #[inline]
    /// Get how long has all measurements elapsed.
    pub const fn total_elapsed(&self) -> Duration {
        self.total_elapsed
    }
}
