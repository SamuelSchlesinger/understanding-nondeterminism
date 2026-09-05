# Completion source and fabrication audit

Audit date: 2026-09-04, America/New_York.

This is the bounded final source pass requested after the earlier
[fabrication audit](fabrication-audit.md) and
[aggregation audit](aggregation-sources.md). It checks the newly added or
revised Dudek--Duenas-Osorio--Vardi, Even--Tarjan, Nurk, and Broering--Lokam
claims, the Savinov metadata discrepancy, and the realization check counts.
The inspected occurrences include `references.bib`, `research/sources.md`,
the final comparison and statement matrix, `sections/prior-work.tex`,
`sections/realization.tex`, `sections/methods.tex`, the main abstract, and
the realization script and its prose record. No manuscript was edited.

Every external verification below uses a fresh retrieval of a primary
publication, institutional record, or explicitly identified reproduction of
the primary text. Training-memory claims and earlier reviewers' confidence
were not evidence. This pass does not repeat the complete earlier bibliography
audit or replace the separate analytical proof review.

## Findings and disposition

No new **Error**, **Gap**, or **Polish** finding was identified in this bounded
pass. The remaining historical access limitations are accurately disclosed.
No source or numerical correction is required before the parent agent's
final assembly check.

| Location | Severity | Finding | Disposition |
| --- | --- | --- | --- |
| `references.bib:241`; `research/sources.md:103`; `research/literature/final-comparison.md:50`; `sections/prior-work.tex:77` | None | Dudek--Duenas-Osorio--Vardi's title, three authors, identifier, v2 date, and attributed tensor results agree with the retrieved records and text. The manuscript preserves the distinction between maximum intermediate rank and contraction time. | No correction. |
| `references.bib:26`; `research/sources.md:117`; `sections/realization.tex:39`; `research/structural/realization.md:75` | None | Even--Tarjan's publication details and the cited st-numbering property agree. The master source entry correctly records that its abstract credits existence to Lempel, Even, and Cederbaum. The manuscript calls the property classical, supplies its own existence argument, and does not attribute its invention to Even--Tarjan. | No correction; no additional bibliography entry is needed to repair a false attribution. |
| `references.bib:252`; `research/sources.md:215`; `research/literature/statement-matrix.md:49`; `research/literature/final-comparison.md:24`; `sections/prior-work.tex:133` | None | Nurk's gate-count exponent, decision contract, theorem locator, and the non-count-preserving nature of Reduce step 7 agree with the full Russian preprint. The inaccessible English link remains explicitly distinguished from the retrieved Russian source. | No correction. |
| `research/sources.md:61`; `research/literature/statement-matrix.md:43`; `research/literature/final-comparison.md:34`; `sections/methods.tex:58` | None | Broering--Lokam's two reported exponent displays are present in the retrieved reproduction. Its limited pagination and lack of page-image confirmation for those displays are disclosed consistently. | No correction; retain the stated access boundary. |
| `research/literature/final-comparison.md:43` | None | The reported Savinov initials discrepancy exists in the two institutional PDFs: Lialina gives S. V.; Kulikov gives S. A. The original thesis was not supplied by either record. | No correction; the discrepancy remains unresolved, as stated. |
| `research/structural/data/check_realization.py:1`; `sections/methods.tex:102`; `sections/methods.tex:122`; `research/structural/realization.md:269` | None | The script's header, comments, printed counts, and prose descriptions agree with its finite domain and a successful fresh execution. Graph contractions and semantic checks are not conflated. | No correction; finite checks remain separate from the analytical theorem. |
| `main.tex:69`; `main.tex:79`; `sections/prior-work.tex:150` | None | The abstract and final positioning preserve the nonminimal-verifier condition and do not present the finite checks or source search as proof of priority. The newly stated realization scope agrees with the theorem statement. | No correction. This is a consistency check, not a fresh proof certification. |

## Fresh source evidence

