# Deep-dive pedagogical completeness and coherence review

Reviewed 2026-09-05. This is a new review of the integrated source, not a
continuation of the foundations authoring assessment. No manuscript or public
entry-point edits were made by this reviewer.

## Coverage and method

Read all of `main.tex`, every currently included `sections/*.tex` file,
root `README.md`, and `research/index.md`. Followed the actual nested include
order, including counting, critical balance, and realization inside
`sections/structure.tex`. Used targeted searches to check where terminology is
first used or defined. Re-read the foundations in their integrated form.

This review evaluates the full pedagogical objective, with the SAT algorithm
emerging from an understanding of nondeterminism. It is not a fresh
mathematical or bibliographic audit, a PDF layout review, or a rerun of all
finite verification artifacts. In-progress README/methods updates are identified
as synchronization work, not evidence that the substantive objective failed.

## Overall assessment

The expansion supplies substantive understanding, rather than merely more
pages. The paths-to-witnesses account, explicit quantifier-scope failure,
canonical padding, exact counting matrices, integer-versus-Boolean operations,
and prefix-count consequences form a strong instructional sequence. The small
example is correctly described as insufficient to illustrate the asymptotic
cubic-kernel theorem. The later frontier update addresses the missing general
mechanism without misrepresenting the example. Expected-time sampling and
projected-input sampling are carefully distinguished.

The main remaining issue is the arrangement of this material. The strongest
pedagogical sequence exists in the files, but the actual include order breaks
it twice. A reader who has just learned witnesses encounters an advanced
algorithm summary before the operational graph vocabulary and then waits
through the universal ceiling and forest simulation to reach the concrete
counting example. Later, critical asymptotics and the lengthy realization
construction intervene between the counting proof and its direct consequences.
These are fixable integration issues, not a need for another broad authoring
stream.

The explicit finite existential scope is appropriate. Missing extended surveys
of alternation, nondeterministic space, quantum computation, or interactive
proofs are not findings. The text states its scope and develops it deeply.
The abstract's title and opening now match that scope. The historical
priority and full-implementation boundaries are also clear throughout.

## Findings

### C1. Teach the table example before the technical algorithm summary

- **Location:** `main.tex` include order at foundations/algorithm-overview and
  counting-walkthrough; `sections/algorithm-overview.tex`, "Why the running time
  improves"; `sections/foundations.tex`, final paragraph.
- **Severity:** Gap.
- **Finding:** The foundations end by promising that the next construction
  will make equality and local tables operational. The next section instead
  uses output cones, cycle rank, cubic graphs, pathwidth, and frontiers before
  their operational introduction. The complete example that actually fulfills
  the promise occurs only after three intervening main-text sections. An expert
  overview may reasonably preview later proofs, but this placement undercuts
  the requested learning route from understanding to algorithm.
- **Fix:** Move `counting-walkthrough` directly after foundations, then present
  the algorithm overview. Its first four subsections need no existing theorem
  beyond foundations. Its final frontier subsection already identifies the
  general machinery as forthcoming. Forward references to the structural
  lemmas can remain explicit. Alternatively retain a brief nontechnical
  headline early and move its "Why the running time improves" paragraph after
  the walkthrough; do not repeat the entire overview.

### C2. Keep the counting consequences next to the counting proof

- **Location:** Final three includes in `sections/structure.tex` and
  `main.tex`'s immediately following `algorithm-consequences` include.
- **Severity:** Gap.
- **Finding:** The uniform counting theorem is followed by finite stability,
  critical asymptotics, sparse-fiber forward references, and the full cubic
  realization construction before the paper returns to low-excess counting,
  search, indexing, and sampling. The latter are the direct payoff of the
  count interpretation and are easiest to understand while that invariant is
  fresh. Critical structure and exact realization are valuable deeper theory,
  but they are not dependencies of these consequences.
- **Fix:** Integrate the counting consequence section immediately after the
  counting algorithm, then resume critical structure and realization under an
  appropriately named structural section. Preserve their full proofs. A
  smaller change is to move the counting-algorithm include out of structure to
  just before algorithm-consequences, with a short transition identifying the
  all-quantified specialization. Make the reading route reflect whichever
  organization is chosen.

### C3. Supply the few graph and circuit terms on which the early argument relies

- **Location:** `sections/algorithm-overview.tex`, normalization and cubic
  reduction paragraphs; `main.tex`, Model and Effective inputs;
  `sections/counting-walkthrough.tex`, final subsection;
  `sections/structure.tex`, cubic-reduction statement.
- **Severity:** Gap.
- **Finding:** Cycle rank, tensor, frontier, cutwidth, and pathwidth eventually
  receive useful operational definitions. Output cone, fan-in/fan-out,
  essential input, and cubic/subcubic do not receive comparably explicit
  definitions before they carry reasoning. In particular, a new reader cannot
  infer that "cubic" means degree three rather than a polynomial running time,
  and the claim that an empty cubic remainder is forced by rank one is opaque
  without that meaning.
