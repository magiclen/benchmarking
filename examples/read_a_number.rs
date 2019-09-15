extern crate benchmarking;

fn main() {
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
}
