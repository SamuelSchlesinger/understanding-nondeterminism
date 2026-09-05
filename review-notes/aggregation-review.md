# Aggregation mathematical review

Reviewed `sections/aggregation.tex`, with the circuit model in `main.tex` and the consistency construction in `sections/structure.tex` and `sections/counting-algorithm.tex` as context. This was a proof review; no experiments or manuscript edits were performed.

The main propositions and the matching application are sound. There are no Error-severity findings. One local parameter condition should be made explicit, and one phrase confuses witness length with witness count.

## Findings

### 1. Repeat the field-size hypothesis for the shared-input variant

- **Location:** `sections/aggregation.tex:67-71`, paragraph following Proposition `prop:fingerprint`.
- **Severity:** Gap.
- **Finding:** The sentence asserting that the same argument uses `t = h + 1` should explicitly carry forward `Q >= 2b`. Under the proposition's general assumption `Q > b`, the union-bound condition is `2^h (b/Q)^t < 1`; `t = h + 1` does not automatically satisfy this inequality. For example, `b = 3`, `Q = 4`, and `h = 1` give `2 (3/4)^2 = 9/8`. This is a missing hypothesis for the stated argument, not a counterexample to the main proposition.
- **Suggested fix:** Write: "When `Q >= 2b`, the same argument uses `t = h + 1` ..." Optionally record the resulting shared-preprocessing cost as `p + t(T_core + lambda) - 1`, where `T_core` excludes those `p` preprocessing gates. Choosing `Q = 2^ceil(log_2(2b))` also makes `lambda = O(log(b+1))` explicit in the polynomial-overhead claim.

### 2. Use "witness bits" in the matching verifier description

- **Location:** `sections/aggregation.tex:114-116`, matching example.
- **Severity:** Polish.
- **Finding:** `b = O(r log r)` is the number of witness bits, not the number of witnesses. There are `r!` valid permutation encodings before testing graph edges. Each row uses `ceil(log_2 r)` bits.
- **Suggested fix:** Replace "`b = O(r log r)` witnesses" with "`b = O(r log r)` witness bits".

## Proof checks

### Polynomial support and cancellation

The support map `y -> {i : y_i = 1}` is injective. Every satisfying witness contributes a different squarefree monomial with coefficient one in characteristic two. Consequently, the formal polynomial is nonzero if and only if the fiber is nonempty. The example `1 + Z_2` correctly distinguishes cancellation at a chosen evaluation point from cancellation in the formal polynomial. Discarding unused witness bits before defining the polynomial is legitimate for existential projection.

### Exact hardwiring and gate costs

For each positive input, independent uniform evaluation points give failure probability at most `(b/Q)^t`. The union bound ranges over at most `2^a` distinct ordinary inputs. A list with no failure therefore exists under the strict inequality in the proposition. Negative inputs yield the zero field element at every point, so ORing the evaluations is exact after hardwiring. The same finite-fiber reasoning works with at most `2^h` distinct formal-signal values; correlated preprocessing values create no problem.

In the manuscript's full binary basis, each field-value zero test uses `lambda - 1` OR gates, and the final OR uses `t - 1`. Thus the displayed count is exactly the claimed upper bound:

`tT + t(lambda - 1) + (t - 1) = t(T + lambda) - 1`.

The assumptions include `b >= 1` and `Q > b`, so `lambda >= 1` and this expression has no degenerate negative-cost case. The result is explicitly nonuniform; the uniform randomized conclusion is correctly conditional on a uniformly efficient evaluator. No algorithm for finding the all-input list is claimed.

In a fixed polynomial basis, field addition costs `O(lambda)` Boolean gates. Schoolbook multiplication costs `O(lambda^2)` gates, and reduction of a degree-at-most-`2lambda-2` polynomial modulo a fixed irreducible polynomial is a fixed linear map with `O(lambda^2)` XOR cost. Constants and Boolean-to-field input embeddings use free constant wires and fan-out. The division-free conversion `T = O((A+1)lambda^2)` is therefore valid under the manuscript's gate model.

