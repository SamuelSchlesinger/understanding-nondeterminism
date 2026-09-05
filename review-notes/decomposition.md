# Decomposition review

Reviewed `research/index.md` and the full initial `main.tex` on 2026-09-04.
This is a Phase 1.5 review under the author-review-revise skill, not a proof
review of subsequent results. No manuscript source was edited.

## Recommendation

Proceed with four owners: root develops the coupled coverage theorem; one
structural author develops quantified-only elimination; one literature author
builds the exact comparison matrix; one expert author develops adversarial
examples and separation families. Root also owns synthesis and pedagogy.
This gives two focused proof engines, a novelty check that runs alongside
them, and an independent source of mathematical stress tests. It fits the
available concurrency without pretending that exposition is a new result.

R1 and R3 are useful lemmas, but neither should be counted as a headline
contribution merely because it has its own theorem number. The user's goal
of several genuine new contributions remains unmet unless the resulting
theorems or constructions establish substantive new content after comparison.
Writing a careful survey and labeling every candidate's priority unestablished
does not by itself fulfill that goal.

## Findings

1. **Location:** `research/index.md`, Research decomposition and R1--R5.
   **Severity:** Gap.
   **Finding:** Structural elimination currently includes core normalization,
   gate-count optimization, separators, and state compression, while synthesis
   is mostly editorial work. The split is not balanced by mathematical effort.
   Coverage and structure also overlap at shared restriction circuits.
   **Fix:** Use the ownership table below. Root owns all sampled-restriction
   constructions; the structural author owns exact constraint elimination.
   Require each author to supply its own worked example and model assumptions.

2. **Location:** `research/index.md`, R1 and R3.
   **Severity:** Gap.
   **Finding:** R1 is a sharper optimization of the existing three-way minimum;
   a threshold density bound plus explicit minterm correction is a standard
   probabilistic alteration argument. They may improve the note without
   constituting two substantive research contributions.
   **Fix:** Make R1 a quantitative corollary of a stronger structural theorem.
   Make R3 a corollary of the joint occupancy/alteration theorem described below.
   Evaluate the latter by its new tradeoffs and examples, not proof length.

3. **Location:** `main.tex`, Circuit structure yields a second easy range;
   `research/index.md`, R5.
   **Severity:** Gap.
   **Finding:** Conditioning on a cutset, solving bounded-width residuals,
   and reusing partial assignments already appear in Dechter's elimination
   framework [dechter99][dechter99]. Generic treewidth compilation is also
   established [amarilli18][amarilli18]. A separator theorem must identify the
   actual improvement over those results.
   **Fix:** Formulate the graph on quantified witnesses and internal consistency
   variables, with ordinary input predicates retained as shared circuit values.
   Specify the factors, their scopes, the decomposition, all state transitions,
   and the exact B2 gate cost. Compare to compiling the full circuit first.

4. **Location:** `research/index.md`, Literature and scope.
   **Severity:** Gap.
   **Finding:** The coverage branch lacks its closest combinatorial comparisons:
   fractional versus integral covers, and submodular costs with uncovered-element
   penalties. The latter is particularly close to support sharing: the function
   `H -> sum_v |projection_(Y_v)(H)|` is a sum of coverage functions and hence
   submodular. A coverage-only distribution optimum can differ from a
   circuit-cost optimum. See [lovasz75][lovasz75] and [wang15][wang15].
   **Fix:** Add these comparisons before making priority claims for the
   occupancy objective. Their optimization framework does not automatically
   subsume the circuit theorem; the matrix must record the exact mapping and
   whatever additional circuit-specific statement is proved.

5. **Location:** `main.tex`, forest elimination and final open question.
   **Severity:** Gap.
   **Finding:** Knowledge compilation uses "deterministic" for disjoint OR
   branches. That property is not required of an ordinary deterministic Boolean
   circuit. Forgetting preserves DNNF efficiently but can lose its determinism
   [darwiche02][darwiche02]. Bounds that restore determinism or support projected
   model counting solve a stronger representation problem [capelli18][capelli18].
   **Fix:** State this distinction explicitly. Do not transfer a DNNF, OBDD,
   or deterministic-DNNF lower bound to unrestricted B2 circuit size.

6. **Location:** `research/index.md`, examples and open questions.
   **Severity:** Gap.
   **Finding:** The current open question principally restates the large
   unrestricted worst-case problem. The candidate list does not specify what
   concrete success or failure would look like for each proposed method.
   **Fix:** Give each target a strict comparison example, a falsifiable stronger
   conjecture, and a stopping criterion. Compare upper-bound guarantees without
   calling those comparisons lower bounds on minimum circuit complexity.

