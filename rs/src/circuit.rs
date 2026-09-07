use crate::natural::Natural;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Wire {
    Input(usize),
    Gate(usize),
    Constant(bool),
}

/// Internal table indices use port 0 as the least significant bit.
#[derive(Clone, Debug)]
pub struct Gate {
    pub inputs: Vec<Wire>,
    pub table: u8,
}

impl Gate {
    /// Public B2 masks use bit (2*a+b), e.g. AND=8, OR=14, XOR=6.
    pub fn binary(mask: u8, a: Wire, b: Wire) -> Self {
        Self {
            inputs: vec![a, b],
            table: (mask & 0xf9) | ((mask & 2) << 1) | ((mask & 4) >> 1),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Circuit {
    pub inputs: usize,
    pub gates: Vec<Gate>,
    pub output: Wire,
}

#[derive(Clone, Debug)]
pub struct Normalized {
    pub circuit: Circuit,
    /// Original input indices, in the order used by the normalized circuit.
    pub used_inputs: Vec<usize>,
    pub declared_inputs: usize,
}

impl Circuit {
    pub fn validate(&self) -> Result<(), String> {
        let check = |wire: Wire, gate_limit: usize| match wire {
            Wire::Input(i) if i >= self.inputs => Err(format!("input x{i} is out of range")),
            Wire::Gate(i) if i >= gate_limit => {
                Err(format!("gate {i} is a forward or invalid reference"))
            }
            _ => Ok(()),
        };
        for (i, gate) in self.gates.iter().enumerate() {
            if gate.inputs.len() > 2 {
                return Err(format!("gate {i} has more than two inputs"));
            }
            let entries = 1u32 << gate.inputs.len();
            if u32::from(gate.table) >= (1u32 << entries) {
                return Err(format!("gate {i} has an invalid truth table"));
            }
            for &wire in &gate.inputs {
                check(wire, i)?;
            }
        }
        check(self.output, self.gates.len())
    }

    pub fn evaluate(&self, assignment: &[bool]) -> Result<bool, String> {
        self.validate()?;
        if assignment.len() != self.inputs {
            return Err("assignment length does not match the input count".into());
        }
        let mut values = vec![false; self.gates.len()];
        Ok(self.evaluate_into(assignment, &mut values))
    }

    fn evaluate_into(&self, assignment: &[bool], values: &mut [bool]) -> bool {
        fn value(wire: Wire, assignment: &[bool], values: &[bool]) -> bool {
            match wire {
                Wire::Input(i) => assignment[i],
                Wire::Gate(i) => values[i],
                Wire::Constant(bit) => bit,
            }
        }
        for (i, gate) in self.gates.iter().enumerate() {
            let index = gate
                .inputs
                .iter()
                .enumerate()
                .fold(0, |index, (port, &wire)| {
                    index | (usize::from(value(wire, assignment, values)) << port)
                });
            values[i] = ((gate.table >> index) & 1) != 0;
        }
        value(self.output, assignment, values)
    }

    /// Independent oracle: enumerates the supplied circuit, without normalization.
    pub fn brute_force(&self, max_inputs: usize) -> Result<Natural, String> {
        self.validate()?;
        if self.inputs > max_inputs || self.inputs >= usize::BITS as usize {
            return Err(format!(
                "enumeration needs {} input bits; limit is {max_inputs}",
                self.inputs
            ));
        }
        let mut assignment = vec![false; self.inputs];
        let mut values = vec![false; self.gates.len()];
        // At most 2^(word_bits-1) assignments: this accumulator cannot overflow.
        let mut count = 0u128;
        for mask in 0..(1usize << self.inputs) {
            for (i, bit) in assignment.iter_mut().enumerate() {
                *bit = ((mask >> i) & 1) != 0;
            }
            count += u128::from(self.evaluate_into(&assignment, &mut values));
        }
        Ok(Natural::from(count))
    }

    pub fn normalize(&self) -> Result<Normalized, String> {
        self.validate()?;
        let mut aliases = Vec::with_capacity(self.gates.len());
        let mut folded = Vec::<Gate>::new();
        for gate in &self.gates {
            let resolved: Vec<_> = gate
                .inputs
                .iter()
                .map(|&wire| match wire {
                    Wire::Gate(i) => aliases[i],
                    other => other,
                })
                .collect();
            let mut variables = Vec::new();
            for &wire in &resolved {
                if !matches!(wire, Wire::Constant(_)) && !variables.contains(&wire) {
                    variables.push(wire);
                }
            }
            let truth: Vec<bool> = (0..(1usize << variables.len()))
                .map(|assignment| {
                    let index = resolved.iter().enumerate().fold(0, |index, (port, wire)| {
                        let bit = match wire {
                            Wire::Constant(bit) => *bit,
                            _ => {
                                ((assignment >> variables.iter().position(|v| v == wire).unwrap())
                                    & 1)
                                    != 0
                            }
                        };
                        index | (usize::from(bit) << port)
                    });
                    ((gate.table >> index) & 1) != 0
                })
                .collect();
            let essential: Vec<_> = (0..variables.len())
                .filter(|&port| (0..truth.len()).any(|a| truth[a] != truth[a ^ (1 << port)]))
                .collect();
            let mut table = 0u8;
            for assignment in 0..(1usize << essential.len()) {
                let old = essential.iter().enumerate().fold(0, |old, (new, &port)| {
                    old | (((assignment >> new) & 1) << port)
                });
                table |= u8::from(truth[old]) << assignment;
            }
            let inputs: Vec<_> = essential.iter().map(|&i| variables[i]).collect();
            let alias = match inputs.len() {
                0 => Wire::Constant(table != 0),
                1 if table == 2 => inputs[0],
                _ => {
                    let wire = Wire::Gate(folded.len());
                    folded.push(Gate { inputs, table });
                    wire
                }
            };
            aliases.push(alias);
        }
        let output = match self.output {
            Wire::Gate(i) => aliases[i],
            other => other,
        };
        // Prune AFTER simplification: folding can disconnect whole input cones.
        let mut live = vec![false; folded.len()];
        let mut used = vec![false; self.inputs];
        let mut todo = vec![output];
        while let Some(wire) = todo.pop() {
            match wire {
                Wire::Input(i) => used[i] = true,
                Wire::Gate(i) if !live[i] => {
                    live[i] = true;
                    todo.extend(folded[i].inputs.iter().copied());
                }
                _ => {}
            }
        }
        let used_inputs: Vec<_> = (0..self.inputs).filter(|&i| used[i]).collect();
        let mut input_map = vec![0; self.inputs];
        for (new, &old) in used_inputs.iter().enumerate() {
            input_map[old] = new;
        }
        let mut gate_map = vec![0; folded.len()];
        let mut gates = Vec::new();
        let remap = |wire, gate_map: &[usize]| match wire {
            Wire::Input(i) => Wire::Input(input_map[i]),
            Wire::Gate(i) => Wire::Gate(gate_map[i]),
            other => other,
        };
        for (old, gate) in folded.into_iter().enumerate() {
            if live[old] {
                gate_map[old] = gates.len();
                gates.push(Gate {
                    inputs: gate
                        .inputs
                        .into_iter()
                        .map(|wire| remap(wire, &gate_map))
                        .collect(),
                    table: gate.table,
                });
            }
        }
        Ok(Normalized {
            circuit: Circuit {
                inputs: used_inputs.len(),
                gates,
                output: remap(output, &gate_map),
            },
            used_inputs,
            declared_inputs: self.inputs,
        })
    }

    pub fn parse(text: &str) -> Result<Self, String> {
        let mut inputs = None;
        let mut gates = Vec::new();
        let mut output = None;
        let mut names = HashMap::from([
            ("0".to_string(), Wire::Constant(false)),
            ("1".to_string(), Wire::Constant(true)),
        ]);
        for (line_no, line) in text.lines().enumerate() {
            let words: Vec<_> = line.split('#').next().unwrap().split_whitespace().collect();
            if words.is_empty() {
                continue;
            }
            let error = |message: &str| format!("line {}: {message}", line_no + 1);
            if words[0] == "inputs" {
                if inputs.is_some() || words.len() != 2 {
                    return Err(error("expected one initial 'inputs N' declaration"));
                }
                let n: usize = words[1].parse().map_err(|_| error("invalid input count"))?;
                for i in 0..n {
                    names.insert(format!("x{i}"), Wire::Input(i));
                }
                inputs = Some(n);
                continue;
            }
            if inputs.is_none() {
                return Err(error("declare 'inputs N' before gates or output"));
            }
            let lookup = |name: &str| {
                names
                    .get(name)
                    .copied()
                    .ok_or_else(|| error(&format!("unknown wire '{name}'")))
            };
            if words[0] == "output" {
                if output.is_some() || words.len() != 2 {
                    return Err(error("expected exactly one 'output WIRE' declaration"));
                }
                output = Some(lookup(words[1])?);
                continue;
            }
            if words.len() < 4 || words[1] != "=" || names.contains_key(words[0]) {
                return Err(error("expected a fresh name followed by '= OP ARG [ARG]'"));
            }
            let gate = if words[2] == "not" || words[2] == "copy" {
                if words.len() != 4 {
                    return Err(error("unary gates take one argument"));
                }
                Gate {
                    inputs: vec![lookup(words[3])?],
                    table: if words[2] == "not" { 1 } else { 2 },
                }
            } else {
                if words.len() != 5 {
                    return Err(error("binary gates take two arguments"));
                }
                let mask = match words[2] {
                    "and" => 8,
                    "or" => 14,
                    "xor" => 6,
                    "nand" => 7,
                    "nor" => 1,
                    "xnor" => 9,
                    value => {
                        let parsed = if let Some(hex) = value.strip_prefix("0x") {
                            u8::from_str_radix(hex, 16)
                        } else {
                            value.parse()
                        };
                        parsed
                            .ok()
                            .filter(|&v| v < 16)
                            .ok_or_else(|| error("unknown operation; use a B2 mask from 0 to 15"))?
                    }
                };
                Gate::binary(mask, lookup(words[3])?, lookup(words[4])?)
            };
            names.insert(words[0].to_string(), Wire::Gate(gates.len()));
            gates.push(gate);
        }
        let circuit = Self {
            inputs: inputs.ok_or("missing 'inputs N' declaration")?,
            gates,
            output: output.ok_or("missing output declaration")?,
        };
        circuit.validate()?;
        Ok(circuit)
    }
}
