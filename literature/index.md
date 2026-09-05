# Primary-literature and novelty audit

Reviewed comparison, 2026-09-04. This audit compares theorem statements, not search-hit counts.
The [statement matrix](statement-matrix.md) records source models and exact overlap.
[Baseline sources](baselines.md) cover synthesis and nonuniform hardness.
The [tensor follow-up](tensor-followup.md) records direct antecedents for
COPY expansion, semiring changes, and exact contraction rewrites.
The [final counting comparison](final-comparison.md) updates the Nurk and
Broering--Lokam access limits and records the additional factoring antecedent.
The [coverage analysis](../coverage/index.md) contains the proposed occupancy theorem.
The [research overview](../index.md) supplies the manuscript context.

## Assessment

The coupled occupancy/alteration bound is one of the proved contributions.
Its defensible description is a circuit-specific synthesis of established techniques.
Priority for the exact combined inequality remains unestablished.
Symbolic elimination itself is an application of existing distributive-law machinery.
The structural exponent has a complete analytical proof and has passed independent proof review. Its exact literature priority remains unresolved.

| Candidate | Classification supported now | Remaining contribution or limitation |
| --- | --- | --- |
| OR over all witnesses | Known consequence | Baseline, including output OR cost and used witnesses. |
| Reuse gates indexed by witness restrictions | Elementary circuit application | Exact occupancy accounting is useful, but caching is established. |
| Sampling witnesses and correcting missed inputs | Known alteration principle applied to circuits | The combination with shared-gate cost is the candidate. |
| Sparse correction `O(nR/log(R+2))` | Classical synthesis technique; no originality claim | Convenient uniform bound, substantially nonoptimal for some small `R`. |
| Joint expected occupancy plus sparse correction | Candidate new synthesis/application | Complete proof and strict analytical examples are integrated; exact priority remains unestablished. |
| Optimize witness distribution | Established fractional-cover idea plus circuit cost | Joint optimization includes occupancies; the density-only part is fractional set cover. |
| Quantified-only symbolic variable elimination | Known GDL specialization | Gate accounting and the chosen structural parameter may be useful applications. |
| Cut a bounded set, then eliminate | Known conditioning/elimination hybrid | Preserve the distinction between selected interface size and induced width. |
| Subcubic `cw <= pw+2` | Known graph theorem | A short direct proof may be included with attribution. |
| Cubic kernel plus excess exponent `d/3+o(d)` | Proved structural synthesis; priority unestablished | The exact reductions and uniform counting consequence have complete reviewed proofs. |
| Universal projection exponent `s/5+o(s)` | Proved by the displayed structural argument | Priority unestablished; do not report as an established literature improvement yet. |
| Critical input budgets, cycle loss, and width saturation | Proved necessary conditions from simultaneous upper bounds | No hard family or converse is established. |
| Exact cubic realization at near-critical supplied budgets | Proved representation theorem with explicit gate accounting | Starts with classical st-numbering; supplied verifiers are nonminimal and their projections are easy. |

## The model that must remain fixed

The manuscript counts internal gates over the full binary basis `B2`.
Inputs, constants, and wires are free; fanout is unrestricted.
The target is an ordinary circuit for `g(x)=exists_y f(x,y)`.
There is no requirement that OR children be mutually exclusive or AND children use disjoint variables.
This target differs materially from d-DNNF, structured d-DNNF, OBDD, and formula targets.

A basis conversion with constant overhead preserves `2^{O(k)}` statements.
It need not preserve a numerical coefficient such as `1/5` in an exponent measured against original gate count.
Likewise, full circuit treewidth, input-deleted circuit treewidth, primal constraint treewidth,
incidence treewidth, layered circuit width, and undirected cutwidth are different parameters.
Every comparison below retains its source parameter.

