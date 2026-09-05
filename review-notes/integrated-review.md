# Integrated manuscript review

Date: 2026-09-04.

Scope: `main.tex` and every included file in `sections/`, including the newly added `counting-algorithm.tex`, consistency-budget figure, and independent-preprocessing example. This was a bounded independent completeness, coherence, and mathematical-endpoint review. The manuscript was not edited. Transient bibliography and build-file assembly were excluded.

## Disposition

No Error-severity finding or mathematical proof gap was identified in this pass. The two findings below are presentation precision issues. The review does not establish optimality or literature priority, and finite checks are evidence only on their specified finite domains.

## Findings

### P1. Specify the roles and representation conventions of the finite enumerations

- **Location:** `sections/methods.tex`, finite-check table, lines 73-80; `scripts/check_claims.py`, `check_forests` and `check_projection_distribution`.
- **Severity:** Polish.
- **Finding:** The table puts the 65,536-relation enumeration in the "Forest compiler" row. That enumeration checks the exact projected-function distribution and zero-count histogram; it does not run the forest compiler on 65,536 verifier circuits. Also, the 9,280-circuit enumeration uses topologically ordered binary truth-table gates, repeated pins allowed, with the final gate designated as output. Saying simply "all one- and two-gate circuits" leaves those representation conventions implicit. The surrounding text correctly limits the force of experiments, but the table can state their exact scope more directly.
- **Suggested fix:** Rename the row to "Forest and distribution checks", explicitly identify the 65,536 relations as a projection-distribution check, and describe the 9,280 cases as one- and two-gate encodings with the last gate as output. Keep the distinction between exhaustive finite domains and seeded finite samples.

### P2. Preview the newly proved uniform counting endpoint

- **Location:** `main.tex`, abstract and "Reading route and contributions"; `sections/counting-algorithm.tex`, Corollary `cor:gate-counting`.
- **Severity:** Polish.
- **Finding:** The new counting corollary is a separate uniform algorithmic conclusion, with a limiting gate exponent of 1/4 and an explicit exponential-space allowance. The abstract and reading route still describe only the two projection simulations and conditional hardness. A reader scanning those parts could miss a consequential new endpoint that is now fully proved in the body.
- **Suggested fix:** Add one concise sentence to the abstract or reading route stating the fixed-epsilon counting consequence and its space scope. Preserve the distinction between this proved implication and the unresolved comparison with prior circuit-SAT algorithms.

## Checked mathematical endpoints

### Consistency budget, cubic reduction, and 1/5 rate

The incidence counts give `kappa = ell - q - b + 1` and `h + b + kappa <= q + 1`. Independent signals are not tensor indices, and after fixing dependent indices a gate table needs only constants, one independent signal, or its complement. This is why ordinary preprocessing is correctly charged once as `p`.

The equality-tree expansion preserves cycle rank and has linear size. Loop diagonal sums and joint contractions of parallel edges preserve the actual represented function. A surviving connected simple cubic graph satisfies `N = 2(kappa(K)-1) <= 2(kappa-1)`; empty kernels, including acyclic and unicyclic cases after reduction, incur only the charged linear work.

The median-order proof handles ordinary cuts and tied medians separately. At an ordinary cut each charged vertex receives at most one edge, and a tied cut has at most one exceptional edge. For the frontier step, `|F_i| = |F_(i-1)| + 3 - 2j` indeed implies `|F_i| + j <= w+1`, so the per-vertex arithmetic count has exponential rate `2^w`, without an extra factor in the width exponent.

For `z = min{h,b,alpha*kappa}`, the inequality `(2+1/alpha)z <= q+1` gives `beta = alpha/(1+2alpha)`, tending to `1/5` as `alpha` tends to `1/3` from above. Synthesis on the formal independent inputs remains a nonuniform circuit-size step. The text does not assert an endpoint bound with an epsilon-independent constant.

