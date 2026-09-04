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
   [baseline sources](literature/baselines.md): exact comparison with
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
approaching 1/4 in gate count. Its [focused source comparison](../review-notes/sat-comparison.md)
keeps priority unresolved. This consequence does not use the nonuniform
ordinary-input synthesis step.

## Status

The main structural, coverage, and example proofs have been integrated.
Independent structural and coverage reviews found no proof-breaking error.
The final integrated review also checked the uniform counting argument.
The source audit found two minor bibliographic-language corrections; both
were applied. The aggregation arguments and their four classical sources
also received separate mathematical and source reviews. Final corpus
checks pass: 14 reachable documents, 37 canonical sources, 75 LaTeX
labels, 30 cited BibTeX entries, and all four retained finite-check records.
The 34-page PDF compiles without warnings; every rendered page was inspected.
Two bibliography capitalization issues found during inspection were corrected,
and the affected pages were rendered and checked again. This verifies the
assembled draft; the mathematical and priority limitations below remain.

Review records are retained separately: [structural audit](../review-notes/structural-audit.md),
[coverage audit](../review-notes/coverage-audit.md), and
[source audit](../review-notes/fabrication-audit.md).
The final additions have an [assembly audit](../review-notes/aggregation-sources.md),
and PDF coverage is recorded for [pages 1-17](../review-notes/final-pdf-root-review.md)
and [pages 18-34](../review-notes/final-pdf-review.md).

## Supplementary code

- [Forest compiler and finite relation counts](../scripts/check_claims.py).
- [Shared restrictions, sparse synthesis, and occupancy checks](../scripts/check_coverage.py).
- [Symbolic tensor compiler](structural/data/check_tensor_compiler.py).
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
- The numerical uniform SAT/#SAT comparison remains unresolved; a space
  difference alone does not establish novelty or explain older benchmarks.
- The exact endpoint coefficients 1/5 and 1/4 with fixed polynomial prefactors,
  and their optimality, are not proved. The statements fix a positive epsilon.
- Model reviewers can share errors. Their agreement is not a mathematical proof.
- The tensor checker does not implement the asymptotic Fomin-Hoie layout
  construction or benchmark the uniform counting algorithm.
