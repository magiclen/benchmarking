use std::time::Duration;

/// The result of measurement.
#[derive(Debug, Clone)]
pub struct MeasureResult {
    pub count: u64,
    pub total_elapsed: Duration,
}

impl MeasureResult {
    #[inline]
    pub(crate) fn new(elapsed: Duration) -> MeasureResult {
        MeasureResult {
            count: 1,
            total_elapsed: elapsed,
        }
    }
}