# Rust circuit counting

This implements exact SAT/#SAT for acyclic, single-output circuits with arbitrary
Boolean gates of fan-in at most two and unrestricted fanout. It follows the
[SAT note](../sat-note.tex): normalize the circuit, encode consistent values,
split equality factors, reduce to a cubic kernel, and count with frontier tables.
Counts use arbitrary-precision integers. There are no external crates.

The default layout now implements the paper's constructive route:
Monien--Preis helpful-set bisection, Fomin--Hoie path decomposition, and the
note's median-edge ordering. The full automatic algorithm has the note's
fixed-epsilon time and space bounds. Its polynomial preprocessing can be
expensive. [ALGORITHM.md](ALGORITHM.md) maps the code to the proofs and accounts
for every stage, including finite-size exceptions and the two elementary
subroutine choices. Exact subset search and greedy layout remain available
as explicit experimental alternatives.

## Run it

From the repository root:

```sh
cargo run --release --manifest-path rs/Cargo.toml -- demo
cargo run --release --manifest-path rs/Cargo.toml -- count rs/examples/k4.circuit --method frontier --verify --trace
cargo run --release --manifest-path rs/Cargo.toml -- random 12 36 42 --verify
cargo run --release --manifest-path rs/Cargo.toml -- parity-k4 201
cargo run --release --manifest-path rs/Cargo.toml -- count rs/examples/k4.circuit --layout paper --epsilon 1/100 --verify
```

All Cargo commands also work with `--offline`. `demo` checks counts 3, 5, and 35
against exhaustive search, and checks a 200-input parity circuit against `2^199`.
Its default nonempty-kernel examples force frontier contraction so that the
pipeline is visible; `--method enumerate` instead enumerates the small cases.

`parity-k4 201` is a 204-input example with a nonempty cubic kernel and exact
count `5 * 2^200`. It replaces one input of the K4 circuit by a parity circuit.
The command checks the result against that closed form; it does not enumerate
all 204 inputs. `--verify` always refers to exhaustive enumeration of the
**original** circuit, independently of normalization.

The output reports normalization, cycle accounting, reductions, the layout
method and measured cutwidth, frontier entries, integer bit lengths, and stage
times. It also reports the bisection result, pathwidth, and median width bound.
`--trace` adds one row per contracted vertex. Peak frontier entries count
table cells, not process resident memory; integer limbs and allocator overhead
also occupy memory. Path decomposition storage is polynomial and separate
from the frontier tables. Experimental exact-layout DP storage is also
reported separately.

## Circuit format

```text
# The note's accepting assignments are 010, 101, and 110.
inputs 3
a = or x0 x1
b = xor x1 x2
c = and a b
output c
```

`inputs N` declares `x0` through `x(N-1)`. Constants are `0` and `1`. Gate names
must be fresh, and references must name an input, constant, or earlier gate.
Use spaces around `=`. Blank lines and `#` comments are allowed. The designated
output need not be the last gate; unused cones are pruned.

Binary operations are `and`, `or`, `xor`, `nand`, `nor`, `xnor`, or a decimal/
hexadecimal mask from 0 to 15. Bit `2*a+b` of the mask is the output on `(a,b)`;
for example `8` is AND and `0xe` is OR. `not` and `copy` take one argument.
Repeated inputs and constant pins are allowed. `count -` reads from stdin.

## Layouts, limits, and resource costs

- `--method auto` reduces first, then prefers enumeration when used inputs
  `b <= w`; otherwise it contracts. If a configured limit prevents the chosen
  method, it returns an error, preserving the exponent `min(b,w)`.
  `frontier` and `enumerate`
  force the respective method; trivial outputs still return directly.
- `--layout auto` and `--layout paper` use the constructive proof for every
  nonempty kernel. `--epsilon P/Q` sets the fixed positive slack in the
  headline exponent `1/4 + epsilon` (default `1/100`, at most `1`). Fractions
  are exact. A finite-size exception is a proved additive-constant case;
  it is explicitly reported and does not mean an approximate count.
- `--layout exact` uses exponential subset DP. `--exact-limit N` limits its
  kernel size (default 18, at most 22). `--layout greedy` uses a deterministic
  heuristic. These two modes do not carry the paper's running-time bound.
  Every mode measures the actual cutwidth.
