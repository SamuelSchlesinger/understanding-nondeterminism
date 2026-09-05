# First accuracy review: algorithm consequences

Reviewed 2026-09-05: `sections/algorithm-consequences.tex` and
`research/consequences/data/check_queries.py`, with `sections/structure.tex`
and `sections/counting-algorithm.tex` as dependencies. No manuscript or
validator files were changed.

Result: **zero Error findings and zero substantive Gap findings** in the new
theorem statements and their proofs. Two small clarifications below would make
the sampling contract completely explicit. This is a review of the new
consequences conditional on the existing graph/layout theorem; it is not a
fresh independent audit of the external pathwidth theorem or historical status.

## Findings

### Sampling's zero-count branch should appear in its own proof

- **Location:** `sections/algorithm-consequences.tex:196`, proof of
  Corollary `cor:exact-sampling`.
- **Severity:** Polish.
- **Finding:** The statement promises deterministic detection of unsatisfiability,
  but the proof starts with `Z=1` and `Z>1`. The preceding search proof already
  contains the missing branch, so this is not a correctness defect.
- **Fix:** Start with: compute `Z`; if `Z=0`, report unsatisfiability; if `Z=1`,
  unrank zero; otherwise perform rejection sampling.

### Say probability-one termination when describing the rejection sampler

- **Location:** `sections/algorithm-consequences.tex:208`, paragraph following
  Corollary `cor:exact-sampling`.
- **Severity:** Polish.
- **Finding:** The phrase "always-output fair-bit implementation" can be read as
  termination for every infinite random-bit stream. The intended guarantee is
  that a nonempty instance produces a solution with probability one and in
  finite expected time. For a non-power-of-two count, a measure-zero stream can
  reject forever. The stated expected-time theorem itself is correct.
- **Fix:** Use "probability-one-terminating fair-bit implementation" or explain
  in a short sentence that it returns a solution almost surely, without a
  bounded worst-case runtime.

## General proof checks

- **Pinning and equality expansion:** A used input's original equality tensor
  becomes the same tensor with its common value fixed. No topology changes.
  For degree at least four, fixing any one ternary equality factor to the
  prescribed common bit forces every connected tree factor and every boundary
  edge to agree. There is exactly one assignment to internal equality-tree
  edges when permitted, and zero otherwise. Degrees one through three need no
  splitting and follow directly from the pinned table definition.
- **Original parameters:** The reduction schedule depends on topology, not
  numerical entries; zero tables need not trigger new simplifications. Thus the
  original residual graph and order remain usable. Enumeration also uses the
  original `b`. Taking the cheaper proved route gives precisely
  `poly_delta(s+u+1) 2^min{b,(1/3+delta)kappa}`. The original rank and `b` are
  not recomputed after restrictions.
- **Bit and representation costs:** Every table entry counts assignments to a
  subset of the fixed expanded graph's `O(s+1)` binary indices. Products combine
  disjoint contracted regions and sums eliminate distinct shared assignments,
  so entries stay within `2^O(s+1)`. Final unused-input factors need at most
  `u+1` bits. Circuit encodings using gate/predecessor labels have polynomial
  size in the stated parameters. Recomputing numerical tables sequentially
  retains the space bound; this is not a claim of constant-time query answers.
- **Rank/unrank:** Prefix branches partition all completions. The invariant
  `0 <= R < Z(p)` is preserved even when a branch has count zero. At a full
  assignment, the invariant forces `Z(p)=1` and residual rank zero. Summing the
  zero-branch counts at each chosen one bit counts exactly the lexicographic
  predecessors. At most `u+1` count queries and `O(u+1)`-bit integer arithmetic
  suffice. The domain restriction on a supplied rank rules out overly long
  valid rank representations, apart from ordinary leading-zero encoding issues.
- **Empty and unused cases:** With zero input bits, the unique possible full
  assignment is the empty tuple; satisfiable constant output has count one and
  unranking executes zero iterations. Unsatisfiable output has no valid rank.
  For originally unused inputs, each unpinned bit contributes two and each
  pinned bit one, so interleaving free bits in the chosen lexicographic order
  does not break any prefix identity. Trivial outputs use the stated direct
  handling instead of a nonexistent nontrivial consistency graph.
- **Sampling:** `L=ceil(log2 Z)` yields acceptance probability `Z/2^L > 1/2`
  for `Z>1`, including acceptance probability one at powers of two. Conditioning
  on acceptance makes every rank exactly uniform; the unranking bijection
  transfers that distribution to full satisfying assignments. Expected random
  work is polynomial, and unranking is invoked only after a rank is accepted.
  Projected positive-input uniformity is correctly distinguished.
- **Low excess and coefficients:** Fixing `delta=1/6` makes
  `2^((1/3+delta)d)=(2^d)^(1/2)`; if `d<=c log2(s+u+1)` for fixed `c`,
  this is a fixed polynomial. The same applies to logarithmic `kappa`.
  Substituting `b=u` gives limiting coefficient `(c-1)/3`; the displayed
  `1/3,2/3,5/6` entries and gate-only `1/2,3/4,7/8` entries are correct.
  The general envelope suppresses the additive `1/(3u)` and fixed positive
  slack as stated. Balancing gives limiting `b=(s+1)/4`; no attainment or
  hardness claim is made.

## Executed validation

The requested command `python3 research/consequences/data/check_queries.py`
completed successfully:

```text
Explicit relations on 0..3 bits: 278; prefix partitions: 1844: OK
Satisfying assignments ranked and unranked: 1061: OK
Invalid ranks and nonsolutions rejected: 1617: OK
Exact rejection probabilities for solution counts 1..16: OK
Running example: ranks 0,1,2 map to 010,101,110; first-bit counts 1,2: OK
Scope: finite explicit counting oracles; no asymptotic layout or solver benchmark.
```

I also ran an independent in-memory exhaustive check, without editing files:

- 2,076 pinned equality cases: degrees one through seven, every external
  assignment, both prescribed bits, and every possible pinned ternary factor.
  Direct summation over the internal chain edges equaled the indicator that all
  external values equal the prescribed bit in every case.
- 2,054 unused-input cases: every relation on zero through two used bits, zero
  through two free bits, and every partial assignment. Direct full-assignment
  counting equaled the used-relation count times two to the number of unpinned
  free bits in every case.
- Exact rational comparison of all three displayed coefficient-table rows.

The retained validator accurately declares its limited oracle scope. It tests
rank/unrank against independently enumerated relations and computes exact
rejection probabilities rather than accepting a statistical approximation as
evidence of uniformity. None of these finite checks validates the asymptotic
layout construction or replaces the general conditional-count invariant.
