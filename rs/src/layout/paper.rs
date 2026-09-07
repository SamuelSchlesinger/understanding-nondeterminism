//! Constructive layout used by the SAT note.
//!
//! Monien--Preis bisection -> Fomin--Hoie anchored path decompositions ->
//! the note's median-edge ordering. The tree subroutine uses centroid bags,
//! replacing the cited optimal tree pathwidth routine by an elementary
//! logarithmic-width construction. This changes only polynomial factors.
//! See `rs/ALGORITHM.md` for the complete resource argument.

use super::bisection;
use std::fmt;
use std::str::FromStr;

pub use super::bisection::{Bisection, Statistics as BisectionStatistics, Stop as BisectionStop};

/// Fixed positive slack in the note's headline exponent `1/4 + epsilon`.
/// Fractions are exact; floating-point comparisons never control the proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Epsilon {
    numerator: u32,
    denominator: u32,
}

impl Epsilon {
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, String> {
        if numerator == 0 || denominator == 0 || numerator > denominator {
            return Err("epsilon must be a fraction P/Q with 0 < P/Q <= 1".into());
        }
        let (mut a, mut b) = (numerator, denominator);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Ok(Self {
            numerator: numerator / a,
            denominator: denominator / a,
        })
    }

    pub fn numerator(self) -> u32 {
        self.numerator
    }
    pub fn denominator(self) -> u32 {
        self.denominator
    }

    /// cut <= (1/6 + epsilon/4) * n.
    pub fn bisection_target_met(self, n: usize, cut: usize) -> bool {
        let p = u128::from(self.numerator);
        let q = u128::from(self.denominator);
        12 * q * cut as u128 <= (2 * q + 3 * p) * n as u128
    }

    /// cut > (1/3 + epsilon/4) * side_size. Apply MP Lemma 2
    /// with its parameter epsilon/8, giving an O_epsilon(1)-size set.
    pub(super) fn helpful_density_holds(self, side_size: usize, cut: usize) -> bool {
        let p = u128::from(self.numerator);
        let q = u128::from(self.denominator);
        12 * q * cut as u128 > (4 * q + 3 * p) * side_size as u128
    }
}

impl Default for Epsilon {
    fn default() -> Self {
        Self {
            numerator: 1,
            denominator: 100,
        }
    }
}

impl fmt::Display for Epsilon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl FromStr for Epsilon {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (p, q) = text.split_once('/').unwrap_or((text, "1"));
        let parse = |value: &str| {
            value
                .parse::<u32>()
                .map_err(|_| "epsilon must be a fraction P/Q with 0 < P/Q <= 1".to_string())
        };
        Self::new(parse(p)?, parse(q)?)
    }
}

#[derive(Clone, Debug)]
pub struct PathDecomposition {
    pub bags: Vec<Vec<usize>>,
}

impl PathDecomposition {
    /// Independently check vertex coverage, consecutive occurrence, edge
    /// coverage, and bag uniqueness. Return the measured pathwidth.
    pub fn validate(&self, adj: &[Vec<usize>]) -> Result<usize, String> {
        validate_graph(adj, false, false)?;
        self.intervals(adj).map(|(_, width)| width)
    }

    fn intervals(&self, adj: &[Vec<usize>]) -> Result<(Vec<(usize, usize)>, usize), String> {
        let n = adj.len();
        let mut intervals = vec![(usize::MAX, usize::MAX); n];
        let mut marks = vec![usize::MAX; n];
        let mut largest = 0;
        for (i, bag) in self.bags.iter().enumerate() {
            largest = largest.max(bag.len());
            for &v in bag {
                if v >= n || marks[v] == i {
                    return Err("path decomposition has an invalid or repeated bag vertex".into());
                }
                marks[v] = i;
                if intervals[v].0 == usize::MAX {
                    intervals[v].0 = i;
                } else if intervals[v].1 + 1 != i {
                    return Err(format!("vertex {v} occurs in nonconsecutive bags"));
                }
                intervals[v].1 = i;
            }
        }
        for (v, neighbors) in adj.iter().enumerate() {
            if intervals[v].0 == usize::MAX {
                return Err(format!("path decomposition omits vertex {v}"));
            }
            for &u in neighbors {
                if intervals[v].0.max(intervals[u].0) > intervals[v].1.min(intervals[u].1) {
                    return Err(format!("no bag covers edge {v}-{u}"));
                }
            }
        }
        Ok((intervals, largest.saturating_sub(1)))
    }
}

