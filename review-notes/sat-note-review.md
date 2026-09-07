# Focused SAT note: validation record

Date: 2026-09-07. Artifact: [five-page SAT note](../output/pdf/sat-note.pdf),
with [LaTeX source](../sat-note.tex) and [BibTeX entries](../sat-note.bib).

The note is prepared for expert discussion as an explicit derivation from
established graph-width and counting methods. Historical priority and
best-known unrestricted-space status are not established by this review.

## Mathematical review

- The model counts arbitrary gates of fan-in at most two, permits unrestricted
  fan-out, and has one output. Normalization repeats pruning; removed inputs
  contribute the exact factor `2^(n-b)`.
- Connected incidence accounting gives `kappa = ell-q-b+1 <= s+1-b`.
  High-degree equality factors remain implicit until their ternary expansion.
- Gate values and expanded equality indices have unique extensions. Loop
  contraction uses the diagonal; shared parallel indices are summed jointly.
  All reductions preserve the count and keep table arity at most three.
- The simple cubic remainder has `N = 2*(kappa(K)-1) <= 2*(kappa-1)`.
  The median-timestamp proof handles ties and constructs cutwidth at most
  pathwidth plus two.
- The frontier invariant gives `O(N*2^w)` arithmetic operations and
  `O(2^w)` stored entries. Intermediate integers have `O(q+1)` bits.
  Exponential space is an implementation bound, not a necessity theorem.
- Balancing enumeration with contraction proves Theorem 1 from Proposition 2.
  The fixed positive slack, four-gate corollary, and `0.7475 + rho` example
  have the stated quantifiers. No proof gap was identified in this review.

## Primary-source checks

- [Fomin--Hoie](https://fedorvf.github.io/articles/2006/2006b.pdf), Theorem 5
  and Section 4, printed p.194: the asymptotic subcubic pathwidth bound and
  polynomial-time constructivity for each fixed positive slack.
- [Bodlaender et al.](https://www.combinatorics.org/ojs/index.php/eljc/article/download/v32i3p36/pdf/),
  proof of Lemma 16, printed p.17: antecedents for the subcubic inequality
  `cw <= pw+2`. The note supplies its own constructive cubic proof.
- [GKST, corrected ECCC revision 2](https://eccc.weizmann.ac.il/report/2016/022/revision/2/download),
  Sections 2.3 and 2.5, Theorem 4, and Section 5.2, Remark 2: matching circuit
  model and counting below `(3-zeta)n`. The polynomial-space statement in the
  note follows from a depth-first evaluation of their recursion and its
  decreasing measure; it is not a quoted theorem statement.
- Markov--Shi and Dudek--Duenas-Osorio--Vardi are cited as established tensor
  and counting antecedents. These citations do not establish priority of the
  gate-minus-input calculation.

## Local validation

- `make sat-note`: passed; no LaTeX or BibTeX warnings, undefined references,
  or overfull/underfull boxes. All five final pages were rendered and visually
  inspected. Every PDF font is embedded.
- Separate source audit: ten unique labels, five used bibliography entries,
  no unresolved citations, ASCII source, and no draft markers.
- `make check`: all eleven finite suites passed, with recorded outputs
  unchanged. These check finite constructions, not the full asymptotic
  algorithm or the imported graph theorem.
- `git diff --check`: passed. The build target and CI configuration include
  the short note. Hosted CI was not run for these local edits.

Final SHA-256 values:

```text
sat-note.tex
a07ebc16d81976b080bfe58be9b8bae35d5eb920adf8745fd36cf6a7041ca4c5
sat-note.bib
6843a7c7da981edf0c3d8ea76e96ffa0312d3a56c65ae920bf04b6b30c8b4519
output/pdf/sat-note.pdf
7355eea07d541f7bdec10bf6109b55dfec2afa6a9bf535fbd53c8f8f51355f65
```
