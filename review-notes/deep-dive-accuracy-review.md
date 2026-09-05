# Full manuscript mathematical accuracy review

Reviewer role: mathematical accuracy and validation-artifact audit.
Date: 2026-09-05.

## Scope and current verdict

Read `main.tex` and every file in `sections/`, including the existing proof
dependencies, the foundations, counting walkthrough, and algorithmic
consequences. Read every Python validation script in `scripts/` and
`research/**/data/`, the recorded outputs, the audit entry point, and the new
foundations/consequences corpus explanations. This is an independent review
of the consequences and foundations; the reviewer authored the walkthrough,
so its rechecking here does not count as independent authorship review.

No mathematical Error-severity finding identified. The pinning argument
preserves boundary-conditioned counts and the original topology; the
rank/unrank and expected-time sampling corollaries follow. Two presentation
findings remain below. The full execution gate is recorded separately below;
the analytical verdict does not claim that finite checks prove the theorems.

## Findings

### A1. Update the experimental record for the two new validation artifacts

- Location: `sections/methods.tex`, finite-check table and paragraphs following
  it; `research/synthesis/methods.md`, corresponding experiment record.
- Severity: Gap.
- Finding: The current method record lists the older compiler, coverage,
  example, and realization domains but omits the new worked integer-table
  checks and the explicit-oracle rank/unrank checks. The sentence that the
  full integer-counting algorithm is not implemented remains correct. The
  newly added local integer checks should nevertheless be recorded, so readers
  can distinguish what is tested from the general counting proof.
- Suggested fix: Add the walkthrough's 27 partial-pinning checks and local
  integer fixtures, and the consequences script's 278 explicit relations,
  1,061 rank/unrank assignments, and exact rejection probabilities. State that
  these are finite local/oracle checks, with no full counting solver or
  asymptotic layout implementation. Match the executed records below.

### A2. State probability-one termination explicitly

- Location: `sections/algorithm-consequences.tex`, exact sampling proof and
  the following paragraph; `research/consequences/index.md`, generation.
- Severity: Polish.
- Finding: The expected-time result is correct, but the label
  "always-output" can be read as termination on every infinite random tape.
  Repeated rejection can continue forever on a probability-zero set of tapes.
  The finite expectation already establishes probability-one termination;
  spelling this out would prevent an unnecessary ambiguity in a tutorial.
- Suggested fix: Say that the sampler terminates with probability one and
  produces no failure outcome when it terminates. For example, the probability
  of still rejecting after j trials is less than `2^(-j)`.

## Dependency checks

- **Gate semantics and counting.** The foundations distinguish accepting paths,
  fixed-length witness encodings, witness pairs, and distinct projected inputs.
  The canonical suffix-zero encoding preserves witnesses bijectively,
  including the zero-length case. Gate equations determine a unique extension
  by topological induction. All-input integer contraction consequently counts
  full assignments, and pinning does not silently change that universe.
- **Structural budget.** Recomputed `V=2q+b+1`, `E=q+ell+1`,
  `kappa=ell-q-b+1`, and `h+b+kappa<=q+1`. Ordinary dependent-gate input
  pins are charged with multiplicity while h counts distinct interface signals.
  Deleting the independent cone cannot increase cycle rank.
- **Reductions.** Degree splitting preserves unique equality extensions;
  loop contraction takes a diagonal sum; joint parallel contraction sums
  common indices once. The rank-three bound, termination measure, and 80N0
  gate budget are compatible. Graph-only reduction choices remain available
  after a pin, regardless of zeros in the modified tables.
- **Width and operation cost.** Checked the median timestamp charging argument,
  including its at-most-two tied vertices. For a cubic frontier step,
  `|F_new|=|F_old|+3-2j`, so `|F_new|+j<=w+1`; its less-than-`2^(w+2)`
  charged gate bound follows. Tables count assignments to only `O(s+1)`
  indices, making the bit arithmetic polynomial per operation. Exponential
  table storage is explicitly allowed.
- **Uniformity.** The constructive graph result supports the SAT algorithm;
  formal-input synthesis is used only for nonuniform projection size.
  The balances `alpha/(1+alpha)` and `alpha/(1+2alpha)` correctly approach
  1/4 and 1/5. Fixed slack and its prefactor are retained. The explicit
  `epsilon=gamma/[8(4-gamma)]` substitution gives the asserted SAT saving.
- **Consequences.** Low excess gives polynomial time with fixed logarithmic
  constants. Pinning used inputs changes equality values only; unpinned free
  declared inputs multiply the answer by two. Original enumeration gives the
  other branch of the minimum. Prefix partitions give unranking and ranking
  in at most u+1 count queries, and sequential execution preserves space.
  Uniform rank rejection has acceptance probability strictly greater than
  one half, fewer than two trials in expectation, and O(u) bits per trial.
- **Critical structure and realization.** Fixed-delta limits precede the
  delta-to-zero limit. The proof retains the supplied/minimum distinction.
  The fork/join accounting, source-tree conflict bounds, sink repair, and
  exact equality splitting establish the stated realization; bridged graph
  fixtures are explicitly additional finite examples, not a claim that all
  connected cubic graphs are 2-vertex-connected.