#[derive(Clone, Debug, Default)]
pub struct PathStatistics {
    pub no_outside_neighbor: usize,
    pub one_outside_neighbor: usize,
    pub tree_components: usize,
    pub enlarged_boundaries: usize,
}

#[derive(Clone, Debug)]
pub struct Certificate {
    pub epsilon: Epsilon,
    pub bisection: Bisection,
    pub decomposition: PathDecomposition,
    pub pathwidth: usize,
    /// An explicit finite-instance bound, including the actual bisection cut.
    pub width_bound: usize,
    pub path_statistics: PathStatistics,
}

#[derive(Clone, Debug)]
pub struct Construction {
    pub order: Vec<usize>,
    pub width: usize,
    pub certificate: Certificate,
}

fn validate_graph(adj: &[Vec<usize>], cubic: bool, connected: bool) -> Result<(), String> {
    let n = adj.len();
    for (v, neighbors) in adj.iter().enumerate() {
        if neighbors.len() > 3 || (cubic && neighbors.len() != 3) {
            return Err("paper layout requires a simple cubic graph".into());
        }
        for (i, &u) in neighbors.iter().enumerate() {
            if u >= n || u == v || neighbors[..i].contains(&u) || !adj[u].contains(&v) {
                return Err(
                    "layout graph has a loop, parallel edge, or asymmetric/invalid neighbor".into(),
                );
            }
        }
    }
    if connected && (n == 0 || components(adj, &vec![true; n]).len() != 1) {
        return Err("paper layout requires a nonempty connected cubic graph".into());
    }
    Ok(())
}

fn members(set: &[bool]) -> Vec<usize> {
    (0..set.len()).filter(|&v| set[v]).collect()
}

fn components(adj: &[Vec<usize>], selected: &[bool]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; adj.len()];
    let mut result = Vec::new();
    for start in 0..adj.len() {
        if !selected[start] || seen[start] {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(v) = stack.pop() {
            component.push(v);
            for &u in &adj[v] {
                if selected[u] && !seen[u] {
                    seen[u] = true;
                    stack.push(u);
                }
            }
        }
        component.sort_unstable();
        result.push(component);
    }
    result
}

fn ceil_log2(n: usize) -> usize {
    if n <= 1 {
        0
    } else {
        (usize::BITS - (n - 1).leading_zeros()) as usize
    }
}

/// Centroid recursion halves the component size. Its bag size is at most
/// floor(log2(|vertices|)) + 1. Recursion depth is logarithmic.
fn tree_bags(adj: &[Vec<usize>], vertices: &[usize]) -> Vec<Vec<usize>> {
    let mut selected = vec![false; adj.len()];
    for &v in vertices {
        selected[v] = true;
    }
    let root = vertices[0];
    let mut parent = vec![usize::MAX; adj.len()];
    parent[root] = root;
    let mut traversal = vec![root];
    let mut i = 0;
    while i < traversal.len() {
        let v = traversal[i];
        for &u in &adj[v] {
            if selected[u] && parent[u] == usize::MAX {
                parent[u] = v;
                traversal.push(u);
            }
        }
        i += 1;
    }
    let mut size = vec![1usize; adj.len()];
    for &v in traversal.iter().rev() {
        if v != root {
            size[parent[v]] += size[v];
        }
    }
    let centroid = *vertices
        .iter()
        .find(|&&v| {
            let largest = adj[v]
                .iter()
                .filter(|&&u| selected[u] && parent[u] == v)
                .map(|&u| size[u])
                .fold(vertices.len() - size[v], usize::max);
            largest <= vertices.len() / 2
        })
        .expect("a tree has a centroid");
    selected[centroid] = false;
    let mut bags = vec![vec![centroid]];
    for component in components(adj, &selected) {
        for mut bag in tree_bags(adj, &component) {
            bag.push(centroid);
            bag.sort_unstable();
            bags.push(bag);
        }
        bags.push(vec![centroid]);
    }
    bags
}

