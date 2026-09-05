# Adversarial structural proof audit

Reviewed on 2026-09-04. Scope: `sections/structure.tex`,
`research/structural/proof.md`, and the circuit model in `main.tex:86-109`.
The reviewer did not edit manuscript or research sources.

## Verdict

No mathematical counterexample or invalid inference was found in the claimed
B2 projection bound. The proof establishes, for every fixed epsilon > 0,

    C(exists_y f) <= O_epsilon((s+1) 2^((1/5+epsilon)(s+1))).

Equivalently, its uniform worst-case circuit-size exponential rate is at most
`s/5+o(s)`. The reductions and coefficients below were checked analytically;
the finite compiler run is only implementation evidence.

The main substantive Gap is an omitted algorithmic consequence and an
unresolved comparison with CircuitSAT/#SAT literature. A real exponential-space
versus branching distinction exists, but it does not establish priority or
explain away every historical best-running-time statement.

## Findings

### S1. Gap: the constructive SAT consequence needs an explicit boundary

- **Location:** `sections/structure.tex:218-239,277-279`;
  `research/structural/proof.md:193-206,230-235`;
  `research/structural/index.md:182-187`.
- **Severity:** Gap. This is not a defect in the projection theorem.
- **Finding:** The caveat about arbitrary truth-table synthesis correctly
  explains why the combined `1/5` circuit-size bound is not an equally fast
  synthesis algorithm. It does not dispose of the algorithmic implication of
  the tensor construction alone. Quantify every input of a supplied circuit.
  With `b` used inputs and at most `s` surviving gates, `h=p=0` and
  `kappa <= s+1-b`. For fixed `alpha=1/3+delta`, the constructive tensor
  algorithm and brute force give

      T(C) <= O_delta^*(2^min{b, alpha*kappa})
           <= O_delta^*(2^((alpha/(1+alpha))*(s+1))).

  The last coefficient tends to `1/4`. Thus, for every fixed epsilon > 0,
  the argument gives a deterministic, uniform CircuitSAT algorithm in
  `O_epsilon^*(2^((1/4+epsilon)(s+1)))` time. It needs exponentially many
  frontier entries in the worst case. This conclusion uses no synthesis oracle.

  The same bound holds for counting satisfying assignments after replacing
  Boolean OR/AND by integer addition/multiplication. Every assignment of the
  original used inputs has exactly one consistent assignment of gate values;
  equality splitting also has unique internal extensions. Hence no unwanted
  multiplicity is introduced. Each intermediate entry counts assignments to
  only `O(s)` binary indices and has `O(s)` bits. Arithmetic consequently
  changes only the polynomial factor. Unused inputs contribute the final
  factor `2^(n-b)`, which must be retained for #SAT.

  The freshly checked primary GKST report uses the same binary basis and
  internal-gate count (Section 2.3), and states the `2.99n` #SAT benchmark.
  Its recursive branching construction can be traversed in polynomial space;
  this is an inference from its construction, not an explicit space claim
  found in the report. Searches for the word `space` in both the fetched
  ECCC report and MFCS paper found none. Lialina's primary publication record
  confirms Savinov's `0.389667s` decision bound. Its full-text link did not
  yield a readable PDF during this audit, so Savinov's space contract was not
  independently established here.

  These facts are compatible with the present proof, but do not establish
  whether the resulting unrestricted-space rate is already known. A limited
  targeted search did not locate the exact `s/4` result. Absence from that
  search is not evidence of novelty, nor evidence of a mathematical flaw.
- **Suggested fix:** Add a short fixed-epsilon SAT/#SAT corollary or an
  explicit remark deriving it. State exponential space and separate this
  algorithm from the nonuniform placeholder-synthesis alternative. Describe
  the cited branching bounds as published benchmarks with their actual scope;
  leave the unrestricted-space literature comparison and priority unestablished
  unless a source resolves them. If writing one algorithm with time
  `2^(s/4+o(s))`, explain an effective epsilon schedule or dovetailing over
  the fixed-epsilon constructions; the fixed-epsilon statement above is the
  immediate, fully quantified consequence.