## Revised mandates

| Owner | Bounded research task | Required output and acceptance test |
| --- | --- | --- |
| Root: coverage and manuscript | Prove the joint occupancy/alteration theorem, including optimized witness distributions and sparse exception synthesis. Integrate R3 and R4 as consequences. | Complete theorem, exact cost accounting, small-parameter cases, finite construction check, and a family where coupled optimization improves on both separate coverage and full support compilation guarantees. |
| Structural author | Develop quantified-only constraint elimination and its relation to witness-dependent cores. Investigate whether its explicit exponent improves the current 1/3 coefficient. | One fully specified compiler and proof; a comparison with full-circuit width and cutset bounds; an example that demonstrates the ordinary/witness distinction. R1 and preprocessing refinements are secondary corollaries. |
| Literature author | Test each exact candidate against its nearest primary results, including the references below and the initial note's nondeterministic-circuit sources. | A statement-by-statement matrix: input/output model, parameter, conclusion, constructive versus existential guarantee, exact overlap, and remaining difference. Classify known consequence, new synthesis/application, or potentially new theorem with priority unestablished. Do not write a second survey. |
| Expert author | Attack the proposed invariants and construct separating families; independently check consistency examples and boundary cases. | Explicit circuit families, parameter calculations, proved comparisons, and counterexamples to plausible overextensions. Keep syntactic compiler obstructions distinct from unrestricted circuit lower bounds. |

Each main research document should target roughly 200--300 lines. The structural
author should put detailed decomposition transitions in a linked detail file
if needed. Source comparison must begin with candidate statements, then be
updated against the proved versions; leaving it until final synthesis is too late.

## Stronger concrete targets and traps

- **Coupled occupancy and exceptions:** for a distribution `mu`, sample count
  `t`, support assignment probability `p_(v,alpha)`, and row mass `p_x`, the
  natural expected cost contains
  `sum_(v,alpha) (1-(1-p_(v,alpha))^t)` together with output occupancy and a
  correction term driven by `sum_(x:g(x)=1) (1-p_x)^t`.
  The proof mechanism is elementary expectation and alteration. Its research
  value would be the precise shared-circuit cost, a stronger exception-synthesis
  theorem, and examples exposing a tradeoff absent from density alone.
  If a nonlinear sparse-synthesis cost is applied to the expected number of
  misses, prove the required concavity/Jensen step or use a justified alternate
  probabilistic argument; substituting expectation inside an arbitrary cost is
  not valid. Also prove zero/one-input and zero-exception cases separately.

- **Quantified-only width:** ordinary values can be shared symbolic predicates
  because each ordinary input is fixed throughout existential evaluation.
  However, removing ordinary vertices from a graph is not itself a correctness
  proof: all factors induced by gates and all shared quantified values must
  remain. A small semantic boundary alphabet is useful only if its realizability
  predicates and transitions have a charged circuit construction. Defining
  states using an already-hard projection merely hides the task.

- **Cyclomatic excess and the exponent:** Fomin--Hoie prove cubic-graph
  pathwidth at most `(1/6+epsilon)N` for sufficiently large `N`
  [fomin06][fomin06]. This is a concrete candidate tool, not an improvement
  already obtained. Track the number of degree-reduction vertices, conversion
  to a constraint graph, boundary states, and gates per transition. A hidden
  constant in `2^O(width)` cannot establish a better numerical exponent.

- **Affine boundary compression:** potentially useful if the boundary values
  obey explicit affine relations and a small circuit computes a particular
  solution from ordinary inputs. Enumerating a nullspace can replace the number
  of guessed bits by a rank deficit. Require the original defining equations,
  a completeness proof for all feasible boundary values, and the cost of the
  affine maps. Affine cancellation alone does not preserve existential AND/OR.
  This is a secondary target until its closest preprocessing/backdoor literature
  is checked; it should not delay a complete primary structural theorem.

- **Counterexamples with a purpose:** construct relations with the same positive
  fiber-size profile and very different common-cover structure; construct
  circuits with expensive ordinary preprocessing but small quantified structure;
  and construct examples where local witness feasibility is inconsistent
  globally. Some can have constant projections: state explicitly when they
  separate parameters or compilers rather than projected circuit complexity.
  Whenever possible avoid padding, dead gates, and unused variables, and report
  when a bound concerns a supplied circuit instead of an optimal verifier.

