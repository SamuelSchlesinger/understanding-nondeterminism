# Final adversarial factual audit of the deep-dive expansion

Date: 2026-09-05. Baseline: `d42748a`.

## Verdict

No Error- or Gap-severity factual finding identified in the revised expansion.
The sampling statement now explicitly permits probability-zero infinite
rejection streams while proving finite expected time and probability-one
termination. The new methods record accurately describes local integer
fixtures and explicit counting-oracle checks, without claiming a full solver.

No actionable findings remain. The wording consistency item listed below and
a grammatical typo were communicated to the integrator, corrected, and re-read.
No source, script, bibliography, or public entrypoint was edited by this audit.
The root is independently building and visually inspecting the PDF; this
report makes no claim that those separate final checks have finished.

## Specific finding, resolved during the pass

- Location: `research/consequences/index.md`, generation paragraph, and
  `research/sources.md`, `jvv-generation` entry.
- Severity: Polish.
- Finding: These two summaries retain "always-output" wording while the
  revised manuscript uses the more precise probability-one termination
  contract. Both summaries correctly state expected time, so this does not
  invalidate a theorem. Consistent wording would avoid the ambiguity already
  removed from the main proof.
- Suggested fix: Replace "always-output" by "probability-one terminating", or
  say the sampler has no failure outcome when it terminates.
- Resolution: Both current corpus summaries now explicitly say that the
  sampler returns a solution with probability one and has an expected-time
  guarantee. The consequences page also explicitly permits a measure-zero
  random stream that rejects forever. Re-read both corrected passages.

The temporary `sections/methods.tex` phrase "examined on the critical" was
reported and re-read after correction to "examined the critical". It is closed.

## Files and contracts checked

Read the tracked diff relative to `d42748a` and separately inspected the new,
untracked foundations, walkthrough, consequences, and their corpus/script
files; `git diff` alone would omit those additions. The full mathematical
dependencies were read in the preceding
[accuracy review](deep-dive-accuracy-review.md), including every manuscript
section and validation script. This final pass concentrated on regression
risk from the actual revisions:

- The introductory circuit vocabulary correctly separates syntactic occurrence
  from essential dependence, supplied size from minimum size, and ordinary
  inputs from witnesses. The fixed-length encoding is bijective only with the
  stated canonical suffix check. Machine acceptance equivalence is never
  silently promoted to path-count preservation.
- The new symbolic example has table entries `x2,x1` when indexed by witness
  values `0,1`. Its Boolean contraction is one OR gate computing `x1 OR x2`;
  its integer witness count is `x1+x2`, equal to two at input `11`. This is
  distinct from enumerating ordinary inputs or counting projected inputs.
- Moving the complete example before the overview does not alter the general
  consistency construction. The small graph still has ten vertices, ten
  edges, and cycle rank one, and reduces completely. It is explicitly not an
  example of a nonempty cubic kernel or an experimental width theorem.
- The pinning proof retains the original graph, index universe, and layout.
  Free declared inputs contribute the appropriate power of two only when
  unpinned. The original enumeration alternative remains available, preserving
  the minimum of the original two costs. Sequential count calls cost a
  polynomial time factor without multiplying the exponential space factor.
- The sampler explicitly handles zero and one solutions, draws exact integer
  ranks from fair bits, rejects out-of-range ranks, and un-ranks bijectively.
  The strict greater-than-one-half success probability and less-than-two
  expected trials remain correct even when the solution count is a power of
  two. The marginal of a uniform satisfying full assignment is correctly
  distinguished from uniform distinct projected inputs.
- Public README coefficients, the fixed-slack quantifiers, the full B2 basis,
  unrestricted fanout, exponential space, and unresolved priority claims agree
  with the manuscript. CFF's summary asserts exact uniform sampling but does
  not introduce a contradictory worst-case sampling guarantee. Title strings
  in the README subtitle, LaTeX title/PDF metadata source, CFF, and LICENSE
  agree. The CI workflow actually invokes `make check` and `make pdf` and lists
  its LaTeX packages, matching the reproducibility instructions.
