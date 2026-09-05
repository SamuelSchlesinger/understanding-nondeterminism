# Source quality and attribution audit of the expanded manuscript

Reviewed 2026-09-05. Scope: citation-bearing passages throughout `main.tex`
and all current `sections/*.tex`, all entries of `references.bib`, the canonical
`research/sources.md`, the literature statement matrix and comparisons, and the
new foundations/walkthrough/consequences interfaces. Read the remaining theorem
sections for misleading implications around citations. Fresh primary retrieval
emphasized Cook, Levin, JVV, Fomin--Hoie, and the other numerically specific
attributions. This review did not edit the manuscript or source corpus.

**Result: no Error-level mathematical misattribution found.** The four actions
below concern a missing current validation summary, precise source provenance,
and publication consistency. Retrieval limitations are recorded separately;
this review does not certify literature completeness or historical priority.

**Closure check (2026-09-05): all four actions below are resolved.** Re-read
the revised `references.bib`, `research/sources.md`, literature statement
matrix, `research/consequences/index.md`, and `sections/methods.tex`. GKST now
identifies corrected revision 2, its date, correction, and versioned PDF;
Morizumi 2015 now records the verified COCOON publication while retaining the
inspected preprint; source prose now states probability-one termination and
expected time; and the methods appendix gives the two new suites' exact
finite scopes and retains the boundary against claiming a full asymptotic
solver implementation. No unresolved actionable source-review findings remain.
The original observations below are retained as the audit trail.

## Actionable findings

### Record the corrected GKST preprint revision

- **Location:** `research/sources.md`, `gkst-elimination`; `references.bib`,
  `GKST2018`; corresponding statement-matrix row.
- **Severity:** Polish.
- **Finding:** The source record says the full ECCC TR16-022 was inspected but
  leaves the revision unspecified. The current primary ECCC record explicitly
  says revision 2, dated 2017-12-01, fixed a critical flaw in Lemma 11. Its
  abstract retains the cited `2.99n` B2 and `3.24n` U2 counting thresholds.
  This audit freshly retrieved revision 2, so a corrected stable target is
  available. No evidence was found that the manuscript's quoted thresholds
  are incorrect.
