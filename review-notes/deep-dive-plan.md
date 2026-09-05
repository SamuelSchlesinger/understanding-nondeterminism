# Deep-dive expansion plan

Objective: make the paper a coherent, substantial exploration of nondeterminism,
with the restricted-instance SAT algorithm and the other proved bounds arising
from the reader's understanding. Preserve the existing mathematical scope and
publish only after substantive review, reproducibility checks, and PDF inspection.

## Current-state audit

The starting edition is the clean, published 40-page manuscript at `d42748a`.
Its SAT theorem, projection bounds, coverage theory, critical structure, and
kernel realizations are present. The previous publication turn made concrete
progress, but publication alone does not establish the new deep-dive objective.

The main missing elements are an accessible operational account of
nondeterminism, a fully worked bridge from shared witnesses to counting tables,
and a synthesis of what the algorithm implies in different parameter regimes.

## Workstreams and ownership

1. Foundations: computation paths, witnesses and projection; decision versus
   search and counting; consistency versus independent local choices; randomness
   versus existential quantification; uniform algorithms versus nonuniform
   circuits. Use one running example and primary sources.
2. Algorithm walkthrough: a complete small circuit translated to constraints,
   equality tables, and count-preserving contraction. Explain the frontier
   invariant and separate arithmetic operations, bit cost, time, and space.
3. Further consequences: derive useful statements from the established budget,
   including low-excess tractability and witness extraction/exact sampling if
   their guarantees can be proved. Investigate without assuming novelty.
4. Integration: organize a progressive reading route, retain complete existing
   proofs, connect all new explanations to the SAT result, improve diagrams,
   update README, source records, methods, and validation coverage.
5. Independent review: mathematical accuracy, pedagogical completeness and
   coherence, primary-source checking, factual audit, finite checks, compiled
   artifact inspection, then push and verify hosted CI.

## Decomposition decisions

Use the running circuit `f(x,y,z) = (x OR y) AND (y XOR z)` across the
foundations and contraction walkthrough. It has three accepting assignments,
fiber sizes one and two when only x remains ordinary, and one consistency
cycle. Distinguish this small example from the asymptotic cubic-layout theorem.

For further search/sampling consequences, pin original input tables and retain
the original graph/layout when claiming the original cycle-sensitive exponent.
Naive re-normalization changes the used-input parameter and is insufficient
for that claim. Exact fair-bit sampling uses rejection with expected running
time; deterministic rank/unrank carries the worst-case bound.

## Completion evidence required

- The manuscript introduces its operational concepts before relying on them.
- The running example is mathematically correct and connects explicitly to
  the general consistency construction and the algorithm.
- Every added theorem has a complete proof, declared model, and quantifiers.
- References have checked identities and support their associated claims;
  unestablished priority stays unestablished.
- Existing results remain present and correctly scoped; no unrestricted
  lower bound or practical speedup is inferred from finite checks.
- Corpus links/citations and all applicable finite checks pass.
- Final PDF compiles without warnings and all pages receive visual inspection.
- The final reviewed state is committed, pushed, and passes hosted CI.
