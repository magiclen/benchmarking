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