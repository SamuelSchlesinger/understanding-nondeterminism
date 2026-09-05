# Exact-statement comparison matrix

Reviewed statement matrix, 2026-09-04. The [parent audit](index.md) states the proved claims and remaining priority limits.
Each row keeps the source's representation and parameter.
"Known consequence" can mean a short derivation from a source; such derivations are identified.
An unverified full text is recorded as a gap, never silently promoted to theorem evidence.

## Elimination and knowledge compilation

| Source and locator | Input, output, parameter, conclusion | Constructivity | Overlap and remaining claim |
| --- | --- | --- | --- |
| Dechter 1999, Theorems 1-2; Theorem 14 and surrounding discussion. [dechter-bucket][dechter-bucket] | Finite-domain constraints and related reasoning tasks; elimination order with induced width `w*`. Adaptive consistency decides consistency and yields a backtrack-free representation in `O(n exp(w*))` time and space. Theorem 14 concerns conditioned most-probable-explanation: space exponential in conditioned induced width, time additionally exponential in the conditioned-set size. | Explicit bucket algorithms. Finding a favorable order is distinct from executing it. | Width-based elimination and conditioning hybrids are known. Boolean symbolic circuit output is a specialization, not literally the output of these theorems. Exact gate accounting remains manuscript work. |
| Aji-McEliece 2000, Sections II-III, Theorem 3.1, Section V. [aji-gdl][aji-gdl] | Local kernels on finite variable scopes valued in an arbitrary commutative semiring; compute marginals of their product using a junction tree. Complexity counts semiring additions/multiplications, with table sizes determined by domains and bags. | Explicit message updates and scheduling correctness. Requires the factorization and valid junction tree. | Taking the semiring of Boolean functions of free inputs makes quantified-only symbolic elimination a known consequence. The exact `B2` cost and graph reduction are applications; the symbolic principle is subsumed. |
| Darwiche-Marquis 2002, Proposition 5.1/Table 7 and Appendix proofs. [darwiche-map][darwiche-map] | Knowledge-representation languages; forgetting selected variables is polynomial for DNNF. Determinism is an extra syntactic condition and is not automatically retained by forgetting. | A polynomial transformation on the explicit compiled representation. | Existential projection after DNNF compilation is established. Ordinary deterministic Boolean circuits do not require disjoint OR children; d-DNNF restrictions cannot be imposed on the manuscript target. |
| Amarilli-Monet-Senellart 2018, Theorem 5. [amarilli-icdt][amarilli-icdt] | AND/OR/NOT circuit and a supplied tree decomposition of width `k` of the full circuit graph; construct equivalent d-SDNNF in time `O(|T| 2^{(4+epsilon)k})` for fixed `epsilon>0`. | Constructive compilation; decomposition supplied. | Supplies full-graph width-based compilation before forgetting. The precise numerical constant does not survive arbitrary `B2` basis encoding automatically. |
| Amarilli-Capelli-Monet-Senellart, published 2020, Theorems 4.2 and 4.4. [amarilli-width][amarilli-width] | Theorem 4.2: supplied full-circuit tree decomposition width `k`, complete extended d-SDNNF of width at most `2^{2(k+1)}`, time `O(|T|2^{(4+epsilon)k})`. Theorem 4.4: a path decomposition gives a complete unambiguous OBDD, analogous width, time `O(|P|2^{(2+epsilon)k})`. | Explicit representation-producing algorithms. Circuit basis and representation conventions are those in the paper. | The journal version is not a three-author 2019 paper: Capelli is an additional author, online date 2019, volume date 2020. Full-width projection is a consequence with forgetting. Quantified-only circuit size can exploit a different parameter. |
| Capelli-Mengel 2019, Theorem 5. [capelli-qbf][capelli-qbf] | Complete structured DNNF `D` of width `w` and variable set `Z`; construct a complete structured d-DNNF of width at most `2^w`, with designated gates for `exists_Z D` and its complement, in `2^{O(w)}|D|` time. | Explicit compilation on a supplied structured representation. | The extra exponential preserves determinism and supports negation. It does not refute linear forgetting into DNNF or small unrestricted circuits. The published title is Tractable QBF by Knowledge Compilation; the 2018 preprint had a different title. |
| Capelli-Mengel 2019, Theorem 10 and counting corollary. [capelli-qbf][capelli-qbf] | Quantified CNF with free variables, `ell` quantifier blocks, incidence treewidth `k`; compilation into a structured deterministic representation in `exp_(ell+1)(O(k))*|F|` time, supporting the associated counting task. | Uniform from the explicit formula and structural machinery. | Alternation and retained deterministic structure are major differences. A single existential block with unrestricted Boolean output does not inherit the tower requirement. |