### S2. Gap: the finite compiler omits the trivial witness-output case

- **Location:** `research/structural/data/check_tensor_compiler.py:280-303`;
  compare `sections/structure.tex:18-20` and
  `research/structural/proof.md:10-13`.
- **Severity:** Gap in implementation coverage, not in the analytic proof.
- **Finding:** `compile_verifier(1, 1, [])` designates the final witness
  input as output but raises `KeyError: 1` at line 303. Its projection should
  be constant one. `direct_projection(1, 1, [])` returns truth mask `3`, as
  expected. The regression driver deliberately uses at least one gate, so the
  reported 9,280 exhaustive cases remain accurate and all pass. There is no
  evidence that this harness handles every circuit admitted by the model.
- **Suggested fix:** Either explicitly restrict `compile_verifier` to
  nonempty gate lists, or implement its zero-gate ordinary/witness-output
  branches and add those specific cases. The manuscript's analytic handling
  of trivial outputs already suffices for the theorem.

### S3. Polish: the core exponential rate must retain the additive p

- **Location:** `research/structural/proof.md:230-233`.
- **Severity:** Polish.
- **Finding:** The phrase "the exponential rate is at most q/5+o(q)" follows
  a correct bound with an additive `p`, but can be read as bounding total
  complexity by `2^(q/5+o(q))`. Independent preprocessing `p` need not be
  polynomial in `q`. For example, attaching one gate `H(x) AND y` to an
  arbitrarily expensive independent computation of `H` gives projection `H`.
- **Suggested fix:** Write the core statement explicitly as
  `C(g) <= p + 2^(q/5+o(q))`, or say that the bound concerns the additional
  cost after the `p` independent gates have been computed. The later
  universal `s/5+o(s)` statement remains correct because `p+q<=s`.

## Proof steps validated analytically

1. **Ordinary preprocessing and pin budget**
   (`structure.tex:18-32`; `proof.md:10-21`). Syntactic witness dependency
   propagates forward. Each dependent gate has at least one dependent pin
   and therefore at most one ordinary pin. The number of distinct ordinary
   boundary signals is no larger than the number of ordinary pins. Thus
   `ell+h<=2q` and also `h<=q`. Sharing the actual independent circuit
   computes its `p` gates once.

2. **Connectivity and cycle-rank accounting**
   (`structure.tex:34-67`; `proof.md:23-55`). The pruned output cone makes
   the dependent portion connected: the directed path from any used witness
   or dependent gate to the output stays dependent. There are `q+b` variable
   vertices, `q+1` factor vertices, `q` defining-output incidences, `ell`
   predecessor incidences, and one output-factor incidence. Multiplicity
   is retained. Consequently

       kappa = (q+ell+1) - (2q+b+1) + 1 = ell-q-b+1,
       h+b+kappa = h+ell-q+1 <= q+1.

   Deleting the independent part from the full undirected cone cannot
   increase the dimension of its binary cycle space. Since that cone is
   connected, has `s+a+b` vertices, and at most `2s` edges,
   `kappa<=s+1-a-b` follows. Constants have been propagated as specified in
   `main.tex:280-284`.

3. **Consistency semantics and coefficient costs**
   (`structure.tex:70-85`; `proof.md:44-70`). Gate equations force all gate
   outputs in topological order, so each witness assignment has one extension.
   Fixing a factor's quantified pins leaves a Boolean function of at most
   one ordinary signal. It is therefore `0`, `1`, `h_i`, or `NOT h_i`.
   At most `h` complement gates suffice; no free realizability oracle or
   uncharged predicate is present.

