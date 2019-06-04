use std::time::{Duration, SystemTime, SystemTimeError};

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
    pub(crate) seq: u64,
    pub(crate) elapsed: Option<Result<Duration, SystemTimeError>>,
}

impl Measurer {
    #[inline]
    /// Get the sequence of the current measurement.
    pub fn get_seq(&self) -> u64 {
        self.seq
    }

    #[inline]
    /// Get the result.
    pub fn get_result(self) -> Option<Result<Duration, SystemTimeError>> {
        self.elapsed
    }

    #[inline]
    /// Check this measurer whether it is measured.
    pub fn is_measured(&self) -> bool {
        self.elapsed.is_none()
    }

    #[inline]
    /// Measure a function by executing it once.
    pub fn measure<M>(&mut self, f: M) where M: FnOnce() {
        if self.elapsed.is_none() {
            let start = SystemTime::now();

            f();

            self.elapsed = Some(start.elapsed());
        } else {
            panic!("this measurer has been measured");
        }
    }

    /// Measure a function by continuous re-executing it, until it returns a `MeasureLoopResult::Break`.
    pub fn measure_loop<M, O: Into<MeasureLoopResult>>(&mut self, mut f: M) where M: FnMut(usize) -> O {
        if self.is_measured() {
            let mut duration_sum = Duration::from_millis(0);

            let mut loop_seq = 0;

            loop {
                let start = SystemTime::now();

                let output = f(loop_seq);

                let elapsed = start.elapsed();

                match elapsed {
                    Ok(elapsed) => {
                        duration_sum += elapsed;

                        loop_seq += 1;

                        match output.into() {
                            MeasureLoopResult::Break => {
                                let elapsed = duration_sum.as_nanos() / loop_seq as u128;

                                let secs = Duration::from_secs((elapsed / 1000000000) as u64);
                                let nanos = Duration::from_nanos((elapsed % 1000000000) as u64);

                                self.elapsed = Some(Ok(secs + nanos));

                                break;
                            }
                            MeasureLoopResult::Continue => {
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        self.elapsed = Some(Err(err));

                        return;
                    }
                }
            }
        } else {
            panic!("this measurer has been measured");
        }
    }

    /// Measure a function by continuous re-executing it, until the input iterator has no next value.
    pub fn measure_for_loop<M, T, I: IntoIterator<Item=T>, O: Into<MeasureLoopResult>>(&mut self, iter: I, mut f: M) where M: FnMut(T) -> O {
        if self.is_measured() {
            let mut duration_sum = Duration::from_millis(0);

            let mut loop_seq = 0;

            for i in iter {
                let start = SystemTime::now();

                let output = f(i);

                let elapsed = start.elapsed();

                match elapsed {
                    Ok(elapsed) => {
                        duration_sum += elapsed;

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
                    Err(err) => {
                        self.elapsed = Some(Err(err));

                        return;
                    }
                }
            }

            if loop_seq > 0 {
                let elapsed = duration_sum.as_nanos() / loop_seq as u128;

                let secs = Duration::from_secs((elapsed / 1000000000) as u64);
                let nanos = Duration::from_nanos((elapsed % 1000000000) as u64);

                self.elapsed = Some(Ok(secs + nanos));
            }
        } else {
            panic!("this measurer has been measured");
        }
    }

    /// Measure a function by continuous re-executing it, until the conditional closure returns a `false`.
    pub fn measure_while_loop<M, C, O: Into<MeasureLoopResult>>(&mut self, mut g: C, mut f: M) where M: FnMut(usize) -> O, C: FnMut(usize) -> bool {
        if self.is_measured() {
            let mut duration_sum = Duration::from_millis(0);

            let mut loop_seq = 0;

            loop {
                if !g(loop_seq) {
                    break;
                }

                let start = SystemTime::now();

                let output = f(loop_seq);

                let elapsed = start.elapsed();

                match elapsed {
                    Ok(elapsed) => {
                        duration_sum += elapsed;

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
                    Err(err) => {
                        self.elapsed = Some(Err(err));

                        return;
                    }
                }
            }

            if loop_seq > 0 {
                let elapsed = duration_sum.as_nanos() / loop_seq as u128;

                let secs = Duration::from_secs((elapsed / 1000000000) as u64);
                let nanos = Duration::from_nanos((elapsed % 1000000000) as u64);

                self.elapsed = Some(Ok(secs + nanos));
            }
        } else {
            panic!("this measurer has been measured");
        }
    }

    #[inline]
    pub fn measure_iter<M, T, I: IntoIterator<Item=T>, O: Into<MeasureLoopResult>>(&mut self, iter: I, f: M) where M: FnMut(T) -> O {
        self.measure_for_loop(iter, f)
    }
}