The circuit-size problem is nonuniform.
A small circuit for a constant projection does not provide a uniform SAT algorithm.
An explicit compiler with a proved runtime can provide one; that stronger claim must be audited independently.
Exponential-size tables also mean exponential space unless a separate space-saving argument is supplied.

## Why symbolic elimination is not a new principle

Dechter gives variable elimination with induced-width time and space bounds, and explicitly studies conditioning/elimination hybrids. [dechter-bucket][dechter-bucket]
Aji and McEliece formulate marginalization of products over an arbitrary commutative semiring and count semiring operations. [aji-gdl][aji-gdl]

Here is the direct specialization, rather than an assertion that either paper states the manuscript's exact gate bound.
Let `A` be the set of Boolean functions of the ordinary inputs `x`.
Use pointwise OR for semiring addition and pointwise AND for multiplication.
Each local constraint is a table in quantified variables whose entries are elements of `A`.
An entry is stored as a pointer to a Boolean circuit, and each semiring operation appends one gate.
Junction-tree or bucket-elimination tables therefore enumerate quantified assignments only.
Repeated occurrences of the same ordinary input remain symbolic and consistent automatically.

The algebra is valid without a finite bound on the number of possible functions in `A`.
The implementation measures circuit-construction operations, not extensional function-table operations.
For bounded Boolean scopes, a supplied width-`k` decomposition gives the usual single-exponential table bound.
Leaf construction, factors, and decomposition size still contribute to the prefactor.
Thus the contribution can be an explicit `B2` compiler and parameter analysis, not the idea of keeping free inputs symbolic.

Darwiche and Marquis record polynomial-time forgetting for DNNF. [darwiche-map][darwiche-map]
Amarilli and collaborators compile bounded full-circuit treewidth into structured deterministic representations. [amarilli-width][amarilli-width]
Combining compilation with DNNF forgetting already supplies a broad single-exponential projection route.
The determinism may be lost during forgetting; the manuscript's target permits that loss.
Capelli and Mengel's quantification cost preserves a substantially stronger target representation. [capelli-qbf][capelli-qbf]
It cannot be used as a lower bound on projection into unrestricted circuits.

## Exact scope of the occupancy candidate

Fix a supplied verifier circuit and let `Y_v` be the witness coordinates syntactically supporting gate `v`.
Let `Y_o` support its output.
For a witness set `H`, define

```
F(H) = sum_v |{y restricted to Y_v : y in H}|
       + |{y restricted to Y_o : y in H}|.
R(H) = |{x : exists_y f(x,y)=1 and f(x,y)=0 for every y in H}|.
```

With exact gate bookkeeping, restriction caching plus sparse synthesis gives
`C(g) <= F(H) + K*n*R(H)/log(R(H)+2)` up to harmless fixed additive conventions.
This is a nonuniform upper bound: the exception list may itself be hard to discover.
The sparse circuit is constructible from that list; obtaining the list is a different task.

For independent samples from a distribution `mu`, define

```
p(v,alpha) = Pr_mu[y restricted to Y_v = alpha].
B_mu(t) = sum_(v,alpha) (1-(1-p(v,alpha))^t)
          + the analogous output-support sum.
E_mu(t) = sum_(x:g(x)=1) (1-mu({y:f(x,y)=1}))^t.
```

The same random sample controls both terms.
Taking expectations and applying concavity to `z/log(z+2)` gives the candidate bound

```
C(g) <= B_mu(t) + K*n*E_mu(t)/log(E_mu(t)+2).
```

This statement is stronger in scope than independently bounding all restricted circuits by `t*s`.
It prices shared restrictions at each gate and compresses the residual support.
These are the precise ingredients that should appear in an originality claim.
The existence argument requires neither an efficient optimizer for `mu` nor an efficient coverage oracle.
Claims of a uniform construction require those costs separately.

## Relation to submodular cover with penalties

