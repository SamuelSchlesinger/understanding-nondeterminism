use crate::circuit::{Circuit, Wire};
use crate::natural::Natural;

#[derive(Clone, Debug)]
pub(crate) struct Tensor {
    pub ports: Vec<usize>,
    pub entries: Vec<Natural>,
}

#[derive(Clone, Debug, Default)]
pub struct ReductionStats {
    pub equality_splits: usize,
    pub scalars: usize,
    pub degree_one: usize,
    pub degree_two: usize,
    pub loops: usize,
    pub parallel_two: usize,
    pub parallel_three: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Network {
    nodes: Vec<Option<Tensor>>,
    next_edge: usize,
    pub scalar: Natural,
    pub stats: ReductionStats,
}

#[derive(Clone, Debug)]
pub(crate) struct Kernel {
    pub tensors: Vec<Tensor>,
    pub edges: Vec<(usize, usize)>,
    pub neighbors: Vec<Vec<usize>>,
}

#[derive(Clone, Debug)]
pub struct FrontierStep {
    pub vertex: usize,
    pub before: usize,
    pub after: usize,
    pub summed: usize,
    pub nonzero_entries: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Contraction {
    pub count: Natural,
    pub peak_entries: usize,
    pub peak_stored_entries: usize,
    pub peak_entry_bits: usize,
    pub products: u64,
    pub steps: Vec<FrontierStep>,
}

impl Network {
    pub fn encode(circuit: &Circuit) -> Result<Self, String> {
        let b = circuit.inputs;
        let q = circuit.gates.len();
        let mut network = Self {
            nodes: Vec::new(),
            next_edge: 0,
            scalar: Natural::one(),
            stats: ReductionStats::default(),
        };
        let variable = |wire| match wire {
            Wire::Input(i) => Ok(i),
            Wire::Gate(i) => Ok(b + i),
            Wire::Constant(_) => Err("constant pin survived normalization".to_string()),
        };
        let mut incidences = vec![Vec::new(); b + q];
        for (i, gate) in circuit.gates.iter().enumerate() {
            let mut ports = Vec::new();
            for &wire in gate.inputs.iter().chain(std::iter::once(&Wire::Gate(i))) {
                let edge = network.edge();
                incidences[variable(wire)?].push(edge);
                ports.push(edge);
            }
            let arity = gate.inputs.len();
            let entries = (0..(1usize << ports.len()))
                .map(|assignment| {
                    let input = assignment & ((1 << arity) - 1);
                    let output = (assignment >> arity) & 1;
                    Natural::from(u128::from(output == usize::from((gate.table >> input) & 1)))
                })
                .collect();
            network.nodes.push(Some(Tensor { ports, entries }));
        }
        let edge = network.edge();
        incidences[variable(circuit.output)?].push(edge);
        network.nodes.push(Some(Tensor {
            ports: vec![edge],
            entries: vec![Natural::zero(), Natural::one()],
        }));
        for ports in incidences {
            if ports.is_empty() {
                return Err("unused value survived normalization".into());
            }
            network.equality(ports);
        }
        network.validate()?;
        Ok(network)
    }

    fn edge(&mut self) -> usize {
        let edge = self.next_edge;
        self.next_edge += 1;
        edge
    }

    fn equality_node(&mut self, ports: Vec<usize>) {
        assert!(!ports.is_empty() && ports.len() <= 3);
        let mut entries = vec![Natural::zero(); 1 << ports.len()];
        entries[0] = Natural::one();
        *entries.last_mut().unwrap() = Natural::one();
        self.nodes.push(Some(Tensor { ports, entries }));
    }

    fn equality(&mut self, ports: Vec<usize>) {
        if ports.len() <= 3 {
            self.equality_node(ports);
            return;
        }
        // Never materialize a high-fanout 2^D-entry equality table.
        self.stats.equality_splits += 1;
        let mut carry = self.edge();
        self.equality_node(vec![ports[0], ports[1], carry]);
        for &port in &ports[2..ports.len() - 2] {
            let next = self.edge();
            self.equality_node(vec![carry, port, next]);
            carry = next;
        }
        self.equality_node(vec![carry, ports[ports.len() - 2], ports[ports.len() - 1]]);
    }