## Coverage, sparse correction, and nondeterminism

| Source and locator | Input, output, parameter, conclusion | Constructivity | Overlap and remaining claim |
| --- | --- | --- | --- |
| Wang-Xu-Du-Wu 2015, publisher abstract; full text unavailable. [wang-penalties][wang-penalties] | Set cover with submodular selection cost and either linear or submodular penalties. Abstract states approximation factors `eta` and `2*eta`, where `eta` is the maximum number of sets containing any element. | Primal-dual algorithms stated; complete hypotheses/oracle conventions not checked. | The abstract optimization class contains the manuscript's tagged-restriction coverage cost. It does not by itself contain the circuit-realization theorem or prove a succinct-input algorithm. Do not import an approximation factor until the full hypotheses are checked. |
| Redkin 2004, Russian original, Theorems 1-2 and introduction. [redkin-sparse][redkin-sparse] | Functions on `n` inputs with `k` ones, in `U2`; for `k` below `log n-c log log n` with fixed `c>1`, derives refined linear asymptotics, and worst-case complexity asymptotic to `2n` for `k>=2` in that range. | Synthesis proofs from the support matrix. Exact leading terms depend on repeated column types and the basis. | Sparse synthesis is established and can beat the draft's `O(nR/log R)` at very small support. Its `U2` lower constants are not `B2` lower constants. The paper identifies earlier Finikov/Lupanov ancestry, not priority for the draft's exact convenient formula. |
| Redkin 2020 Russian/2021 English, Theorem 2. [redkin-implementation][redkin-implementation] | Same small-support range, basis AND/NOT (or OR/NOT) with operation weights `p1,p2`; worst-case complexity asymptotic to `(p1+2p2)n`. Unit weights give a `3n` asymptotic upper bound. | Explicit circuit construction and matching analysis in the specified weighted basis. | Again establishes `O(n)` correction in a substantial small-support range. The manuscript may use a simpler bound, but should not call it uniformly sharp. Upper bounds transfer to `B2`; lower bounds do not. |
| Morizumi 2015, Theorem 2. [morizumi-parity][morizumi-parity] | Nondeterministic `U2` circuits with unrestricted guess count computing parity on `n` ordinary inputs need exactly `3(n-1)` gates, matching deterministic complexity in that basis. | An explicit function and lower-bound proof, not a projection compiler. | This proves one function gains nothing from nondeterminism. It supplies neither a general exponential separation nor an exponential deterministic lower bound for projected small verifiers. |
| Morizumi 2019 v2, Theorems 3-4 and Section 2 conventions. [morizumi-width][morizumi-width] | Theorem 3 gives a `U2` function with nondeterministic size `2n+o(n)` and deterministic size `3n-o(n)`. Theorem 4 simulates size-`s`, layered-width-`w` nondeterministic circuits by deterministic circuits of size `2^{O((w+log s)log s)}` and width `w+O(log s)`. Guess labels occur at most once as input nodes. | Explicit recursive simulation. Width is measured after layering, using COPY gates as needed. | A genuine prior structural nondeterministic simulation. Its layered width and exponent differ from quantified-variable elimination width. It is neither a general lower-bound separation nor a theorem about the manuscript's graph excess. |
| Cavalar-Oliveira 2025, Section 1.2, Equation (1), Sections 3 and 4. [cavalar-cover][cavalar-cover] | Discrete complexity from set unions/intersections, a cover graph on pairs and filters, and cover complexity `rho`; proves `rho <= D_intersection <= rho^2` and develops deterministic/nondeterministic circuit connections. | Combinatorial characterizations and transference of lower bounds. | This is important nearby circuit-cover literature, but its selectable objects are not verifier witnesses covering accepting ordinary inputs. No direct implication of the occupancy/alteration upper bound was found in the inspected statements. |

