//! Reproducible circuits for demos and differential tests.

use crate::{Circuit, Gate, Wire};

pub fn toy() -> Circuit {
    Circuit::parse(include_str!("../examples/toy.circuit")).unwrap()
}

pub fn parity(inputs: usize) -> Circuit {
    let mut gates = Vec::new();
    let mut output = if inputs == 0 {
        Wire::Constant(false)
    } else {
        Wire::Input(0)
    };
    for i in 1..inputs {
        gates.push(Gate::binary(6, output, Wire::Input(i)));
        output = Wire::Gate(gates.len() - 1);
    }
    Circuit {
        inputs,
        gates,
        output,
    }
}

/// Accepts vertex covers: an OR for each edge, then an AND of those clauses.
pub fn vertex_covers(inputs: usize, edges: &[(usize, usize)]) -> Circuit {
    let mut gates: Vec<_> = edges
        .iter()
        .map(|&(u, v)| Gate::binary(14, Wire::Input(u), Wire::Input(v)))
        .collect();
    let mut layer: Vec<_> = (0..gates.len()).map(Wire::Gate).collect();
    while layer.len() > 1 {
        let mut next = Vec::new();
        for pair in layer.chunks(2) {
            if pair.len() == 1 {
                next.push(pair[0]);
            } else {
                gates.push(Gate::binary(8, pair[0], pair[1]));
                next.push(Wire::Gate(gates.len() - 1));
            }
        }
        layer = next;
    }
    Circuit {
        inputs,
        gates,
        output: layer.first().copied().unwrap_or(Wire::Constant(true)),
    }
}

pub fn cube() -> Circuit {
    let edges: Vec<_> = (0..8)
        .flat_map(|u| {
            [1, 2, 4].into_iter().filter_map(move |bit| {
                let v = u ^ bit;
                (u < v).then_some((u, v))
            })
        })
        .collect();
    vertex_covers(8, &edges)
}

/// Replace x0 in the K4 example by parity of `parity_inputs` fresh inputs.
/// The exact count is 5 * 2^(parity_inputs-1). Requires parity_inputs >= 1.
pub fn parity_k4(parity_inputs: usize) -> Circuit {
    assert!(parity_inputs >= 1);
    let mut prefix = parity(parity_inputs);
    let variables = [
        prefix.output,
        Wire::Input(parity_inputs),
        Wire::Input(parity_inputs + 1),
        Wire::Input(parity_inputs + 2),
    ];
    let k4 = Circuit::parse(include_str!("../examples/k4.circuit")).unwrap();
    let offset = prefix.gates.len();
    let remap = |wire| match wire {
        Wire::Input(i) => variables[i],
        Wire::Gate(i) => Wire::Gate(offset + i),
        other => other,
    };
    prefix.gates.extend(k4.gates.into_iter().map(|gate| Gate {
        inputs: gate.inputs.into_iter().map(remap).collect(),
        table: gate.table,
    }));
    prefix.inputs += 3;
    prefix.output = remap(k4.output);
    prefix
}

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^ (value >> 31)
    }

    pub fn below(&mut self, limit: usize) -> usize {
        assert!(limit > 0);
        (self.next_u64() % limit as u64) as usize
    }
}

/// Generates a syntactic circuit of the requested size. Pruning may shrink it.
/// `all_masks` includes constant/inessential gates for normalization tests.
pub fn random(inputs: usize, size: usize, seed: u64, all_masks: bool) -> Circuit {
    let mut rng = Rng::new(seed);
    let mut wires: Vec<_> = (0..inputs).map(Wire::Input).collect();
    if inputs == 0 || all_masks {
        wires.extend([Wire::Constant(false), Wire::Constant(true)]);
    }
    let mut gates = Vec::new();
    let masks = [1, 2, 4, 6, 7, 8, 9, 11, 13, 14];
    for _ in 0..size {
        let a = wires[rng.below(wires.len())];
        let b = wires[rng.below(wires.len())];
        let mask = if all_masks {
            rng.below(16) as u8
        } else {
            masks[rng.below(masks.len())]
        };
        gates.push(Gate::binary(mask, a, b));
        wires.push(Wire::Gate(gates.len() - 1));
    }
    Circuit {
        inputs,
        gates,
        output: *wires.last().unwrap(),
    }
}