The external load-bearing width statement was freshly checked against [Fomin and Hoie, Theorem 5 and Section 4](https://fedorvf.github.io/articles/2006/2006b.pdf): the paper states the asymptotic bound for maximum degree at most three and explicitly gives polynomial-time construction for fixed epsilon. This source check does not establish priority of the manuscript's combined bounds.

### Necessary window and sparse-fiber population

Canceling the positive factor `s+1` from the assumed hardness and the universal simulation gives exactly

`m - log(A_epsilon L) <= (1/5+epsilon)(s+1)`.

For a family with `m -> infinity` and `log L = o(m)`, hold epsilon fixed first. The term `log A_epsilon / m` then vanishes, giving `liminf (s+1)/m >= 1/(1/5+epsilon)`. Taking the supremum over fixed positive epsilons gives the stated lower limit 5. No epsilon-dependent constant is silently treated as uniform. The ceiling independently gives `s+1 <= L M_n / 2^m`.

The sparse-profile argument covers all rows with at least `K` witnesses and corrects only missed positive rows below that threshold. Choosing `K = ceil(c n L)` with a sufficiently large absolute constant makes the dense-row contribution at most half the assumed hardness. The resulting inequality `R/log(R+2) >= c_0 T/n` first gives `R >= c_0 T/n`, and substituting this back into the logarithm gives the uniform lower bound of order `(T/n) log(2+T/n)`. The large-threshold case is correctly handled by applying sparse synthesis to the entire positive support. The assumptions `n >= 2`, zero exceptions, integer thresholds, and small `T/n` do not invalidate the argument.

These remain necessary conditions. The text does not turn a large fiber-cover requirement into a lower bound on the minimum projected circuit size.

### Occupancy and analytical separations

The selected-restriction proof uses a single witness set throughout the circuit; gate predecessor restrictions are compatible because supports are nested along dependencies. The output-support occurrence is intentionally charged in addition to the output gate's occurrence. Empty samples and the `t=0` convention are explicit. Sparse correction uses the concavity of `z/log(z+2)`, with an absolute construction constant, and the final OR is absorbed correctly.

The cyclic family's gate and support sums agree with the supplied representation, and its exact projected complexity follows from a matching essential-input lower bound. The two same-row-mass distributions have the asserted identical row masses. In the lower bound for the diffuse distribution's guarantee expression, the split at `t = M/8` and the estimate `E >= 2^(n-2) exp(-N/6)` are valid for `k >= 3`. The coverage-versus-occupancy tradeoff at `L >= 8` also has consistent constants. The manuscript repeatedly identifies these as separations of guarantee expressions, rather than hardness lower bounds for their easy projections.

### New independent-preprocessing example

For the specified implementation of `H_t`, the independent-gate count is `t + 2*binom(t,2) - 1 = t^2 - 1`. The dependent witness OR chain and final AND give `q=b=t`, `h=1`, and `kappa=0`. Exactly the `t` pair-indicator gates have fan-out at least two when `t >= 3`, so `r=t`. The pair-product gates exhibit a subdivision of `K_t` in the full undirected graph. Every input is essential by the assignments described in the text. The text does not claim that this quadratic supplied implementation is minimum size.

### New uniform counting consequence

Each assignment to the used inputs extends uniquely to gate values and to incidence-edge values. Ternary equality expansions also have unique internal extensions. Thus replacing Boolean OR/AND by integer addition/multiplication counts assignments with the intended multiplicity. The loop and parallel-edge operations are genuine distributive identities over the nonnegative integers.

The region invariant bounds each intermediate entry by the number of assignments to only `O(s+1)` original expanded indices, hence by `2^O(s+1)`, with polynomial bit cost. Inputs removed from the normalized cone contribute the factor `2^(u-b)` only at the end; constant and designated-input outputs are handled directly. The constructive width theorem supplies the order without advice for fixed epsilon. Balancing `b` and `alpha(s+1-b)` gives `alpha/(1+alpha)`, tending to `1/4`. The consequence for `s <= (4-gamma)u` follows after fixing sufficiently small epsilon. The space statement is a valid majorant and does not imply polynomial space.

### Remaining integrated claims

The basic synthesis and counting arguments, worst-case definition, nonuniform complexity-class equivalence, conditional nonuniform-SETH quantifiers, and ceiling comparisons are mutually consistent on the stated domains. The conditional lower bound is correctly limited to infinitely many lengths for a width `k` that may depend on epsilon; it is not promoted to one fixed-family `2^(m-o(m))` lower bound.

## Validation rerun

All four existing validation scripts completed successfully during this review:

| Command | Observed finite domain |
| --- | --- |
| `python3 scripts/check_claims.py` | 9,280 exhaustive small encodings; 2,000 seeded circuits and three fixtures; projection-distribution identities for all 65,536 relations with `(n,m)=(2,2)` |
| `python3 scripts/check_coverage.py` | 362 sparse supports; 896 circuit/sample pairs; 140 exact rational occupancy and exception cases |
| `python3 research/structural/data/check_tensor_compiler.py` | Five zero-gate cases; 9,280 exhaustive small encodings; 600 seeded circuits; 140 symbolic tensor networks |
| `python3 research/examples/data/check_examples.py` | 25,600 input/witness pairs; support formulas for `k=3,...,10`; displayed finite distribution comparisons; 141 sparse-correction instances; the incompatible-cover fixture |

The existing tensor checker checks Boolean projection, not the newly added integer-counting implementation. The counting corollary above was reviewed analytically. None of these runs verifies the asymptotic pathwidth theorem or an asymptotic lower bound.

## Final closing-section delta review

Scope: only the three new closing research avenues in `main.tex`, lines 867-911. No experiments or full-script reruns were performed for this delta.

**Disposition:** No mathematical or quantifier error found; no additional finding requiring revision.

- The conditional cutwidth transfer has the stated coefficients. For a fixed nonnegative rate `c`, `N <= 2*kappa` gives the cycle rate `2c*kappa + o(kappa)`; bounded kernels contribute only an asymptotically negligible additive term. For `c > 0`, balancing against both `h` and `b` under `h+b+kappa <= q+1` gives `2c/(1+4c)`, while balancing against `b` under `b+kappa <= s+1` gives `2c/(1+2c)`. The zero-rate case follows directly as a subexponential bound, without dividing by `c`. At `c=1/6`, the formulas recover `1/5` and `1/4`. The text explicitly makes these conditional width consequences and separately states the necessary layout-construction condition for a uniform algorithm.
- The joint objective fixes its synthesis constant and explicitly restricts `t` to integers. Here "synthesis constant" is read as a valid absolute constant from the proved sparse-correction bound. Taking an infimum over distributions and sample counts does not assert attainment or efficient optimization. Comparing equivalent verifier representations is stated as additional work, and an objective lower bound is explicitly separated from a lower bound on unrestricted projected circuit complexity.
- The final paragraph correctly separates reduced space usage, comparison with prior algorithms, and a hardness argument. None is presented as following automatically from the counting corollary or sparse-fiber population theorem.