- **Nonuniformity:** the universal-synthesis arm in the three-way bound provides
  circuit existence, not an efficient method of finding a circuit from the
  verifier. In particular, the fact that a zero-input projection has a constant
  circuit does not solve SAT efficiently. Keep circuit-size and compilation-time
  conclusions separate in every comparison and every new theorem.

## Verified primary comparisons

The following records were fetched during this review. This is a focused
comparison set, not an exhaustive novelty search. Published claims below stay
within what the fetched source states; candidate extensions above are proposed
work, not attributed theorems.

## Local References

- **darwiche02:** Adnan Darwiche and Pierre Marquis. *A Knowledge Compilation
  Map*. Journal of Artificial Intelligence Research 17 (2002), 229--264.
  [Author paper PDF](https://arxiv.org/pdf/1106.1819). Section 5, Table 7 and
  Proposition 5.1 distinguish forgetting into DNNF from preserving d-DNNF.
- **dechter99:** Rina Dechter. *Bucket elimination: A unifying framework for
  reasoning*. Artificial Intelligence 113(1--2) (1999), 41--85.
  DOI: [10.1016/S0004-3702(99)00059-4](https://doi.org/10.1016/S0004-3702(99)00059-4).
  [Author PDF](https://ics.uci.edu/~csp/r76A.pdf). Section 10, Theorem 14 and
  the discussion following it cover conditioning/elimination hybrids and
  explicitly mention exploiting shared partial assignments.
- **amarilli18:** Antoine Amarilli, Mikael Monet and Pierre Senellart.
  *Connecting Width and Structure in Knowledge Compilation*. ICDT 2018,
  LIPIcs 98, article 6, 6:1--6:17.
  DOI: [10.4230/LIPIcs.ICDT.2018.6](https://doi.org/10.4230/LIPIcs.ICDT.2018.6).
  Theorem 5 gives d-SDNNF compilation in `O(|T| 2^((4+epsilon)k))` when a
  width-`k` tree decomposition is supplied. The precise source gate basis
  must be accounted for when applying it to B2.
- **capelli18:** Florent Capelli and Stefan Mengel. *Knowledge Compilation,
  Width and Quantification*. arXiv:1807.04263v1 (2018).
  [Verified version](https://arxiv.org/pdf/1807.04263).
  Theorem 5 builds both a projection and its complement as a complete
  structured d-DNNF, with exponential dependence on the input width.
  Verify the published version before using it as the canonical bibliography
  record; the preprint is the version read here.
- **lovasz75:** Laszlo Lovasz. *On the ratio of optimal integral and fractional
  covers*. Discrete Mathematics 13(4) (1975), 383--390.
  DOI: [10.1016/0012-365X(75)90058-8](https://doi.org/10.1016/0012-365X(75)90058-8).
  Publisher metadata and abstract verified. The abstract bounds the integral
  versus fractional cover ratio in terms of maximum degree. Recheck the full
  theorem and the primal/dual incidence convention before applying its formula.
- **wang15:** Fengmin Wang, Dachuan Xu, Donglei Du and Chenchen Wu.
  *Primal-dual approximation algorithms for submodular cost set cover problems
  with linear/submodular penalties*. Numerical Algebra, Control and Optimization
  5(2) (2015), 91--100.
  [Publisher record](https://www.aimsciences.org/article/doi/10.3934/naco.2015.5.91).
  DOI: 10.3934/naco.2015.5.91. Author list, metadata and abstract verified.
  This establishes a directly relevant pre-existing optimization framework;
  its full hypotheses and guarantees still need comparison with the circuit
  objective before asserting subsumption.
- **fomin06:** Fedor V. Fomin and Kjartan Hoie. *Pathwidth of cubic graphs and
  exact algorithms*. Information Processing Letters 97 (2006), 191--196.
  DOI: [10.1016/j.ipl.2005.10.012](https://doi.org/10.1016/j.ipl.2005.10.012).
  [Author PDF](https://fedorvf.github.io/articles/2006/2006b.pdf). The abstract
  and theorem confirm the asymptotic `(1/6+epsilon)N` cubic-graph bound.

[darwiche02]: https://arxiv.org/pdf/1106.1819
[dechter99]: https://ics.uci.edu/~csp/r76A.pdf
[amarilli18]: https://doi.org/10.4230/LIPIcs.ICDT.2018.6
[capelli18]: https://arxiv.org/pdf/1807.04263
[lovasz75]: https://doi.org/10.1016/0012-365X(75)90058-8
[wang15]: https://www.aimsciences.org/article/doi/10.3934/naco.2015.5.91
[fomin06]: https://fedorvf.github.io/articles/2006/2006b.pdf
