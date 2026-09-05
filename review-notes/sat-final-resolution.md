# Final focused CircuitSAT source audit

Audit date: 2026-09-05 UTC (2026-09-04 in New York).

## Finding and recommended status

The Russian Nurk preprint has now been retrieved in full. The
Broering--Lokam gap has narrowed from two preview pages to the chapter text
through printed p.168, including its two relevant explicit width bounds.
Savinov's thesis and Broering--Lokam pp.169--171 remain access gaps.

I found neither a proof-breaking model mismatch in the manuscript's counting
argument nor a retrieved earlier statement of its gate-minus-input coefficient.
This does **not** establish priority. The strongest supported description is
an explicitly proved application of established tensor elimination and cubic
layout methods, with circuit-specific accounting. The uniform counting result
should remain a derived corollary, not a claimed first or best-known algorithm.

The displayed exponent is smaller than the exponent of the particular
branching algorithms cited in the manuscript, after fixing a sufficiently small
positive epsilon. This arithmetic comparison is valid. It neither certifies the
current best unrestricted-space algorithm nor explains why later authors cite
the branching bounds. Exponential space alone does not supply that explanation.

## Nurk: the full-text gap is closed for the Russian version

Primary index: <http://www.pdmi.ras.ru/preprint/2009/09-10.html>.
Working full primary source:
<http://ftp.pdmi.ras.ru/pub/publicat/preprint/2009/10-09_rus.pdf.gz>.
The linked English `10-09.pdf.gz` still returns HTTP 404.

The complete PDF has ten physical pages, including an added institutional
cover; printed pp.3--9 contain the paper. Printed p.3 defines finite acyclic
circuits with arbitrary binary Boolean gates, one output, and internal-gate
size `m`. Printed p.8, Theorem 3.1, states `O(2^(0.4058m))` and proves it
from `f(m) <= f(m-2) + f(m-3)`. The source is a branching algorithm, with no
tensor, cubic-kernel, or width-based alternative. The theorem page was also
rendered and visually checked.

Printed pp.4--6 specify Boolean output and an equisatisfiability contract.
`Reduce`, step 7, replaces a gate on private input variables by a fresh input.
That operation does not preserve unweighted solution counts: replacing
`x OR y` by `z` changes three accepting assignments to one. Thus the
published decision procedure is not, as written, an exact counting procedure.

Chen--Kabanets' author manuscript, p.2, nevertheless describes Nurk's bound as
a #SAT bound. This is a later attribution, not a counting proof in the
retrieved Nurk text. Do not silently upgrade the certified theorem on that
basis. Primary author PDF:
<https://www2.cs.sfu.ca/~kabanets/papers/linsize-eccc.pdf>.

## Broering--Lokam: exact constants retrieved, final pages still missing

Canonical primary publication:
<https://doi.org/10.1007/978-3-540-24605-3_13>.
Publisher preview:
<https://page-one.springer.com/pdf/preview/10.1007/978-3-540-24605-3_13>.

The public text reproduction of the actual chapter below exposes printed
pp.162--168, ending inside Theorem 2's proof:
<https://dokumen.pub/theory-and-applications-of-satisfiability-testing-6th-internatly-may-5.html>.
This is a reproduction of the primary paper, **not a publisher-hosted full
copy**; its formulas were not independently checked against page images.

- Definition 6, p.167: `G_C` is the underlying undirected circuit graph,
  including inputs; it differs from the CNF primal graph.
- Lemma 4, pp.167--168: for fanin-two AND/OR/NOT circuits, the displayed
  running time is `O(n * 2^(2 W_C) * (W_C + log n))`, where
  `W_C = cw(G_C)`. The proof charges at most two cut clauses to each cut
  edge and does not depend on fanout.
- Theorem 2, p.168: CNF satisfiability takes
  `O(2^(2 pw(G_phi)) * n * (pw(G_phi) + log n))` for the primal graph.

Neither displayed theorem gives `2^w` time on the manuscript's cubic tensor
graph. A cubic reduced table need not be a deterministic circuit gate, so
applying Lemma 4 to that graph would also require a new semantic reduction.
The primary full-PDF URL returned a publisher access page; the public
reproduction's download did not succeed. Do not mark the entire chapter read.

## Savinov: a narrower but unresolved access gap

English and Russian title searches, institutional-domain searches, and
follow-up bibliography trails did not retrieve the 2014 thesis. The primary
Lialina article remains a complete presentation of the `0.389667m`
decision algorithm:
<https://ftp.pdmi.ras.ru/pub/publicat/znsl/v475/p122.pdf>.

The Russian title is additionally recorded in Kulikov's institutional thesis,
printed p.142, reference 70, as *Verkhnie otsenki dlya zadachi vypolnimosti
bulevykh skhem* (upper bounds for Boolean circuit satisfiability), Academic
University RAS, 2014:
<https://www.pdmi.ras.ru/pdmi/system/files/dissertations/thesis-kulikov-18j.pdf>.
Its initials differ from Lialina's bibliography (`S. A.` versus `S. V.`).
Without the original title page, retain the existing cited identity and record
the discrepancy rather than silently deciding which initials are correct.
Additional statements in the thesis remain unchecked.

## The closest additional tensor antecedents