    fn endpoints(&self) -> Vec<Vec<(usize, usize)>> {
        let mut ends = vec![Vec::new(); self.next_edge];
        for (v, tensor) in self.nodes.iter().enumerate() {
            if let Some(tensor) = tensor {
                for (port, &edge) in tensor.ports.iter().enumerate() {
                    ends[edge].push((v, port));
                }
            }
        }
        ends
    }

    fn validate(&self) -> Result<(), String> {
        for tensor in self.nodes.iter().flatten() {
            if tensor.ports.len() > 3 || tensor.entries.len() != (1 << tensor.ports.len()) {
                return Err("invalid tensor arity or table length".into());
            }
        }
        if self
            .endpoints()
            .iter()
            .any(|ends| !ends.is_empty() && ends.len() != 2)
        {
            return Err("an edge does not have exactly two ports".into());
        }
        Ok(())
    }

    /// Works for multigraphs, counting a loop as one edge and two ports.
    pub fn shape(&self) -> (usize, usize, usize) {
        let ends = self.endpoints();
        let vertices = self.nodes.iter().flatten().count();
        let edges = ends.iter().filter(|e| !e.is_empty()).count();
        let mut seen = vec![false; self.nodes.len()];
        let mut components = 0;
        for start in 0..self.nodes.len() {
            if self.nodes[start].is_none() || seen[start] {
                continue;
            }
            components += 1;
            seen[start] = true;
            let mut todo = vec![start];
            while let Some(v) = todo.pop() {
                for &edge in &self.nodes[v].as_ref().unwrap().ports {
                    for &(u, _) in &ends[edge] {
                        if !seen[u] {
                            seen[u] = true;
                            todo.push(u);
                        }
                    }
                }
            }
        }
        (vertices, edges, edges + components - vertices)
    }

    fn sum_loop(&mut self, v: usize, edge: usize) {
        let tensor = self.nodes[v].take().unwrap();
        let outer: Vec<_> = tensor
            .ports
            .iter()
            .enumerate()
            .filter(|(_, e)| **e != edge)
            .collect();
        let mut entries = Vec::new();
        for assignment in 0..(1usize << outer.len()) {
            let base = outer.iter().enumerate().fold(0, |base, (new, (old, _))| {
                base | (((assignment >> new) & 1) << old)
            });
            let diagonal = tensor
                .ports
                .iter()
                .enumerate()
                .fold(base, |index, (port, &e)| {
                    index | (usize::from(e == edge) << port)
                });
            let mut entry = tensor.entries[base].clone();
            entry.add_assign(&tensor.entries[diagonal]);
            entries.push(entry);
        }
        self.nodes[v] = Some(Tensor {
            ports: outer.iter().map(|(_, e)| **e).collect(),
            entries,
        });
    }

    fn contract_pair(&mut self, u: usize, v: usize, shared: &[usize]) {
        assert_ne!(u, v);
        let a = self.nodes[u].take().unwrap();
        let b = self.nodes[v].take().unwrap();
        let mut outer = Vec::new();
        for (side, tensor) in [&a, &b].iter().enumerate() {
            for (port, &edge) in tensor.ports.iter().enumerate() {
                if !shared.contains(&edge) {
                    outer.push((side, port, edge));
                }
            }
        }
        assert!(outer.len() <= 3);
        let mut entries = Vec::new();
        for assignment in 0..(1usize << outer.len()) {
            let mut base = [0, 0];
            for (new, &(side, port, _)) in outer.iter().enumerate() {
                base[side] |= ((assignment >> new) & 1) << port;
            }
            let mut entry = Natural::zero();
            for internal in 0..(1usize << shared.len()) {
                let mut index = base;
                for (side, tensor) in [&a, &b].iter().enumerate() {
                    for (port, edge) in tensor.ports.iter().enumerate() {
                        if let Some(bit) = shared.iter().position(|e| e == edge) {
                            index[side] |= ((internal >> bit) & 1) << port;
                        }
                    }
                }
                entry.add_assign(&a.entries[index[0]].mul(&b.entries[index[1]]));
            }
            entries.push(entry);
        }
        // Any uncontracted shared edge remains twice among the ports: a loop.
        self.nodes[u] = Some(Tensor {
            ports: outer.iter().map(|&(_, _, e)| e).collect(),
            entries,
        });
    }

