# Circuit nondeterminism: research record

The main deliverable is [main.tex](../main.tex), with its [compiled PDF](../main.pdf)
and [individual BibTeX entries](../references.bib). This directory records the
supporting proofs, source comparisons, examples, and verification. The research
phase began from a 12-page note dated September 4, 2026.

## Objective and completion requirements

Develop several substantive, proved contributions explaining when existential
projection is easy, integrate them with prior results, and produce a rigorous,
pedagogical manuscript. Independently deriving a statement does not establish
its novelty in the literature.

Completion requires analytical proofs with models and quantifiers; examples
showing what the bounds add; primary-source comparison with honest priority
limits; integration with baseline upper bounds and conditional hardness;
independent proof and source review; a checked bibliography and readable PDF;
and an accurate account of the actual AI-assisted methods.

## Results and reading route

1. [Structural elimination](structural/index.md), with the
   [full constructive proof](structural/proof.md): the consistency budget
   h+b+kappa <= q+1, linear-cost reduction to a cubic tensor graph, and the
   fixed-epsilon universal projection rate 1/5. Ordinary preprocessing stays
   outside the exponent. Conditioning can further reduce component costs.
   The [critical-structure theorem](structural/critical-balance.md) derives
   simultaneous necessary input, cycle-loss, and width conditions.
   The [exact realization theorem](structural/realization.md) attains nearly
   critical supplied budgets with easy projections, and calculates the
   supplied verifier's precise excess over its minimum size.
2. [Witness coverage and distribution](coverage/index.md): a single sampled
   family controls shared restriction occupancies and sparse correction.
   The resulting bound includes gatewise density minima and a necessary
   population of positive inputs with few witnesses.
3. [Analytical worked examples](examples/index.md) and
   [stress tests](examples/stress-tests.md): exponential separations between
   compiler guarantees, identical row masses with exponentially different
   occupancy costs, and conflicts between coverage and sharing objectives.
4. [Literature and scope](literature/index.md), the
   [statement matrix](literature/statement-matrix.md), and
   [baseline sources](literature/baselines.md), and
   [final counting comparison](literature/final-comparison.md): exact comparison with
   elimination, knowledge compilation, synthesis, circuit lower bounds, and
   uniform versus nonuniform hypotheses.
5. [Weaker aggregation](aggregation/index.md): a classical polynomial
   fingerprint transfer, cycle-preserving witness tags, and the obstruction
   to a short universal linear fingerprint. This bounded investigation
   supplies a research target, not a better general exponent.
6. [Synthesis and pedagogy](synthesis/index.md), including
   [methods and provenance](synthesis/methods.md).

The all-quantified [counting corollary](../sections/counting-algorithm.tex)
proves an exponential-space uniform algorithm with fixed-epsilon rate
approaching 1/4 in gate count. Its [final source comparison](literature/final-comparison.md)
narrows specific access gaps while keeping priority unresolved. This consequence does not use the nonuniform
ordinary-input synthesis step.

## Status

The final focused phase proved the finite stability inequality and the
critical-structure theorem, including the conclusions for every legal residual
kernel. It also proved exact realization for all simple 2-connected cubic
graphs, with zero cycle loss and explicit near-critical budgets. The latter
construction uses nonminimal AND-only verifiers; its exact minimum sizes
prevent interpreting graph occurrence as circuit hardness.

The source follow-up recovered Nurk's full Russian preprint and further
Broering--Lokam chapter text, and checked an additional tensor-factoring
antecedent. The precise access limits are recorded in the final comparison.
This resolves specific retrieval questions while leaving historical priority
unestablished. The aggregation investigation is now an appendix.

The structural, coverage, critical-structure, realization, and example proofs
are integrated. Separate analytical reviews checked the new deductions and
the construction without using the finite checks as proof. The complete
corpus audit passes: 17 reachable documents, 39 canonical sources, 80 LaTeX
labels, 33 cited BibTeX entries, and all five retained finite-check records.
The 39-page PDF compiles without warnings. The final integrated review and
fresh-source audit are complete, with all findings resolved. Every rendered
page was inspected; the final paragraph-break correction and all affected
pages were checked again. The authorized manuscript work is complete.
The mathematical and priority limitations below remain explicit.

Review records are retained separately: [structural audit](../review-notes/structural-audit.md),
[coverage audit](../review-notes/coverage-audit.md), and
[source audit](../review-notes/fabrication-audit.md).
The earlier aggregation additions have an
[assembly audit](../review-notes/aggregation-sources.md).
The new proofs have separate [critical-balance](../review-notes/critical-balance-review.md)
and [realization](../review-notes/realization-review.md) reviews.
The final expanded manuscript has an
[integrated analytical review](../review-notes/completion-integrated-review.md),
[fresh-source audit](../review-notes/completion-source-audit.md), and visual
reviews for [pages 1-19](../review-notes/completion-pdf-first-half.md) and
[pages 20-39](../review-notes/completion-pdf-second-half.md).
The older PDF reviews for [pages 1-17](../review-notes/final-pdf-root-review.md)
and [pages 18-34](../review-notes/final-pdf-review.md) concern the preceding
34-page draft, not the final expanded manuscript.

Final PDF: 39 pages, 387559 bytes. Its SHA-256 is
`c2899a9481d64d8ecc98b429532bb62a48b3fbbef66ca69ccb412b4e5c3ac84b`.
The supporting research corpus has local checkpoints; the project-root
manuscript is a local artifact, not a publication or submission.

## Supplementary code

- [Forest compiler and finite relation counts](../scripts/check_claims.py).
- [Shared restrictions, sparse synthesis, and occupancy checks](../scripts/check_coverage.py).
- [Symbolic tensor compiler](structural/data/check_tensor_compiler.py).
- [Exact kernel realization and graph reductions](structural/data/check_realization.py).
- [Worked-family arithmetic and evaluations](examples/data/check_examples.py).
- [Corpus audit](data/audit.py), run from the project root with `make check`.

The audit checks links and citation keys, reruns the finite checks, and compares
output with retained records. These are implementation checks or proofs of
stated finite enumeration claims; none proves an asymptotic theorem.

## Bibliography

The [canonical source record](sources.md) supplies one anchor per primary work.
Every manuscript citation also has its own entry in `references.bib`.

## Known limitations

- No unrestricted superpolynomial circuit lower bound or matching description
  of the hard range is established.
- Exact priority of the combined statements is unestablished beyond the
  documented primary-source comparison. An older equivalent theorem may exist.
- The counting exponent is arithmetically smaller than the specific decision
  benchmarks inspected. Its historical priority and the best unrestricted-space
  comparison remain unresolved; a space difference alone establishes neither.
- Critical supplied budgets do not describe minimum verifiers. The realization
  theorem does not attain the critical-structure theorem's hardness hypothesis
  or establish a converse to its simultaneous necessary conditions.
- The exact endpoint coefficients 1/5 and 1/4 with fixed polynomial prefactors,
  and their optimality, are not proved. The statements fix a positive epsilon.
- Model reviewers can share errors. Their agreement is not a mathematical proof.
- The tensor checker does not implement the asymptotic Fomin-Hoie layout
  construction or benchmark the uniform counting algorithm.
