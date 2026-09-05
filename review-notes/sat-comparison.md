# Uniform CircuitSAT and counting comparison

Focused primary-source follow-up, 2026-09-04.
This note accompanies the [structural proof audit](structural-audit.md)
and the [literature statement matrix](../research/literature/statement-matrix.md).
It separates the manuscript's derived algorithmic consequence from claims of priority.

## Finding

The checked literature does not resolve the priority of the resulting gate-count bound.
The structural proof gives, for every fixed `epsilon>0`, a uniform deterministic
CircuitSAT and #CircuitSAT algorithm with time

    O_epsilon^*(2^((1/4+epsilon)(s+1))).

The immediate implementation uses exponential space.
This is a proved consequence under the current manuscript arguments, with
its uniform counting proof in [the counting corollary](../sections/counting-algorithm.tex).
It uses the compiler and the constructive Fomin--Hoie theorem, as reviewed
in the [structural audit](structural-audit.md#s1-gap-the-constructive-sat-consequence-needs-an-explicit-boundary).
It does not rely on finite compiler experiments or on a truth-table synthesis oracle.

Lialina's reproduced Savinov algorithm uses the same full binary basis,
unrestricted fanout, and internal-gate size measure. Its stated decision/search
time is `O^*(2^(0.389667s))`. The full paper contains no explicit space restriction.
Its recursive construction admits polynomial-space depth-first execution;
that is an implementation inference, not a qualification in its theorem statement.
The space difference therefore does not, on its own, explain the historical
running-time comparison. [lialina-sat][lialina-sat]

Established tensor contraction and width-based CircuitSAT methods overlap with
the compiler framework, and subcubic width conversion is established graph theory.
No inspected primary statement supplies the exact gate-minus-input exponent or
the resulting `1/4` gate coefficient. This bounded finding establishes neither
novelty nor a defect in the reviewed proof.

## The derived statement and its quantifiers

Let `s` count internal gates over `B2`, the sixteen Boolean functions of two inputs.
Let `b` count distinct input variables retained in the output cone after normalization.
Input occurrences share their variable value and may have arbitrary fanout.
Track the original declared inputs separately when counting assignments.

Quantify all retained inputs in the structural compiler.
The proof gives cycle rank `kappa<=s+1-b` and, for every fixed `delta>0`,
a constructible contraction with at most

    poly_delta(s) * 2^((1/3+delta)*kappa)

operations. The polynomial degree and multiplicative constant may depend on `delta`.
Fomin--Hoie Theorem 5 gives the subcubic pathwidth coefficient `1/6+eta`;
Section 4 gives a polynomial-time construction for fixed `eta`.
The manuscript's cubic kernel has at most `2(kappa-1)` vertices,
and its cutwidth conversion adds at most two. [fomin-pathwidth][fomin-pathwidth]

The cutwidth/pathwidth relation is not a new ingredient:
Makedon--Sudborough Corollary 3.3 and Ellis--Sudborough--Turner Theorem 2.1
imply `cw<=pw+2` for subcubic graphs. Bodlaender et al. reproduce this
combination in the proof of Lemma 16. Original bibliographic identities and
that recent primary proof were checked; the two older complete papers were
not both retrieved. [bodlaender-cubic][bodlaender-cubic]

With `alpha=1/3+delta`, running either the compiler or exhaustive assignment
enumeration according to the smaller certified exponent gives

    T(s,b) <= poly_delta(s) * 2^min{b, alpha*(s+1-b)}
           <= poly_delta(s) * 2^((alpha/(1+alpha))*(s+1)).

Indeed, the increasing term `b` and decreasing term `alpha*(s+1-b)`
intersect at `b=alpha*(s+1)/(1+alpha)`. The coefficient is

    alpha/(1+alpha) = (1+3*delta)/(4+3*delta)
                   = 1/4 + 9*delta/(16+12*delta).

This proves the fixed-epsilon statement above. Writing a single uniform
algorithm as `O^*(2^(s/4+o(s)))` additionally requires an effective schedule
or dovetailing of the fixed-epsilon constructions. Merely taking the infimum
of their exponents does not supply that schedule. The fixed-epsilon version
avoids this extra implementation assertion.

For #CircuitSAT use nonnegative integer addition and multiplication.
Each input assignment uniquely determines every gate value.
A tree replacing a high-degree equality constraint has a unique internal
extension when its external values agree. Tensor contractions sum the actual
internal indices, so loops, parallel edges, and degree reductions preserve
multiplicity when implemented with the stated contraction identities.
Every intermediate entry counts assignments to only `O(s)` binary indices,
so its integer representation has `O(s)` bits. Arithmetic costs add a polynomial
factor. If `n-b` declared variables are unused, multiply by `2^(n-b)` at the end;
the time bound then has a polynomial factor in the original input size.

For any fixed gate/input ratio below four, the derived bound consequently gives
a nontrivial exponential saving over `2^b` after choosing sufficiently small
fixed `delta`. This makes comparison with the published near-`3b` counting
benchmarks substantive. The manuscript should state this consequence explicitly.

## Source-by-source comparison

| Primary source and checked location | Model and output | Actual parameter/result | Space and overlap |
| --- | --- | --- | --- |
| [Broering--Lokam][broering-width], SAT 2003 proceedings published 2004, pp.162-163 preview | Bounded-fanin CircuitSAT; the introductory results explicitly remove dependence on fanout | Algorithms exponential in circuit graph width; abstract gives polynomial time at logarithmic cutwidth/pathwidth | Establishes earlier width-based CircuitSAT. Only the first two primary pages were retrieved, so no exact general-width coefficient or space theorem is attributed here. |
| [Markov--Shi][markov-tensor], Proposition 4.2 and Theorem 4.6 | Bounded-arity tensor networks / strong quantum-circuit simulation | Contraction complexity equals treewidth of the line graph; a supplied decomposition yields an order; simulation time is polynomial in network size times exponential in width | Established contraction framework. The statement does not preserve a numerical coefficient measured against the manuscript's `B2` gate count. |
| [Johnson et al.][johnson-search], sections on geometric contraction and Boolean counters | Search/counting through tensor networks representing checking circuits | Exact Boolean gate constraints and efficient contraction on suitable bounded-width networks | Direct prior overlap with SAT as tensor contraction; no inspected `s-b` or `s/4` theorem. |
| [Nurk][nurk-sat], POMI preprint 10/2009 institutional abstract | CircuitSAT decision; abstract measures internal gates `s` | `O(2^(0.4058s))` | Full text not retrieved, so model details and space contract are not certified from this source. Lialina identifies it as the preceding branching bound. |
| [Lialina][lialina-sat], original 2018 paper, Section 2 pp.123-124 and Section 5 pp.130-133 | Acyclic circuits, all sixteen binary functions, unrestricted fanout, internal-gate count; outputs a satisfying assignment or rejection | Reproduces Savinov's `O^*(2^(0.389667s))` general CircuitSAT algorithm | Decreasing-size recursive branching permits polynomial-space DFS. Neither Section 5 nor the full paper states that arbitrary competing algorithms must use polynomial space. |
| [Lialina][lialina-sat], Section 6 and Theorem 3 pp.133-135 | Same circuits promised to have at most one satisfying assignment | `O^*(2^(0.374589s))` Unique CircuitSAT | Promise decision/search result. The general simplifications do not establish an unrestricted #SAT algorithm. |
| [Golovnev--Kulikov--Smal--Tamaki][gkst-elimination], full ECCC report, Section 2.3 and Theorem 4 | `B2` and `U2` circuits; counts satisfying assignments | Nontrivial counting below `3b` gates for `B2` and below `3.25b` for `U2`; advertised examples are `2.99b` and `3.24b` | The recursive construction also permits polynomial-space DFS. No explicit space hypothesis occurs in the inspected full report. These are gate/input-ratio results, not the same statement as a gate-only exponent. |
| [Allender et al.][allender-space], Remark 1.2 and Section 4.3 | CNF SAT with an appropriate supplied decomposition | Width-parameterized time/space tradeoffs; e.g. Theorem 4.9 has time `3^(1.441(1-epsilon)*TW*log\|phi\|) poly(\|phi\|)` and space `2^(2*epsilon*TW) poly(\|phi\|)` | Shows why exponential-space dynamic programming and polynomial-space branching are distinct resource regimes. It does not identify the manuscript's exact gate-count consequence or settle its priority. |
| [Lokshtanov et al.][lokshtanov-treewidth], Theorem 1 and introductory remarks | Counts satisfying assignments of circuits; its width graph deletes all input vertices | With `b` inputs, at most `r*b` distinct input successors and `omega=tw(C-I)=o(log b)`, time `2^((1-epsilon/10)*b) poly(\|C\|)`, where `epsilon=1/(48*r*(omega+1)*4^omega)` | The paper explicitly describes exponential space. Its input-deleted width differs from a tensor graph retaining consistency at every input; it is not an unrestricted gate-only result. |
| [Hoza--Lv][hoza-trees], 2025 revision 1, Section 1.4 | Pseudorandomness for small circuits, with comparisons to previous SAT algorithms | Cites Savinov's `0.389667s` and GKST's `2.99b` `B2` #SAT benchmark | Confirms these remain visible benchmarks in recent primary research. It is not a proof that no better unrestricted-space bound exists. |

The source links and complete metadata appear below.

## What the full Lialina text resolves

The original POMI paper is in English, published in volume 475 (2018),
pp.122-136. The later Journal of Mathematical Sciences version is volume
247(3) (2020), pp.457-466. The complete original PDF was retrieved from
the institute's public server and checked, including Sections 2, 5, 6 and
the references. [lialina-sat][lialina-sat]

The model is not restricted to formulas, bounded fanout, or `U2`.
There is one designated output and the size is the number of internal gates.
Savinov's bound therefore concerns the same general circuit class as the
derived structural algorithm.

Section 5 presents the algorithm rather than merely citing its numerical bound.
It branches on variables or gate values, simplifies, and analyzes a tree of
recursive calls by decreases in circuit size. Polynomial-time work at a node
and polynomial recursion depth permit sequential depth-first execution using
polynomial space. This conclusion follows from the construction; the words
"space" and "memory" do not occur in the full extracted paper text.

Section 2's simplification contract requires preservation of satisfiability,
recoverability of a satisfying assignment, and no increase in the number
of solutions. It does not require equality of solution counts.
Consequently Section 5 must not be cited as an unrestricted #CircuitSAT theorem.
The manuscript's counting claim instead follows from its own exact tensor
identities. GKST is the relevant inspected general counting comparator.
[gkst-elimination][gkst-elimination]

The source credits Savinov as an MSc thesis at St. Petersburg Academic
University RAS, 2014. Its title and identity are verified in Lialina's
reference 10 and Hoza--Lv's bibliography. The thesis itself was not located
at a verified public URL, so its separate wording and any additional results
remain unchecked. Lialina is a primary presentation of the reproduced algorithm,
and is sufficient to verify the bound and circuit model used here.

## What the width literature resolves

Broering--Lokam's primary preview already precludes an explanation based
solely on unrestricted fanout: page 163 expressly describes removal of
fanout dependence from a CircuitSAT width bound. The same page introduces
an improved dependence on pathwidth. Its abstract treats logarithmic
cutwidth/pathwidth and treewidth/branchwidth separately.
[broering-width][broering-width]

Markov--Shi explicitly situate tensor simulation beside existing CircuitSAT
width algorithms and variable elimination. Generalized distributive-law
elimination and contraction are therefore established antecedents of the
compiler, including its arithmetic version. [markov-tensor][markov-tensor]
[johnson-search][johnson-search]

The sharp gate coefficient requires the additional circuit-specific accounting:
preserving input consistency, bounding cycle rank by `s+1-b`, removing
degree-one/two portions without increasing tensor rank, and retaining numerical
constants through the cubic kernel and layout. A generic `2^O(width)` theorem
does not settle that coefficient. An equivalent composition of prior results
may still exist; the present audit has not identified one.

## Bounded search and remaining limits

The follow-up inspected the primary sources above and ran twelve targeted
queries around CircuitSAT with exponential space, gate coefficients `1/4`
and `0.25`, gate-minus-input parameters, cyclomatic number, width algorithms,
Fomin--Hoie, and tensor/Holant contraction. It also followed Lialina's
references to Savinov and Nurk. This was a focused comparison, not an
exhaustive citation-graph or multilingual literature review.

A subsequent targeted pass followed Calabro's 2009 thesis to its related
conference paper and checked the complete tensor-rewriting paper discussed below.
This additional pass did not identify a prior theorem with the exact numerical bound.

Full-text gaps are Broering--Lokam beyond the two-page publisher preview,
Savinov's thesis, and Nurk's preprint. Nurk's verified institutional index
still links an English compressed PDF, but that URL returned HTTP 404.
These gaps prevent claiming that all possibly relevant statements in those
works have been ruled out. Primary work from 2025 provides recent context,
but this audit does not certify a September 2026 best-known running time.

No source inspected states the exact `poly(s)*2^((s-b)/3+o(s-b))` algorithm
or its balanced `1/4` coefficient. Absence from this search is not evidence
of priority. Nor does the existence of a weaker published algorithm logically
contradict a faster valid construction: both bounds can hold.

Recommended manuscript wording is: the proved construction implies the
fixed-epsilon uniform SAT/#SAT bound, with exponential space; its exact
relationship to the unrestricted-space gate-count literature and its priority
remain unestablished. Attribute the established width and contraction
ingredients, and describe the numerical result as a derived consequence.
Do not call it the first, fastest, or best-known CircuitSAT algorithm on the
basis of this audit, and do not say that space alone resolves the comparison.

## Follow-up: the approximate 4n remark and tensor rewriting

Calabro's thesis, printed p.78, mentions older circuit-size algorithms losing
their advantage near `4n`, without a citation attached to that sentence.
Section 8.2.1, p.80, defines the chapter's own model as unrestricted-fanin
AND/OR gates with literal inputs and one output, counting noninput gates.
That constant-depth model differs from `B2`. The thesis bibliography alone
does not identify an exact `1/4` theorem. [calabro-thesis][calabro-thesis]

The later Calabro--Impagliazzo--Paturi author version resolves the reference
trail: Section 1.1, PDF p.2, identifies the baseline as converting a fanin-two
circuit to 3-SAT and applying a 3-SAT solver. It then credits an improved
communicated bound to Sergey "Nurik". Reference `[Nur09]`, PDF p.11, gives
only a 2009 personal communication expected to appear in ECCC, with no report
number or URL. The passage supplies no derivation of its approximate `4n`
threshold and no counting, cycle-rank, or cubic-layout bound.
[cip-small-depth][cip-small-depth]

Thus the numeral `4` in that remark is not a verified match to the manuscript's
coefficient. The identified comparison is a 3-SAT reduction, not a documented
instance of the present tensor construction. The communication's precise
relationship to Nurk's verified POMI preprint remains unchecked.

De Beaudrap--Kissinger--Meichanetzidis, Theorem 2.1, represent a CNF counting
instance by a closed ZH diagram whose scalar is its solution count.
Section 3.2 expressly obtains decision from counting by changing from natural
numbers to the Boolean semiring. Sections 2.3 and 4 analyze diagram rewriting:
some tractable classes simplify efficiently, while general rewrites can increase
diagram complexity exponentially. This supplies direct antecedents for the
decision/counting interpretation and exact tensor rewrites. The complete
fourteen-page published text contains no theorem parameterized by circuit
gate-minus-input count, no cubic kernel or Fomin--Hoie argument, and no `1/4`
gate exponent. It therefore does not resolve the numerical priority question.
[beaudrap-rewriting][beaudrap-rewriting]

## COPY removal and planar contraction: two further primary comparisons

Biamonte--Morton--Turner, arXiv v2 Theorem 17 (p.9), gives
`O((g+c*d)^O(1)*2^c)`, where `g` counts gates, `c` counts COPY tensors,
and `d` is their maximum degree. Removing one COPY tensor gives two
terms, fixing every incident value consistently; removing all gives `2^c`
tree contractions. Remark 18 explicitly restricts the theorem to the
canonical Boolean-state representation in Section 2.1.
Corollary 19 uses `O(log n)` COPY tensors and polynomial degree for
polynomial-time counting. The abstract's `O(log c)` is inconsistent with
this precise statement and should not be repeated. Proposition 22 gives
`treewidth<=c` in that representation. [biamonte-copy][biamonte-copy]

**Comparison inference:** the mechanism behind the manuscript's original
forest-cut simulation is established COPY expansion followed by tree evaluation.
It should be attributed accordingly. The exact mixed-input theorem is not a
literal instance of their stated theorem: it keeps ordinary inputs symbolic,
counts shared witnesses and internal gate outputs, and derives
`r<=s+1-a-b` for `B2`. Retaining each cut gate's defining equation enforces
the same consistency as retaining its gate tensor after COPY removal.
The symbolic lift and the circuit-specific bound on `r` are applications and
accounting steps; they do not make the copying-to-trees principle original.
The inspected paper supplies no excess coefficient `1/3` or gate coefficient `1/4`.

Kourtis--Chamon--Mucciolo--Ruckenstein, Section 2.2 Corollaries 1--2,
proves a planar contraction bound: for `N` tensors, constant initial maximum
rank `Delta` and index dimension `D`, time is `D^O(sqrt(Delta*N))`.
The contraction graph counts tensor vertices, not `B2` gates.
For general cubic graphs, Section 3 evaluates greedy, METIS, and community
heuristics on sampled counting instances. Fitted runtime curves are empirical;
they are not worst-case exponential coefficients. Section 3.1.1 explicitly
allows exponential space. Section 3.2 reports that fixed-precision counts
for larger cubic-vertex-cover instances are approximate due to rounding.
Section 4 also notes exact rank-three ring decompositions of COPY tensors.
No gate-minus-input or Fomin--Hoie coefficient is established there.
[kourtis-counting][kourtis-counting]

**Comparison inference:** replacing a degree-`D` vertex by a `D`-vertex COPY
ring adds `D-1` vertices and `D` edges, increasing cycle rank by one.
The manuscript's equality tree adds equally many edges and vertices and thus
preserves cycle rank. Exact rank reduction is established; preserving this
particular budget remains essential to the manuscript's numerical derivation.

## Local References

[fomin-pathwidth]: https://doi.org/10.1016/j.ipl.2005.10.012 "Fedor V. Fomin and Kjartan Hoie. Pathwidth of cubic graphs and exact algorithms. Information Processing Letters 97(5), 191-196, 2006. DOI:10.1016/j.ipl.2005.10.012. Full author PDF: https://fedorvf.github.io/articles/2006/2006b.pdf."
[bodlaender-cubic]: https://doi.org/10.37236/13205 "Hans L. Bodlaender, Edouard Bonnet, Lars Jaffke, Dusan Knop, Paloma T. Lima, Martin Milanic, Sebastian Ordyniak, Sukanya Pandey, and Ondrej Suchy. Treewidth is NP-Complete on Cubic Graphs. Electronic Journal of Combinatorics 32(3), P3.36, 2025. DOI:10.37236/13205. Lemma 16 proof cites Fillia Makedon and Ivan Hal Sudborough, On minimizing width in linear layouts, Discrete Applied Mathematics 23(3), 243-265, 1989, DOI:10.1016/0166-218X(89)90016-4, Corollary 3.3; and John A. Ellis, Ivan Hal Sudborough, Jonathan S. Turner, The vertex separation and search number of a graph, Information and Computation 113(1), 50-79, 1994, DOI:10.1006/inco.1994.1064, Theorem 2.1. Full published PDF: https://www.combinatorics.org/ojs/index.php/eljc/article/download/v32i3p36/pdf/."
[broering-width]: https://doi.org/10.1007/978-3-540-24605-3_13 "Elizabeth Broering and Satyanarayana V. Lokam. Width-Based Algorithms for SAT and CIRCUIT-SAT (Extended Abstract). In Theory and Applications of Satisfiability Testing, SAT 2003, Lecture Notes in Computer Science 2919, 162-171, published 2004. DOI:10.1007/978-3-540-24605-3_13. First two primary pages inspected: https://page-one.springer.com/pdf/preview/10.1007/978-3-540-24605-3_13."
[markov-tensor]: https://arxiv.org/abs/quant-ph/0511069 "Igor L. Markov and Yaoyun Shi. Simulating quantum computation by contracting tensor networks. SIAM Journal on Computing 38(3), 963-981, 2008. DOI:10.1137/050644756. Author preprint arXiv:quant-ph/0511069, version 7 inspected."
[johnson-search]: https://doi.org/10.1038/srep01235 "T. H. Johnson, J. D. Biamonte, S. R. Clark, and D. Jaksch. Solving search problems by strongly simulating quantum circuits. Scientific Reports 3, article 1235, 2013. DOI:10.1038/srep01235. Full primary text inspected."
[nurk-sat]: https://www.pdmi.ras.ru/preprint/2009/09-10.html "Sergey Nurk. An O(2^{0.4058m}) upper bound for Circuit SAT. POMI preprint 10/2009, accepted December 1, 2009. Institutional abstract and identity verified; full text not retrieved. The institute's original HTTP index was accessible during this audit."
[lialina-sat]: https://www.mathnet.ru/eng/znsl6688 "A. A. Lialina. On the complexity of unique Circuit SAT. Zap. Nauchn. Sem. POMI 475, 122-136, 2018; original article in English. Later version: Journal of Mathematical Sciences 247(3), 457-466, 2020, DOI:10.1007/s10958-020-04813-1. Complete original primary PDF inspected at http://ftp.pdmi.ras.ru/pub/publicat/znsl/v475/p122.pdf. Reference 10 identifies S. V. Savinov, Upper bound for Circuit SAT, MSc thesis, St. Petersburg Academic University RAS, 2014; thesis URL unknown."
[gkst-elimination]: https://doi.org/10.1016/j.tcs.2017.11.008 "Alexander Golovnev, Alexander S. Kulikov, Alexander V. Smal, and Suguru Tamaki. Gate elimination: Circuit size lower bounds and #SAT upper bounds. Theoretical Computer Science 719, 46-63, 2018. DOI:10.1016/j.tcs.2017.11.008. Conference version: Circuit Size Lower Bounds and #SAT Upper Bounds Through a General Framework, MFCS 2016, LIPIcs 58, article 45, DOI:10.4230/LIPIcs.MFCS.2016.45. Full author report inspected: https://eccc.weizmann.ac.il/report/2016/022/download/."
[allender-space]: https://doi.org/10.4086/toc.2014.v010a012 "Eric Allender, Shiteng Chen, Tiancheng Lou, Periklis A. Papakonstantinou, and Bangsheng Tang. Width-Parameterized SAT: Time-Space Tradeoffs. Theory of Computing 10(12), 297-339, 2014. DOI:10.4086/toc.2014.v010a012. Full published PDF: https://theoryofcomputing.org/articles/v010a012/v010a012.pdf."
[lokshtanov-treewidth]: https://doi.org/10.1137/1.9781611975031.18 "Daniel Lokshtanov, Ivan Mikhailin, Ramamohan Paturi, and Pavel Pudlak. Beating Brute Force for (Quantified) Satisfiability of Circuits of Bounded Treewidth. SODA 2018, 247-261, 2018. DOI:10.1137/1.9781611975031.18. Full author PDF: https://sites.cs.ucsb.edu/~daniello/papers/boundedTreewidthCircuitSatSODA18.pdf."
[hoza-trees]: https://eccc.weizmann.ac.il/report/2025/003/revision/1/ "William M. Hoza and Zelin Lv. Fooling Near-Maximal Decision Trees. Electronic Colloquium on Computational Complexity TR25-003, revision 1, 2025. Full revision inspected; its Section 1.4 supplies recent primary context, not a priority certificate."
[calabro-thesis]: https://cseweb.ucsd.edu/~ccalabro/thesis.pdf "Chris Calabro. The Exponential Complexity of Satisfiability Problems. PhD dissertation, Computer Science, University of California, San Diego, 2009. Full primary thesis retrieved; Chapter 8, preliminaries, and bibliography checked."
[cip-small-depth]: https://doi.org/10.1007/978-3-642-11269-0_6 "Chris Calabro, Russell Impagliazzo, and Ramamohan Paturi. The Complexity of Satisfiability of Small Depth Circuits. IWPEC 2009, Lecture Notes in Computer Science 5917, 75-85, 2009, as dated by the publisher. DOI:10.1007/978-3-642-11269-0_6. Author version: https://cseweb.ucsd.edu/~paturi/myPapers/pubs/CalabroImpagliazzoPaturi_2009_iwpec.pdf; Section 1.1 and bibliography inspected."
[beaudrap-rewriting]: https://doi.org/10.4204/EPTCS.340.3 "Niel de Beaudrap, Aleks Kissinger, and Konstantinos Meichanetzidis. Tensor Network Rewriting Strategies for Satisfiability and Counting. QPL 2020 proceedings, Electronic Proceedings in Theoretical Computer Science 340, 46-59, 2021. DOI:10.4204/EPTCS.340.3. arXiv:2004.06455v2, revised September 6, 2021; complete published text inspected."
[biamonte-copy]: https://arxiv.org/abs/1405.7375 "Jacob D. Biamonte, Jason Morton, and Jacob W. Turner. Tensor Network Contractions for #SAT. Journal of Statistical Physics 160(5), 1389-1404, 2015. DOI:10.1007/s10955-015-1276-z. Full primary arXiv:1405.7375v2, September 26, 2014, inspected; theorem numbers refer to that version."
[kourtis-counting]: https://arxiv.org/abs/1805.00475 "Stefanos Kourtis, Claudio Chamon, Eduardo R. Mucciolo, and Andrei E. Ruckenstein. Fast counting with tensor networks. SciPost Physics 7(5), article 060, 2019. DOI:10.21468/SciPostPhys.7.5.060. Full primary arXiv:1805.00475v2, November 12, 2019, inspected; arXiv identifies it as the published version."
