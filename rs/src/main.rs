use circuit_sat::{
    examples, solve, Algorithm, Circuit, LayoutMode, Method, Natural, Options, Report,
};
use std::io::{self, Read};
use std::time::Instant;

const HELP: &str = "Exact circuit SAT/#SAT

Usage:
  circuit-sat demo [OPTIONS]                     built-in examples (default)
  circuit-sat count FILE [OPTIONS]               read a circuit; '-' means stdin
  circuit-sat random INPUTS GATES SEED [OPTIONS]  deterministic generated circuit
  circuit-sat parity INPUTS [OPTIONS]            XOR chain, including large counts
  circuit-sat parity-k4 INPUTS [OPTIONS]         K4 with one input replaced by parity

Options:
  --method auto|frontier|enumerate  default: auto
  --layout auto|paper|exact|greedy  default: auto (paper construction)
  --epsilon P/Q                    fixed headline exponent slack (1/100)
  --exact-limit N                  maximum exact-search vertices (18; at most 22)
  --max-width N                    maximum frontier bits (20)
  --max-enumeration N              maximum enumerated input bits (24)
  --verify                         compare with exhaustive ORIGINAL circuit
  --trace                          print every frontier transition
  --help                           show this text

The default layout implements the paper's fixed-epsilon polynomial construction.
Its polynomial degree/constants can be large. Exact/greedy layouts are experimental.
Counts are exact arbitrary-precision integers; no external crates are needed.
";

fn number<T: std::str::FromStr>(text: &str, name: &str) -> Result<T, String> {
    text.parse()
        .map_err(|_| format!("invalid {name}: '{text}'"))
}

fn print_report(label: &str, report: &Report) {
    println!("{label}");
    println!(
        "  SAT: {}    #SAT: {}",
        !report.count.is_zero(),
        report.count
    );
    println!(
        "  Circuit: {} inputs, {} gates -> {} used inputs, {} gates",
        report.original_inputs, report.original_gates, report.used_inputs, report.normalized_gates
    );
    println!(
        "  Unused inputs: {} (multiply core count by 2^{})",
        report.original_inputs - report.used_inputs,
        report.original_inputs - report.used_inputs
    );
    if let Some(graph) = &report.graph {
        println!(
            "  Cycle rank: {} <= gate/input budget {}",
            graph.cycle_rank, graph.cycle_budget
        );
        println!(
            "  Expanded network: {} vertices, {} edges",
            graph.expanded_vertices, graph.expanded_edges
        );
        println!(
            "  Cubic kernel: {} vertices, {} edges, cycle rank {}",
            graph.kernel_vertices, graph.kernel_edges, graph.kernel_rank
        );
        let r = &graph.reductions;
        println!("  Reductions: equality splits={}, degree 1={}, degree 2={}, loops={}, parallel 2={}, parallel 3={}", r.equality_splits, r.degree_one, r.degree_two, r.loops, r.parallel_two, r.parallel_three);
    }
    if let Some(layout) = &report.layout {
        println!(
            "  Layout: {}; cutwidth={}",
            layout.kind.description(),
            layout.width
        );
        if layout.subset_states != 0 {
            println!("  Layout search: {} subset states", layout.subset_states);
            println!(
                "  Layout DP arrays: {} bytes (separate from frontier tables)",
                5 * layout.subset_states
            );
        }
        if let Some(certificate) = &layout.paper {
            let b = &certificate.bisection;
            println!(
                "  Paper epsilon: {}; bisection edges: {}; {}",
                certificate.epsilon,
                b.cut,
                b.stop.description()
            );
            println!(
                "  Path decomposition: {} bags ({} vertex entries), width {}; median bound: {}",
                certificate.decomposition.bags.len(),
                certificate
                    .decomposition
                    .bags
                    .iter()
                    .map(Vec::len)
                    .sum::<usize>(),
                certificate.pathwidth,
                certificate.pathwidth + 2
            );
            println!("  Helpful-set search: {} improvements, {} subsets checked, largest helpful/balancing sets: {}/{}", b.statistics.improvements, b.statistics.subsets_examined, b.statistics.largest_helpful_set, b.statistics.largest_balance_set);
            if b.stop != circuit_sat::layout::paper::BisectionStop::TargetReached {
                println!("  Finite-size exception: rolled back {} moved vertices, cut gain {}; additive epsilon-dependent constant applies", b.exception_moved, b.exception_gain);
            }
        }
    }
    let algorithm = match report.algorithm {
        Algorithm::Trivial => "constant or designated input",
        Algorithm::ScalarReduction => "scalar after exact reduction",
        Algorithm::Enumeration => "input enumeration",
        Algorithm::Frontier => "frontier contraction",
    };
    println!("  Evaluation: {algorithm}");
    if report.algorithm == Algorithm::Frontier {
        println!(
            "  Peak table: {} entries; two tables: {} entries; max frontier value: {} bits",
            report.peak_entries, report.peak_stored_entries, report.peak_entry_bits
        );
        println!("  Frontier product terms visited: {}", report.products);
    }
    if !report.trace.is_empty() {
        println!("  Step  Vertex  Before  After  Summed  Nonzero entries");
        for (i, step) in report.trace.iter().enumerate() {
            println!(
                "  {:>4}  {:>6}  {:>6}  {:>5}  {:>6}  {:>15}",
                i + 1,
                step.vertex,
                step.before,
                step.after,
                step.summed,
                step.nonzero_entries
            );
        }
    }
    println!(
        "  Time (ms): normalize={:.3}, reduce={:.3}, layout={:.3}, evaluate={:.3}, total={:.3}",
        report.normalization_time.as_secs_f64() * 1000.0,
        report.reduction_time.as_secs_f64() * 1000.0,
        report.layout_time.as_secs_f64() * 1000.0,
        report.evaluation_time.as_secs_f64() * 1000.0,
        report.elapsed.as_secs_f64() * 1000.0
    );
}

