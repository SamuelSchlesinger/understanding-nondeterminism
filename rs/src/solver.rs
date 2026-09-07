use crate::circuit::{Circuit, Wire};
use crate::layout::{self, Epsilon, Layout, LayoutMode};
use crate::natural::Natural;
use crate::network::{FrontierStep, Network, ReductionStats};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Auto,
    Frontier,
    Enumerate,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub method: Method,
    pub layout: LayoutMode,
    pub epsilon: Epsilon,
    pub exact_limit: usize,
    pub max_width: usize,
    pub max_enumeration: usize,
    pub trace: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            method: Method::Auto,
            layout: LayoutMode::Auto,
            epsilon: Epsilon::default(),
            exact_limit: 18,
            max_width: 20,
            max_enumeration: 24,
            trace: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Trivial,
    ScalarReduction,
    Enumeration,
    Frontier,
}

#[derive(Clone, Debug)]
pub struct GraphStats {
    pub cycle_rank: usize,
    pub cycle_budget: usize,
    pub expanded_vertices: usize,
    pub expanded_edges: usize,
    pub kernel_vertices: usize,
    pub kernel_edges: usize,
    pub kernel_rank: usize,
    pub reductions: ReductionStats,
}

#[derive(Clone, Debug)]
pub struct Report {
    pub count: Natural,
    pub algorithm: Algorithm,
    pub original_inputs: usize,
    pub original_gates: usize,
    pub used_inputs: usize,
    pub normalized_gates: usize,
    pub graph: Option<GraphStats>,
    pub layout: Option<Layout>,
    pub peak_entries: usize,
    pub peak_stored_entries: usize,
    pub peak_entry_bits: usize,
    pub products: u64,
    pub trace: Vec<FrontierStep>,
    pub normalization_time: Duration,
    pub reduction_time: Duration,
    pub layout_time: Duration,
    pub evaluation_time: Duration,
    pub elapsed: Duration,
}

/// Exact counts on every successful return. With the default paper layout and
/// automatic method, the algorithm has the note's fixed-epsilon time bound.
/// Configured resource limits may stop it with an error. Explicit experimental
/// layouts and forced evaluation methods do not carry the headline guarantee.
pub fn solve(circuit: &Circuit, options: &Options) -> Result<Report, String> {
    if options.exact_limit > layout::MAX_EXACT_VERTICES {
        return Err(format!(
            "--exact-limit must be at most {}",
            layout::MAX_EXACT_VERTICES
        ));
    }
    if options.max_width >= usize::BITS as usize || options.max_enumeration >= usize::BITS as usize
    {
        return Err(format!(
            "width and enumeration limits must be less than {}",
            usize::BITS
        ));
    }
    let start = Instant::now();
    let normalized = circuit.normalize()?;
    let core = &normalized.circuit;
    let b = core.inputs;
    let q = core.gates.len();
    let mut report = Report {
        count: Natural::zero(),
        algorithm: Algorithm::Trivial,
        original_inputs: circuit.inputs,
        original_gates: circuit.gates.len(),
        used_inputs: b,
        normalized_gates: q,
        graph: None,
        layout: None,
        peak_entries: 0,
        peak_stored_entries: 0,
        peak_entry_bits: 0,
        products: 0,
        trace: Vec::new(),
        normalization_time: start.elapsed(),
        reduction_time: Duration::ZERO,
        layout_time: Duration::ZERO,
        evaluation_time: Duration::ZERO,
        elapsed: Duration::ZERO,
    };
    let mut evaluation_start = Instant::now();
    let count = match core.output {
        Wire::Constant(bit) => Natural::from(u128::from(bit)),
        Wire::Input(_) => Natural::one(),
        Wire::Gate(_) if options.method == Method::Enumerate => {
            report.algorithm = Algorithm::Enumeration;
            core.brute_force(options.max_enumeration)?
        }
        Wire::Gate(_) => {
            let reduction_start = Instant::now();
            if b > q + 1 {
                return Err("normalized connected circuit violates the gate/input budget".into());
            }
            let mut network = Network::encode(core)?;
            let (expanded_vertices, expanded_edges, rank) = network.shape();
            let pins: usize = core.gates.iter().map(|g| g.inputs.len()).sum();
            if expanded_vertices + rank != expanded_edges + 1
                || rank + q + b != pins + 1
                || rank > q + 1 - b
            {
                return Err("incidence cycle accounting failed".into());
            }
            network.reduce()?;
            let (kernel_vertices, kernel_edges, kernel_rank) = network.shape();
            if kernel_rank > rank
                || (kernel_vertices > 0
                    && (kernel_rank == 0 || kernel_vertices != 2 * (kernel_rank - 1)))
            {
                return Err("cubic kernel cycle accounting failed".into());
            }
            report.graph = Some(GraphStats {
                cycle_rank: rank,
                cycle_budget: q + 1 - b,
                expanded_vertices,
                expanded_edges,
                kernel_vertices,
                kernel_edges,
                kernel_rank,
                reductions: network.stats.clone(),
            });
            let scalar = network.scalar.clone();
            let kernel = network.into_kernel()?;
            report.reduction_time = reduction_start.elapsed();
            if kernel.tensors.is_empty() {
                report.algorithm = Algorithm::ScalarReduction;
                evaluation_start = Instant::now();
                scalar
            } else {
                let layout_start = Instant::now();
                let layout = layout::find(
                    &kernel,
                    options.layout,
                    options.exact_limit,
                    options.epsilon,
                )?;
                report.layout_time = layout_start.elapsed();
                let can_contract = layout.width <= options.max_width;
                let enumerate = match options.method {
                    // Preserve the exponent min{b,w}. Resource caps may stop
                    // the preferred branch; they must not silently select a
                    // branch with a larger exponential running time.
                    Method::Auto => b <= layout.width,
                    Method::Frontier => false,
                    Method::Enumerate => unreachable!(),
                };
                evaluation_start = Instant::now();
                let result = if enumerate {
                    report.algorithm = Algorithm::Enumeration;
                    core.brute_force(options.max_enumeration)?
                } else {
                    if !can_contract {
                        return Err(format!("layout width {} exceeds --max-width {}; used inputs {b}, enumeration limit {}; use a different layout or explicitly raise a limit", layout.width, options.max_width, options.max_enumeration));
                    }
                    report.algorithm = Algorithm::Frontier;
                    let contraction =
                        kernel.contract(&layout.order, options.max_width, options.trace)?;
                    report.peak_entries = contraction.peak_entries;
                    report.peak_stored_entries = contraction.peak_stored_entries;
                    report.peak_entry_bits = contraction.peak_entry_bits;
                    report.products = contraction.products;
                    report.trace = contraction.steps;
                    scalar.mul(&contraction.count)
                };
                report.layout = Some(layout);
                result
            }
        }
    };
    report.count = count;
    report.count.shl_assign(circuit.inputs - b);
    report.evaluation_time = evaluation_start.elapsed();
    report.elapsed = start.elapsed();
    Ok(report)
}