**Dudek--Duenas-Osorio--Vardi, arXiv:1908.04381v2, 2020.** Full PDF retrieved:
<https://arxiv.org/pdf/1908.04381v2>; version metadata:
<https://arxiv.org/abs/1908.04381>.
Theorem 1, pp.8--9, gives an exact weighted CNF-counting tensor network.
Theorem 3, p.11, identifies maximum intermediate tensor rank with carving
width. Definition 10 and Theorem 6, pp.15--16, give tree factorizations into
rank-at-most-three tensors and, from a width-`w` decomposition, a contraction
tree of maximum rank at most `ceil(4(w+1)/3)` with unchanged bond dimension.
Thus neither exact counting by tensors nor replacing COPY tensors by trees is
a new principle here. Their paper contains no gate-minus-input theorem or
Fomin--Hoie coefficient. Its rank bound controls space; arbitrary binary
contractions can cost more than `2^rank` operations. The manuscript's
sequential degree-three frontier analysis supplies the needed time constant.

**Markov--Shi, constant-degree expansions.** Primary statement:
<https://arxiv.org/abs/0707.3622>; published DOI:
<https://doi.org/10.1007/s00453-009-9312-5>.
They construct degree-at-most-three expansions with treewidth at most the
original treewidth plus one, efficiently from a decomposition. This is another
direct antecedent for replacing high-degree vertices by trees. Their stated
vertex bound `2|E|+|V|` does not by itself provide the circuit budget
`N <= 2(s+1-b)`. That numerical budget must still be derived and preserved
through exact table operations.

**Fomin--Hoie, constructive status.** Full author PDF retrieved:
<https://fedorvf.github.io/articles/2006/2006b.pdf>.
Theorem 5, printed p.194, has the fixed-positive-epsilon subcubic pathwidth
bound. Section 4 on the same page explicitly says that its proof yields a
polynomial-time construction of the path decomposition. Thus the uniform
algorithm is not blocked by merely existential access to a layout. Their
algorithmic corollaries address graph problems, not the manuscript's circuit
accounting.

**Inference from these antecedents.** Once the manuscript has produced a
binary cubic tensor graph on `N` vertices, the asymptotic
`2^((1/6+eta)N)` contraction bound is a composition of established layout
and elimination methods. It should be presented that way even if no earlier
paper displays that exact composition. The circuit-specific addition is the
conservation of consistency and counts while charging the remaining cubic
vertices to `s+1-b`, followed by balancing against `2^b`. The audit has not
established whether that accounting or the combined corollary was previously
published.

## Independent check of the potential model gap

The manuscript quantifies inputs and uniquely determined gate values. Its
incidence graph has cycle rank at most `s+1-b`; splitting equality vertices
into trees preserves that rank and has a unique internal extension. All
degree-one/two, loop, and parallel-edge reductions are integer tensor
identities. They can change local tensors away from deterministic gates, which
is harmless for tensor contraction but matters for comparisons to circuit
algorithms. A nonempty simple cubic remainder has
`N = 2(kappa(K)-1)`. The median construction uses each cubic vertex's single
timestamp above and below its median, with at most one extra crossing edge
when splitting a tied pair. A frontier update contracts one rank-three tensor,
so its arithmetic work is bounded by a constant times `2^w`, not the generic
cost of combining two large tensors. Counts have polynomial bit length.

I found no failure in these steps. This review is not an exhaustive
verification of the cited graph theorem or a proof that no better algorithm
is known. The directly checked chain supports keeping the corollary with its
fixed-epsilon quantifier, polynomial prefactor depending on epsilon, and
exponential-space contract.

## Exact proposed final manuscript positioning

Suggested replacement comparison paragraph:

> Our all-quantified construction yields a deterministic exact counting
> algorithm for single-output B2 circuits with running time
> `poly_epsilon(s+u+1) 2^((1/4+epsilon)(s+1))` for every fixed
> `epsilon > 0`, using exponential space. This is a derived application of
> established tensor elimination and cubic-layout methods; the proof tracks
> the circuit-specific budget `kappa <= s+1-b` through count-preserving
> reductions. Nurk's full preprint proves the branching decision bound
> `O(2^(0.4058s))`, and Lialina presents Savinov's improvement to
> `O^*(2^(0.389667s))`. The exponent in our bound is smaller than those
> displayed decision exponents, but our focused source comparison does not
> establish priority or the current best unrestricted-space running time.
> In particular, a difference in space usage alone does not explain the
> historical comparison. The exact combined statement may be a previously
> known consequence of width-based methods.

Suggested contribution-list wording:

> A quantitative application of tensor elimination to the consistency graph,
> including an explicit uniform counting corollary. Priority of the combined
> numerical statements is unestablished.

Keep the established width/contraction citations adjacent to this wording.
Do not describe tensor counting, equality-tree factorization, or cubic
pathwidth as original ingredients. Do not state that all three historical
full-text gaps are closed. The finished work should report the narrowed
source gaps as limitations of the comparison, rather than a gap in the
present proof or evidence of novelty.

## Retrieval record

Working files were retained under `/tmp/circuit-sat-final-sources/`, including
the institutional Nurk index, compressed and decompressed Russian preprint,
original and encoding-repaired text extraction, the rendered theorem page,
and full Fomin--Hoie, Chen--Kabanets, and Dudek--Duenas-Osorio--Vardi PDFs.
The Russian PDF is 540170 bytes; its compressed source is 459601 bytes.
The text needed a Latin-1-to-Windows-1251 repair for the original Russian
pages; the rendered theorem verified the numerical extraction independently.
No researchers were contacted, and nothing was published.
