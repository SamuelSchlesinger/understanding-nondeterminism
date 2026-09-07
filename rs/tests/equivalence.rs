use circuit_sat::{
    examples, solve, Algorithm, Circuit, Gate, LayoutKind, LayoutMode, Method, Natural, Options,
    Wire,
};

fn frontier_options() -> Options {
    Options {
        method: Method::Frontier,
        max_width: 16,
        trace: true,
        ..Options::default()
    }
}

#[test]
fn every_binary_mask_has_the_documented_meaning() {
    for mask in 0..16u8 {
        let circuit = Circuit::parse(&format!("inputs 2\nf = {mask} x0 x1\noutput f")).unwrap();
        for a in [false, true] {
            for b in [false, true] {
                let expected = ((mask >> (2 * usize::from(a) + usize::from(b))) & 1) != 0;
                assert_eq!(
                    circuit.evaluate(&[a, b]).unwrap(),
                    expected,
                    "mask {mask}, {a}, {b}"
                );
            }
        }
    }
}

#[test]
fn all_4096_gate_triples_match_original_circuit_enumeration() {
    for a in 0..16 {
        for b in 0..16 {
            for c in 0..16 {
                // Reconvergence, all sixteen operations, and two unused inputs.
                let circuit = Circuit {
                    inputs: 5,
                    gates: vec![
                        Gate::binary(a, Wire::Input(0), Wire::Input(1)),
                        Gate::binary(b, Wire::Gate(0), Wire::Input(2)),
                        Gate::binary(c, Wire::Gate(0), Wire::Gate(1)),
                    ],
                    output: Wire::Gate(2),
                };
                let expected = circuit.brute_force(5).unwrap();
                assert_eq!(
                    solve(&circuit, &frontier_options()).unwrap().count,
                    expected,
                    "masks {a}, {b}, {c}"
                );
            }
        }
    }
}

#[test]
fn seeded_circuits_with_constants_repeated_pins_and_unused_cones() {
    let mut rng = examples::Rng::new(20260907);
    for case in 0..800 {
        let inputs = rng.below(9);
        let size = rng.below(31);
        let seed = rng.next_u64();
        let circuit = examples::random(inputs, size, seed, case % 2 == 0);
        let expected = circuit.brute_force(8).unwrap();
        let result = solve(&circuit, &frontier_options()).unwrap();
        assert_eq!(
            result.count, expected,
            "case {case}, inputs={inputs}, gates={size}, seed={seed}"
        );
    }
}

#[test]
fn nonempty_kernels_match_enumeration_with_all_three_layout_methods() {
    let base = Circuit::parse(include_str!("../examples/k4.circuit")).unwrap();
    let masks = [1, 2, 4, 6, 7, 8, 9, 11, 13, 14];
    let mut rng = examples::Rng::new(982451653);
    for case in 0..160 {
        let mut circuit = base.clone();
        for gate in &mut circuit.gates[..6] {
            *gate = Gate::binary(
                masks[rng.below(masks.len())],
                gate.inputs[0],
                gate.inputs[1],
            );
        }
        let expected = circuit.brute_force(4).unwrap();
        for layout in [LayoutMode::Paper, LayoutMode::Exact, LayoutMode::Greedy] {
            let options = Options {
                layout,
                ..frontier_options()
            };
            let result = solve(&circuit, &options).unwrap();
            assert_eq!(result.algorithm, Algorithm::Frontier);
            assert_eq!(result.graph.as_ref().unwrap().kernel_vertices, 14);
            assert_eq!(result.count, expected, "case {case}, layout {layout:?}");
            let width = result.layout.as_ref().unwrap().width;
            assert_eq!(result.peak_entries, 1 << width);
            assert!(result.peak_stored_entries <= 2 * (1 << width));
            assert_eq!(result.trace.last().unwrap().after, 0);
            assert!(result.products <= (result.trace.len() as u64) * (1 << (width + 1)));
        }
    }
}

#[test]
fn normalization_rechecks_reachability_after_folding() {
    let circuit = Circuit::parse(
        "inputs 5\na = xor x0 x1\nb = and a 0\nc = or b x4\nunused = xor x2 x3\noutput c",
    )
    .unwrap();
    let normalized = circuit.normalize().unwrap();
    assert_eq!(normalized.used_inputs, vec![4]);
    assert!(normalized.circuit.gates.is_empty());
    assert_eq!(
        solve(&circuit, &Options::default()).unwrap().count,
        Natural::from(16)
    );
    let repeated = Circuit::parse("inputs 3\na = xor x0 x1\nb = xor a a\noutput b").unwrap();
    assert_eq!(
        solve(&repeated, &Options::default()).unwrap().count,
        Natural::zero()
    );
}