Use the positive ordinary inputs as the universe and `S_y={x:f(x,y)=1}` as selectable sets.
Each summand of `F` is a coverage function on tagged restriction keys `(v,alpha)`.
Consequently `F` is a nonnegative monotone submodular function of the selected witness set.
Charging `n` for each uncovered point gives a submodular-cost set-cover problem with linear penalties.
Charging `n*r/log(r+2)` to an explicitly selected exception set gives a concave-cardinality, hence submodular, penalty.
This abstract problem class is established. [wang-penalties][wang-penalties]

The publisher abstract verifies approximation results for linear and submodular penalties.
The full paper was not retrieved in this audit, so its oracle model and every approximation hypothesis remain unchecked.
No particular approximation guarantee is imported into the circuit manuscript.
Even a polynomial-time oracle algorithm would not imply polynomial time in succinct verifier size:
the ordinary-input universe and available witness family can both be exponential.

There is also an exact elementary connection to fractional cover.
Assume the positive-input universe is nonempty, and let `tau*` be its fractional witness-cover number.
Then

```
max_mu min_(x:g(x)=1) mu({y:f(x,y)=1}) = 1/tau*.
```

To prove this, divide a distribution of minimum coverage `q` by `q` to obtain a fractional cover of weight `1/q`.
Conversely normalize an optimal fractional cover by its total weight.
Thus optimizing only minimum witness density is an existing fractional-cover formulation.
Optimizing the coupled expression additionally considers gate-dependent projection collisions.
The latter is a concrete circuit-aware objective, not a new abstract set-cover problem class.

## Sparse synthesis and the priority boundary

Redkin studies circuits for functions with few ones, including sharp small-support asymptotics. [redkin-sparse][redkin-sparse] [redkin-implementation][redkin-implementation]
His papers explicitly place this line after Finikov and Lupanov's local-coding work.
The familiar block-decoder construction for `O(nR/log(R+2))` therefore deserves a proof for convenience, not a claim of a new sparse-synthesis principle.
The exact earliest source for that particular convenient formula was not established here.

There is a material sharpness issue, independent of priority.
For `R` at most about `log n`, the literature contains `O(n)` synthesis bounds.
The manuscript bound can be `n log n/log log n` in this range.
It is a valid broadly applicable correction bound, but it is not uniformly optimal.
A simple column-pattern grouping construction also gives `O(n+R*2^R)` from the exception list.
The minimum of available bounds can be used if the extra case distinction helps the main theorem.
Upper bounds transfer from restricted complete bases to `B2`; their lower bounds and leading constants do not.

## Structural exponent and the scope of the SAT comparison

The graph-theoretic ingredients are largely established.
Fomin and Hoie give `pw(G) <= (1/6+epsilon)*|V(G)|` for sufficiently large subcubic graphs and fixed positive `epsilon`, constructively. [fomin-pathwidth][fomin-pathwidth]
Makedon and Sudborough identify cutwidth with edge-search number for maximum degree three. [makedon-layout][makedon-layout]
Ellis, Sudborough, and Turner place search number between vertex separation and vertex separation plus two. [ellis-search][ellis-search]
The resulting subcubic `cw <= pw+2` is explicitly used in a recent primary proof. [bodlaender-cubic][bodlaender-cubic]

Tensor contraction with width-dependent complexity is also established. [markov-tensor][markov-tensor]
Direct tensor encodings of Boolean checking circuits and counting/search consequences appear in Johnson and collaborators. [johnson-search][johnson-search]
The manuscript's potential contribution is therefore the precise circuit-to-kernel reduction,
its excess accounting, the symbolic compiler's exact base, and the resulting tradeoff.
None of the sources inspected states the proposed `d/3` or universal `s/5` projection formula.
That observation is a bounded search result, not evidence establishing priority.

The analytical proof gives cycle rank kappa <= d=s+1-a-b and a cubic core
with at most 2(kappa-1) vertices. It establishes, for each fixed positive delta,
a polynomial prefactor times 2^((1/3+delta)kappa). Balancing with enumeration
and formal-input synthesis proves the fixed-epsilon projection rate 1/5.
These are mathematical conclusions of the displayed proof; literature priority
is a separate unresolved question.

