use std::mem::MaybeUninit;

fn main() {
    const VEC_LENGTH: usize = 100;

    benchmarking::warm_up();

    let bench_result = benchmarking::bench_function(|measurer| {
        let mut vec: Vec<MaybeUninit<usize>> = Vec::with_capacity(VEC_LENGTH);

        unsafe {
            vec.set_len(VEC_LENGTH);
        }

        for e in vec.iter().cloned() {
            measurer.measure(|| unsafe { e.assume_init() });
        }

        vec
    })
    .unwrap();

    println!("Reading a number from a vec takes {:?}!", bench_result.elapsed());
}