## Graph structure and tensor algorithms

| Source and locator | Input, output, parameter, conclusion | Constructivity | Overlap and remaining claim |
| --- | --- | --- | --- |
| Fomin-Hoie 2006, Theorem 5 and Section 4. [fomin-pathwidth][fomin-pathwidth] | For each fixed `epsilon>0`, all sufficiently large `N`-vertex graphs of maximum degree at most three satisfy `pw <= (1/6+epsilon)N`. | The paper explicitly explains polynomial-time construction for fixed `epsilon`. The threshold and polynomial can depend on `epsilon`. | The `1/6` source of the candidate excess exponent is known. A uniform `o(N)` algorithm needs a careful diagonalization or an explicit uniform bound; the safest immediate algorithmic statement is fixed `epsilon`. |
| Makedon-Sudborough 1989, Corollary 3.3 as restated in the next source. [makedon-layout][makedon-layout] | Finite undirected subcubic graphs: cutwidth equals edge-search number. | Publisher abstract inspected; original proof not retrieved. | Half of the known `cw <= pw+2` conversion. The author should not claim priority for this conversion after independently reproving it. |
| Ellis-Sudborough-Turner 1994, Theorem 2.1 as restated in the next source. [ellis-search][ellis-search] | Search number lies between vertex separation and vertex separation plus two. Vertex separation is pathwidth under the standard convention. | Earlier institutional technical-report abstract and exact later attribution checked; journal full text not retrieved. | The other half of `cw <= pw+2`. The additive two and the type of graph search matter; do not substitute node-search or a different width convention. |
| Bodlaender et al. 2025, proof of Lemma 16. [bodlaender-cubic][bodlaender-cubic] | Explicitly uses both preceding facts to compare pathwidths of cubic graphs and line graphs of their subdivisions. | Full published proof inspected. | A primary-paper restatement verifies the precise combined inequality while original-proof access remains incomplete. This source does not claim the manuscript's excess-to-projection theorem. |
| Markov-Shi 2008, Proposition 4.2 and Theorem 4.6 in author version 7. [markov-tensor][markov-tensor] | Tensor-network contraction complexity equals treewidth of the line graph; a supplied line-graph decomposition yields a contraction order in polynomial time. A size-`T` bounded-arity quantum circuit has deterministic strong simulation time `T^{O(1)}exp(O(tw))`. | Explicit tensor construction and contraction. Numerical input representation/precision is part of the original quantum model. | General width-driven tensor elimination is established. Its hidden constants and local tensor dimensions do not directly certify base `2` or coefficient `1/3` for the manuscript's Boolean reduction. |
| Johnson-Biamonte-Clark-Jaksch 2013, "Contracting geometrically" and "Counters based on Boolean circuits." [johnson-search][johnson-search] | Uniform Boolean checking circuits are encoded as tensor counters; logarithmic treewidth gives polynomial-time counting and finding accepted witnesses via geometric contraction. | Explicit uniform gate-to-tensor construction and counting/search reduction. | Direct Boolean-to-tensor counting is established. This source states a tractability criterion, not the candidate gate-minus-input excess exponent. Symbolic free inputs require the semiring specialization recorded above. |
| Lokshtanov-Mikhailin-Paturi-Pudlak 2018, Theorem 1 and Section 2. [lokshtanov-treewidth][lokshtanov-treewidth] | Circuit with `n` inputs, `m` gates, at most `s*n` distinct successors of inputs, and `omega=tw(C-inputs)=o(log n)`; #SAT time `2^{n(1-epsilon/10)}m^{O(1)}`, where `epsilon=1/(48*s*(omega+1)*4^omega)`. The stated basis is unbounded-fanin De Morgan. | Uniform; the paper explicitly discusses exponential space, roughly `2^{n/2}`. | This parameter deletes all input vertices, unlike retaining quantified inputs as consistency vertices. Formulas have width one even with repeated labels. Arbitrary-basis conversion changes size and width; numerical constants cannot be copied directly. |
| Allender-Chen-Lou-Papakonstantinou-Tang 2014, Section 4.3; Theorem 4.9. [allender-space][allender-space] | CNF and supplied incidence-graph decomposition: polynomial-space SAT time `3^{tw*log|phi|}|phi|^{O(1)}`. Theorem 4.9 gives a spectrum of exponential-space/time bounds with an adjustable parameter. | Explicit algorithms; the decomposition is assumed supplied throughout. | Confirms that exponential-space width DP and polynomial-space branching have materially different guarantees. The theorem does not state a gate-count exponent or certify that previous Circuit-SAT gate benchmarks universally impose polynomial space. |
| Broering-Lokam SAT 2003, published 2004, publisher preview and reproduced chapter text through p.168. [broering-width][broering-width] | Lemma 4 displays time `O(n 2^(2W) (W+log n))` for the underlying AND/OR/NOT circuit cutwidth W. Theorem 2 displays exponent `2pw` for CNF primal pathwidth. | The publisher preview explicitly removes fanout dependence. Later formulas were retrieved as a public textual reproduction of the primary chapter, without page-image confirmation; pp.169-171 remain unavailable. | Prior width-based Circuit-SAT algorithms; these retrieved constants do not supply the manuscript's exact cubic-tensor time coefficient. The reduced tensors need not themselves be deterministic circuit gates. |