- **Coverage and examples.** The same sample is used throughout the occupancy
  proof. The sparse majorant and concavity/Jensen calculation have the needed
  domains. The lower-tail inversion is a necessary condition. The cyclic
  example's support formulas and distribution comparisons concern named
  guarantee expressions, with separate lower bounds only where proved by
  essential-input counting.
- **Aggregation and hardness.** Distinct witness tags prevent characteristic-two
  cancellation between different monomials; the field zero bound and
  simultaneous hardwiring proof are correctly nonuniform. The universal
  fingerprint obstruction concerns fixed linear aggregates only. The
  nonuniform SETH implication is stated infinitely often for a k depending on
  epsilon, and no unrestricted lower bound, practical benchmark, endpoint
  coefficient, or historical priority follows from the paper's finite checks.

## Fresh primary-source verification

1. Fomin and Hoie, *Pathwidth of cubic graphs and exact algorithms*,
   DOI [10.1016/j.ipl.2005.10.012](https://doi.org/10.1016/j.ipl.2005.10.012).
   Fetched the [author PDF](https://fedorvf.github.io/articles/2006/2006b.pdf).
   Theorem 5 on printed p. 194 covers maximum degree at most three with fixed
   positive slack. Section 4 on that page explicitly supplies polynomial-time
   construction of the decomposition. This verifies the load-bearing
   uniformity step, not just an existential width estimate.
2. Jerrum, Valiant, and Vazirani, *Random generation of combinatorial structures
   from a uniform distribution*, DOI
   [10.1016/0304-3975(86)90174-X](https://doi.org/10.1016/0304-3975(86)90174-X).
   Fetched the [primary article scan](https://www2.stat.duke.edu/~scs/Courses/Stat376/Papers/ConvergeRates/RandomizedAlgs/JerrumValiantVaziraniTCS1986.pdf).
   Printed pp. 172-173 discuss fair coins, the three-output obstruction, and
   the generation model's possibility of no output. Theorem 3.3 on p. 174
   gives uniform generation with a counting oracle. The manuscript correctly
   proves its own expected-time exact sampler instead of importing a stronger
   always-terminating guarantee from that theorem.

## Data-file audit

No new external statistical dataset or per-row literature attribution is
hidden in the data artifacts. The expected text records deterministic script
outputs. Inspected their actual loops and independent comparison paths:

| Artifact | What its implementation checks | Boundary retained |
|---|---|---|
| `scripts/check_claims.py` | Raw circuit evaluation independently checks normalization and generated forest circuits; all 16 binary operations and repeated pins; exact relation distribution | Finite circuits, no minimum-size calculation |
| `scripts/check_coverage.py` | Actual sparse/shared circuits; exact `Fraction` occupancy and miss expectations | Jensen finite checks use floats with tolerance; analytical concavity is separate |
| `structural/data/check_tensor_compiler.py` | Generated B2 gates compared by full Boolean masks; explicit loop/parallel operations, gate costs, supplied-bag median conversion | Boolean projection only; no Fomin-Hoie construction |
| `structural/data/check_realization.py` | Actual AND gate lists, incidence certification, legal contractions, labeled kernel equality; seven semantic cases | Most cases verify graph construction, not exhaustive semantic truth tables |
| `examples/data/check_examples.py` | Independent gate evaluation, support formulas, exact rational distribution objectives, sparse circuits | Finite scans do not replace the all-t/asymptotic arguments |
| `walkthrough/data/check_walkthrough.py` | Independent incidence-bit enumeration for 27 partial pins; gate-extension, equality, parallel, and weighted fixtures | Local integer mechanics only |
| `consequences/data/check_queries.py` | Every relation on 0..3 bits; direct oracle lookup against rank/unrank; invalid cases; rational rejection probabilities | Explicit oracle, no general layout-preserving counting implementation |
| `research/data/audit.py` | Re-executes scripts, diffs expected outputs, checks local links/reachability and canonical citations | Does not prove analytical or priority claims |

## Executed validation

After the parent confirmed its build had finished, independently executed
`make check` from the project root. It exited zero. All seven suites reran
and exactly matched their recorded outputs. The structural gate reported
20 reachable Markdown documents, 42 canonical sources, 101 LaTeX labels,
and 35 cited BibTeX entries.

The executed numerical records agree with the manuscript's older counts:
9,280 exhaustive circuit encodings; 2,000 forest and 600 tensor seeded
circuits; 65,536 relations for the projection distribution; 362 sparse
supports; 896 sampled circuit pairs; 140 occupancy cases; 140 tensor
fixtures; 3,729 realization constructions and 72,401 legal contractions;
seven exhaustive realization semantic representatives; 25,600 worked-family
input/witness pairs and 141 sparse-correction instances. The new suites
report the local walkthrough checks listed above and 278 relations, 1,844
prefix partitions, 1,061 ranked/unranked solutions, and 1,617 rejected invalid
queries. Rejection probabilities for solution counts 1 through 16 are exact
rational checks. No statistical confidence claim is inferred.

No manuscript, script, expected-output, or shared bibliographic file was
edited by this review; only this review note was written.
