use std::time::{Duration, Instant};

use crate::MeasureResult;

#[derive(Debug, Clone)]
/// To control whether to continue running the loop.
pub enum MeasureLoopResult<K> {
    Continue,
    Break(K),
}

impl From<()> for MeasureLoopResult<()> {
    #[inline]
    fn from(_: ()) -> MeasureLoopResult<()> {
        MeasureLoopResult::Continue
    }
}

#[derive(Debug, Default)]
/// To measure the execution time.
pub struct Measurer {
    pub(crate) seq: u128,
    pub(crate) result: Option<MeasureResult>,
    pub(crate) pass: bool,
}

impl Measurer {
    #[inline]
    /// Get the sequence of the current measurement.
    pub fn get_seq(&self) -> u128 {
        self.seq
    }

    #[inline]
    /// Get the result.
    pub fn get_result(&self) -> Option<&MeasureResult> {
        self.result.as_ref()
    }

    #[inline]
    /// Check this measurer whether it is measured.
    pub fn is_measured(&self) -> bool {
        self.result.is_none()
    }

    #[inline]
    /// Check this measurer whether it is passed.
    pub fn is_passed(&self) -> bool {
        self.pass
    }

    #[inline]
    fn update(&mut self, elapsed: Duration) {
        match &mut self.result {
            Some(result) => {
                result.times += 1;

                result.total_elapsed += elapsed;
            }
            None => {
                self.result = Some(MeasureResult::new(elapsed));
            }
        }
    }

    #[inline]
    /// Measure a function by executing it once.
    pub fn measure<M, K>(&mut self, f: M) where M: FnOnce() -> K {
        let start = Instant::now();

        let rtn = f();

        self.update(start.elapsed());

        drop(rtn);
    }

    #[inline]
    /// Pass the current measurement.
    pub fn pass(&mut self) {
        self.pass = true;
    }
}