4. **Equality splitting**
   (`structure.tex:104-113`; `proof.md:72-83`). A tree of `D-2` ternary
   equality vertices has `D-3` internal edges and `D` external incidences.
   It enforces equality of all external bits and has a unique internal
   extension when they agree. Replacing one vertex increases both `V` and
   `E` by `D-3`, preserving cycle rank and connectivity. The equality-vertex
   count is at most the sum `q+ell+1` of old variable degrees, giving
   `N0<=4q+2`.

5. **All multigraph reductions and their costs**
   (`structure.tex:115-138`; `proof.md:87-115`). A loop contracts only its
   two diagonal values, with at most two remaining table entries. Contracting
   a degree-one/two vertex into a neighbor has rank at most three and costs
   at most `8*(2+1)=24` gates. Uncontracted parallel ports become a loop,
   whose equality constraint is retained. Double-edge cubic contraction has
   rank two and costs `4*(4+3)=28`; triple-edge contraction costs `8+7=15`.
   Each operation reduces `V+E`, and `E<=3N0/2` makes `80N0` a valid loose
   total including scalar accumulation. Ordinary contraction preserves
   cycle rank, loop removal lowers it by one, and simultaneous contraction
   of `j` parallel edges lowers it by `j-1`. A remaining nonempty graph
   stays connected and becomes simple cubic. Hence
   `N=2*(kappa(K)-1)<=2*(kappa-1)`.

6. **Median pathwidth-to-cutwidth conversion, including ties**
   (`structure.tex:154-178`; `proof.md:123-154`). Expanding integer bag
   intervals by `1/4` does not introduce overlap exceeding a bag size:
   away from an integer, all covering intervals occur in a neighboring bag
   (in the middle gap they occur in both). Every edge intersection has
   nonempty interior, allowing distinct rational timestamps of polynomial
   bit length. For a cut at `t`, charge a crossing edge to its left endpoint
   when its timestamp is above `t`, otherwise to its right endpoint. The
   charged interval contains `t`, and a left vertex can receive only its one
   timestamp above its median; a right vertex can receive only its one below.
   Distinct timestamps allow at most two vertices at a tied median. For a
   cut between those two vertices, the same strict charging rule still works
   for every other crossing edge, even when an endpoint itself has median
   `t`. Only their common median edge can be uncharged. Thus `cw<=pw+2`
   is justified without a topological ordering assumption.

7. **Exact frontier invariant and exponent**
   (`structure.tex:184-205`; `proof.md:158-186`). Boundary states fix
   precisely the crossing edge indices; internal indices are summed once.
   Distributivity gives the recurrence. With `j` incoming edges, the step
   has `2^|F_i|` entries and `2^j` terms per entry, costing exactly the
   stated naive count `2^|F_i|*(2^(j+1)-1)`. Since
   `|F_i|=|F_(i-1)|+3-2j`, the quantities `|F_i|+j` are bounded by
   `w`, `w+1`, `w+1`, `w` for `j=0,1,2,3`, respectively. Thus every step
   costs less than `4*2^w`, not `8*2^w` or a larger exponential power.
   With `N<=N0<=4q+2`, `h<=q`, and `2^w>=1`, the finite bound
   `p+h+80N0+4N*2^w+1 <= p+400(q+1)*2^w` is valid.

8. **Published pathwidth theorem and epsilon constants**
   (`structure.tex:218-239`; `proof.md:193-210`). Fomin--Hoie Theorem 5,
   on printed page 194, states the required maximum-degree-three pathwidth
   theorem for every positive epsilon above a size threshold. Their graph
   convention is simple undirected graphs (Section 2), exactly the class
   reached after the reductions. Section 4 on that same page states that
   a suitable path decomposition can be constructed in polynomial time.
   Using `eta=delta/2` and `N<=2kappa` gives
   `w<=(1/3+delta)kappa+2` on large kernels. For small kernels,
   `pw<=N-1` is enough; the stated `A_delta=400*2^(n_(delta/2)+2)` safely
   absorbs them. This proves every fixed-delta bound; it makes no endpoint
   claim `O((q+1)2^(kappa/3))`.