For an all-existential verifier, the [counting corollary](../../sections/counting-algorithm.tex)
proves count-preserving reductions and polynomial bit costs, obtaining
poly_delta(s+u)*2^min{b,(1/3+delta)(s+1-b)}. Balancing yields a fixed-epsilon
rate approaching 1/4 with exponential space. The
[final comparison](final-comparison.md) records the recovered original texts,
the additional factoring antecedent, and the remaining precise access limits.
In contrast, the inspected circuit-SAT literature gives Nurk's gate-count exponent `0.4058`
and Savinov's `0.389667`, the latter reproduced by Lialina. [nurk-sat][nurk-sat] [lialina-sat][lialina-sat]
GKST give nontrivial `B2` counting algorithms below `3n` gates. [gkst-elimination][gkst-elimination]
Recent primary work still cites these benchmarks. [hoza-trees][hoza-trees]

The branching algorithms admit a natural depth-first implementation with polynomially represented states.
Polynomial space is an implementation inference here; no explicit universal restriction to polynomial space was located in the checked statements.
There is a substantial explicit time-space tradeoff in width-parameterized SAT. [allender-space][allender-space]
Earlier circuit-SAT work also analyzes cutwidth, pathwidth, and treewidth. [broering-width][broering-width]
An exponential-space comparison may explain different goals, but it does not by itself resolve why the smaller displayed coefficient is absent from the benchmark discussion.
The manuscript must not silently declare a new best SAT algorithm, nor dismiss the discrepancy as solved.
Finite compiler checks test the implementation and examples; they do not prove the kernel theorem or its asymptotic exponent.

## Other nondeterminism comparisons

Morizumi supplies an explicit deterministic simulation of bounded layered-width nondeterministic circuits. [morizumi-width][morizumi-width]
That width is not the manuscript's undirected quantified-variable width.
His parity lower bound is for `U2` and does not establish a general deterministic-versus-nondeterministic exponential gap. [morizumi-parity][morizumi-parity]
Cavalar and Oliveira connect circuit complexity with fusion-based cover problems. [cavalar-cover][cavalar-cover]
Their cover universe concerns pairs and filters rather than selecting witness columns to cover positive inputs.
The shared word "cover" is not a theorem-level equivalence.

## Revision requirements and search boundary

Retain the exact input/output model in every headline and comparison.
Attribute GDL, sparse synthesis, forgetting, and subcubic width conversion as established ingredients.
Give the coupled bound a single coherent proof using one shared sample.
State whether optimization is existential, oracle-based, or polynomial in an explicit truth table.
Retain fixed-epsilon quantifiers and keep proof correctness separate from priority.
Keep numerical priority of the uniform gate-SAT consequence unestablished while the source comparison remains incomplete.

The search covered primary elimination/compilation papers, sparse synthesis, submodular penalties,
nondeterministic simulation, graph width, tensor counting, and circuit-SAT references through inspected 2025 work.
It was bounded and does not establish the absence of an equivalent older theorem.
The full-text gaps are explicit in the matrix; metadata-only verification is not theorem verification.
No keyword search, citation count, or finite experiment is used as a novelty or asymptotic proof.

[aji-gdl]: ../sources.md#aji-gdl
[allender-space]: ../sources.md#allender-space
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
[makedon-layout]: ../sources.md#makedon-layout
[markov-tensor]: ../sources.md#markov-tensor
[morizumi-parity]: ../sources.md#morizumi-parity
[morizumi-width]: ../sources.md#morizumi-width
[nurk-sat]: ../sources.md#nurk-sat
[redkin-implementation]: ../sources.md#redkin-implementation
[redkin-sparse]: ../sources.md#redkin-sparse
[wang-penalties]: ../sources.md#wang-penalties
