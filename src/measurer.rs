use std::time::{Duration, Instant};

use crate::MeasureResult;

#[derive(Debug, Clone)]
/// To control whether to continue running the loop.
pub enum MeasureLoopResult {
    Continue,
    Break,
}

impl From<()> for MeasureLoopResult {
    #[inline]
    fn from(_: ()) -> MeasureLoopResult {
        MeasureLoopResult::Continue
    }
}

#[derive(Debug, Default)]
/// To measure the execution time.
pub struct Measurer {
    pub(crate) seq: u128,
    pub(crate) result: Option<MeasureResult>,
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
    pub fn measure<M>(&mut self, f: M) where M: FnOnce() {
        let start = Instant::now();

        f();

        self.update(start.elapsed());
    }

    /// Measure a function by continuous re-executing it, until it returns a `MeasureLoopResult::Break`.
    pub fn measure_loop<M, O: Into<MeasureLoopResult>>(&mut self, mut f: M) where M: FnMut(usize) -> O {
        let mut loop_seq = 0;

        loop {
            let start = Instant::now();

            let output = f(loop_seq);

            self.update(start.elapsed());

            loop_seq += 1;

            match output.into() {
                MeasureLoopResult::Break => {
                    break;
                }
                MeasureLoopResult::Continue => {
                    continue;
                }
            }
        }
    }

    /// Measure a function by continuous re-executing it, until the input iterator has no next value.
    pub fn measure_for_loop<M, T, I: IntoIterator<Item=T>, O: Into<MeasureLoopResult>>(&mut self, iter: I, mut f: M) where M: FnMut(usize, T) -> O {
        for (loop_seq, i) in iter.into_iter().enumerate() {
            let start = Instant::now();

            let output = f(loop_seq, i);

            self.update(start.elapsed());

            match output.into() {
                MeasureLoopResult::Break => {
                    break;
                }
                MeasureLoopResult::Continue => {
                    continue;
                }
            }
        }
    }

    /// Measure a function by continuous re-executing it, until the conditional closure returns a `false`.
    pub fn measure_while_loop<M, C, O: Into<MeasureLoopResult>>(&mut self, mut g: C, mut f: M) where M: FnMut(usize) -> O, C: FnMut(usize) -> bool {
        let mut loop_seq = 0;

        loop {
            if !g(loop_seq) {
                break;
            }

            let start = Instant::now();

            let output = f(loop_seq);

            self.update(start.elapsed());

            loop_seq += 1;

            match output.into() {
                MeasureLoopResult::Break => {
                    break;
                }
                MeasureLoopResult::Continue => {
                    continue;
                }
            }
        }
    }

    #[inline]
    pub fn measure_iter<M, T, I: IntoIterator<Item=T>, O: Into<MeasureLoopResult>>(&mut self, iter: I, f: M) where M: FnMut(usize, T) -> O {
        self.measure_for_loop(iter, f)
    }
}