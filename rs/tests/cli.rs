use std::io::Write;
use std::process::{Command, Stdio};

fn run(input: &str, options: &[&str]) -> std::process::Output {
    let mut process = Command::new(env!("CARGO_BIN_EXE_circuit-sat"))
        .args(["count", "-"])
        .args(options)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    process.wait_with_output().unwrap()
}

#[test]
fn stdin_count_verification_and_trace_work_together() {
    let result = run(
        include_str!("../examples/k4.circuit"),
        &["--verify", "--trace", "--method", "frontier"],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let output = String::from_utf8(result.stdout).unwrap();
    assert!(output.contains("#SAT: 5"));
    assert!(output.contains("frontier contraction"));
    assert!(output.contains("paper construction"));
    assert!(output.contains("Path decomposition:"));
    assert!(output.contains("Nonzero entries"));
    assert!(output.contains("Exhaustive check: PASS"));
}

#[test]
fn unsatisfiable_and_invalid_inputs_have_different_outcomes() {
    let unsat = run("inputs 1\na = xor x0 x0\noutput a", &["--verify"]);
    assert!(unsat.status.success());
    assert!(String::from_utf8(unsat.stdout)
        .unwrap()
        .contains("SAT: false    #SAT: 0"));
    let invalid = run("inputs 1\na = and missing x0\noutput a", &[]);
    assert!(!invalid.status.success());
    assert!(String::from_utf8(invalid.stderr)
        .unwrap()
        .contains("unknown wire"));
    let capped = run(
        include_str!("../examples/k4.circuit"),
        &["--method", "frontier", "--max-width", "1"],
    );
    assert!(!capped.status.success());
    assert!(String::from_utf8(capped.stderr)
        .unwrap()
        .contains("exceeds --max-width"));
}

#[test]
fn explicit_paper_slack_is_exact_and_invalid_values_are_rejected() {
    let input = include_str!("../examples/k4.circuit");
    let result = run(
        input,
        &[
            "--layout",
            "paper",
            "--epsilon",
            "2/200",
            "--method",
            "frontier",
        ],
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let output = String::from_utf8(result.stdout).unwrap();
    assert!(output.contains("#SAT: 5"));
    assert!(output.contains("Paper epsilon: 1/100"));
    for value in ["0", "0/1", "1/0", "NaN", "1/2/3"] {
        let result = run(input, &["--epsilon", value]);
        assert!(!result.status.success());
        assert!(String::from_utf8(result.stderr)
            .unwrap()
            .contains("epsilon must be a fraction"));
    }
}
