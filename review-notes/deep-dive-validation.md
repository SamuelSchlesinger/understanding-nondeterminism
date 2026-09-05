# Deep-dive edition: validation and review resolution

Date: 2026-09-05. Expansion baseline: published commit `d42748a`.

The expanded manuscript is **Understanding Nondeterminism: From Witnesses and
Structure to Faster SAT**. Its 54 pages develop computation paths, certificates,
global consistency, a complete numerical example, symbolic table entries, the
restricted SAT algorithm, and further counting-query consequences. The existing
projection, coverage, critical-structure, realization, and aggregation proofs
remain integrated. The README highlights the SAT and exact-counting guarantee.

## Review resolutions

- Decomposition review: the three-gate example is shared across the introductory
  sections; its empty cubic remainder is explicit. A separate frontier fixture
  explains the general update. Pinning preserves the original graph, and exact
  fair-bit generation has expected time.
- Coherence review: moved the worked example before the technical overview and
  placed the algorithm's consequences immediately after the counting proof.
  Defined the required circuit, graph, P, CNF, and #P vocabulary. Added a
  nonconstant symbolic example and refreshed the reading routes.
- Mathematical and consequences reviews: no substantive proof errors found.
  Clarified zero-count handling and probability-one termination in sampling.
  Preserved the distinction between full-assignment and projected-input counts.
- Source review: pinned the corrected GKST preprint revision; synchronized the
  Morizumi 2015 published bibliography; preserved primary-source retrieval limits
  and unresolved historical priority. Updated both current sampler summaries.
- Final adversarial factual audit: no actionable findings remain. The methods
  appendix now records both added suites and their exact finite scope. Historical
  research snapshots retain their dated counts.

The detailed [review records](README.md) preserve the findings and checks.
Agreement between agents is not mathematical certification; the proofs and
precisely cited graph theorem justify the general statements.

## Local verification

- `make check`: all seven suites pass and match retained outputs. Corpus audit:
  20 reachable research documents, 42 canonical sources, 102 LaTeX labels,
  35 directly cited BibTeX keys.
- `make pdf`: successful; no LaTeX/package warnings, undefined references,
  overfull boxes, or underfull boxes in the final log.
- Citation metadata: `cffconvert --validate -i CITATION.cff` passes schema 1.2.0.
- `git diff --check`: passes.
- PDF inspection: all 54 pages rendered and inspected; final methods and
  bibliography pages rendered again after wording and citation corrections.
  Tables, graph figures, equations, headers, and references are legible.
- Final PDF: 455,506 bytes; SHA-256
  `23c4bfbe58cdca60042aec7bccb75bd08c7e39a396d3281d6c8bc0be6f32e8c8`.

These checks cover local construction identities and explicit finite query
oracles. The full asymptotic layout/counting solver is not implemented or
benchmarked. The fixed-slack exponent, exponential-space allowance, B2 basis,
and restricted gate budget remain explicit.

## Publication contract

Code and build automation use MIT; manuscript and research/review notes use
CC BY 4.0. The destination is the existing public
[understanding-nondeterminism repository](https://github.com/SamuelSchlesinger/understanding-nondeterminism).
This record describes local validation of the publication candidate. Hosted
validation is checked on the exact pushed commit through the repository's
Validate manuscript workflow; it is separate from the local evidence above.
