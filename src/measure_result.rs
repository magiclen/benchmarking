use std::time::Duration;

/// The result of measurement.
#[derive(Debug, Clone)]
pub struct MeasureResult {
    pub(crate) times:         u128,
    pub(crate) total_elapsed: Duration,
}

unsafe impl Sync for MeasureResult {}

impl MeasureResult {
    #[inline]
    pub(crate) fn new(elapsed: Duration) -> MeasureResult {
        MeasureResult {
            times: 1, total_elapsed: elapsed
        }
    }

    #[inline]
    pub(crate) fn empty() -> MeasureResult {
        MeasureResult {
            times: 0, total_elapsed: Duration::from_secs(0)
        }
    }

    #[inline]
    /// Determine how long does an iteration take on average.
    pub fn elapsed(&self) -> Duration {
        let nano_secs = self.total_elapsed.as_nanos() / self.times;

        let secs = (nano_secs / 1_000_000_000) as u64;

        let nano_secs = (nano_secs % 1_000_000_000) as u32;

        Duration::new(secs, nano_secs)
    }

    #[inline]
    /// Determine how many iterations can be executed within one second.
    pub fn speed(&self) -> f64 {
        (self.times as f64 / self.total_elapsed.as_nanos() as f64) * 1_000_000_000.0
    }

    #[inline]
    /// Get how many times the measurements has been executed.
    pub fn times(&self) -> u128 {
        self.times
    }

    #[inline]
    /// Get how long has all measurements elapsed.
    pub fn total_elapsed(&self) -> Duration {
        self.total_elapsed
    }
}
