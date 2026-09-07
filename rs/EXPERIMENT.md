# Scaling checks

## Initial prototype: exact layout

Run date: 2026-09-07. Local macOS release build, Rust
`1.99.0-nightly (d0babd8b6 2026-07-15)`, no external crates.

This is the historical run before the paper construction was implemented.
The protocol in `examples/scaling.rs` uses one warmup, seven measured
repetitions, and median times. The circuit is the K4 vertex-cover example with
one input replaced by a parity chain. Every count agrees with exhaustive
evaluation of the original circuit and with the formula `5 * 2^(n-4)`.

| Inputs | Gates | Kernel vertices | Cutwidth | Peak table entries | Layout ms | Total solver ms | Exhaustive ms |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 11 | 14 | 6 | 64 | 0.129 | 0.226 | 0.001 |
| 8 | 15 | 14 | 6 | 64 | 0.128 | 0.264 | 0.018 |
| 12 | 19 | 14 | 6 | 64 | 0.089 | 0.247 | 0.380 |
| 16 | 23 | 14 | 6 | 64 | 0.054 | 0.181 | 4.377 |
| 20 | 27 | 14 | 6 | 64 | 0.056 | 0.168 | 56.310 |

Times vary by machine and run. The solver total includes normalization,
reduction, layout search, frontier contraction, and restoring unused inputs.
The displayed decline in small solver times is timing variation, not a claim
that the algorithm gets faster as input size grows.

The input count grows while the remaining kernel stays fixed. Consequently
the frontier needs only 64 entries per table (96 simultaneous entries at its
peak), while brute force enumerates `2^n` inputs. Exact layout search separately
uses 16,384 subset states and 81,920 bytes for its three DP arrays. These are
logical storage counts, not measurements of process resident memory.

The separate command `parity-k4 201` counted a 204-input, 211-gate circuit in
about 11.4 ms on this run. Its count is `5 * 2^200`; its largest frontier value
has 203 bits, and its peak table still has 64 entries. That result was checked
against the closed form, not exhaustive enumeration.

This is a deliberately structured correctness and scaling demonstration.
The prototype was slower than enumeration on the smallest cases, and this
experiment did not test a general four-gate SAT guarantee. The then-default
exact/greedy layout did not supply the paper's preprocessing guarantee.

## Paper construction: current default

Run date: 2026-09-07, same environment and fixed protocol. This run uses the
complete bisection/bag/median construction with `epsilon=1/100`. The experiment
forces frontier evaluation even when automatic mode would enumerate instead.
Each count was checked against the original circuit and the closed form.

| Inputs | Gates | Kernel vertices | Cutwidth | Peak table entries | Layout ms | Total solver ms | Exhaustive ms |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 11 | 14 | 7 | 128 | 0.011 | 0.049 | 0.001 |
| 8 | 15 | 14 | 7 | 128 | 0.009 | 0.067 | 0.009 |
| 12 | 19 | 14 | 7 | 128 | 0.008 | 0.077 | 0.174 |
| 16 | 23 | 14 | 7 | 128 | 0.010 | 0.104 | 3.151 |
| 20 | 27 | 14 | 7 | 128 | 0.015 | 0.141 | 62.448 |

The new order has width 7 on this family, compared with the old optimum of 6.
Its frontier peaks at 128 entries per table and 192 entries across both tables.
It constructs path bags instead of allocating an exponential subset-DP table.
As in the earlier run, these small structured instances do not estimate the
algorithm's worst-case polynomial degree or its asymptotic exponent.

The current `parity-k4 201` run returned `5 * 2^200` in 6.180 ms, including
0.041 ms of layout construction and 6.047 ms of graph reduction. The layout
had 30 bags with 129 stored vertex entries, pathwidth 5, and cutwidth 7. Its
largest frontier count had 203 bits. The bisection construction performed four
improving rounds and returned its documented finite-size balancing case. The
count was independently checked against the closed form.

The source-based bound is proved in [ALGORITHM.md](ALGORITHM.md). These timings
are evidence that the implemented construction executes and counts correctly
on these inputs, not a proof by benchmarking or a comparison with modern SAT
solvers. Times from separate runs should not be treated as a controlled
head-to-head performance comparison.

Run the current protocol with:

```sh
cargo run --offline --release --manifest-path rs/Cargo.toml --example scaling
cargo run --offline --release --manifest-path rs/Cargo.toml -- parity-k4 201
```