## Circuit-SAT comparison that remains open in this audit

| Source and locator | Input, output, parameter, conclusion | Constructivity and space | Relation to the candidate |
| --- | --- | --- | --- |
| Nurk 2009, full Russian institutional preprint, Theorem 3.1, printed p.8. [nurk-sat][nurk-sat] | General acyclic binary Boolean circuit with `s` internal gates: decision Circuit-SAT in `O(2^{0.4058s})` time. | Full branching construction inspected and theorem page rendered. Reduce step 7 is not count-preserving, so the stated decision procedure is not directly an exact counting algorithm. The linked English PDF remains unavailable. | A verified historical gate-count benchmark. The proved uniform consequence has a smaller gate exponent with exponential space; exact historical priority is unresolved. |
| Lialina 2018/2020, Section 2 pp.123-124 and Section 5 pp.130-133 of the original POMI paper. [lialina-sat][lialina-sat] | Unique Circuit-SAT in `O(2^{0.374589s})`; also gives Savinov's general Circuit-SAT algorithm in `O(2^{0.389667s})`. | Full institutional PDF verified in the focused follow-up. The model is unrestricted-fanout B2 with internal-gate size. No space restriction is stated; polynomial-space DFS is an implementation inference. | Do not confuse the unique-instance bound with the general bound. Savinov's result is from a 2014 master's thesis, reproduced here; the thesis itself was not inspected. |
| Golovnev-Kulikov-Smal-Tamaki 2016/2018, introduction and framework; full ECCC version. [gkst-elimination][gkst-elimination] | Faster-than-`2^n` deterministic #SAT for `B2` circuits with at most `2.99n` gates and `U2` circuits with at most `3.24n` gates. The general analysis uses circuit measures and paired substitutions eliminating a variable. | Recursive branching into two disjoint cases; source bounds recursion-tree size. Polynomial space is a depth-first implementation inference, not an explicit theorem condition found here. | The manuscript counting corollary proves count multiplicities and gives a nontrivial exponential-space algorithm at every fixed gate/input ratio below four. Exact priority remains unresolved. |
| Hoza-Lv 2025, Section 1.4. [hoza-trees][hoza-trees] | Constructs PRGs/hitting sets for smaller linear-size circuits, and explicitly compares with Savinov/Lialina and GKST's stronger SAT/#SAT tasks and size thresholds. | The paper's new results are explicit pseudorandom constructions; used here only as a recent primary contextual comparison. | Confirms these benchmarks remain actively cited. It neither proves no faster exponential-space algorithm exists nor settles novelty of the candidate. |

