# Final integrated Accuracy and Completeness/Coherence review

Reviewed on 2026-09-04. Scope: `main.tex`, all included sections, the supporting
critical-balance and realization documents, structural summary, final comparison,
methods/provenance records, all five finite-check scripts and their retained
output files, and the corpus audit. Primary-source fabrication checking and PDF
visual inspection are separate reviews.

**Outcome: PASS after the scope and provenance corrections recorded below.**
No remaining Error-severity finding or in-scope mathematical Gap was found.
The review checked the arguments and implementations directly; the earlier
analytical PASS for realization was not used as evidence of correctness.

## Findings and their resolution

### G1. The realization opening overstated which converse was refuted

- **Severity:** Gap in the stated significance.
- **Location:** `sections/realization.tex:4`; corresponding closing language in
  `research/structural/critical-balance.md:137-139`.
- **Finding:** the original sentence said that the preceding necessary
  conditions "do not have a converse." The construction proves that the
  critical gate/input budgets, essential inputs, and zero cycle loss can
  coexist with an easy projection. It does not assert that its examples
  simultaneously satisfy the width-saturation and quantitative sparse-fiber
  population conclusions for hypothetical hard families.
- **Exact fix:** narrow the opening to "The critical gate and cycle budgets
  alone do not force a hard projection." Say that the critical deductions
  establish no converse, rather than asserting that the full converse has
  been disproved.
- **Resolution:** verified in the current files. The LaTeX opening now has
  exactly the narrowed claim. The Markdown says "These deductions establish
  no converse." `research/index.md:121-123` also preserves this boundary.

### G2. The earlier validation snapshot was presented as current

- **Severity:** Gap in the completion/provenance record.
- **Location:** `research/index.md`, Status and Supplementary code;
  `research/synthesis/methods.md`, earlier completion paragraph.
- **Finding:** the original record still reported 14 documents, 37 sources,
  75 labels, 30 cited BibTeX entries, four retained checks, and the inspected
  34-page draft. The assembled state has new proofs, another checker, and a
  different PDF. Its Supplementary code list also omitted the new checker.
- **Exact fix:** record the current audit's 17/39/80/33 counts and five checks;
  add the realization checker; identify the 34-page evidence as belonging to
  the preceding draft; report the new visual inspection separately.
- **Resolution:** verified in `research/index.md:62-101` and
  `research/synthesis/methods.md:87-140`. The new proof/source work is recorded,
  the old snapshot is explicitly historical, and the current PDF inspection
  is described as in progress rather than inherited from the old draft.

No additional Polish finding needs a manuscript change.

## Analytical checks of the new material

### Critical balance

`sections/critical-balance.tex` matches the reviewed Markdown argument,
including its corrected nontrivial-output scope and amplification assumptions.

The finite lemma compares lower and upper bounds containing the same
`p+A_delta(q+1)` prefactor. It therefore forces
`h,b,alpha*kappa>=t`, and the sum of the three excesses is bounded by exactly
`Delta=q+1-(2+1/alpha)t`. For the family theorem, subtracting `p=O(m)` from
`C(g)>=2^(m-o(m))` preserves its logarithmic scale. With each `delta` fixed,
the prefactor costs only `O_delta(log m)`. The displayed limsup estimates
then give `p=o(m)`, `h,b~m`, and `kappa~3m` by sending the fixed slack to zero
after the size limit. There is no hidden uniform bound on `A_delta`.

The independent-subgraph edge count proves `|a-h|<=p` under the explicitly
stated witness-dependent-output restriction. The fanin-slack identity is exact.
For every legal residual kernel, the universal finite compiler cost forces
`cw(K)>=m-o(m)`. In particular `N->infinity`. Combining this with
`N<=2(kappa-1)` and the fixed-slack graph-width input gives `N~6m`, both
minimum widths asymptotic to `m`, and cycle loss `o(m)`. The argument is
uniform over permitted equality trees and complete reduction sequences.

The final sparse paragraph uses declared witnesses throughout. Removing unused
witness bits divides both fiber cardinalities and their threshold by
`2^(m-b)`. Exact enumeration implies `m-b<=log L` under the amplification
hypothesis. The LaTeX explicitly requires `log L=o(m)` when using amplification
to imply the family theorem's logarithmic lower bound.

### Realization

`sections/realization.tex` correctly proves the class theorem stated in
`research/structural/realization.md`. Ear insertion retains earlier and later
neighbors for every internal vertex and gives the required acyclic orientation.
Degree balance gives `F=J=k-2`, hence `k` skeleton gates including the two sink
gates.

