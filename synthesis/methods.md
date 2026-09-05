# Methods and provenance

Parent: [synthesis](index.md).

## Actual setup

- A user directed an OpenAI Codex agent to investigate existential projection
  of Boolean circuits and develop an editable LaTeX manuscript in main.tex.
- The active workspace is a local macOS/zsh directory. Tools permit shell
  execution, local file editing, web retrieval, PDF rendering, and child-agent
  tasks. Work is local; no submission or publication has been requested.
- The initial artifact for this research phase was a 12-page note with complete
  baseline proofs, seven bibliographic entries, and a Python finite compiler
  checker. The current research goal was activated on September 4, 2026.
- The root agent used the author-review-revise skill to organize independent
  tasks and review, while retaining the requested LaTeX manuscript as the main
  artifact. The PDF skill supplies the render-and-inspect workflow.
- Child agents share a filesystem. Bounded mandates and distinct file ownership
  reduce accidental overwrites. Independent prompts do not imply independent
  training or independent mathematical judgment.

## Research protocol

Candidate statements are recorded before integration, with explicit parameters
and proof sketches. Primary sources are checked for their exact statements and
models. Short corollaries are distinguished from substantial contributions and
from unresolved priority claims. Reviewers are asked to find counterexamples,
proof gaps, missing source comparisons, and overly broad interpretations.

Finite checks compare actual synthesized circuits with direct existential
evaluation. These checks test implementations and small instances; displayed
mathematical proofs carry the asymptotic claims. We do not infer minimum circuit
size, general lower bounds, or originality from experiment counts.

## Record of this phase

- Read the authoritative current manuscript, bibliography, and verification code.
- Wrote a research decomposition and dispatched one independent critique before
  parallel research tasks.
- Developed an occupancy/alteration candidate integrating witness sampling with
  shared restrictions, then refined correction cost using block synthesis and
  concavity. Independent coverage and integrated reviews found no proof-breaking error.
- Implemented and ran an actual sampled-restriction circuit compiler. The first
  size-accounting check failed because the output OR had not deduplicated
  restrictions. Deduplicating only the sampled full witnesses was insufficient:
  different witnesses can agree on the output support. The compiler now removes
  duplicate output wires before ORing; the mathematical proof explicitly requires
  output-support deduplication.
- The coverage checker passed 362 sparse support sets, 896 verifier/sample pairs,
  and 140 exact rational expectation cases, including biased distributions with
  zero masses and zero samples. The original forest/distribution checker also
  passed unchanged. These finite counts describe actual completed checks.
- A separate analytical reviewer found that the tensor compiler omitted its
  proof's zero-gate witness-output branch. Added that branch and five designated
  input cases. The proof already handled this case; the finding corrected the
  implementation's scope rather than supporting an asymptotic extrapolation.

## User clarification on experiments

The user explicitly required experiments only where they are useful, analytical
proofs for asymptotic statements, and a distinction between implementation checks
and exhaustive proofs of genuinely finite statements. This is the governing
verification standard. Tests are not used as proof of an infinite theorem or
as evidence that a result is new to the literature.

Structural and coverage reviewers checked the analytical proofs; a later
integrated reviewer also checked the integer counting consequence. A separate
fresh-source audit corrected two minor bibliographic-language errors. The
[focused SAT comparison](../../review-notes/sat-comparison.md) retains the
remaining priority uncertainty and precise retrieval gaps.

No end-to-end runtime experiment or Fomin-Hoie implementation is claimed.
The counting theorem is justified by its analytical multiplicity, bit-cost,
and constructive-layout proof.

The user's suggestion of a cheaper aggregate prompted a bounded analytical
investigation of weighted characteristic-two fingerprints. It produced a
transfer from efficient field evaluation to exact nonuniform projection,
cycle-preserving unary tags, and a universal linear-fingerprint obstruction.
These arguments use classical polynomial identity testing; no improved
general exponent is claimed. Separate [proof](../../review-notes/aggregation-review.md)
and [source](../../review-notes/aggregation-sources.md) reviews were completed.
Two wording corrections from the proof review were applied. No experiment
was needed for this investigation: the transfer, obstruction, and matching
example have analytical proofs in the manuscript.

The preceding draft passed the corpus audit and all four then-retained finite
verification records. That 34-page PDF compiled without warnings and was
inspected page by page. The final visual pass corrected two bibliography
capitalization issues; the three affected pages were rendered and inspected
again, with unchanged text and pagination on pages 1-31. Source assembly
was audited separately from the mathematical reviews.

## Final focused phase

The user then authorized completing the paper through three bounded tasks:
critical-balance deductions, a budgeted kernel realization theorem or
obstruction, and resolution of specific source-comparison gaps. A preliminary
review checked this decomposition before the independent construction and
source tasks began. The root developed the critical-balance proof while a
separate author developed the realization construction.

The analytical results include a finite stability inequality, the necessary
critical input and cycle budgets, and asymptotically saturated widths for
every legal residual kernel. The realization proof constructs normalized
AND-only verifiers for every simple 2-connected cubic graph, repairs repeated
input wires within an explicit budget, and gives a legal reduction retaining
the exact graph with zero cycle loss. Computing the exact functions and their
minimum sizes exposes the supplied verifier's nonminimality.

Separate reviewers checked the fixed-slack limits, input/interface counting,
sparse-fiber conventions, conflict budget, sink construction, equality trees,
and contraction tables. Review clarified which amplification hypothesis is
retained when combining the sparse and structural conclusions, made the
kernel-size limit explicit, and narrowed the realization's opening: the
construction concerns critical gate and cycle budgets, not a converse to all
simultaneous necessary conditions. These are analytical and scope checks,
not statistical evidence.

One new finite checker was justified by a concrete risk: the sketched graph
expansion must actually split the incidence graph of the constructed circuit.
It verifies gate lists, pin distinctness, budgets, equality-tree incidences,
and legal degree-one/two graph contractions. It passed all 3,729 specified
graph constructions and 72,401 contractions; seven representatives also
passed exhaustive truth-table and symbolic projection checks. The other graph
cases do not claim per-contraction tensor evaluation. The construction's
general correctness and minimum sizes have independent analytical proofs.

The source task recovered Nurk's complete Russian preprint and more text of
Broering--Lokam, and checked the Dudek--Duenas-Osorio--Vardi factoring theorems.
The [final comparison](../literature/final-comparison.md) supersedes the earlier
retrieval limits, preserving the remaining unavailable text and priority
uncertainty. A fresh adversarial source audit is separate from proof review.

The expanded manuscript now passes all five retained finite verification
records, with 17 reachable research documents, 39 canonical sources,
80 LaTeX labels, and 33 cited BibTeX entries. Its 39-page PDF compiles without
warnings. Final source and integrated reviews and a fresh visual inspection
are in progress; the earlier 34-page inspection is not evidence for newly
added or repaginated pages.

This record reports methods and evidence, not private model reasoning.
