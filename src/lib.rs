//! # Benchmarking
//!
//! This crate can be used to execute something and measure the execution time. It does not output anything to screens and filesystems.
//!
//! ## Examples
//!
//! ```rust
//! extern crate benchmarking;
//!
//! const VEC_LENGTH: usize = 100;
//!
//! benchmarking::warm_up();
//!
//! let bench_result = benchmarking::measure_function(|measurer| {
//!     let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);
//!
//!     unsafe {
//!         vec.set_len(VEC_LENGTH);
//!     }
//!
//!     for i in 0..VEC_LENGTH {
//!         measurer.measure(|| vec[i]);
//!     }
//!
//!     vec
//! })
//! .unwrap();
//!
//! println!("Reading a number from a vec takes {:?}!", bench_result.elapsed());
//! ```
//!
//! ```rust
//! extern crate benchmarking;
//!
//! const VEC_LENGTH: usize = 100;
//!
//! benchmarking::warm_up();
//!
//! let bench_result = benchmarking::measure_function(|measurer| {
//!     let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);
//!
//!     measurer.measure(|| {
//!         for i in 0..VEC_LENGTH {
//!             vec.push(i);
//!         }
//!     });
//!
//!     vec
//! })
//! .unwrap();
//!
//! println!("Filling 0 to 99 into a vec takes {:?}!", bench_result.elapsed());
//! ```
//!
//! ```rust
//! extern crate benchmarking;
//!
//! const VEC_LENGTH: usize = 100;
//!
//! benchmarking::warm_up();
//!
//! let bench_result = benchmarking::measure_function(|measurer| {
//!     let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);
//!
//!     for loop_seq in 0..VEC_LENGTH {
//!         measurer.measure(|| {
//!             vec.push(loop_seq);
//!         });
//!     }
//!
//!     vec
//! })
//! .unwrap();
//!
//! println!("Pushing a number into a vec takes {:?}!", bench_result.elapsed());
//! ```
//!
//! ```rust
//! extern crate benchmarking;
//!
//! const VEC_LENGTH: usize = 100;
//!
//! benchmarking::warm_up();
//!
//! let bench_result = benchmarking::measure_function_n(2, |measurers| {
//!     let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);
//!
//!     for i in 0..VEC_LENGTH {
//!         measurers[1].measure(|| {
//!             vec.push(i);
//!         });
//!     }
//!
//!     for i in 0..VEC_LENGTH {
//!         measurers[0].measure(|| vec[i]);
//!     }
//!
//!     vec
//! })
//! .unwrap();
//!
//! println!("Reading a number from a vec takes {:?}!", bench_result[0].elapsed());
//! println!("Pushing a number into a vec takes {:?}!", bench_result[1].elapsed());
//! ```
//!
//! The `warm_up` and `warm_up_with_duration` functions of the `benchmarking` crate runs on one thread. To warm up all CPUs, you can use the `warm_up_multi_thread` and `warm_up_multi_thread_with_duration` functions instead.
//! The `measure_function` and `measure_function_with_times` functions of the `benchmarking` crate can execute a closure for N times. To execute it repeatly for a while instead, you can use the `bench_function` and `bench_function_with_duration` functions.
//! To execute a closure with multiple threads to measure the throughput, you can use the `multi_thread_bench_function` and `multi_thread_bench_function_with_duration` functions of the `benchmarking` crate.
//!

mod measure_result;
mod measurer;

use std::mem::forget;
use std::ptr::read_volatile;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread;
use std::time::{Duration, Instant};

pub use measure_result::MeasureResult;
pub use measurer::Measurer;

const DEFAULT_MEASURE_TIMES: u64 = 10;
const DEFAULT_MEASURE_DURATION: u64 = 5000;
const DEFAULT_WARM_UP_DURATION: u64 = 3000;

#[derive(Debug)]
pub enum BenchmarkError {
    MeasurerNotMeasured,
}

#[inline]
/// To stimulate CPU to wake up. The running duration is `3` seconds.
pub fn warm_up() {
    warm_up_with_duration(Duration::from_millis(DEFAULT_WARM_UP_DURATION));
}

/// To stimulate CPU to wake up.
pub fn warm_up_with_duration(duration: Duration) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(duration);

        tx.send(()).unwrap();
    });

    loop {
        if rx.try_recv().is_ok() {
            break;
        }
    }
}