**Dudek--Duenas-Osorio--Vardi.** The
[arXiv v2 record](https://arxiv.org/abs/1908.04381v2) confirms submission on
August 12, 2019 and revision on April 27, 2020, the displayed title and author
list, and DOI `10.48550/arXiv.1908.04381`. The
[complete v2 PDF](https://arxiv.org/pdf/1908.04381v2) confirms Theorem 1's
exact weighted-CNF construction, Theorem 3's max-rank/carving-width
correspondence, and Definition 10/Theorem 6's restricted tree factoring.
Theorem 6 assumes tree-factorable tensors, at most three free indices, and
decomposition width at least one. Pages 15-16 were rendered and checked.
The manuscript's phrase "suitable high-degree tensors" preserves this
restriction. Pages 7-8 explicitly distinguish contraction complexity and
maximum rank; the latter measures memory under simultaneous contraction of
shared indices. Theorem 3 also credits the earlier no-free-index
correspondence; the manuscript makes no contrary origin claim.

**Even--Tarjan.** The freshly retrieved
[publisher abstract](https://www.sciencedirect.com/science/article/pii/0304397576900864)
and [Princeton publication record](https://collaborate.princeton.edu/en/publications/computing-an-st-numbering/)
agree on *Computing an st-numbering*, Theoretical Computer Science 2(3),
339-344, September 1976, DOI `10.1016/0304-3975(76)90086-4`. They distinguish
Lempel--Even--Cederbaum's existence result for any chosen edge of a
biconnected graph from Even--Tarjan's linear-time computation. The
manuscript only needs existence. This pass did not retrieve the full
Even--Tarjan article; the accessed abstract states the exact property used.

**Nurk.** The [institutional index](http://www.pdmi.ras.ru/preprint/2009/09-10.html)
confirms preprint 10/2009, the English title, and acceptance on December 1,
2009. Its [linked Russian PDF archive](http://ftp.pdmi.ras.ru/pub/publicat/preprint/2009/10-09_rus.pdf.gz)
was downloaded and decompressed: 459,601 compressed bytes and 540,170 PDF
bytes, ten physical pages. Printed p.3 defines the circuit model and gate
size. Printed p.8, Theorem 3.1, displays the 0.4058 exponent and recurrence
with decreases two and three. Printed p.6, Reduce step 7, substitutes a
fresh input for a gate whose input variables have no other uses. Pages 6
and 8 were visually checked. The private-OR example in the corpus is a
valid direct counterexample to unweighted count preservation. The
[English archive link](http://ftp.pdmi.ras.ru/pub/publicat/preprint/2009/10-09.pdf.gz)
freshly returned HTTP 404. The freshly retrieved
[Chen--Kabanets author manuscript](https://www2.cs.sfu.ca/~kabanets/papers/linsize-eccc.pdf),
p.2, really does describe Nurk's result as a #SAT bound; the corpus correctly
distinguishes that later description from the inspected decision procedure.

**Broering--Lokam.** The [publisher record](https://link.springer.com/chapter/10.1007/978-3-540-24605-3_13)
confirms the authors, title, SAT 2003 venue, LNCS 2919, pp.162-171, 2004
publication, and DOI. Its [two-page preview](https://page-one.springer.com/pdf/preview/10.1007/978-3-540-24605-3_13)
was downloaded and visually checked; p.163 states the removal of fanout
dependence. The [public book-text reproduction](https://dokumen.pub/theory-and-applications-of-satisfiability-testing-6th-internatly-may-5.html)
was also freshly retrieved. Definition 6, Lemma 4, and Theorem 2 contain
the graph definitions and reported `2W` and `2pw` displays. The reproduction
ends during the p.168 proof. This is primary chapter content on a
nonpublisher host, not a full publisher PDF. The later formula pages were
not available as images in this pass, and pp.169-171 remain unchecked.

**Savinov discrepancy.** Fresh copies of
[Lialina's original article](http://ftp.pdmi.ras.ru/pub/publicat/znsl/v475/p122.pdf)
and [Kulikov's institutional thesis](https://www.pdmi.ras.ru/pdmi/system/files/dissertations/thesis-kulikov-18j.pdf)
were retrieved. Their bibliography pages were rendered and checked:
Lialina p.136, reference 10, has S. V. Savinov; Kulikov p.142, reference 70,
has the Russian equivalent of S. A. Savinov. Both name a 2014 master's
thesis at Academic University RAS. This verifies the discrepancy, not
which identity is correct or the unavailable thesis's full contents.

## Realization artifact audit

Executed successfully from the project root:

```text
python3 research/structural/data/check_realization.py
```

The entire displayed output agrees with
`research/structural/data/realization_expected.txt` and the retained output
in `research/structural/realization.md`.

| Actual finite domain or result | Fresh output |
| --- | ---: |
| Fixed Hamiltonian cycle plus every disjoint perfect matching, n=4,6,8,10,12 | 1, 4, 31, 293, 3326 |
| K4-based bridged chains, 0 through 20 internal blocks | 21 |
| K_(3,3)-based triangle-free bridged chains, same block range | 21 |
| Seeded 2-connected cubic cases, n=14,20,30,60 | 8 at each order |
| Triangle-free Hamiltonian cases | 913 |
| Triangle-free Hamiltonian cases with k divisible by 3 | 68 |
| Exhaustive verifier truth tables and symbolic projections | 7 |
| Legal degree-one/two contractions | 72401 |
| Exact labeled retained kernels with zero cycle loss | 3729 |

The total is `1+4+31+293+3326+42+32=3729`. These are labeled constructions,
not a census of nonisomorphic cubic graphs. The seed is `20260904`.
The seven semantic runs are selected by the actual call sites: the first
Hamiltonian case at each of five orders and the zero-internal-block member
of each of the two chain families (`check_realization.py:353,363`). Random
cases do not receive the exhaustive semantic test.

The code separately checks formal-wire conflicts and budgets, gate-list
topology and input usage, equality-tree agreement with actual incidences,
legality of each degree-one/two contraction, cycle rank after every
contraction, and exact final labeled edge multiplicities. Its general
graph-contraction routine does not propagate semantic tables through every
one of the 3,729 constructions; the manuscript correctly reserves the
semantic claim for the seven checks through the existing tensor compiler.
The bridged chains are additional supplied-order fixtures, not purported
2-vertex-connected inputs or a claim that all connected cubic graphs have
the theorem's orientation.

The existing `research/data/audit.py:22` includes this script in `make check`
and compares its output to the expected file. This pass ran the realization
script directly; the parent agent retains responsibility for the final
whole-project gate.

## Limits retained

Fresh download artifacts and extracted text are under
`/tmp/completion-source-audit/`. Certificate validation in Python failed
for some institutional HTTPS endpoints; the corresponding public HTTP
endpoint or system HTTPS client retrieved the primary PDFs. No missing
document was replaced with recollection or an inference from search absence.

Savinov's original thesis, Broering--Lokam's final pages, and the exact
historical status of the combined counting bound remain unresolved as
stated. The successful finite checks establish the recorded implementation
agreement. They neither prove the asymptotic graph theorem nor establish
novelty, a best-algorithm claim, or kernels of minimum verifiers. Nothing
was published and no external correspondence was sent.

## Final assembly and provenance disposition

The final local delta was checked after convergence. **No new Error, Gap, or
Polish finding.** The realization opening now concerns critical gate and
cycle budgets alone; the supporting text says the deductions establish no
converse. This agrees with the resolved scope finding in
`completion-integrated-review.md` and `critical-balance-review.md`. The
methods chronology, appendix reference, and `samepage` paragraph repair are
present. No external attribution changed, so no additional external search
was needed.

The completion claims at `research/index.md:75` and
`research/synthesis/methods.md:135` match the actual records. A fresh execution
of `check_documents()` reports **17 reachable documents, 39 canonical
sources, 80 LaTeX labels, and 33 cited BibTeX entries**. The audit registers
five existing script/expected-output pairs. The integrated review retains
the successful full `make check` output for all five; this delta pass did not
repeat those unchanged finite suites. Both final visual-review notes record
completed inspection with no outstanding corrections.

Direct file inspection confirms `main.pdf` has **39 pages, 387559 bytes**, and
SHA-256 **`c2899a9481d64d8ecc98b429532bb62a48b3fbbef66ca69ccb412b4e5c3ac84b`**.
The current LaTeX/BibTeX logs contain no warning, undefined-reference, or
overfull/underfull-box diagnostic. All 39 final page images exist; direct
byte comparison with the earlier completion renders finds changes only on
pages 15 and 35-39, exactly as the final review notes record. Those notes
document the renewed inspection of the changed pages. Thus the final
provenance distinguishes the historical 34-page draft from the current
artifact and accurately records the completed checks. The mathematical,
historical-priority, and publication limits above remain unchanged.
