//! Monien--Preis helpful-set bisection, specialized to connected cubic graphs.
//!
//! We enumerate helpful sets by increasing cardinality under the density
//! premise of their Lemma 2 (ALCOMFT-TR-01-116). That lemma bounds the first
//! successful cardinality by a constant depending only on epsilon. Lemma 5
//! supplies the balancing set. Thus these searches are polynomial for fixed
//! epsilon, although their polynomial degree can be large. See ALGORITHM.md.

use super::paper::Epsilon;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    TargetReached,
    SmallCaseDensity,
    SmallCaseBalance,
}

impl Stop {
    pub fn description(self) -> &'static str {
        match self {
            Self::TargetReached => "bisection target reached",
            Self::SmallCaseDensity => "finite-size exception: helpful-set density premise",
            Self::SmallCaseBalance => "finite-size exception: balancing premise",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub improvements: usize,
    pub helpful_calls: usize,
    pub largest_helpful_set: usize,
    pub largest_balance_set: usize,
    pub subsets_examined: u64,
}

#[derive(Clone, Debug)]
pub struct Bisection {
    pub left: Vec<usize>,
    pub cut: usize,
    pub stop: Stop,
    pub statistics: Statistics,
    /// The last unsuccessful round is rolled back. These fields record its
    /// imbalance and cut reduction, explaining the finite-size exception.
    pub exception_moved: usize,
    pub exception_gain: usize,
}

pub(super) fn cut_size(adj: &[Vec<usize>], right: &[bool]) -> usize {
    adj.iter()
        .enumerate()
        .map(|(v, neighbors)| {
            neighbors
                .iter()
                .filter(|&&u| v < u && right[v] != right[u])
                .count()
        })
        .sum()
}

/// Find a set of the specified size with helpfulness at least `minimum`.
/// Only a combination and its membership array are retained, not a subset DP.
fn find_set(
    adj: &[Vec<usize>],
    right: &[bool],
    side: bool,
    size: usize,
    minimum: i128,
    examined: &mut u64,
) -> Option<(Vec<usize>, i128)> {
    let vertices: Vec<_> = (0..adj.len()).filter(|&v| right[v] == side).collect();
    if size == 0 || size > vertices.len() {
        return None;
    }
    let mut indices: Vec<_> = (0..size).collect();
    let mut selected = vec![false; adj.len()];
    loop {
        for &i in &indices {
            selected[vertices[i]] = true;
        }
        let mut helpfulness = 0i128;
        for &i in &indices {
            for &u in &adj[vertices[i]] {
                if right[u] != side {
                    helpfulness += 1;
                } else if !selected[u] {
                    helpfulness -= 1;
                }
            }
        }
        *examined = examined.saturating_add(1);
        if helpfulness >= minimum {
            return Some((indices.iter().map(|&i| vertices[i]).collect(), helpfulness));
        }
        for &i in &indices {
            selected[vertices[i]] = false;
        }
        let pos = (0..size)
            .rev()
            .find(|&i| indices[i] < vertices.len() - size + i)?;
        indices[pos] += 1;
        for i in pos + 1..size {
            indices[i] = indices[i - 1] + 1;
        }
    }
}

pub(super) fn construct(adj: &[Vec<usize>], epsilon: Epsilon) -> Result<Bisection, String> {
    let n = adj.len();
    let initial: Vec<_> = (0..n).map(|v| v >= n / 2).collect();
    improve(adj, epsilon, initial)
}

fn improve(
    adj: &[Vec<usize>],
    epsilon: Epsilon,
    mut right: Vec<bool>,
) -> Result<Bisection, String> {
    let n = adj.len();
    let mut cut = cut_size(adj, &right);
    let mut statistics = Statistics::default();
    let mut stop = Stop::TargetReached;
    let mut exception_moved = 0;
    let mut exception_gain = 0;
    'rounds: while !epsilon.bisection_target_met(n, cut) {
        let before = right.clone();
        let old_cut = cut;
        let mut moved = 0usize;
        let mut gain = 0usize;
        // Once gain > 1 + floor(log2(moved)), Lemma 5's balancing
        // cost is strictly less than the improvement already obtained.
        while moved == 0 || gain <= 1 + moved.ilog2() as usize {
            let left_size = n / 2 - moved;
            if !epsilon.helpful_density_holds(left_size, cut) {
                stop = Stop::SmallCaseDensity;
                exception_moved = moved;
                exception_gain = gain;
                right = before;
                cut = old_cut;
                break 'rounds;
            }
            let mut found = None;
            for size in 1..=left_size {
                found = find_set(
                    adj,
                    &right,
                    false,
                    size,
                    1,
                    &mut statistics.subsets_examined,
                );
                if found.is_some() {
                    break;
                }
            }
            let (set, helpfulness) =
                found.ok_or("helpful-set lemma premise held but no set exists")?;
            statistics.helpful_calls += 1;
            statistics.largest_helpful_set = statistics.largest_helpful_set.max(set.len());
            moved += set.len();
            for v in set {
                right[v] = true;
            }
            gain += helpfulness as usize;
            cut -= helpfulness as usize;
            debug_assert_eq!(cut_size(adj, &right), cut);
        }
        if (n / 2 + moved) as u128 >= 3 * cut as u128 {
            stop = Stop::SmallCaseBalance;
            exception_moved = moved;
            exception_gain = gain;
            right = before;
            cut = old_cut;
            break;
        }
        let minimum = -1 - i128::from(moved.ilog2());
        let (set, helpfulness) = find_set(
            adj,
            &right,
            true,
            moved,
            minimum,
            &mut statistics.subsets_examined,
        )
        .ok_or("balancing lemma premise held but no balancing set exists")?;
        statistics.largest_balance_set = statistics.largest_balance_set.max(set.len());
        for v in set {
            right[v] = false;
        }
        cut = (cut as i128 - helpfulness) as usize;
        if cut >= old_cut || right.iter().filter(|&&b| b).count() != n / 2 {
            return Err("helpful-set round did not produce a strictly improved bisection".into());
        }
        debug_assert_eq!(cut_size(adj, &right), cut);
        statistics.improvements += 1;
    }
    Ok(Bisection {
        left: (0..n).filter(|&v| !right[v]).collect(),
        cut,
        stop,
        statistics,
        exception_moved,
        exception_gain,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combination_search_agrees_with_independent_cut_differences() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        for partition in 0..16 {
            let right: Vec<_> = (0..4).map(|v| partition & (1 << v) != 0).collect();
            let old = cut_size(&adj, &right) as i128;
            for side in [false, true] {
                for size in 1..=4 {
                    for minimum in -6..=6 {
                        let mut exists = false;
                        for mask in 1u32..16 {
                            if mask.count_ones() as usize != size
                                || (0..4).any(|v| mask & (1 << v) != 0 && right[v] != side)
                            {
                                continue;
                            }
                            let new: Vec<_> =
                                (0..4).map(|v| right[v] ^ (mask & (1 << v) != 0)).collect();
                            exists |= old - cut_size(&adj, &new) as i128 >= minimum;
                        }
                        let result = find_set(&adj, &right, side, size, minimum, &mut 0);
                        assert_eq!(result.is_some(), exists);
                        if let Some((set, gain)) = result {
                            let mut new = right.clone();
                            for v in set {
                                new[v] = !new[v];
                            }
                            assert_eq!(gain, old - cut_size(&adj, &new) as i128);
                        }
                    }
                }
            }
        }
    }
}