#[inline]
/// To stimulate CPUs to wake up.
pub fn warm_up_multi_thread(thread_count: usize) {
    warm_up_multi_thread_with_duration(
        thread_count,
        Duration::from_millis(DEFAULT_WARM_UP_DURATION),
    );
}

/// To stimulate CPUs to wake up. The running duration is `3` seconds.
pub fn warm_up_multi_thread_with_duration(thread_count: usize, duration: Duration) {
    if thread_count > 1 {
        let lock = Arc::new(AtomicBool::new(true));

        for _ in 0..thread_count {
            let lock = Arc::clone(&lock);

            thread::spawn(move || {
                loop {
                    if !lock.load(Ordering::Relaxed) {
                        break;
                    }
                }
            });
        }

        thread::sleep(duration);

        lock.store(false, Ordering::Relaxed);
    } else {
        warm_up_with_duration(duration);
    }
}

#[inline]
/// Run a function 10 times and measure its execution time.
pub fn measure_function<F, O>(f: F) -> Result<MeasureResult, BenchmarkError>
where
    F: FnMut(&mut Measurer) -> O + 'static, {
    measure_function_with_times(DEFAULT_MEASURE_TIMES, f)
}

/// Run a function with a specific times and measure its execution time.
pub fn measure_function_with_times<F, O>(
    times: u64,
    mut f: F,
) -> Result<MeasureResult, BenchmarkError>
where
    F: FnMut(&mut Measurer) -> O + 'static, {
    debug_assert!(times > 0);

    let mut measurer = Measurer::default();

    black_box(f(&mut measurer));

    let mut measure_result = if measurer.pass {
        measurer.pass = false;
        measurer.result = None;

        MeasureResult::empty()
    } else {
        measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?
    };

    for _ in 1..times {
        black_box(f(&mut measurer));

        if measurer.pass {
            measurer.pass = false;
            measurer.result = None;
        } else {
            let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

            measure_result.times += result.times;
            measure_result.total_elapsed += result.total_elapsed;
        }

        measurer.seq += 1;
    }

    Ok(measure_result)
}

#[inline]
/// Run a function for 5 seconds and measure its execution time.
pub fn bench_function<F, O>(f: F) -> Result<MeasureResult, BenchmarkError>
where
    F: FnMut(&mut Measurer) -> O + 'static, {
    bench_function_with_duration(Duration::from_millis(DEFAULT_MEASURE_DURATION), f)
}

/// Run a function with a specific duration and measure its execution time.
pub fn bench_function_with_duration<F, O>(
    duration: Duration,
    mut f: F,
) -> Result<MeasureResult, BenchmarkError>
where
    F: FnMut(&mut Measurer) -> O + 'static, {
    let mut measurer = Measurer::default();

    black_box(f(&mut measurer));

    let mut measure_result = if measurer.pass {
        measurer.pass = false;
        measurer.result = None;

        MeasureResult::empty()
    } else {
        measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?
    };

    let start = Instant::now();

    loop {
        black_box(f(&mut measurer));

        if measurer.pass {
            measurer.pass = false;
            measurer.result = None;
        } else {
            let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

            measure_result.times += result.times;
            measure_result.total_elapsed += result.total_elapsed;
        }

        if start.elapsed() >= duration {
            break;
        }

        measurer.seq += 1;
    }

    Ok(measure_result)
}

#[inline]
/// Run a function with a number of threads for 5 seconds and measure its execution time.
pub fn multi_thread_bench_function<F, O>(
    number_of_threads: usize,
    f: F,
) -> Result<MeasureResult, BenchmarkError>
where
    F: Fn(&mut Measurer) -> O + Send + Sync + 'static, {
    multi_thread_bench_function_with_duration(
        number_of_threads,
        Duration::from_millis(DEFAULT_MEASURE_DURATION),
        f,
    )
}

