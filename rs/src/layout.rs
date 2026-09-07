//! The paper's constructive layout, plus explicit experimental alternatives.

mod bisection;
pub mod paper;

pub use paper::Epsilon;

use crate::network::Kernel;

pub const MAX_EXACT_VERTICES: usize = 22;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutMode {
    /// Use the paper's construction; this is the default.
    Auto,
    Paper,
    Exact,
    Greedy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutKind {
    Paper,
    Exact,
    Greedy,
}

impl LayoutKind {
    pub fn description(self) -> &'static str {
        match self {
            Self::Paper => "paper construction (fixed-epsilon polynomial preprocessing)",
            Self::Exact => "exact cutwidth (exponential subset search)",
            Self::Greedy => "greedy heuristic (no asymptotic width guarantee)",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub order: Vec<usize>,
    pub width: usize,
    pub kind: LayoutKind,
    pub subset_states: usize,
    pub paper: Option<paper::Certificate>,
}

pub(crate) fn find(
    kernel: &Kernel,
    mode: LayoutMode,
    exact_limit: usize,
    epsilon: Epsilon,
) -> Result<Layout, String> {
    if exact_limit > MAX_EXACT_VERTICES {
        return Err(format!(
            "exact layout limit must be at most {MAX_EXACT_VERTICES}"
        ));
    }
    if matches!(mode, LayoutMode::Auto | LayoutMode::Paper) {
        let constructed = paper::construct(&kernel.neighbors, epsilon)?;
        return Ok(Layout {
            order: constructed.order,
            width: constructed.width,
            kind: LayoutKind::Paper,
            subset_states: 0,
            paper: Some(constructed.certificate),
        });
    }
    let n = kernel.tensors.len();
    let exact = mode == LayoutMode::Exact;
    if exact && n > exact_limit {
        return Err(format!("exact layout search is limited to {exact_limit} vertices; kernel has {n}; use --layout paper or raise --exact-limit (maximum {MAX_EXACT_VERTICES})"));
    }
    Ok(if exact {
        exact_layout(kernel)
    } else {
        greedy_layout(kernel)
    })
}

pub(crate) fn score(kernel: &Kernel, order: &[usize]) -> (usize, usize) {
    let mut seen = vec![false; kernel.tensors.len()];
    let mut crossing = 0;
    let mut width = 0;
    let mut area = 0;
    for &v in order {
        let back = kernel.neighbors[v].iter().filter(|&&u| seen[u]).count();
        crossing = crossing + kernel.neighbors[v].len() - 2 * back;
        seen[v] = true;
        width = width.max(crossing);
        area += crossing;
    }
    (width, area)
}

fn exact_layout(kernel: &Kernel) -> Layout {
    let n = kernel.tensors.len();
    let states = 1usize << n;
    let neighbors: Vec<usize> = kernel
        .neighbors
        .iter()
        .map(|adj| adj.iter().fold(0, |mask, &u| mask | (1 << u)))
        .collect();
    let mut cut = vec![0u16; states];
    let mut best = vec![u16::MAX; states];
    let mut last = vec![0u8; states];
    best[0] = 0;
    for set in 1usize..states {
        let v = set.trailing_zeros() as usize;
        let rest = set & (set - 1);
        cut[set] = (usize::from(cut[rest]) + kernel.neighbors[v].len()
            - 2 * (neighbors[v] & rest).count_ones() as usize) as u16;
        let mut choices = set;
        while choices != 0 {
            let v = choices.trailing_zeros() as usize;
            choices &= choices - 1;
            // DP[S] = max(cut(S), min_v DP[S without v]).
            let candidate = best[set ^ (1 << v)].max(cut[set]);
            if candidate < best[set] {
                best[set] = candidate;
                last[set] = v as u8;
            }
            if best[set] == cut[set] {
                break;
            }
        }
    }
    let mut order = Vec::with_capacity(n);
    let mut set = states - 1;
    while set != 0 {
        let v = usize::from(last[set]);
        order.push(v);
        set ^= 1 << v;
    }
    order.reverse();
    let width = score(kernel, &order).0;
    debug_assert_eq!(width, usize::from(best[states - 1]));
    Layout {
        order,
        width,
        kind: LayoutKind::Exact,
        subset_states: states,
        paper: None,
    }
}

fn mixed(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn greedy_layout(kernel: &Kernel) -> Layout {
    let n = kernel.tensors.len();
    let mut best: Vec<_> = (0..n).collect();
    let mut best_score = score(kernel, &best);
    for trial in 0..n.min(32) {
        let mut seen = vec![false; n];
        let mut order = Vec::with_capacity(n);
        let mut crossing = 0;
        while order.len() != n {
            let v = (0..n)
                .filter(|&v| !seen[v])
                .min_by_key(|&v| {
                    let back = kernel.neighbors[v].iter().filter(|&&u| seen[u]).count();
                    let after = crossing + kernel.neighbors[v].len() - 2 * back;
                    (
                        after,
                        mixed(
                            (v as u64)
                                .wrapping_add((trial as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)),
                        ),
                    )
                })
                .unwrap();
            let back = kernel.neighbors[v].iter().filter(|&&u| seen[u]).count();
            crossing = crossing + kernel.neighbors[v].len() - 2 * back;
            seen[v] = true;
            order.push(v);
        }
        let candidate = score(kernel, &order);
        if candidate < best_score {
            best = order;
            best_score = candidate;
        }
    }
    // Adjacent exchanges minimize (peak boundary, total boundary sizes).
    for _ in 0..8 {
        let mut changed = false;
        for i in 0..n.saturating_sub(1) {
            best.swap(i, i + 1);
            let candidate = score(kernel, &best);
            if candidate < best_score {
                best_score = candidate;
                changed = true;
            } else {
                best.swap(i, i + 1);
            }
        }
        if !changed {
            break;
        }
    }
    Layout {
        order: best,
        width: best_score.0,
        kind: LayoutKind::Greedy,
        subset_states: 0,
        paper: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::natural::Natural;
    use crate::network::Tensor;

    fn graph(n: usize, edges: Vec<(usize, usize)>) -> Kernel {
        let mut neighbors = vec![Vec::new(); n];
        let mut tensors = vec![
            Tensor {
                ports: Vec::new(),
                entries: vec![Natural::one(); 8]
            };
            n
        ];
        for (edge, &(u, v)) in edges.iter().enumerate() {
            neighbors[u].push(v);
            neighbors[v].push(u);
            tensors[u].ports.push(edge);
            tensors[v].ports.push(edge);
        }
        Kernel {
            tensors,
            edges,
            neighbors,
        }
    }

    fn exhaustive_width(kernel: &Kernel, order: &mut [usize], pos: usize) -> usize {
        if pos == order.len() {
            return score(kernel, order).0;
        }
        let mut best = usize::MAX;
        for i in pos..order.len() {
            order.swap(pos, i);
            best = best.min(exhaustive_width(kernel, order, pos + 1));
            order.swap(pos, i);
        }
        best
    }

    #[test]
    fn subset_layout_matches_all_permutations() {
        let k4 = graph(
            4,
            (0..4)
                .flat_map(|u| (u + 1..4).map(move |v| (u, v)))
                .collect(),
        );
        let cube = graph(
            8,
            (0..8)
                .flat_map(|u| {
                    [1, 2, 4].into_iter().filter_map(move |bit| {
                        let v = u ^ bit;
                        (u < v).then_some((u, v))
                    })
                })
                .collect(),
        );
        for kernel in [k4, cube] {
            let layout = exact_layout(&kernel);
            let mut permutation: Vec<_> = (0..kernel.tensors.len()).collect();
            assert_eq!(layout.width, exhaustive_width(&kernel, &mut permutation, 0));
            let heuristic = greedy_layout(&kernel);
            assert!(heuristic.width >= layout.width);
            let result = kernel.contract(&heuristic.order, 20, true).unwrap();
            let mut expected = Natural::one();
            expected.shl_assign(kernel.edges.len());
            assert_eq!(result.count, expected);
        }
    }
}
