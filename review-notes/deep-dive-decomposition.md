# Decomposition review before expansion

Reviewed: `deep-dive-plan.md`, `README.md`, `main.tex`,
`sections/counting-algorithm.tex`, and the algorithm overview. This is a bounded
pedagogical and theorem-contract review, not a new audit of every existing proof.

Verdict: the three substantive streams are sensible, but the plan needs explicit
interfaces and a stronger middle bridge. Integration and review are workflow
stages, not two additional research subtopics. Do not manufacture five content
streams merely to match the skill's default fan-out.

## Findings

1. **Gap: workstream sizes and ownership.** Foundations currently owns six
   substantial distinctions, while the walkthrough owns both a worked example
   and the entire graph algorithm. Aim for roughly 200-300 source lines of
   foundations, 250-350 lines of walkthrough (split example and general mechanism
   if longer), and 150-250 lines of proved consequences. Assign the definitions
   of projection, witness, accepting computation, and the shared example to
   foundations; assign graph/width notation and the worked contraction to the
   walkthrough; assign restrictions, extraction, and sampling to consequences.
   The integrator owns one consistent symbol table and cross-references.

2. **Gap: the actual conceptual bridge to SAT is not explicit enough.** Add a
   dependency chain: one accepting path means one globally consistent witness;
   the verifier is deterministic after that witness is fixed; reused values must
   agree; gate relations plus equality factors encode that agreement; summing
   tables counts assignments; a frontier stores precisely the unresolved boundary.
   Explain why a directed acyclic circuit can have an undirected consistency
   cycle. Define cycle rank and frontier width operationally before giving their
   inequalities. The elementary failure `(exists y y) AND (exists y NOT y)` versus
   `exists y (y AND NOT y)` should appear before equality tables.

3. **Gap: counting has two different meanings in this manuscript.** Explicitly
   distinguish `sum_(x,y) f(x,y)` from `sum_x [exists y f(x,y)]`. The counting
   corollary computes the former when all circuit inputs are quantified. Counting
   witness pairs does not count distinct projected inputs, and uniformly sampling
   accepting pairs generally biases the distribution of ordinary inputs toward
   larger fibers. Also distinguish counting fixed-length verifier witnesses from
   counting paths of an arbitrary nondeterministic machine: padding and encoding
   conventions can change multiplicities. For decision, existential acceptance
   alone matters; for counting, a bijection must be established.

4. **Error risk: search and sampling cannot inherit every bound by assertion.**
   Sequentially restrict one input and count the two branches. Nonzero counts
   support deterministic witness extraction; branch probabilities proportional
   to the counts support uniform sampling. Prove these by induction and handle
   an initial zero count explicitly. Repeated calls preserve the gate-only
   exponent because restriction need not add gates. They do not automatically
   preserve the original instance-sensitive expression: decreasing `b` can
   increase `s+1-b`. To retain the original graph bound, prove that pinning input
   tables preserves a fixed network topology, layout, and entry bit bound. The
   original enumeration alternative can also support all prefix queries. Account
   for polynomially many queries, repeated preprocessing, and the free inputs
   removed during normalization.

5. **Error risk: exact randomness needs a declared time model.** A random choice
   with probability `C0/(C0+C1)` is not a unit-cost fair-coin operation. Use exact
   integer arithmetic and rejection from a power-of-two interval. This gives
   expected polynomial random-bit overhead, hence an expected-time sampler, with
   probability-one termination for a nonempty solution set. A sampler that must
   return a solution on every run cannot have bounded worst-case fair-bit time
   for a solution set of size three: finite bounded coin trees give dyadic output
   probabilities. Alternatively permit a bounded-time failure outcome and state
   uniformity conditional on success. Jerrum--Valiant--Vazirani explicitly discuss
   this modeling issue on pp. 172-173; their generation convention permits failure
   [jvv86][jvv86]. Do not cite their theorem as if it automatically proved this
   manuscript's particular exponent or an always-output worst-case guarantee.

6. **Gap: the route needs a deliberate scope boundary.** Teach verification
   versus discovery; uniform algorithms versus nonuniform circuit existence;
   decision versus exact counting; and existential versus random choices before
   the broad projection results. State that circuit input length includes its
   description and named inputs. Keep the abstract/headline brief, then let the
   reader reach the algorithm through foundations and the concrete example.
   Low-excess tractability should use a named parameter, for example
   `k=s+1-b`: fixed positive slack gives `poly(s+u+1) 2^(O(k))`, and
   `k=O(log(s+u+1))` gives polynomial time. Separate this easy corollary from
   novelty claims. Retain basis dependence, fixed-slack quantifiers,
   exponential space, and the distinction between supplied and minimum verifier
   size when reconnecting the SAT and projection results.

## Recommended running example

Use three gates and three inputs:

```text
a = x OR y
b = y XOR z
f = a AND b
```

Treat `x` as the ordinary input and `(y,z)` as witnesses initially. Then quantify
all three inputs for SAT/counting. The satisfying triples are `010`, `101`, and
`110`; direct enumeration during this review verified these values. There is one
witness when `x=0` and two when `x=1`, although the projection is the constant-one
function. Thus the example separates witnesses, positive projected inputs, and
uniform distributions with the smallest useful non-dyadic counting example.

Its reused `y` creates a consistency cycle: the gate branches diverge at `y` and
recombine at the output AND. With all inputs quantified, `s=b_inputs=3` and the
gate-minus-input budget is one. Avoid calling the intermediate XOR gate `b` in
the manuscript if `b` already denotes the retained input count; use `v_1,v_2`.
Show the two gate tables, the equality constraint on the two occurrences of `y`,
the output-one condition, and one actual contraction table. Recover total three
and the branch counts one and two. A uniform solution sampler chooses `x=1` with
probability `2/3`, whereas a uniform sampler of positive projected inputs would
choose it with probability `1/2`.

This example reduces entirely through low-degree operations. Say so explicitly;
it teaches consistency and counting but cannot illustrate a nonempty cubic
kernel or the asymptotic width theorem. Follow it with a separate small abstract
frontier diagram for the width mechanism, rather than making the introductory
circuit complicated enough to carry every later theorem.

## Local References

- [jvv86] Mark R. Jerrum, Leslie G. Valiant, and Vijay V. Vazirani.
  *Random generation of combinatorial structures from a uniform distribution*.
  Theoretical Computer Science 43 (1986), 169-188.
  DOI: https://doi.org/10.1016/0304-3975(86)90174-X.
  Primary article inspected on 2026-09-05, especially pp. 170 and 172-173:
  https://www2.stat.duke.edu/~scs/Courses/Stat376/Papers/ConvergeRates/RandomizedAlgs/JerrumValiantVaziraniTCS1986.pdf

[jvv86]: https://doi.org/10.1016/0304-3975(86)90174-X