/// Fomin--Hoie Lemma 4: construct a decomposition ending at `anchor`.
/// The elementary tree routine gives width at most
/// max(|anchor|, floor(|vertices|/3)+1) + ceil(log2(|vertices|)) + 1.
fn anchored(
    adj: &[Vec<usize>],
    vertices: &[usize],
    boundary: &[usize],
    statistics: &mut PathStatistics,
) -> Result<Vec<Vec<usize>>, String> {
    let mut active = vec![false; adj.len()];
    let mut anchor = vec![false; adj.len()];
    for &v in vertices {
        active[v] = true;
    }
    for &v in boundary {
        anchor[v] = true;
    }
    let mut count = vertices.len();
    // Explicit continuations avoid linear-depth call stacks.
    let mut suffixes = Vec::<Vec<Vec<usize>>>::new();
    while count != 0 {
        let bag = members(&anchor);
        let removable = bag.iter().find_map(|&v| {
            let outside: Vec<_> = adj[v]
                .iter()
                .copied()
                .filter(|&u| active[u] && !anchor[u])
                .collect();
            (outside.len() <= 1).then_some((v, outside))
        });
        if let Some((v, outside)) = removable {
            if let Some(&u) = outside.first() {
                let mut enlarged = bag.clone();
                enlarged.push(u);
                enlarged.sort_unstable();
                suffixes.push(vec![enlarged, bag]);
                anchor[u] = true;
                statistics.one_outside_neighbor += 1;
            } else {
                suffixes.push(vec![bag]);
                statistics.no_outside_neighbor += 1;
            }
            active[v] = false;
            anchor[v] = false;
            count -= 1;
        } else if bag.len() <= count / 3 {
            let extra = count / 3 + 1 - bag.len();
            let additions: Vec<_> = (0..adj.len())
                .filter(|&v| active[v] && !anchor[v])
                .take(extra)
                .collect();
            for v in additions {
                anchor[v] = true;
            }
            suffixes.push(vec![bag]);
            statistics.enlarged_boundaries += 1;
        } else {
            let outside: Vec<_> = (0..adj.len()).map(|v| active[v] && !anchor[v]).collect();
            let tree = components(adj, &outside)
                .into_iter()
                .find(|component| {
                    let degree_sum: usize = component
                        .iter()
                        .map(|&v| adj[v].iter().filter(|&&u| outside[u]).count())
                        .sum();
                    degree_sum == 2 * (component.len() - 1)
                })
                .ok_or("anchored decomposition requires a tree component but none exists")?;
            let mut suffix = tree_bags(adj, &tree);
            for tree_bag in &mut suffix {
                tree_bag.extend_from_slice(&bag);
                tree_bag.sort_unstable();
            }
            suffix.push(bag);
            suffixes.push(suffix);
            count -= tree.len();
            for v in tree {
                active[v] = false;
            }
            statistics.tree_components += 1;
        }
    }
    let mut bags = vec![Vec::new()];
    for suffix in suffixes.into_iter().rev() {
        bags.extend(suffix);
    }
    Ok(bags)
}

/// Apply the note's median-timestamp proof to a supplied decomposition.
/// Integer ranks of (common bag, edge) stand for distinct rational timestamps.
pub fn median_order(
    adj: &[Vec<usize>],
    decomposition: &PathDecomposition,
) -> Result<Vec<usize>, String> {
    validate_graph(adj, true, false)?;
    let (intervals, width) = decomposition.intervals(adj)?;
    let mut events = Vec::new();
    for (v, neighbors) in adj.iter().enumerate() {
        for &u in neighbors {
            if v < u {
                events.push((intervals[v].0.max(intervals[u].0), v, u));
            }
        }
    }
    events.sort_unstable();
    let mut incident = vec![Vec::new(); adj.len()];
    for (time, &(_, u, v)) in events.iter().enumerate() {
        incident[u].push(time);
        incident[v].push(time);
    }
    let mut order: Vec<_> = (0..adj.len()).collect();
    // Each incident list is already ordered by its unique edge timestamps.
    order.sort_unstable_by_key(|&v| (incident[v][1], v));
    if cutwidth(adj, &order)? > width + 2 {
        return Err("median order violates cutwidth <= pathwidth + 2".into());
    }
    Ok(order)
}