9. **Nonuniform synthesis, minimization, and universal quantifiers**
   (`structure.tex:260-274`; `proof.md:212-235`). The formal core function
   `phi(u)=exists_y core(u,y)` is defined on all `h`-bit assignments.
   Substituting `u_i=h_i(x)` after synthesis is valid even if only a correlated
   subset of formal assignments is realizable. Shannon expansion gives
   `M_h=O(2^h)` including `h=0,1`; witness enumeration shares the independent
   part and costs `p+(q+1)2^b-1`. Taking the minimum with the tensor bound,
   putting `z=min{h,b,alpha*kappa}`, and using the consistency budget gives
   `(2+1/alpha)z<=q+1`. Therefore
   `beta=alpha/(1+2alpha)=(1+3delta)/(5+6delta)` tends to `1/5`.
   The constants depend only on epsilon, not on the ordinary/witness input
   counts. Together with `p+q<=s`, this yields the stated worst-case
   `D(n,m,s)` bound uniformly in `n,m`. The `o(s)` formulation is justified
   for this circuit-size statement by the family of fixed-epsilon bounds;
   there is no improper setting of epsilon to zero.

10. **Conditioning and components**
    (`structure.tex:286-302`; `proof.md:237-249`). Fixing consistency
    variables while retaining every defining factor gives `H-S`. Distinct
    components have disjoint quantified indices and may be projected
    independently, even when their coefficient circuits share ordinary
    inputs. Scalar factors are handled by the accumulator. The total
    post-splitting vertex count over components is `O(q+1)`, and the largest
    component cycle rank bounds every exponential factor. ANDing components
    and ORing the `2^t` branches gives the claimed cost. No multiplicative
    factor exponential in the number of components is needed.

## Reproduced implementation evidence

Command:

    python3 research/structural/data/check_tensor_compiler.py

The run exited successfully and exactly matched
`research/structural/data/expected_output.txt`:

    Exhaustive verifier projections: 9280 OK
    Seeded verifier projections: 600 OK (seed 20260904)
    Adversarial symbolic tensor networks: 140 OK
    Reduction and boundary checks: degree_1=18535, degree_2=15590, loop=3850, parallel_2=181, parallel_3=659, equality_split=140, cubic_kernel=162, median_ties=204
    All generated circuits, cycle-rank inequalities and gate-cost checks: OK

The script uses a supplied elementary path decomposition. It neither
implements nor tests the Fomin--Hoie asymptotic construction. Its success
cannot establish the universal exponent or literature priority.

## Local References

- Fedor V. Fomin and Kjartan Hoie. *Pathwidth of cubic graphs and exact
  algorithms.* Information Processing Letters 97 (2006), 191-196.
  DOI: https://doi.org/10.1016/j.ipl.2005.10.012.
  Independently fetched author-hosted full text:
  https://fedorvf.github.io/articles/2006/2006b.pdf.
  Relevant locations: Section 2, Theorem 5, and Section 4; printed pp. 192,194.
- Alexander Golovnev, Alexander S. Kulikov, Alexander V. Smal, and
  Suguru Tamaki. *Circuit size lower bounds and #SAT upper bounds through
  a general framework.* ECCC TR16-022 (2016).
  https://eccc.weizmann.ac.il/report/2016/022/download.
  Relevant locations: abstract; Sections 1.3 and 2.3; Theorem 4.
  MFCS 2016 published paper also fetched:
  https://drops.dagstuhl.de/storage/00lipics/lipics-vol058-mfcs2016/LIPIcs.MFCS.2016.45/LIPIcs.MFCS.2016.45.pdf.
- A. A. Lialina. *On the complexity of unique Circuit SAT.* Zapiski
  Nauchnykh Seminarov POMI 475 (2018), 122-136.
  Primary publication record and abstract:
  https://www.mathnet.ru/eng/znsl6688.
  The record confirms the cited Savinov exponent; its linked PDF could
  not be inspected in this audit.