    pub fn reduce(&mut self) -> Result<(), String> {
        self.validate()?;
        loop {
            if let Some(v) = self
                .nodes
                .iter()
                .position(|t| t.as_ref().is_some_and(|t| t.ports.is_empty()))
            {
                self.scalar = self.scalar.mul(&self.nodes[v].take().unwrap().entries[0]);
                self.stats.scalars += 1;
                continue;
            }
            let ends = self.endpoints();
            if let Some((edge, positions)) = ends
                .iter()
                .enumerate()
                .find(|(_, e)| e.len() == 2 && e[0].0 == e[1].0)
            {
                self.sum_loop(positions[0].0, edge);
                self.stats.loops += 1;
                continue;
            }
            if let Some(u) = self
                .nodes
                .iter()
                .position(|t| t.as_ref().is_some_and(|t| t.ports.len() <= 2))
            {
                let tensor = self.nodes[u].as_ref().unwrap();
                let degree = tensor.ports.len();
                let edge = tensor.ports[0];
                let v = ends[edge].iter().find(|&&(v, _)| v != u).unwrap().0;
                self.contract_pair(u, v, &[edge]);
                if degree == 1 {
                    self.stats.degree_one += 1;
                } else {
                    self.stats.degree_two += 1;
                }
                continue;
            }
            let mut pair = None;
            for positions in ends.iter().filter(|e| !e.is_empty()) {
                let (u, v) = (positions[0].0, positions[1].0);
                let a = self.nodes[u].as_ref().unwrap();
                let b = self.nodes[v].as_ref().unwrap();
                let shared: Vec<_> = a
                    .ports
                    .iter()
                    .copied()
                    .filter(|e| b.ports.contains(e))
                    .collect();
                if shared.len() >= 2 {
                    pair = Some((u, v, shared));
                    break;
                }
            }
            if let Some((u, v, shared)) = pair {
                if shared.len() == 2 {
                    self.stats.parallel_two += 1;
                } else {
                    self.stats.parallel_three += 1;
                }
                self.contract_pair(u, v, &shared);
                continue;
            }
            break;
        }
        self.validate()
    }

