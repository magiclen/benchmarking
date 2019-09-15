extern crate benchmarking;

fn main() {
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
}
