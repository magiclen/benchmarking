Benchmarking
====================

[![Build Status](https://travis-ci.org/magiclen/benchmarking.svg?branch=master)](https://travis-ci.org/magiclen/benchmarking)

This crate can be used to execute something and measure the execution time. It does not output anything to screens and filesystems.

## Examples

```rust
use benchmarking;

const VEC_LENGTH: usize = 100;

benchmarking::warm_up();

let bench_result = benchmarking::bench_function(|measurer| {
    let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

    unsafe {
        vec.set_len(VEC_LENGTH);
    }

    for i in 0..VEC_LENGTH {
        measurer.measure(|| vec[i]);
    }

    vec
})
.unwrap();

println!("Reading a number from a vec takes {:?}!", bench_result.elapsed());
```

```rust
use benchmarking;

const VEC_LENGTH: usize = 100;

benchmarking::warm_up();

let bench_result = benchmarking::bench_function(|measurer| {
    let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

    measurer.measure(|| {
        for i in 0..VEC_LENGTH {
            vec.push(i);
        }
    });

    vec
})
.unwrap();

println!("Filling 0 to 99 into a vec takes {:?}!", bench_result.elapsed());
```

```rust
use benchmarking;

const VEC_LENGTH: usize = 100;

benchmarking::warm_up();

let bench_result = benchmarking::bench_function(|measurer| {
    let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

    for i in 0..VEC_LENGTH {
        measurer.measure(|| {
            vec.push(i);
        });
    }

    vec
})
.unwrap();

println!("Pushing a number into a vec takes {:?}!", bench_result.elapsed());
```

```rust
use benchmarking;

const VEC_LENGTH: usize = 100;

benchmarking::warm_up();

let bench_result = benchmarking::bench_function_n(2, |measurers| {
    let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

    for i in 0..VEC_LENGTH {
        measurers[1].measure(|| {
            vec.push(i);
        });
    }

    for i in 0..VEC_LENGTH {
        measurers[0].measure(|| vec[i]);
    }

    vec
})
.unwrap();

println!("Reading a number from a vec takes {:?}!", bench_result[0].elapsed());
println!("Pushing a number into a vec takes {:?}!", bench_result[1].elapsed());
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