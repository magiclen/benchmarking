/*!
# Benchmarking

This crate can be used to execute something and measure the execution time in nano seconds. It does not output anything to the screen and the filesystem.

## Examples

```rust
extern crate benchmarking;

use benchmarking::Benchmark;

fn main() {
    const VEC_LENGTH: usize = 100;

    let mut benchmark = Benchmark::default();

    benchmark.warm_up();

    let bench_result = benchmark.bench_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure(|| {
            for i in 0..VEC_LENGTH {
                vec.push(i);
            }
        });

        /*
            // Start the measurement
            for i in 0..VEC_LENGTH {
                vec.push(i);
            }
            // End the measurement
        */
    }).unwrap();

    println!("Filling 0 to 99 into a vec takes {} nano seconds!", bench_result);
}
```

```rust
extern crate benchmarking;

use benchmarking::Benchmark;

fn main() {
    const VEC_LENGTH: usize = 100;

    let mut benchmark = Benchmark::default();

    benchmark.warm_up();

    let bench_result = benchmark.bench_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure_for_loop(0..VEC_LENGTH, |loop_seq| {
            vec.push(loop_seq);
        });

        /*
            for i in 0...VEC_LENGTH {
                // Start the measurement
                vec.push(i);
                // End the measurement
            }
        */
    }).unwrap();

    println!("Pushing a number into a vec takes {} nano seconds!", bench_result);
}
```

```rust
extern crate benchmarking;

use benchmarking::Benchmark;

fn main() {
    const VEC_LENGTH: usize = 100;

    let mut benchmark = Benchmark::default();

    benchmark.warm_up();

    let bench_result = benchmark.bench_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure_while_loop(|loop_seq| {
            loop_seq < VEC_LENGTH
        }, |loop_seq| {
            vec.push(loop_seq);
        });

        /*
            let mut i = 0;

            while i < VEC_LENGTH {
                // Start the measurement
                vec.push(i);
                // End the measurement

                i += 1;
            }
        */
    }).unwrap();

    println!("Pushing a number into a vec takes {} nano seconds!", bench_result);
}
```

The `warm_up` and `warm_up_with_duration` methods of a `Benchmark` instance runs on one thread. To warm up all CPUs, you can use the `warm_up_multi_thread` and `warm_up_multi_thread_with_duration` methods instead.
*/



mod measurer;

use std::time::{Duration, SystemTimeError};
use std::sync::{mpsc, atomic::{Ordering, AtomicBool}, Arc};
use std::thread;

pub use measurer::{Measurer, MeasureLoopResult};

const DEFAULT_BENCHMARK_COUNT: u64 = 10;
const DEFAULT_WARM_UP_DURATION: u64 = 3000;

#[derive(Debug)]
pub enum BenchmarkError {
    MeasurerNotMeasured,
    SystemTimeError(SystemTimeError),
}

impl From<SystemTimeError> for BenchmarkError {
    #[inline]
    fn from(err: SystemTimeError) -> BenchmarkError {
        BenchmarkError::SystemTimeError(err)
    }
}

#[derive(Debug, Clone)]
/// To execute something and measure the execution time.
pub struct Benchmark {
    count: u64,
}

impl Default for Benchmark {
    #[inline]
    /// Create an default instance of `Benchmark` whose count is assigned to `10`.
    fn default() -> Benchmark {
        Benchmark {
            count: DEFAULT_BENCHMARK_COUNT
        }
    }
}

impl Benchmark {
    #[inline]
    /// Create an default instance of `Benchmark` and assign its `count`.
    pub fn new(count: u64) -> Benchmark {
        assert!(count > 0);

        Benchmark {
            count
        }
    }

    #[inline]
    pub fn get_count(&self) -> u64 {
        self.count
    }

    #[inline]
    /// To stimulate CPU to wake up. The running duration is `3` seconds.
    pub fn warm_up(&self) {
        self.warm_up_with_duration(Duration::from_millis(DEFAULT_WARM_UP_DURATION));
    }

    /// To stimulate CPU to wake up.
    pub fn warm_up_with_duration(&self, duration: Duration) {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            thread::sleep(duration);

            tx.send(()).unwrap();
        });

        loop {
            if let Ok(_) = rx.try_recv() {
                break;
            }
        }
    }

    #[inline]
    /// To stimulate CPUs to wake up.
    pub fn warm_up_multi_thread(&self, thread_count: usize) {
        self.warm_up_multi_thread_with_duration(thread_count, Duration::from_millis(DEFAULT_WARM_UP_DURATION));
    }

    /// To stimulate CPUs to wake up. The running duration is `3` seconds.
    pub fn warm_up_multi_thread_with_duration(&self, thread_count: usize, duration: Duration) {
        let lock = Arc::new(AtomicBool::new(true));

        for _ in 0..thread_count {
            let lock = lock.clone();

            thread::spawn(move || {
                loop {
                    if !lock.load(Ordering::Relaxed) {
                        break;
                    }
                }
            });
        }

        thread::sleep(duration);

        lock.store(true, Ordering::Relaxed);
    }

    /// Run a function and measure its execution time.
    pub fn bench_function<F>(&mut self, mut f: F) -> Result<u128, BenchmarkError> where F: FnMut(&mut Measurer) + 'static {
        let mut measurer = Measurer::default();

        let mut duration_sum = Duration::from_millis(0);

        for _ in 0..self.count {
            f(&mut measurer);

            duration_sum += measurer.elapsed.take().ok_or(BenchmarkError::MeasurerNotMeasured)??;

            measurer.seq += 1;
        }

        Ok(duration_sum.as_nanos() / self.count as u128)
    }
}