- `--max-width N` caps frontier bits (default 20); `--max-enumeration N` caps
  enumerated input bits (default 24). These are separate limits. Failure to fit
  returns an error, never an approximate count or an UNSAT result.

For a supplied order of cutwidth `w` on `N` cubic vertices, the counter uses
`O(N * 2^w)` integer arithmetic operations, with polynomial costs for indexing
and arbitrary-precision arithmetic. It retains two successive frontiers:
`O(2^w)` cells, each holding an integer with polynomially many bits. High-degree
equality tables are expanded into ternary factors before listing entries.

For each fixed epsilon, the default layout uses polynomial time and space.
The helpful-set search enumerates sets only until a source lemma supplies a
witness of epsilon-dependent bounded size; balancing sets also have bounded
size. This is a polynomial of potentially large degree, not a claim of small
or near-linear overhead. The path decomposition contains `O(N)` bags and
uses `O(N^2)` stored vertex entries. Its validity and `w <= pathwidth+2` are
checked before contraction. The finite-instance width bound and the complete
derivation of the `1/4+epsilon` exponent are in [ALGORITHM.md](ALGORITHM.md).

Exact layout search uses

```text
DP[S] = max(cut(S), min over v in S of DP[S without v])
```

and costs `O(N * 2^N)` time and `O(2^N)` space. Its three arrays contain
approximately `5 * 2^N` bytes, excluding graph and container overhead. This can
dominate the frontier tables. Greedy layout uses multiple deterministic tie
orders and adjacent exchanges; it has no proved `w <= kappa/3 + o(kappa)` bound.
The reduction implementation is polynomial but currently rebuilds incidence
lists after operations, so it is not optimized for very large input circuits.

## Tests and the scaling experiment

```sh
cargo test --offline --manifest-path rs/Cargo.toml
cargo clippy --offline --manifest-path rs/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path rs/Cargo.toml --all -- --check
cargo run --offline --release --manifest-path rs/Cargo.toml --example scaling
```

The tests cover all 16 gate masks, 4,096 gate triples, 800 seeded circuits,
160 nonempty-kernel variants using all three layout methods, and weighted contraction
against independent edge enumeration. They also cover repeated pins, constants,
pruning, high fanout, loops, parallel edges, counts beyond 128 bits, layout
optimality on small graphs, CLI errors, and configured limits. Layout checks
exhaust every subcubic graph and every anchor through five vertices, validate
centroid bags on trees through 511 vertices, and check the complete construction
on seeded cubic graphs, a graph with a bridge, and an 80-vertex prism. These
finite checks complement the source-based resource proof; they do not prove
the imported graph lemmas by testing.

The fixed scaling protocol uses K4 with one input replaced by parity of
`1, 5, 9, 13, 17` bits: total input counts `4, 8, 12, 16, 20`. It performs one
warmup and seven measured repetitions per case, reports median times, includes
layout time in the solver total, and compares every count with both exhaustive
search and `5 * 2^(parity_inputs-1)`. The kernel remains small while input
enumeration grows. This deliberately structured experiment illustrates the
mechanism; it does not demonstrate a general SAT speedup or the four-gate
worst-case guarantee.

[EXPERIMENT.md](EXPERIMENT.md) records the initial prototype measurements and
the subsequent run with the paper construction. The algorithms and dates of
the measurements are distinguished there.

## Source map

| File | Role |
| --- | --- |
| `src/circuit.rs` | Input format, evaluation, normalization, exhaustive oracle |
| `src/network.rs` | Equality expansion, exact reductions, frontier invariant |
| `src/layout.rs` | Layout selection and experimental alternatives |
| `src/layout/bisection.rs` | Guaranteed helpful-set bisection |
| `src/layout/paper.rs` | Path decompositions, median orders, certificates |
| `ALGORITHM.md` | Source correspondence and complete resource argument |
| `src/natural.rs` | Addition, multiplication, shifts, decimal output |
| `src/solver.rs` | Method selection, limits, counts, instrumentation |
| `src/main.rs` | CLI and demos |
| `tests/` | Differential checks and CLI integration |

Code is covered by the repository's [MIT license](../LICENSES/MIT.txt).