/// Run a function with a number of threads and a specific duration and measure its execution time.
pub fn multi_thread_bench_function_with_duration<F, O>(
    number_of_threads: usize,
    duration: Duration,
    f: F,
) -> Result<MeasureResult, BenchmarkError>
where
    F: Fn(&mut Measurer) -> O + Send + Sync + 'static, {
    debug_assert!(number_of_threads > 0);

    let (tx, rx) = mpsc::channel();

    let f = Arc::new(f);

    let start = Instant::now();

    for _ in 1..number_of_threads {
        let tx = tx.clone();

        let f = f.clone();

        thread::spawn(move || {
            let mut measurer = Measurer::default();

            black_box(f(&mut measurer));

            let mut measure_result = if measurer.pass {
                measurer.pass = false;
                measurer.result = None;

                MeasureResult::empty()
            } else {
                measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured).unwrap()
            };

            loop {
                black_box(f(&mut measurer));

                if measurer.pass {
                    measurer.pass = false;
                    measurer.result = None;
                } else {
                    let result =
                        measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured).unwrap();

                    measure_result.times += result.times;
                    measure_result.total_elapsed += result.total_elapsed;
                }

                if start.elapsed() >= duration {
                    break;
                }

                measurer.seq += 1;
            }

            tx.send(measure_result).unwrap();
        });
    }

    let mut measurer = Measurer::default();

    black_box(f(&mut measurer));

    let mut measure_result = if measurer.pass {
        measurer.pass = false;
        measurer.result = None;

        MeasureResult::empty()
    } else {
        measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?
    };

    let start = Instant::now();

    loop {
        black_box(f(&mut measurer));

        if measurer.pass {
            measurer.pass = false;
            measurer.result = None;
        } else {
            let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

            measure_result.times += result.times;
            measure_result.total_elapsed += result.total_elapsed;
        }

        if start.elapsed() >= duration {
            break;
        }

        measurer.seq += 1;
    }

    for _ in 1..number_of_threads {
        let result = rx.recv().unwrap();

        measure_result.times += result.times;
        measure_result.total_elapsed += result.total_elapsed;
    }

    measure_result.total_elapsed /= number_of_threads as u32;

    Ok(measure_result)
}

// TODO n

#[inline]
/// Run a function 10 times and measure its execution time.
pub fn measure_function_n<F, O>(n: usize, f: F) -> Result<Vec<MeasureResult>, BenchmarkError>
where
    F: FnMut(&mut [Measurer]) -> O + 'static, {
    measure_function_n_with_times(n, DEFAULT_MEASURE_TIMES, f)
}

/// Run a function with a specific times and measure its execution time.
pub fn measure_function_n_with_times<F, O>(
    n: usize,
    times: u64,
    mut f: F,
) -> Result<Vec<MeasureResult>, BenchmarkError>
where
    F: FnMut(&mut [Measurer]) -> O + 'static, {
    debug_assert!(times > 0);

    let mut measurers = {
        let mut v = Vec::with_capacity(n);

        for _ in 0..n {
            v.push(Measurer::default());
        }

        v
    };

    black_box(f(&mut measurers));

    let mut measure_results = {
        let mut v = Vec::with_capacity(n);

        for measurer in measurers.iter_mut() {
            let measure_result = if measurer.pass {
                measurer.pass = false;
                measurer.result = None;

                MeasureResult::empty()
            } else {
                measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?
            };

            v.push(measure_result);
        }

        v
    };

    for _ in 1..times {
        black_box(f(&mut measurers));

        for (i, measure_result) in measure_results.iter_mut().enumerate() {
            let measurer = &mut measurers[i];

            if measurer.pass {
                measurer.pass = false;
                measurer.result = None;
            } else {
                let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

                measure_result.times += result.times;
                measure_result.total_elapsed += result.total_elapsed;
            }

            measurer.seq += 1;
        }
    }

    Ok(measure_results)
}

#[inline]
/// Run a function for 5 seconds and measure its execution time.
pub fn bench_function_n<F, O>(n: usize, f: F) -> Result<Vec<MeasureResult>, BenchmarkError>
where
    F: FnMut(&mut [Measurer]) -> O + 'static, {
    bench_function_n_with_duration(n, Duration::from_millis(DEFAULT_MEASURE_DURATION), f)
}

/// Run a function with a specific duration and measure its execution time.
pub fn bench_function_n_with_duration<F, O>(
    n: usize,
    duration: Duration,
    mut f: F,
) -> Result<Vec<MeasureResult>, BenchmarkError>
where
    F: FnMut(&mut [Measurer]) -> O + 'static, {
    let mut measurers = {
        let mut v = Vec::with_capacity(n);

        for _ in 0..n {
            v.push(Measurer::default());
        }

        v
    };

    black_box(f(&mut measurers));

    let mut measure_results = {
        let mut v = Vec::with_capacity(n);

        for measurer in measurers.iter_mut() {
            let measure_result = if measurer.pass {
                measurer.pass = false;
                measurer.result = None;

                MeasureResult::empty()
            } else {
                measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?
            };

            v.push(measure_result);
        }

        v
    };

    let start = Instant::now();

    loop {
        black_box(f(&mut measurers));

        for (i, measure_result) in measure_results.iter_mut().enumerate() {
            let measurer = &mut measurers[i];

            if measurer.pass {
                measurer.pass = false;
                measurer.result = None;
            } else {
                let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

                measure_result.times += result.times;
                measure_result.total_elapsed += result.total_elapsed;
            }

            measurer.seq += 1;
        }

        if start.elapsed() >= duration {
            break;
        }
    }

    Ok(measure_results)
}