- **Fix:** Add a compact circuit-model paragraph defining fan-in as incoming
  wire count, fan-out as number of uses, the output cone as all vertices with
  a directed path to the output, and an essential input as one whose flip
  changes the function on some assignment. At first cubic use define cubic as
  degree exactly three and subcubic as degree at most three; distinguish simple
  graphs from the intermediate multigraphs there. These need only several
  sentences, not a graph-theory survey.

### C4. Make the scalar-to-symbolic transition tangible once

- **Location:** `sections/structure.tex`, opening and "Separate ordinary
  computation from consistency"; `sections/foundations.tex`, running example.
- **Severity:** Gap.
- **Finding:** The numerical walkthrough thoroughly explains integer tables.
  The next structural construction changes their entries to circuit wires
  depending on ordinary inputs and simultaneously introduces p, q, h, b, and
  ell. Its formal definitions are adequate, but there is no small symbolic
  calculation illustrating the change. The running example projects to a
  constant, so it does not demonstrate an output wire retaining nontrivial
  ordinary-input dependence.
- **Fix:** Add one short local illustration before the parameter accounting:
  for `f(a,b;y)=(a AND y) OR (b AND NOT y)`, the two witness entries are b and
  a, and their Boolean contraction returns the wire `a OR b`. Explain that
  this circuit is built once and works for every (a,b); ordinary inputs are
  not enumerated during symbolic contraction. Keep it clearly separate from
  the shared three-gate example. Then state the all-quantified specialization
  in one sentence: no ordinary signals remain, so p=h=0 and the retained
  witness count b becomes the retained input count used by the SAT bound.

### C5. Define P and the small amount of later complexity vocabulary actually used

- **Location:** `sections/foundations.tex`, "From bounded certificates to
  polynomial verification"; `main.tex`, nonuniform boundary and conditional
  hardness; `sections/aggregation.tex`, last paragraph.
- **Severity:** Gap.
- **Finding:** NP and P/poly are explained, but P itself is never defined
  before the text says that its nonuniform boundary is stronger than P != NP.
  Similarly the conditional example relies on k-CNF without explaining
  clauses/CNF, and the appendix invokes #P-completeness without defining what
  #P counts. Readers following the accessible opening should not need to
  supply these specific meanings from an outside course.
- **Fix:** Add one sentence defining P as languages decided by a deterministic
  polynomial-time algorithm and explaining the verification-versus-decision
  question. At the conditional example define a literal, a clause, and a
  k-CNF in one sentence and expand SETH's name. At first #P use identify its
  functions as accepting-path counts of polynomial-time nondeterministic
  machines under a fixed encoding convention. The existing caveat about
  witness/path multiplicity can be referenced. Detailed surveys are unnecessary.

### C6. Refresh the reading promises and trim one repeated argument

- **Location:** `main.tex`, "Reading route and contributions" and "Consistency
  obstructs naive aggregation"; `sections/algorithm-overview.tex`, final
  "From SAT to understanding nondeterminism" paragraph.
- **Severity:** Polish.
- **Finding:** The old reading route does not name the new walkthrough or
  algorithm-consequences section. The overview's "rest of the manuscript"
  description focuses solely on symbolic projection even though later sections
  return to uniform counting and generation. The two-independent-witness
  counterexample is also rederived almost verbatim after the coverage theorem,
  despite its full explanation in foundations and its stronger numerical
  demonstration in the walkthrough. This last repetition teaches no new local
  mechanism until the final observation about parity aggregation.
- **Fix:** Rewrite the route around actual final includes. Preserve purposeful
  reminders at semantic transitions, but replace the late duplicate
  counterexample with a reference to the foundational scope failure and retain
  its specific consequence for parity versus existential aggregation. Change
  the overview's final paragraph to name both the uniform consequences and
  the separate symbolic projection route.

### C7. Synchronize public descriptions without rewriting historical evidence

- **Location:** `README.md`, opening, "Reproduce the checks and paper", and
  repository guide; `research/index.md`, historical review descriptions;
  `sections/methods.tex`.
- **Severity:** Polish (known authoring-in-progress synchronization).
- **Finding:** The current README still describes the compiled manuscript as
  starting with the SAT algorithm and describes five finite suites. The new
  research-index reading route is substantially better aligned with the
  expansion. Its September 4 snapshot is clearly labeled historical, but
  phrases such as "the final expanded manuscript" within that block can now
  refer ambiguously to the current expansion. The methods file has not yet
  described the new pedagogical workstreams or two new finite artifacts in the
  reviewed snapshot. Root work on these files was explicitly in progress.
- **Fix:** Synchronize the final README route, suite description, and methods
  after integration. Keep old counts and PDF hashes in the dated snapshot;
  call its reviews the "September 4 expanded manuscript" rather than the
  unqualified final manuscript. Do not substitute current counts into historical
  validation records.

## Review boundary and completion recommendation

No mathematical Error finding is asserted by this coherence-only review.
The full proofs and substantive content should be retained. Resolve C1-C5 as
small ordering and teaching changes, then update the exact route and public
descriptions. An expanded field survey or more asymptotic results are not
required to satisfy the stated pedagogical scope.
