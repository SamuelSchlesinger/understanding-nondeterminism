# Focused completion-plan critique

Reviewed 2026-09-04 against `research/index.md`, `sections/structure.tex`,
`sections/prior-work.tex`, `sections/coverage.tex`, and
`review-notes/sat-comparison.md`. This is a decomposition review, not a new
proof or literature audit.

The three-way split is appropriate for this final phase after tightening the
contracts below. The root can develop stability independently of realization;
neither result depends on establishing numerical priority. Final framing depends
on all three outputs and their independent review.

## Findings

1. **Gap: define what is near-critical before proving stability.**
   **Location:** `research/index.md:58`; `sections/structure.tex:242-282`.
   At fixed `delta>0`, the proved exponent estimate is
   `z=min{h,b,alpha*kappa}`, with `alpha=1/3+delta` and budget ceiling
   `beta*(q+1)`, where `beta=alpha/(1+2*alpha)`. Its balancing ratio is
   `1:1:1/alpha`; `1:1:3` is the limiting ratio. Near-maximality of this
   certified exponent, a hypothetical lower bound on `C(g)`, and a bound
   expressed in the full size `s=p+q` are distinct premises.
   **Suggested fix:** prove finite-`alpha` stability first, including budget
   slack, then state the limiting consequence. For a hardness corollary,
   explicitly absorb the polynomial factor and require hardness above the
   additive preprocessing cost `p`. Fix the quantifier order for `delta`, the
   allowed deficit, and the size threshold.

2. **Gap: "which cubic kernels" overpromises a classification.**
   **Location:** `research/index.md:60-62`; `sections/structure.tex:92-139`.
   The reduction permits choices of equality trees and contraction order.
   Occurrence as a subgraph or minor of the circuit graph does not supply an
   exact output of this reduction. Unspecified linear gate overhead is also too
   weak to assess the numerical critical balance.
   **Suggested fix:** commission a precise constructive theorem for a stated
   graph class, or a precise obstruction, with an explicit bounded fallback if
   a general theorem fails. Require a normalized, single-output, acyclic `B2`
   construction; a specified legal reduction sequence; and exact formulas or
   explicit inequalities for `p,q,h,b,kappa`, kernel size, and lost cycle rank.
   State whether realization means existence of such a reduction or invariance
   under every allowed reduction. The former is an adequate initial target.

3. **Gap: joint attainability is the missing bridge.**
   **Location:** `sections/structure.tex:280-307,337-354`;
   `sections/coverage.tex:198-225`.
   A large realizable cubic kernel alone does not demonstrate near-critical
   `h,b,kappa`, near-saturation of the kernel-size estimate, costly actual
   layouts, resistance to conditioning, or expensive projection. Likewise,
   satisfying the sparse-fiber necessary condition does not imply hardness.
   **Suggested fix:** ask explicitly which of these properties the realization
   construction achieves simultaneously. If it only establishes graph
   realizability, present that as its conclusion. Combining stability with the
   sparse-fiber result should produce a conditional necessary-condition
   theorem, with no converse. A construction with an easy projection is a
   useful explicit control showing the limits of the structural statement.

4. **Gap: structural and fiber parameters need a shared interface.**
   **Location:** `sections/structure.tex:18-32`;
   `sections/coverage.tex:179-205`.
   Structural accounting uses used witnesses `b` and the normalized core;
   the fiber corollary uses declared witnesses `m`, ordinary-input count `n`,
   and full verifier size `s`. Unused witnesses change absolute fiber counts.
   **Suggested fix:** state one normalization convention for the combined
   theorem, and translate counts explicitly if `m>b`. Specify whether ordinary
   inputs in the population count are all declared inputs or only used inputs.
   Give the parameter range where the population lower bound is nonvacuous.

5. **Gap: source closure needs a bounded success criterion.**
   **Location:** `research/index.md:63-64`;
   `review-notes/sat-comparison.md:171-202`.
   The current note identifies inaccessible full texts and expressly limits its
   priority conclusion. "Close" cannot guarantee retrieval or an exhaustive
   negative result. A statement sharing the numeral `1/4` may still use a
   different model, resource bound, or output task.
   **Suggested fix:** give the literature author the listed unresolved sources
   and concrete citation trails, with a finite search scope. Require a
   theorem-level comparison of basis, gate measure, distinct inputs, exact
   counting versus decision, uniformity, fixed-epsilon quantifiers, time, and
   space. Accept either a verified match, a verified distinction, or a clearly
   documented remaining gap. Keep "best known" and priority unresolved unless
   the resulting evidence actually supports them.

## Recommended dependency and completion order

Freeze the shared notation and the realization contract above, then run the
three tasks independently. Root integrates only proved outputs, separately
labeling necessary conditions, realizations, and literature findings. Review
the new proofs and the exact uniform counting comparison before changing the
abstract's claims. Finally rerun the existing checks, assemble references, and
inspect the newly rendered PDF. The preexisting draft's recorded validation in
`research/index.md:71-82` does not validate the new additions.

This phase can finish with a stronger conditional structural theorem, a bounded
realization result, and a precise comparison that still leaves priority open.
It need not classify all cubic kernels or resolve minimum circuit complexity.