    pub fn into_kernel(self) -> Result<Kernel, String> {
        let ends = self.endpoints();
        let mut vertex_map = vec![0; self.nodes.len()];
        for (new, (old, _)) in self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.is_some())
            .enumerate()
        {
            vertex_map[old] = new;
        }
        let mut edge_map = vec![0; self.next_edge];
        let mut edges = Vec::new();
        let mut neighbors = vec![Vec::new(); self.nodes.iter().flatten().count()];
        for (old, positions) in ends.iter().enumerate().filter(|(_, e)| !e.is_empty()) {
            let (u, v) = (vertex_map[positions[0].0], vertex_map[positions[1].0]);
            if u == v || neighbors[u].contains(&v) {
                return Err("reduced kernel is not simple".into());
            }
            edge_map[old] = edges.len();
            edges.push((u, v));
            neighbors[u].push(v);
            neighbors[v].push(u);
        }
        if neighbors.iter().any(|adj| adj.len() != 3) {
            return Err("nonempty reduced kernel is not cubic".into());
        }
        let tensors = self
            .nodes
            .into_iter()
            .flatten()
            .map(|t| Tensor {
                ports: t.ports.into_iter().map(|edge| edge_map[edge]).collect(),
                entries: t.entries,
            })
            .collect();
        Ok(Kernel {
            tensors,
            edges,
            neighbors,
        })
    }
}

impl Kernel {
    pub fn contract(
        &self,
        order: &[usize],
        max_width: usize,
        trace: bool,
    ) -> Result<Contraction, String> {
        let mut seen = vec![false; self.tensors.len()];
        let mut frontier = Vec::<usize>::new();
        let mut previous = vec![Natural::one()];
        let mut report = Contraction {
            count: Natural::one(),
            peak_entries: 1,
            peak_stored_entries: 1,
            peak_entry_bits: 1,
            products: 0,
            steps: Vec::new(),
        };
        if order.len() != self.tensors.len() {
            return Err("layout does not cover the kernel".into());
        }
        for &v in order {
            if v >= seen.len() || seen[v] {
                return Err("layout is not a vertex permutation".into());
            }
            seen[v] = true;
            let tensor = &self.tensors[v];
            let mut joins = Vec::new();
            let mut forward = Vec::new();
            for (port, &edge) in tensor.ports.iter().enumerate() {
                let (a, b) = self.edges[edge];
                let other = if a == v {
                    b
                } else if b == v {
                    a
                } else {
                    return Err("tensor port is not incident to its vertex".into());
                };
                if let Some(old) = frontier.iter().position(|&e| e == edge) {
                    if !seen[other] {
                        return Err("frontier edge has no processed endpoint".into());
                    }
                    joins.push((port, old));
                } else {
                    if seen[other] {
                        return Err("processed edge is missing from the frontier".into());
                    }
                    forward.push((port, edge));
                }
            }
            let unchanged: Vec<_> = frontier
                .iter()
                .enumerate()
                .filter(|(_, e)| !tensor.ports.contains(e))
                .map(|(i, &e)| (i, e))
                .collect();
            let next_frontier: Vec<_> = unchanged
                .iter()
                .map(|&(_, e)| e)
                .chain(forward.iter().map(|&(_, e)| e))
                .collect();
            let width = next_frontier.len();
            if width > max_width || width >= usize::BITS as usize {
                return Err(format!(
                    "frontier width {width} exceeds the configured limit {max_width}"
                ));
            }
            let entries = 1usize << width;
            let mut next = Vec::new();
            next.try_reserve_exact(entries)
                .map_err(|e| format!("cannot allocate {entries} frontier entries: {e}"))?;
            next.resize_with(entries, Natural::zero);
            report.peak_entries = report.peak_entries.max(entries);
            report.peak_stored_entries = report.peak_stored_entries.max(previous.len() + entries);
            for (assignment, target) in next.iter_mut().enumerate() {
                let mut old_base = 0;
                let mut tensor_base = 0;
                for (new, &(old, _)) in unchanged.iter().enumerate() {
                    old_base |= ((assignment >> new) & 1) << old;
                }
                for (new, &(port, _)) in forward.iter().enumerate() {
                    tensor_base |= ((assignment >> (unchanged.len() + new)) & 1) << port;
                }
                for internal in 0..(1usize << joins.len()) {
                    let mut old = old_base;
                    let mut local = tensor_base;
                    for (bit, &(port, old_port)) in joins.iter().enumerate() {
                        old |= ((internal >> bit) & 1) << old_port;
                        local |= ((internal >> bit) & 1) << port;
                    }
                    report.products += 1;
                    if !previous[old].is_zero() && !tensor.entries[local].is_zero() {
                        target.add_assign(&previous[old].mul(&tensor.entries[local]));
                    }
                }
                report.peak_entry_bits = report.peak_entry_bits.max(target.bit_len());
            }
            if trace {
                report.steps.push(FrontierStep {
                    vertex: v,
                    before: frontier.len(),
                    after: width,
                    summed: joins.len(),
                    nonzero_entries: next.iter().filter(|n| !n.is_zero()).count(),
                });
            }
            previous = next;
            frontier = next_frontier;
        }
        if !frontier.is_empty() {
            return Err("contraction ended with open indices".into());
        }
        report.count = previous.pop().unwrap();
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weighted(ports: &[&[usize]], offset: u128) -> Network {
        let nodes: Vec<_> = ports
            .iter()
            .enumerate()
            .map(|(v, p)| {
                Some(Tensor {
                    ports: p.to_vec(),
                    entries: (0..(1 << p.len()))
                        .map(|i| Natural::from((i as u128 + v as u128 + offset) % 7))
                        .collect(),
                })
            })
            .collect();
        let next_edge = ports
            .iter()
            .flat_map(|p| p.iter())
            .max()
            .map_or(0, |e| e + 1);
        Network {
            nodes,
            next_edge,
            scalar: Natural::one(),
            stats: ReductionStats::default(),
        }
    }

    fn direct(network: &Network) -> Natural {
        let mut result = Natural::zero();
        for assignment in 0..(1usize << network.next_edge) {
            let mut product = network.scalar.clone();
            for tensor in network.nodes.iter().flatten() {
                let index = tensor
                    .ports
                    .iter()
                    .enumerate()
                    .fold(0, |index, (port, edge)| {
                        index | (((assignment >> edge) & 1) << port)
                    });
                product = product.mul(&tensor.entries[index]);
            }
            result.add_assign(&product);
        }
        result
    }

    #[test]
    fn reductions_preserve_weighted_sums_including_loops_and_parallel_edges() {
        let cases: &[&[&[usize]]] = &[
            &[&[0, 0]],
            &[&[0, 1], &[0, 1]],
            &[&[0, 1, 2], &[0, 1, 2]],
            &[&[0, 1, 2], &[0, 1, 3], &[2, 4, 5], &[3, 4, 5]],
        ];
        let mut parallel_two = 0;
        let mut parallel_three = 0;
        let mut loops = 0;
        for ports in cases {
            for offset in 0..20 {
                let mut network = weighted(ports, offset);
                let expected = direct(&network);
                network.reduce().unwrap();
                parallel_two += network.stats.parallel_two;
                parallel_three += network.stats.parallel_three;
                loops += network.stats.loops;
                assert!(network.nodes.iter().all(Option::is_none));
                assert_eq!(network.scalar, expected);
            }
        }
        assert!(parallel_two > 0 && parallel_three > 0 && loops > 0);
    }

    #[test]
    fn equality_expansion_has_unique_internal_extensions() {
        for degree in 1..=8 {
            let mut network = Network {
                nodes: Vec::new(),
                next_edge: degree,
                scalar: Natural::one(),
                stats: ReductionStats::default(),
            };
            network.equality((0..degree).collect());
            for boundary in 0..(1usize << degree) {
                let mut extensions = 0;
                for internal in 0..(1usize << (network.next_edge - degree)) {
                    let assignment = boundary | (internal << degree);
                    let valid = network.nodes.iter().flatten().all(|tensor| {
                        let index = tensor
                            .ports
                            .iter()
                            .enumerate()
                            .fold(0, |index, (port, edge)| {
                                index | (((assignment >> edge) & 1) << port)
                            });
                        !tensor.entries[index].is_zero()
                    });
                    extensions += usize::from(valid);
                }
                assert_eq!(
                    extensions,
                    usize::from(boundary == 0 || boundary == (1 << degree) - 1)
                );
            }
        }
    }

    #[test]
    fn weighted_frontier_matches_independent_edge_enumeration() {
        let ports: &[&[usize]] = &[&[0, 1, 2], &[0, 3, 4], &[1, 3, 5], &[2, 4, 5]];
        for offset in 0..20 {
            let network = weighted(ports, offset);
            let expected = direct(&network);
            let kernel = network.into_kernel().unwrap();
            for order in [[0, 1, 2, 3], [3, 1, 0, 2], [2, 0, 3, 1]] {
                assert_eq!(kernel.contract(&order, 10, true).unwrap().count, expected);
            }
        }
    }
}