#[test]
fn high_fanout_and_large_counts_remain_exact() {
    let mut high_fanout = Circuit::parse(include_str!("../examples/k4.circuit")).unwrap();
    high_fanout
        .gates
        .push(Gate::binary(6, Wire::Input(0), high_fanout.output));
    high_fanout.output = Wire::Gate(high_fanout.gates.len() - 1);
    let result = solve(&high_fanout, &frontier_options()).unwrap();
    assert!(result.graph.as_ref().unwrap().reductions.equality_splits > 0);
    assert_eq!(result.count, high_fanout.brute_force(4).unwrap());

    let large = examples::parity_k4(201);
    let result = solve(&large, &frontier_options()).unwrap();
    let mut expected = Natural::from(5);
    expected.shl_assign(200);
    assert_eq!(result.count, expected);
    assert_eq!(result.algorithm, Algorithm::Frontier);
    assert!(result.peak_entry_bits > 128);
    assert_eq!(result.used_inputs, 204);

    let mut expected = Natural::one();
    expected.shl_assign(299);
    for output in [
        Wire::Input(217),
        Wire::Constant(false),
        Wire::Constant(true),
    ] {
        let circuit = Circuit {
            inputs: 300,
            gates: Vec::new(),
            output,
        };
        let result = solve(&circuit, &Options::default()).unwrap();
        match output {
            Wire::Input(_) => assert_eq!(result.count, expected),
            Wire::Constant(false) => assert_eq!(result.count, Natural::zero()),
            _ => {
                let mut all = expected.clone();
                all.shl_assign(1);
                assert_eq!(result.count, all);
            }
        }
    }
}

#[test]
fn automatic_choice_and_limits_are_enforced() {
    let small = Circuit::parse(include_str!("../examples/k4.circuit")).unwrap();
    let result = solve(&small, &Options::default()).unwrap();
    assert_eq!(result.algorithm, Algorithm::Enumeration);
    assert_eq!(result.layout.unwrap().kind, LayoutKind::Paper);
    let wider_input = examples::parity_k4(5);
    let result = solve(&wider_input, &Options::default()).unwrap();
    assert_eq!(result.algorithm, Algorithm::Frontier);
    assert_eq!(result.count, wider_input.brute_force(8).unwrap());

    let mut limited = Options {
        max_width: 1,
        ..frontier_options()
    };
    assert!(solve(&small, &limited)
        .unwrap_err()
        .contains("exceeds --max-width"));
    limited.method = Method::Auto;
    assert_eq!(
        solve(&small, &limited).unwrap().algorithm,
        Algorithm::Enumeration
    );
    limited.max_enumeration = 0;
    assert!(solve(&small, &limited).is_err());
    let exact = Options {
        layout: LayoutMode::Exact,
        ..frontier_options()
    };
    assert!(solve(&examples::cube(), &exact)
        .unwrap_err()
        .contains("exact layout search is limited"));
}

#[test]
fn resource_caps_do_not_replace_the_minimum_exponent_by_a_larger_one() {
    let small = Circuit::parse(include_str!("../examples/k4.circuit")).unwrap();
    // b < w: a low enumeration cap must not force the more costly frontier.
    let options = Options {
        max_enumeration: 3,
        ..Options::default()
    };
    assert!(solve(&small, &options).is_err());
    // w < b: a low frontier cap must not force input enumeration either.
    let wider = examples::parity_k4(17);
    let options = Options {
        max_width: 1,
        ..Options::default()
    };
    assert!(solve(&wider, &options)
        .unwrap_err()
        .contains("exceeds --max-width"));
    let epsilon = "1/7".parse().unwrap();
    let result = solve(
        &wider,
        &Options {
            epsilon,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(
        result
            .layout
            .as_ref()
            .unwrap()
            .paper
            .as_ref()
            .unwrap()
            .epsilon,
        epsilon
    );
    assert_eq!(result.count, wider.brute_force(20).unwrap());
}

#[test]
fn malformed_circuits_and_out_of_range_tables_are_rejected() {
    for text in [
        "output 1",
        "inputs 2",
        "inputs 2\na = and x0 nope\noutput a",
        "inputs 1\nx0 = not x0\noutput x0",
        "inputs 1\na = 16 x0 x0\noutput a",
        "inputs 1\noutput x0\noutput 0",
        "inputs 1\na = not x0 x0\noutput a",
    ] {
        assert!(Circuit::parse(text).is_err(), "{text}");
    }
    let invalid = Circuit {
        inputs: 1,
        gates: vec![Gate::binary(255, Wire::Input(0), Wire::Input(0))],
        output: Wire::Gate(0),
    };
    assert!(solve(&invalid, &Options::default()).is_err());
    let cycle = Circuit {
        inputs: 1,
        gates: vec![Gate::binary(8, Wire::Gate(0), Wire::Input(0))],
        output: Wire::Gate(0),
    };
    assert!(solve(&cycle, &Options::default()).is_err());
}
