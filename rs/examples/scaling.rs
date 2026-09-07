//! Fixed, deliberately structured experiment; not a random-SAT benchmark.
//! Seven measured repetitions after one warmup per case. Release mode only
//! makes the timings useful. Compare all counts with original-circuit brute force.

use circuit_sat::{examples, solve, Method, Natural, Options};
use std::time::{Duration, Instant};

fn median(values: &mut [Duration]) -> f64 {
    values.sort_unstable();
    values[values.len() / 2].as_secs_f64() * 1000.0
}

fn main() {
    let options = Options {
        method: Method::Frontier,
        ..Options::default()
    };
    println!("K4 with one input replaced by parity; all counts checked against exhaustive search.");
    println!("One warmup, then seven repetitions per case; displayed times are medians.");
    println!(
        "Paper layout, epsilon={}; frontier evaluation is forced for this comparison.",
        options.epsilon
    );
    println!("n   gates  kernel  width  entries  layout_ms  total_ms  brute_ms  ratio");
    for parity_inputs in [1, 5, 9, 13, 17] {
        let circuit = examples::parity_k4(parity_inputs);
        let mut expected = Natural::from(5);
        expected.shl_assign(parity_inputs - 1);
        let warmup = solve(&circuit, &options).unwrap();
        assert_eq!(warmup.count, expected);
        assert_eq!(circuit.brute_force(20).unwrap(), expected);
        let mut total_times = Vec::new();
        let mut layout_times = Vec::new();
        let mut brute_times = Vec::new();
        for _ in 0..7 {
            let report = solve(&circuit, &options).unwrap();
            total_times.push(report.elapsed);
            layout_times.push(report.layout_time);
            let start = Instant::now();
            let brute = circuit.brute_force(20).unwrap();
            brute_times.push(start.elapsed());
            assert_eq!(report.count, expected);
            assert_eq!(brute, expected);
        }
        let total_ms = median(&mut total_times);
        let brute_ms = median(&mut brute_times);
        println!(
            "{:<3} {:<6} {:<7} {:<6} {:<8} {:<10.3} {:<9.3} {:<9.3} {:.3}x",
            circuit.inputs,
            circuit.gates.len(),
            warmup.graph.unwrap().kernel_vertices,
            warmup.layout.unwrap().width,
            warmup.peak_entries,
            median(&mut layout_times),
            total_ms,
            brute_ms,
            brute_ms / total_ms
        );
    }
    println!("The kernel stays at 14 vertices; layout uses helpful-set bisection, path bags, and median ordering.");
    println!("This family demonstrates reuse across a fixed small boundary, not the general four-gate bound.");
}