- **Fix:** Record revision 2 and its date in the author-version inspection
  note, while keeping the final 2018 journal article as canonical. Link the
  versioned [revision-2 PDF](https://eccc.weizmann.ac.il/report/2016/022/revision/2/download).
  The correction notice is on the [primary ECCC record](https://eccc.weizmann.ac.il/report/2016/022/).

### Match the Morizumi 2015 bibliography to its verified publication

- **Location:** `references.bib`, `Morizumi2015`; `research/sources.md`,
  `morizumi-parity`.
- **Severity:** Polish.
- **Finding:** The manuscript bibliography presents this only as an arXiv item,
  while the canonical research record already gives COCOON 2015, LNCS 9198,
  pp. 289-296, DOI `10.1007/978-3-319-21398-9_23`. The freshly fetched
  [publisher record](https://link.springer.com/chapter/10.1007/978-3-319-21398-9_23)
  confirms that publication and the parity lower bound. Theorem 2 of the
  [inspected preprint](https://arxiv.org/pdf/1504.06731v1), p. 4, agrees.
- **Fix:** Use an `@inproceedings` entry for the published work, retaining the
  author PDF and a note identifying its theorem numbering if desired.

### Propagate the sampler's termination clarification into source prose

- **Location:** `research/consequences/index.md`, generation paragraph;
  `research/sources.md`, `jvv-generation`.
- **Severity:** Polish.
- **Finding:** These passages still call the rejection construction
  "always-output". Its finite expectation and probability-one termination are
  correct; termination on every random-bit stream is not promised. The
  manuscript's analogous wording was identified in the separate consequences
  accuracy review. JVV's failure-permitting convention is otherwise accurately
  described.
- **Fix:** Say it returns a solution almost surely on satisfiable inputs, with
  expected running time; synchronize these passages with the final manuscript.
  [JVV Section 2](https://ics.uci.edu/~vazirani/JVV.pdf), printed pp. 172-173,
  distinguishes bounded fair-coin generation and its permitted failure event.

### Complete the methods appendix for the newly retained validation

- **Location:** `sections/methods.tex`, finite-check table and subsequent scope
  discussion; `research/data/audit.py`, `CHECKS`.
- **Severity:** Gap.
- **Finding:** At inspection, the methods appendix still summarized the five
  older verification suites, while the reproducibility entry point now runs
  seven, including walkthrough and query-indexing validators. The existing
  statement that the full integer-counting algorithm is not implemented is
  correct and must remain. The new local integer-table/pinning checks and the
  explicit-oracle rank/unrank checks deserve their own scope lines so readers
  can tell what the expanded edition actually validated.
- **Fix:** Add the two new suite descriptions and exact retained finite domains,
  using their actual outputs. Distinguish local integer identities and explicit
  relation oracles from an implementation of the asymptotic solver. If this
  update is already in progress under root ownership, incorporate it there.

## Load-bearing primary evidence checked

| Manuscript attribution | Fresh primary evidence | Assessment |
| --- | --- | --- |
| Cook's accepting-computation encoding | [Cook transcription](https://www.cs.cmu.edu/~15455/resources/Cook1971-complx-thm-proof.pdf), Theorem 1 proof, transcription pp. 2-3, constructs a CNF satisfiable exactly on acceptance; Section 2, pp. 4-5, gives bounded existential relations. [Author homepage](https://www.cs.utoronto.ca/~sacook/) supplies historical publication information. | Accurate. The manuscript cites the construction and does not silently change Cook's stated oracle-style tautology theorem into a modern many-one formulation. The file explicitly identifies itself as a transcription. |
| Levin's universal-search formulation | [Journal record](https://www.mathnet.ru/eng/ppi914) and its [two-page original](https://www.mathnet.ru/php/getFT.phtml?jrnid=ppi&option_lang=eng&paperid=914&what=fullt). | Title, author, 1973 volume/issue, Russian pp. 115-116 and English pp. 265-266 agree. The narrow universal-search attribution does not claim a count-preserving theorem. |
| JVV counting-to-generation connection | [Author-hosted published PDF](https://ics.uci.edu/~vazirani/JVV.pdf), Section 2 pp. 172-173 and Theorem 3.3(1), p. 174. | Accurate. The source permits failure; the manuscript supplies its own rank/unrank and expected-time proof and does not attribute the new numerical exponent to JVV. |
| Fomin--Hoie `1/6` and constructivity | [Published article](https://fedorvf.github.io/articles/2006/2006b.pdf), Theorem 5 and first paragraph of Section 4, both p. 194. | Exact match: maximum degree at most three, sufficiently large order for each fixed positive slack; Section 4 expressly supplies polynomial-time construction. The fixed-slack algorithm is supported, not merely a nonconstructive width existence result. |
| Cutwidth at most pathwidth plus two for subcubic graphs | [Bodlaender et al. published PDF](https://www.combinatorics.org/ojs/index.php/eljc/article/download/v32i3p36/pdf/), proof of Lemma 16, p. 17. | The two graph-search inequalities are explicitly combined there. The manuscript calls this a restatement and retains its direct proof. It does not pretend to have retrieved both original graph-search proofs. |
| COPY expansion baseline | [Biamonte--Morton--Turner v2](https://arxiv.org/pdf/1405.7375v2), Theorem 17 and Remark 18, p. 9. | Correct version and locator. The source's canonical-representation restriction is retained; its `poly(g+cd) 2^c` mechanism is not generalized to arbitrary representations without proof. |
| Tensor counting/decision semirings | [De Beaudrap et al. v2](https://arxiv.org/pdf/2004.06455v2), Section 3.2, p. 51. | Explicitly uses natural-number versus Boolean semirings; the manuscript's statement is supported. |
| Planar guarantees versus cubic experiments | [Kourtis et al. v2](https://arxiv.org/pdf/1805.00475v2), Corollaries 1-2 and Section 3. | Correctly separated. Their unrestricted-instance experiments are not a worst-case coefficient theorem. |
| Contraction/line-graph width | [Markov--Shi v7](https://arxiv.org/pdf/quant-ph/0511069v7), Proposition 4.2, p. 10. | Supplies the contraction-width identity and polynomial construction from a given decomposition. The manuscript independently charges exact integer arithmetic. |
| Boolean checking and finding solutions from counts | [Johnson et al.](https://www.nature.com/articles/srep01235), sections "Searching by counting" and "Counters based on Boolean circuits". | Direct precedent. The manuscript preserves the distinction between that mechanism and its gate-budget application. |
| Structured compilation and quantification | [Amarilli et al. v2](https://arxiv.org/pdf/1811.02944v2), Theorems 4.2/4.4 and Corollary 4.5; [Capelli--Mengel published paper](https://drops.dagstuhl.de/storage/00lipics/lipics-vol126-stacs2019/LIPIcs.STACS.2019.18/LIPIcs.STACS.2019.18.pdf), Theorem 5. | The source targets, width parameters, and arithmetic-operation conventions are kept separate from unrestricted B2 circuit size. No deterministic-DNNF property is assumed of the manuscript's output. |
| Morizumi separation and bounded-width simulation | [2019 v2](https://arxiv.org/pdf/1811.01347v2), Theorems 3-4; [2015 v1](https://arxiv.org/pdf/1504.06731v1), Theorem 2. | Exact constants, U2 basis restriction, layered width, and 2018/2019 version distinction are accurately retained. |
| Nonuniform SETH | [Aggarwal et al. v1](https://arxiv.org/pdf/1911.02440v1), Definition 2.9, printed p. 11. | Matching quantified hypothesis. The manuscript's big-O circuit-family formulation is equivalent at the global for-every-epsilon level by absorbing constants with fixed slack. It is not presented as an unconditional bound. |
| SAT algorithms imply lower bounds / conditional resource bounds | [Williams author paper](https://people.csail.mit.edu/rrw/improved-algs-lbs2.pdf); [Paturi--Pudlak proceedings paper](https://cseweb.ucsd.edu/~paturi/myPapers/pubs/PaturiPudlak_2010_stoc.pdf). | Broad, appropriately conditional summaries; no extrapolation to an unrestricted lower bound for the paper's restricted instances. |
| Different circuit width and space conventions | [Lokshtanov et al. author paper](https://sites.cs.ucsb.edu/~daniello/papers/boundedTreewidthCircuitSatSODA18.pdf); [Allender et al. published paper](https://theoryofcomputing.org/articles/v010a012/v010a012.pdf). | The manuscript preserves the input-deleted parameter and the role of space rather than treating width models as interchangeable. |
| Counting permanent | [Valiant published paper](https://www.cs.bu.edu/faculty/gacs/courses/cs535/papers/Valiant_permanent.pdf), Theorem 1 and its counting-class formulation. | Supports the zero-one permanent claim. The manuscript does not claim parsimonious hardness or a proved separation from polynomial-time matching decision. |
| Lupanov and sparse-synthesis identities | [Lupanov journal record](https://radiophysics.unn.ru/issues/1958/1/120); [Redkin 2004 record](https://www.mathnet.ru/eng/dm172); [Redkin 2020/2021 record](https://www.mathnet.ru/eng/dm1592). | Publication identities and translations agree. This pass checked these records, not all original synthesis proofs; the manuscript includes its needed elementary majorants. |

Darwiche--Marquis primary article and the exact counting/conditioning/forgetting
distinctions were also freshly inspected earlier in this same expansion run;
see `deep-dive-expert-connections.md` for page-specific evidence. The
knowledge-compilation comparison in the current manuscript remains consistent
with those statements.

## Retrieval and certification boundary

Some established-source claims were checked against the existing explicit
inspection records plus current identities, rather than a newly downloaded
complete article. In this pass, the Aji--McEliece institutional record was
accessible but its PDF download failed; the Dudek et al. versioned abstract
was accessible but the v2 PDF failed; the Even--Tarjan publisher endpoint,
Schwartz DOI endpoint, and Berkeley MVV PDF failed; Nurk's site and Lialina's
institutional PDF also failed. The Lialina journal record remained accessible.
Those failures do not falsify previous documented inspections. They do prevent
claiming a fresh full-text recheck of every historical source in this pass.

The manuscript retains proofs of the algebraic identities it uses, explicit
limitations for abstract-only source inspections, and the unresolved priority
statement. In particular, neither the historical decision exponents nor the
graph-theorem citation certify that the resulting gate-counting exponent is a
new best unrestricted-space bound. No such certification is offered here.

## Mechanical citation check

Executed the existing `check_documents()` function from
`research/data/audit.py` without rerunning unrelated numerical suites:

```text
Documents: 20 reachable; 42 canonical sources; 102 LaTeX labels; 35 cited BibTeX entries: OK
```

This verifies key resolution and local corpus structure, not external URL
availability or theorem correctness. It supplements the primary-source checks.