The discrepancy is unresolved: the candidate could be a known consequence omitted from these particular benchmark discussions,
or a quantitative improvement with a different space cost. Complete analytical
proofs and independent review checked the cubic reduction, loops and parallel
edges, repeated witness consistency, multiplicities, bit costs, decomposition
construction, and fixed-epsilon quantifiers. These reviews found no flaw, but
do not establish historical priority. The [focused comparison](../../review-notes/sat-comparison.md)
records the initial retrieval gaps; the [final comparison](final-comparison.md)
updates them after inspecting Nurk's full preprint and more of Broering--Lokam.

## Bibliographic corrections and retrieval boundary

The expanded Amarilli paper has four authors, including Florent Capelli; use its 2020 volume date and optionally record online publication in 2019.
Capelli-Mengel's published STACS title differs from its 2018 preprint title.
Redkin's 2020 Russian original and 2021 English translation are the same work with different journal pagination and DOIs.
Morizumi's bounded-width result was checked in the 2019 version of a preprint first posted in 2018.
Lialina's original POMI publication is already English in 2018; the Journal of Mathematical Sciences publication is 2020. Neither is Savinov's original thesis.
Fomin-Hoie's pathwidth result is an IPL 2006 article, DOI containing its 2005 acceptance-era identifier.
The graph-search reference is Makedon-Sudborough 1989, not Monien-Sudborough's different cutwidth NP-completeness paper.

Primary PDFs or author-deposited full texts were inspected for Dechter, Aji-McEliece,
Darwiche-Marquis, Amarilli, Capelli-Mengel, both Redkin papers, Fomin-Hoie,
Bodlaender et al., Markov-Shi, Johnson et al., Lokshtanov et al., GKST,
Morizumi, Cavalar-Oliveira, and (in the focused follow-ups) Lialina and Nurk.
Wang et al., the final pages of Broering-Lokam, Savinov's thesis, and the
original graph-search papers have the expressly noted full-text gaps.
No theorem is attributed to Lupanov's 1965 local-coding paper on the basis of having read it:
the historical pointer comes from Redkin's own introduction, which is not a substitute for checking the original.

[aji-gdl]: ../sources.md#aji-gdl
[allender-space]: ../sources.md#allender-space
[amarilli-icdt]: ../sources.md#amarilli-icdt
[amarilli-width]: ../sources.md#amarilli-width
[bodlaender-cubic]: ../sources.md#bodlaender-cubic
[broering-width]: ../sources.md#broering-width
[capelli-qbf]: ../sources.md#capelli-qbf
[cavalar-cover]: ../sources.md#cavalar-cover
[darwiche-map]: ../sources.md#darwiche-map
[dechter-bucket]: ../sources.md#dechter-bucket
[ellis-search]: ../sources.md#ellis-search
[fomin-pathwidth]: ../sources.md#fomin-pathwidth
[gkst-elimination]: ../sources.md#gkst-elimination
[hoza-trees]: ../sources.md#hoza-trees
[johnson-search]: ../sources.md#johnson-search
[lialina-sat]: ../sources.md#lialina-sat
[lokshtanov-treewidth]: ../sources.md#lokshtanov-treewidth
[makedon-layout]: ../sources.md#makedon-layout
[markov-tensor]: ../sources.md#markov-tensor
[morizumi-parity]: ../sources.md#morizumi-parity
[morizumi-width]: ../sources.md#morizumi-width
[nurk-sat]: ../sources.md#nurk-sat
[redkin-implementation]: ../sources.md#redkin-implementation
[redkin-sparse]: ../sources.md#redkin-sparse
[wang-penalties]: ../sources.md#wang-penalties
