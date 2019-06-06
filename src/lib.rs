/*!
# Benchmarking

This crate can be used to execute something and measure the execution time in nano seconds. It does not output anything to the screen and the filesystem.

## Examples

```rust
extern crate benchmarking;

fn main() {
    const VEC_LENGTH: usize = 100;

    benchmarking::warm_up();

    let bench_result = benchmarking::measure_function(|measurer| {
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

fn main() {
    const VEC_LENGTH: usize = 100;

    benchmarking::warm_up();

    let bench_result = benchmarking::measure_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure_for_loop(0..VEC_LENGTH, |_, loop_seq| {
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

fn main() {
    const VEC_LENGTH: usize = 100;

    benchmarking::warm_up();

    let bench_result = benchmarking::measure_function(|measurer| {
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



mod measure_result;
mod measurer;

use std::time::Duration;
use std::sync::{mpsc, atomic::{Ordering, AtomicBool}, Arc};
use std::thread;

pub use measure_result::MeasureResult;
pub use measurer::{Measurer, MeasureLoopResult};

const DEFAULT_BENCHMARK_COUNT: u64 = 10;
const DEFAULT_WARM_UP_DURATION: u64 = 3000;

#[derive(Debug)]
pub enum BenchmarkError {
    MeasurerNotMeasured
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
        if let Ok(_) = rx.try_recv() {
            break;
        }
    }
}

#[inline]
/// To stimulate CPUs to wake up.
pub fn warm_up_multi_thread(thread_count: usize) {
    warm_up_multi_thread_with_duration(thread_count, Duration::from_millis(DEFAULT_WARM_UP_DURATION));
}

/// To stimulate CPUs to wake up. The running duration is `3` seconds.
pub fn warm_up_multi_thread_with_duration(thread_count: usize, duration: Duration) {
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

#[inline]
/// Run a function and measure its execution time.
pub fn measure_function<F>(f: F) -> Result<u128, BenchmarkError> where F: FnMut(&mut Measurer) + 'static {
    measure_function_with_count(DEFAULT_BENCHMARK_COUNT, f)
}

/// Run a function and measure its execution time.
pub fn measure_function_with_count<F>(count: u64, mut f: F) -> Result<u128, BenchmarkError> where F: FnMut(&mut Measurer) + 'static {
    let mut measurer = Measurer::default();

    let mut count_sum = 0;
    let mut duration_sum = Duration::from_millis(0);

    for _ in 0..count {
        f(&mut measurer);

        let result = measurer.result.take().ok_or(BenchmarkError::MeasurerNotMeasured)?;

        count_sum += result.count as u128;
        duration_sum += result.total_elapsed;

        measurer.seq += 1;
    }

    Ok(duration_sum.as_nanos() / count_sum)
}