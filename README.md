Benchmarking
====================

[![Build Status](https://travis-ci.org/magiclen/benchmarking.svg?branch=master)](https://travis-ci.org/magiclen/benchmarking)
[![Build status](https://ci.appveyor.com/api/projects/status/y0iwlve66ral4peo/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/benchmarking/branch/master)

This crate can be used to execute something and measure the execution time. It does not output anything to screens and filesystems.

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

    println!("Filling 0 to 99 into a vec takes {:?}!", bench_result.elapsed());
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

    println!("Pushing a number into a vec takes {:?}!", bench_result.elapsed());
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

    println!("Pushing a number into a vec takes {:?}!", bench_result.elapsed());
}
```

* The `warm_up` and `warm_up_with_duration` functions of the `benchmarking` crate runs on one thread. To warm up all CPUs, you can use the `warm_up_multi_thread` and `warm_up_multi_thread_with_duration` functions instead.
* The `measure_function` and `measure_function_with_times` functions of the `benchmarking` crate can execute a closure for N times. To execute it repeatly for a while instead, you can use the `bench_function` and `bench_function_with_duration` functions.
* To execute a closure with multiple threads to measure the throughput, you can use the `multi_thread_bench_function` and `multi_thread_bench_function_with_duration` functions of the `benchmarking` crate.

## Crates.io

https://crates.io/crates/benchmarking

## Documentation

https://docs.rs/benchmarking

## License

[MIT](LICENSE)