The shorter last-join argument in `sections/realization.tex:69-76` is valid.
After a topologically last join there can be no later join. A nonempty
descendant fork tree would therefore have a last fork with both outgoing
edges to the sink, contradicting simplicity. Thus this join supplies the sink
directly on only one edge. The sink has at least two distinct wire names, and
its two AND gates can both have distinct input wires.

The copied-wire component counts give
`R<=floor(2k/3)`, or `R<=floor((2k-1)/3)` in the triangle-free case.
Each chosen value of `t` supplies at least `R` of the `2t-1` fresh inputs.
Fresh gate names split old names without merging previously different ones.
The resulting gate list is acyclic, every gate remains witness-dependent,
and every input reaches the output. This establishes the stated exact budgets.
The conjunction function and the essential-input lower bound give
`C(f)=2t-1`, `C(g)=t-1`, and the exact supplied-size excess `k`.

The chosen equality trees have the actual external incidences of the circuit
wires, including the pieces separated by attachments. Contracting the output
assertion and sink tree leaves the three-input conjunction table. The fresh
witness attachment is handled correctly at `sections/realization.tex:154-160`:
`OR_y[v=u AND y]=[v<=u]`, not equality. Retaining this table and contracting
the remaining two-port tables gives the asserted topology without dropping
their semantic effect. Every step is an allowed single-edge contraction;
the marked terminal graph is exactly `K`, with zero cycle loss.

## Other mathematical and significance checks

The existing consistency and frontier proofs retain their numerical constants.
The gate-counting consequence preserves multiplicities, charges polynomial
integer bit costs, and uses constructive layouts for each fixed slack. Its
`1/4+epsilon` statement does not use the nonuniform ordinary-input synthesis
step. The conditional width-transfer formulas are balanced correctly and
retain the additional constructibility requirement for a uniform algorithm.

The coverage, sparse-correction, distribution examples, baseline lower bounds,
conditional SETH formulation, and fingerprint appendix are consistent with
their stated models. The examples distinguish compiler guarantee expressions
from minimum circuit size. The new critical theorem is conditional on a
hypothetical lower bound; realization is an unconditional representation
theorem using explicitly nonminimal circuits. The current abstract and
prior-work section do not turn either into an unrestricted hardness theorem
or claim an optimal exponent or a best-known counting algorithm.

## Implementation and data audit

Ran `make check` once during this review. It completed successfully with:

```text
Documents: 17 reachable; 39 canonical sources; 80 LaTeX labels; 33 cited BibTeX entries: OK
scripts/check_claims.py: finite checks and recorded output OK
scripts/check_coverage.py: finite checks and recorded output OK
research/structural/data/check_tensor_compiler.py: finite checks and recorded output OK
research/structural/data/check_realization.py: finite checks and recorded output OK
research/examples/data/check_examples.py: finite checks and recorded output OK
Corpus audit: OK. General mathematical claims require the written proofs.
```

The realization script checks actual predecessor-ordered gate lists, distinct
gate inputs, all-input reachability, witness dependencies, and the exact pin
budgets. `certify_incidence` compares the complete per-wire multiset of gate
incidences with connected equality trees of the permitted sizes.
`retain` checks each contracted vertex has degree one or two, the resulting
rank is at most three, the contracted edge is unique between its endpoints,
and cycle rank is unchanged. Its final labeled edge multiset equals the
target's, rather than merely testing an isomorphic subgraph or minor.

The stated finite counts agree with the code and retained output:

- 3,655 fixed-cycle/matching graphs, 42 bridged chains, and 32 seeded
  2-connected graphs give 3,729 constructions and 72,401 legal contractions.
- There are 913 triangle-free Hamiltonian cases and 68 cases with the exact
  divisible-by-three budget. These are counted subsets of the Hamiltonian
  domain, not additional graph cases.
- Exactly seven representatives run truth-table and symbolic-projection checks:
  one at each of the five Hamiltonian orders and one from each chain family.
  The symbolic checks use the existing compiler's own reductions; they do not
  claim per-contraction tensor evaluation along all prescribed retained-`K`
  sequences. The manuscript's present description respects this distinction.
- The older counts also match: 9,280 one/two-gate encodings; 2,000 seeded
  forest cases and three fixtures; all 65,536 four-by-four relations;
  362 sparse supports; 896 circuit/sample pairs; 140 rational expectation
  cases; five designated-input tensor cases, 600 seeded tensor circuits,
  and 140 tensor networks; 25,600 worked-family input pairs and 141 sparse
  correction instances. The coverage Jensen check uses floating arithmetic
  with tolerance; its occupancy and expected-miss identities are exact
  rational equalities, as the manuscript states.

The subsequent scope/provenance wording corrections were reread directly.
They did not alter a proof construction or a finite-check artifact. No
manuscript or implementation file was edited by this reviewer.
