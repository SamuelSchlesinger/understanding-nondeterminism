# Independent mathematical review: critical balance

Reviewed `research/structural/critical-balance.md`, lines 1-137, on 2026-09-04,
against the finite compiler, consistency budget, graph-width input, and
sparse-population corollary in the current manuscript. This is an analytical
review of the new deductions; no experiment or new external attribution was
needed.

**Verdict:** no proof-breaking error in the finite stability argument, the
critical-family deductions, or the conclusion about every legal residual
kernel. The two scope clarifications below should be made before integration.

## Findings

### G1. State which lower-bound assumptions remain in the sparse paragraph

- **Severity:** Gap.
- **Location:** `research/structural/critical-balance.md:62-63,119-130`.
- **Finding:** the claim at line 62 is valid in the direction
  `C(g)>=2^m(s+1)/L` with `log L=o(m)` implies
  `log C(g)>=m-o(m)`, when `s+1=(5+o(1))m`. The wording "includes"
  does not make that direction explicit. At line 119, "the stronger
  hypothesis" is stated with only `L>=1`. Taken as a replacement for the
  critical lower bound, it need not be stronger: exponentially large `L`
  can make the threshold polynomial or smaller. The sparse-population bound
  itself and the exact enumeration consequence remain valid for such `L`;
  the critical-family conclusions need their original hypothesis.
- **Suggested fix:** write at line 62 that the amplification condition
  *implies* the asymptotic lower bound when `log L=o(m)`. Begin the final
  paragraph either "Under the critical-family assumptions, suppose in
  addition ... with L>=1" or require `log L=o(m)` when using amplification
  as the sufficient hypothesis for all the conclusions.

### G2. Make the nontrivial output restriction explicit

- **Severity:** Gap in statement scope, not in the proof under the inherited
  structural-theorem hypotheses.
- **Location:** `research/structural/critical-balance.md:5-7,34-44`.
- **Finding:** `|a-h|<=p` uses the fact that every independent component
  eventually reaches an interface leading to a witness-dependent gate output.
  Normalization alone does not imply that restriction. For the normalized
  circuit consisting solely of an ordinary input, `a=1,p=h=0`, so the displayed
  inequality would be false if the paragraph were read without the structural
  theorem's exclusion of trivial or witness-independent outputs.
- **Suggested fix:** explicitly state that the output is a syntactically
  witness-dependent gate and `q>=1`; the cases excluded by the structural
  theorem remain excluded here. The critical-family lower bound itself rules
  out those cases eventually, so this does not alter its result.

### P1. Explain why the residual kernels cross the graph-theorem threshold

- **Severity:** Polish.
- **Location:** `research/structural/critical-balance.md:102-106`.
- **Finding:** applying the pathwidth theorem for every fixed `eta` requires
  `N` to tend to infinity, not merely to be nonzero. This follows immediately
  from the already proved `cw(K)>=m-o(m)` and `cw(K)<=|E(K)|=3N/2`.
- **Suggested fix:** insert that one-line implication before invoking the
  large-graph theorem. It also makes uniformity over the reduction choices
  transparent.

## Verified deductions

- **Finite stability:** subtracting `p` and dividing by the same positive
  `A_delta(q+1)` gives `min{h,b,alpha*kappa}>=t`. The three excesses are
  nonnegative, and their sum is at most the stated `Delta` by the consistency
  budget. No uniform control of `A_delta` is used.

- **Ordinary-input interface and slack:** under the nontrivial scope above,
  every connected component of the independent subgraph has an interface
  vertex. Thus `c<=h` and `a+p-c<=E<=2p`, giving `a<=p+h`; the reverse
  comparison follows from `h<=a+p`. Parallel wires do not invalidate either
  edge estimate. Substitution gives exactly
  `q+1-h-b-kappa=2q-ell-h=U+(r-h)`, with both summands nonnegative.

- **Critical asymptotics:** `p=O(m)` is negligible compared with the assumed
  exponential `C(g)`, so `log(C(g)-p)>=m-o(m)`. For every fixed `delta`,
  `log(A_delta(q+1))=O_delta(log m)` is negligible. Consequently the three
  parameter lower bounds hold. The consistency budget gives
  `limsup p/m<=3-1/alpha` and
  `limsup h/m, limsup b/m<=4-1/alpha`. Sending the fixed constant `delta`
  to zero after taking the size limits proves all the stated limits.
  Nonnegativity and the slack identity give `U+(r-h)=o(m)`. There is no
  hidden choice of a size-dependent `delta`.

- **Every legal residual kernel:** the bound
  `C(g)<=p+400(q+1)2^cw(K)` is valid for every complete reduction sequence
  allowed by the manuscript, with its prescribed equality trees. A minimum
  cutwidth ordering may be used nonuniformly; finding it is not required for
  this circuit-size upper bound. Hence every such `K` has
  `cw(K)>=m-o(m)`. Combined with `N<=2(kappa-1)` and, for each fixed
  `eta`, `cw(K)<=pw(K)+2<=(1/6+eta)N+2`, this forces `N/m->6`.
  The cubic identity then gives the cycle count and sublinear loss. The same
  inequalities bound both widths below by `m-o(m)` and above by `m+o(m)`.
  The constants and size thresholds do not depend on the reduction choice,
  so the conclusion also holds uniformly over those choices.

- **Declared witnesses:** the final paragraph correctly applies the existing
  sparse corollary on the original ordinary-input domain. Removing `m-b`
  unused witness variables divides every fiber cardinality by exactly
  `2^(m-b)` and divides its threshold by the same amount; the number of
  qualifying ordinary inputs stays unchanged. Finally, exact enumeration gives
  `C(g)<=(s+1)2^b-1`. Together with `C(g)>=2^m(s+1)/L` this proves the
  stated weaker inequality `m-b<=log L` without hidden constants. If `n`
  includes many unused ordinary inputs the population estimate can be weak,
  but it remains valid. Replacing `n` by used-input count `a` would require
  explicitly changing the ordinary-input domain and then lifting its count.

These deductions establish necessary conditions under the specified lower
bound. They establish neither such a family's existence nor a converse from
large kernels, sparse fibers, or balanced budgets to circuit hardness.

## Resolution in the integrated manuscript

The root applied the scope and limit clarifications before integration.
The amplification paragraph now explicitly assumes `log L=o(m)` when using
that hypothesis to imply the critical-family lower bound; the supporting
Markdown keeps all critical-family hypotheses when adding an arbitrary
`L>=1`. The proof explicitly derives `N->infinity` before invoking the
asymptotic width theorem. The concluding text now says that the deductions
establish no converse, without claiming a counterexample to the full joint
list of necessary conditions. The independent completion review checked
these changes in the integrated TeX and supporting record.