#[inline]
/// Run a function with a number of threads for 5 seconds and measure its execution time.
pub fn multi_thread_bench_function_n<F, O>(
    n: usize,
    number_of_threads: usize,
    f: F,
) -> Result<Vec<MeasureResult>, BenchmarkError>
where
    F: Fn(&mut [Measurer]) -> O + Send + Sync + 'static, {
    multi_thread_bench_function_n_with_duration(
        n,
        number_of_threads,
        Duration::from_millis(DEFAULT_MEASURE_DURATION),
        f,
    )
}

/// Run a function with a number of threads and a specific duration and measure its execution time.
pub fn multi_thread_bench_function_n_with_duration<F, O>(
    n: usize,
    number_of_threads: usize,
    duration: Duration,
    f: F,
) -> Result<Vec<MeasureResult>, BenchmarkError>
where
    F: Fn(&mut [Measurer]) -> O + Send + Sync + 'static, {
    debug_assert!(number_of_threads > 0);

    let (tx, rx) = mpsc::channel();

    let f = Arc::new(f);

    let start = Instant::now();

    for _ in 1..number_of_threads {
        let tx = tx.clone();

        let f = f.clone();

        thread::spawn(move || {
            let mut measurers = {
                let mut v = Vec::with_capacity(n);

                for _ in 0..n {
                    v.push(Measurer::default());
                }

                v
            };

            black_box(f(&mut measurers));

            let mut measure_results = {
                let mut v = Vec::with_capacity(n);

                for measurer in measurers.iter_mut() {
                    let measure_result = if measurer.pass {
                        measurer.pass = false;
                        measurer.result = None;

                        MeasureResult::empty()
                    } else {
                        measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured).unwrap()
                    };

                    v.push(measure_result);
                }

                v
            };

            loop {
                black_box(f(&mut measurers));

                for (i, measure_result) in measure_results.iter_mut().enumerate() {
                    let measurer = &mut measurers[i];

                    if measurer.pass {
                        measurer.pass = false;
                        measurer.result = None;
                    } else {
                        let result = measurer
                            .result
                            .take()
                            .ok_or(BenchmarkError::MeasurerNotMeasured)
                            .unwrap();

                        measure_result.times += result.times;
                        measure_result.total_elapsed += result.total_elapsed;
                    }

                    measurer.seq += 1;
                }

                if start.elapsed() >= duration {
                    break;
                }
            }

            tx.send(measure_results).unwrap();
        });
    }

    let mut measurers = {
        let mut v = Vec::with_capacity(n);

        for _ in 0..n {
            v.push(Measurer::default());
        }

        v
    };

    black_box(f(&mut measurers));

    let mut measure_results = {
        let mut v = Vec::with_capacity(n);

        for measurer in measurers.iter_mut() {
            let measure_result = if measurer.pass {
                measurer.pass = false;
                measurer.result = None;

                MeasureResult::empty()
            } else {
                measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?
            };

            v.push(measure_result);
        }

        v
    };

    let start = Instant::now();

    loop {
        black_box(f(&mut measurers));

        for (i, measure_result) in measure_results.iter_mut().enumerate() {
            let measurer = &mut measurers[i];

            if measurer.pass {
                measurer.pass = false;
                measurer.result = None;
            } else {
                let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

                measure_result.times += result.times;
                measure_result.total_elapsed += result.total_elapsed;
            }

            measurer.seq += 1;
        }

        if start.elapsed() >= duration {
            break;
        }
    }

    for _ in 1..number_of_threads {
        let results = rx.recv().unwrap();

        for (i, result) in results.into_iter().enumerate() {
            let measure_result = &mut measure_results[i];

            measure_result.times += result.times;
            measure_result.total_elapsed += result.total_elapsed;
        }

        for measure_result in measure_results.iter_mut() {
            measure_result.total_elapsed /= number_of_threads as u32;
        }
    }

    Ok(measure_results)
}

pub(crate) fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = read_volatile(&dummy);
        forget(dummy);
        ret
    }
}
