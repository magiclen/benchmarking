fn main() {
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
}