### Cycle rank and tensor interpretation

Attaching the weight table `(1, Z_i)` to an existing witness equality vertex adds exactly one vertex and one edge and keeps the graph connected. Thus `|E| - |V| + 1` is unchanged. Repeating this for all used witnesses adds `O(b)` vertices; the consistency budget bounds `b` by `q+1`, so this remains linear in the core size.

The original gate-value extensions and the internal equality-tree extensions are unique. Each surviving assignment therefore receives precisely its intended monomial weight. There is no spurious even multiplicity that would destroy the characteristic-two interpretation. Equality splitting, diagonal loop sums, simultaneous parallel-edge sums, and the subsequent contractions are distributive identities and remain valid over the field. Field arithmetic adds a polynomial bit cost but does not alter the cycle exponent. These statements justify preservation for the witness-bit unary tags used in this paragraph.

### Universal linear obstruction

The obstruction is exactly a dimension argument. A dependence over `F_2` has coefficients zero or one, hence gives a nonempty zero-sum subset of witness weights. Conversely, forbidding every such subset forces all `2^b` vectors to be independent, and hence `L >= 2^b`.

For fixed evaluation points, concatenating the field outputs is indeed a binary linear map of the witness-indicator vector: each witness column is the concatenation of its monomial evaluations. The verifier-dependent choice of points only has to separate that verifier's at most `2^a` fibers, so it does not meet the universal hypothesis. The stated limitation to universal linear aggregates, and the exclusion of general circuit lower bounds and restricted-verifier lower bounds, are accurate.

### Matching's different tags are covered by the proof

The matching example uses `r^2` edge variables, whereas the earlier polynomial uses `b` witness-bit variables. These are different polynomial encodings. The text correctly says to apply the **proof** of the transfer proposition: the zero-probability estimate depends on total degree, not on equality between the number of tag variables and the witness length. For matching, that degree is `r`.

For a fixed Boolean matrix `X`, determinant expansion in characteristic two has coefficient one for every present permutation matching. Two distinct permutations use different edge sets and therefore produce different monomials. The polynomial is nonzero exactly on graphs with a perfect matching.

Gaussian elimination can use fixed-length scans to choose pivots, conditional row swaps, and a flag for a missing pivot; branches are implemented by Boolean multiplexers. A nonzero pivot inverse is `u^(Q-2)`, computable using `O(log Q)` field multiplications. Missing-pivot branches avoid relying on any value assigned to the inverse of zero. Thus there is a Boolean evaluator of size polynomial in `r` and `log Q`. Taking, explicitly, `Q = 2^ceil(log_2(2r))` gives `log Q = O(log(r+1))`, and the `t = r^2 + 1` evaluations and all zero tests retain polynomial size.

The unary graph-preservation claim is not needed for this matching proof. With binary column witnesses, selecting the weight `Z_{i,sigma(i)}` depends on a block of witness bits, so it would require a separate construction to claim that these particular edge tags preserve the binary verifier's original cycle rank. The manuscript currently makes no such claim, and its matching conclusion is valid without one.

The `O(r^2 log r)` verifier-size bound is conservative and valid: pairwise column comparisons cost `O(r^2 log r)`, and row multiplexers plus out-of-range rejection cost no more. Integer counting of these uniquely encoded permutation witnesses is the permanent. The closing text correctly makes no unconditional complexity separation and no improved universal projection exponent.

## Source spot check and review boundary

The primary [Valiant paper](https://www.cs.bu.edu/faculty/gacs/courses/cs535/papers/Valiant_permanent.pdf), Theorem 1 and its introduction, confirms the permanent's zero-one counting completeness and its correspondence with bipartite perfect matchings. The algebraic identities, root-bound induction, and circuit conversions above were checked directly from the manuscript's arguments. This note is not a full bibliographic audit or an independent review of the prior `1/5` theorem.
