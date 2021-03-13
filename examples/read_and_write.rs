use benchmarking;

fn main() {
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
}