- `research/synthesis/methods.md` expressly identifies itself as the historical
  September 4 research-phase record. Its older suite and page counts should
  not be rewritten as current results. The current manuscript methods now
  record the two added suites, resolving the substantive experimental-record
  gap from the preceding accuracy review.
- The 1/4 uniform counting and 1/5 nonuniform projection coefficients still
  follow from different balances. Neither the added examples nor the
  reordered reading route turns these upper bounds into hardness or novelty
  assertions. Exact counting continues to mean full named-input assignments.

## Fresh bibliographic checks

The three newly added references were checked by fresh retrieval, not by
model recollection:

1. **Cook1971 / cook-sat.** The
   [ACM publication record](https://doi.org/10.1145/800157.805047) was retrieved
   through search after direct DOI navigation was blocked by the web tool.
   It confirms author, title, STOC 1971, pages 151-158, and DOI. The linked
   [article transcription](https://www.cs.cmu.edu/~15455/resources/Cook1971-complx-thm-proof.pdf)
   explicitly identifies its transcription status. Its Theorem 1 proof
   constructs an accepting-computation formula, and Section 2 discusses bounded
   existential quantification. The manuscript attributes those statements
   without silently replacing Cook's original oracle-reduction definition.
2. **Levin1973 / levin-search.** The
   [journal's MathNet record](https://www.mathnet.ru/eng/ppi914) confirms the
   universal-search title and formulation, year 1973, volume 9 issue 3,
   original Russian pages 115-116, and English translation pages 265-266.
   The source entry and BibTeX preserve the two pagination conventions.
3. **JerrumValiantVazirani1986 / jvv-generation.** The exact newly linked
   [author-hosted PDF](https://ics.uci.edu/~vazirani/JVV.pdf) resolves to the
   cited 20-page article, with the stated authors, title, journal volume 43,
   year 1986, and printed pages 169-188. Section 2 and Theorem 3.3 support
   the claimed fair-coin and counting-oracle background. Direct DOI navigation
   was blocked at the publisher redirect; the primary PDF remained accessible.

The load-bearing Fomin-Hoie theorem and its explicit polynomial construction
were freshly checked in the preceding accuracy pass against the
[author PDF](https://fedorvf.github.io/articles/2006/2006b.pdf), Theorem 5 and
Section 4. Nothing in the current diff changes that invocation.
Unchanged historical source-comparison and priority statements retain their
previously documented retrieval limits; this pass does not claim an exhaustive
new search of the entire literature.

## Executed evidence

Independently reran `make check` on the revised state. It exited zero, reran
all seven finite suites, and exactly matched every retained expected output.
The structural gate reported 20 reachable documents, 42 canonical sources,
102 LaTeX labels, and 35 cited BibTeX entries. The extra label relative to the
preceding accuracy pass is the new symbolic example, not a missing proof.

An additional disposable Python check enumerated all four ordinary assignments
of the new symbolic example. It verified both its Boolean projection and its
integer count. Exact `Fraction` calculations independently verified the three
new coefficient pairs `(1/2,1/3)`, `(3/4,2/3)`, `(7/8,5/6)` and four rational
instances of the explicit positive-slack substitution. The initial disposable
coefficient check accidentally compared a binary float with an exact fraction;
after making its inputs rational, all checks passed. No repository code or
mathematical statement changed as a result.

The new methods numbers match the actual scripts and their rerun output:
4,096 gate-table triples on eight inputs, 27 partial pins, 256 binary matrix
pairs, 278 small relations, 1,844 prefix partitions, 1,061 ranked/unranked
solutions, 1,617 invalid queries, and exact rejection probabilities for counts
one through sixteen. These remain finite mechanism checks. No full counting
implementation, asymptotic layout test, benchmark, optimality result, or
historical-priority conclusion is asserted.
