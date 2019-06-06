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