/// Measure the edge boundary of every prefix, checking the permutation.
pub fn cutwidth(adj: &[Vec<usize>], order: &[usize]) -> Result<usize, String> {
    validate_graph(adj, false, false)?;
    if order.len() != adj.len() {
        return Err("layout order omits vertices".into());
    }
    let mut seen = vec![false; adj.len()];
    let mut crossing = 0usize;
    let mut width = 0;
    for &v in order {
        if v >= adj.len() || seen[v] {
            return Err("layout order is not a permutation".into());
        }
        let back = adj[v].iter().filter(|&&u| seen[u]).count();
        crossing = crossing + adj[v].len() - 2 * back;
        seen[v] = true;
        width = width.max(crossing);
    }
    Ok(width)
}

/// Construct the paper's layout on a nonempty simple connected cubic graph.
/// For fixed epsilon this takes polynomial time and returns width at most
/// `(1/6 + epsilon/4) * N + O(log N) + O_epsilon(1)`.
pub fn construct(adj: &[Vec<usize>], epsilon: Epsilon) -> Result<Construction, String> {
    validate_graph(adj, true, true)?;
    let n = adj.len();
    let bisection = bisection::construct(adj, epsilon)?;
    let mut left = vec![false; n];
    for &v in &bisection.left {
        left[v] = true;
    }
    let right: Vec<_> = (0..n).filter(|&v| !left[v]).collect();
    let left_boundary: Vec<_> = bisection
        .left
        .iter()
        .copied()
        .filter(|&v| adj[v].iter().any(|&u| !left[u]))
        .collect();
    let right_boundary: Vec<_> = right
        .iter()
        .copied()
        .filter(|&v| adj[v].iter().any(|&u| left[u]))
        .collect();
    let mut path_statistics = PathStatistics::default();
    let mut bags = anchored(adj, &bisection.left, &left_boundary, &mut path_statistics)?;
    // FH Theorem 5: bridge from the left boundary to the right boundary.
    let mut bridge = vec![false; n];
    for &v in &left_boundary {
        bridge[v] = true;
    }
    for &v in &left_boundary {
        for &u in &adj[v] {
            if !left[u] {
                bridge[u] = true;
            }
        }
        bags.push(members(&bridge));
        bridge[v] = false;
        bags.push(members(&bridge));
    }
    let mut other = anchored(adj, &right, &right_boundary, &mut path_statistics)?;
    other.reverse();
    bags.extend(other);
    let decomposition = PathDecomposition { bags };
    let pathwidth = decomposition.validate(adj)?;
    let width_bound = bisection.cut.max(n / 6 + 1) + ceil_log2(n) + 3;
    if pathwidth + 2 > width_bound {
        return Err("constructed path decomposition exceeds the proved finite bound".into());
    }
    let order = median_order(adj, &decomposition)?;
    let width = cutwidth(adj, &order)?;
    Ok(Construction {
        order,
        width,
        certificate: Certificate {
            epsilon,
            bisection,
            decomposition,
            pathwidth,
            width_bound,
            path_statistics,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::Rng;

    fn graph(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut adj = vec![Vec::new(); n];
        for &(u, v) in edges {
            adj[u].push(v);
            adj[v].push(u);
        }
        for neighbors in &mut adj {
            neighbors.sort_unstable();
        }
        adj
    }

    fn independent_width(adj: &[Vec<usize>], order: &[usize]) -> usize {
        (0..=order.len())
            .map(|i| {
                (0..adj.len())
                    .map(|v| {
                        adj[v]
                            .iter()
                            .filter(|&&u| {
                                v < u && order[..i].contains(&u) != order[..i].contains(&v)
                            })
                            .count()
                    })
                    .sum()
            })
            .max()
            .unwrap_or(0)
    }

    #[test]
    fn anchored_decompositions_cover_every_subcubic_graph_and_anchor_through_five_vertices() {
        let mut statistics = PathStatistics::default();
        for n in 0..=5 {
            let possible: Vec<_> = (0..n)
                .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
                .collect();
            for mask in 0..1usize << possible.len() {
                let edges: Vec<_> = possible
                    .iter()
                    .enumerate()
                    .filter_map(|(e, &pair)| (mask & (1 << e) != 0).then_some(pair))
                    .collect();
                let adj = graph(n, &edges);
                if adj.iter().any(|neighbors| neighbors.len() > 3) {
                    continue;
                }
                let vertices: Vec<_> = (0..n).collect();
                for anchor_mask in 0..1usize << n {
                    let anchor: Vec<_> = (0..n).filter(|&v| anchor_mask & (1 << v) != 0).collect();
                    let bags = anchored(&adj, &vertices, &anchor, &mut statistics).unwrap();
                    assert_eq!(bags.last(), Some(&anchor));
                    let decomposition = PathDecomposition { bags };
                    let width = decomposition.validate(&adj).unwrap();
                    assert!(width <= anchor.len().max(n / 3 + 1) + ceil_log2(n) + 1);
                }
            }
        }
        assert!(statistics.no_outside_neighbor > 0);
        assert!(statistics.one_outside_neighbor > 0);
        assert!(statistics.tree_components > 0);
        assert!(statistics.enlarged_boundaries > 0);
    }

    #[test]
    fn centroid_tree_decompositions_have_logarithmic_bags() {
        for n in [1, 2, 3, 7, 31, 127, 511] {
            let edges: Vec<_> = (1..n).map(|v| ((v - 1) / 2, v)).collect();
            let adj = graph(n, &edges);
            let vertices: Vec<_> = (0..n).collect();
            let decomposition = PathDecomposition {
                bags: tree_bags(&adj, &vertices),
            };
            assert!(decomposition.validate(&adj).unwrap() <= n.ilog2() as usize);
        }
    }

    fn cycle_with_matching(n: usize, seed: u64) -> Vec<Vec<usize>> {
        let mut rng = Rng::new(seed);
        loop {
            let mut permutation: Vec<_> = (0..n).collect();
            for i in (1..n).rev() {
                permutation.swap(i, rng.below(i + 1));
            }
            if permutation
                .as_chunks::<2>()
                .0
                .iter()
                .any(|pair| (pair[0] + 1) % n == pair[1] || (pair[1] + 1) % n == pair[0])
            {
                continue;
            }
            let mut edges: Vec<_> = (0..n).map(|v| (v, (v + 1) % n)).collect();
            edges.extend(
                permutation
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|pair| (pair[0], pair[1])),
            );
            return graph(n, &edges);
        }
    }

    #[test]
    fn full_construction_certificates_and_finite_exceptions_are_checked() {
        let mut improvements = 0;
        let mut exceptions = 0;
        let mut exception_types = 0;
        let mut targets = 0;
        for n in [4, 6, 8, 10, 12, 14, 16, 20, 30] {
            for seed in 0..8 {
                let adj = cycle_with_matching(n, seed + 20260907);
                for epsilon in [Epsilon::default(), Epsilon::new(1, 3).unwrap()] {
                    let result = construct(&adj, epsilon).unwrap();
                    let certificate = result.certificate;
                    assert_eq!(result.width, independent_width(&adj, &result.order));
                    assert!(result.width <= certificate.pathwidth + 2);
                    assert!(certificate.pathwidth + 2 <= certificate.width_bound);
                    assert_eq!(
                        certificate.pathwidth,
                        certificate.decomposition.validate(&adj).unwrap()
                    );
                    let b = certificate.bisection;
                    let side: Vec<_> = (0..n).map(|v| !b.left.contains(&v)).collect();
                    assert_eq!(b.left.len(), n / 2);
                    assert_eq!(b.cut, bisection::cut_size(&adj, &side));
                    improvements += b.statistics.improvements;
                    match b.stop {
                        BisectionStop::TargetReached => {
                            assert!(epsilon.bisection_target_met(n, b.cut));
                            targets += 1;
                        }
                        BisectionStop::SmallCaseDensity => {
                            assert!(!epsilon.helpful_density_holds(
                                n / 2 - b.exception_moved,
                                b.cut - b.exception_gain
                            ));
                            exceptions += 1;
                            exception_types |= 1;
                        }
                        BisectionStop::SmallCaseBalance => {
                            assert!(n / 2 + b.exception_moved >= 3 * (b.cut - b.exception_gain));
                            exceptions += 1;
                            exception_types |= 2;
                        }
                    }
                }
            }
        }
        assert!(improvements > 0 && targets > 0 && exceptions > 0);
        assert_eq!(
            exception_types, 3,
            "both finite-size exit premises are exercised"
        );
    }

    #[test]
    fn median_ties_and_invalid_certificates_are_handled() {
        let adj = graph(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        let one_bag = PathDecomposition {
            bags: vec![vec![0, 1, 2, 3]],
        };
        let order = median_order(&adj, &one_bag).unwrap();
        // Edge (1,2) is the median of both 1 and 2. Ties must still
        // give a valid permutation and obey the width inequality.
        assert_eq!(independent_width(&adj, &order), 4);
        for bags in [
            vec![vec![0, 1, 2]],
            vec![vec![0, 1, 2, 3, 3]],
            vec![vec![0, 1, 2, 4]],
            vec![vec![0, 1, 2, 3], vec![], vec![0]],
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
        ] {
            assert!(median_order(&adj, &PathDecomposition { bags }).is_err());
        }
        assert!(cutwidth(&adj, &[0, 1, 2, 2]).is_err());
        assert!(construct(&[vec![0, 0, 0]], Epsilon::default()).is_err());
        assert!(construct(&[vec![]], Epsilon::default()).is_err());
        assert_eq!("2/200".parse::<Epsilon>().unwrap(), Epsilon::default());
        for value in ["0/1", "1/0", "2/1", "-1/4", "1/2/3", "NaN"] {
            assert!(value.parse::<Epsilon>().is_err());
        }
    }

    #[test]
    fn cubic_bridges_and_a_larger_constructed_bisection_work() {
        // Two K4s, each with one edge subdivided, joined at the subdivisions.
        let mut edges = vec![(4, 9)];
        for offset in [0, 5] {
            edges.extend(
                [(0, 4), (1, 4), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
                    .map(|(u, v)| (u + offset, v + offset)),
            );
        }
        let adj = graph(10, &edges);
        let result = construct(&adj, Epsilon::default()).unwrap();
        assert_eq!(result.certificate.bisection.cut, 1);
        assert_eq!(
            result.certificate.bisection.stop,
            BisectionStop::TargetReached
        );
        assert_eq!(result.width, independent_width(&adj, &result.order));

        // A prism starts with all matching edges crossing. Helpful-set rounds
        // must improve this bisection; the two cycles are large enough for
        // the balancing lemma to apply repeatedly.
        let half = 40;
        let mut edges: Vec<_> = (0..half).map(|v| (v, v + half)).collect();
        for offset in [0, half] {
            edges.extend((0..half).map(|v| (v + offset, (v + 1) % half + offset)));
        }
        let adj = graph(2 * half, &edges);
        for epsilon in [Epsilon::default(), Epsilon::new(1, 3).unwrap()] {
            let result = construct(&adj, epsilon).unwrap();
            assert!(result.certificate.bisection.statistics.improvements > 0);
            assert!(result.certificate.bisection.cut < half);
            assert!(result.width <= result.certificate.width_bound);
            assert_eq!(result.width, independent_width(&adj, &result.order));
            if epsilon == Epsilon::new(1, 3).unwrap() {
                assert_eq!(
                    result.certificate.bisection.stop,
                    BisectionStop::TargetReached
                );
            }
        }
    }
}