fn run_case(
    label: &str,
    circuit: &Circuit,
    options: &Options,
    verify: bool,
) -> Result<Natural, String> {
    let report = solve(circuit, options)?;
    print_report(label, &report);
    if verify {
        let start = Instant::now();
        let expected = circuit.brute_force(options.max_enumeration)?;
        if report.count != expected {
            return Err(format!(
                "COUNT MISMATCH: frontier result {}, exhaustive result {expected}",
                report.count
            ));
        }
        println!(
            "  Exhaustive check: PASS ({expected}; {:.3} ms)",
            start.elapsed().as_secs_f64() * 1000.0
        );
    }
    Ok(report.count)
}

fn run() -> Result<(), String> {
    let mut options = Options::default();
    let mut verify = false;
    let mut positional = Vec::new();
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < arguments.len() {
        let argument = &arguments[i];
        match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(());
            }
            "--verify" => verify = true,
            "--trace" => options.trace = true,
            "--method" | "--layout" | "--epsilon" | "--exact-limit" | "--max-width"
            | "--max-enumeration" => {
                i += 1;
                let value = arguments
                    .get(i)
                    .ok_or_else(|| format!("{argument} needs a value"))?;
                match argument.as_str() {
                    "--method" => {
                        options.method = match value.as_str() {
                            "auto" => Method::Auto,
                            "frontier" => Method::Frontier,
                            "enumerate" => Method::Enumerate,
                            _ => return Err("--method must be auto, frontier, or enumerate".into()),
                        }
                    }
                    "--layout" => {
                        options.layout = match value.as_str() {
                            "auto" => LayoutMode::Auto,
                            "paper" => LayoutMode::Paper,
                            "exact" => LayoutMode::Exact,
                            "greedy" => LayoutMode::Greedy,
                            _ => {
                                return Err("--layout must be auto, paper, exact, or greedy".into())
                            }
                        }
                    }
                    "--epsilon" => options.epsilon = value.parse()?,
                    "--exact-limit" => options.exact_limit = number(value, "exact layout limit")?,
                    "--max-width" => options.max_width = number(value, "frontier width limit")?,
                    "--max-enumeration" => {
                        options.max_enumeration = number(value, "enumeration limit")?
                    }
                    _ => unreachable!(),
                }
            }
            flag if flag.starts_with('-') && flag != "-" => {
                return Err(format!("unknown option '{flag}'"))
            }
            _ => positional.push(argument.clone()),
        }
        i += 1;
    }
    if positional.is_empty() {
        positional.push("demo".into());
    }
    match positional[0].as_str() {
        "demo" if positional.len() == 1 => {
            println!("Exact counting demos; small cases are checked against exhaustive search.");
            if options.method == Method::Auto {
                println!(
                    "The nonempty-kernel demos force frontier contraction to show the pipeline."
                );
            }
            println!();
            run_case(
                "Three-gate example (expected count 3)",
                &examples::toy(),
                &options,
                true,
            )?;
            println!();
            let mut frontier_options = options.clone();
            if options.method == Method::Auto {
                frontier_options.method = Method::Frontier;
            }
            let k4 = Circuit::parse(include_str!("../examples/k4.circuit"))?;
            run_case(
                "K4 vertex covers (expected count 5)",
                &k4,
                &frontier_options,
                true,
            )?;
            println!();
            run_case(
                "Cube vertex covers (expected count 35)",
                &examples::cube(),
                &frontier_options,
                true,
            )?;
            if options.method != Method::Enumerate {
                println!();
                let count = run_case(
                    "200-input parity (expected count 2^199)",
                    &examples::parity(200),
                    &options,
                    false,
                )?;
                let mut expected = Natural::one();
                expected.shl_assign(199);
                if count != expected {
                    return Err("parity count disagrees with 2^199".into());
                }
                println!(
                    "  Closed-form check: PASS (arbitrary precision; no exhaustive enumeration)"
                );
            }
        }
        "count" if positional.len() == 2 => {
            let text = if positional[1] == "-" {
                let mut text = String::new();
                io::stdin()
                    .read_to_string(&mut text)
                    .map_err(|e| e.to_string())?;
                text
            } else {
                std::fs::read_to_string(&positional[1])
                    .map_err(|e| format!("{}: {e}", positional[1]))?
            };
            run_case(&positional[1], &Circuit::parse(&text)?, &options, verify)?;
        }
        "random" if positional.len() == 4 => {
            let inputs = number(&positional[1], "input count")?;
            let size = number(&positional[2], "gate count")?;
            let seed = number(&positional[3], "seed")?;
            let label = format!("Generated circuit: inputs={inputs}, gates={size}, seed={seed}");
            run_case(
                &label,
                &examples::random(inputs, size, seed, false),
                &options,
                verify,
            )?;
        }
        "parity" if positional.len() == 2 => {
            let inputs = number(&positional[1], "input count")?;
            run_case(
                &format!("Parity on {inputs} inputs"),
                &examples::parity(inputs),
                &options,
                verify,
            )?;
        }
        "parity-k4" if positional.len() == 2 => {
            let inputs: usize = number(&positional[1], "parity input count")?;
            if inputs == 0 {
                return Err("parity-k4 needs at least one parity input".into());
            }
            let count = run_case(
                &format!("K4 with one vertex replaced by parity of {inputs} inputs"),
                &examples::parity_k4(inputs),
                &options,
                verify,
            )?;
            let mut expected = Natural::from(5);
            expected.shl_assign(inputs - 1);
            if count != expected {
                return Err("parity-K4 count disagrees with its closed form".into());
            }
            println!("  Closed-form check: PASS (5 * 2^{})", inputs - 1);
        }
        _ => return Err("invalid command or argument count; use --help".into()),
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
