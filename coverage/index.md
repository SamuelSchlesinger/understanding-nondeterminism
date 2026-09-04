# Joint witness coverage and circuit sharing

Parent: [research record](../index.md). Status: analytical proof and independent review completed; priority unestablished.

## The coupling that the original note misses

Choosing fewer witnesses and sharing restrictions are compatible operations.
For a fixed witness set H, a gate v is copied only for the assignments to its
syntactic witness support Y_v that actually occur in H. Write p_v(H) for their
number. The cost is at most sum_v p_v(H) plus p_out(H)-1 output OR gates.
The family H need not cover every positive input: uncovered positives can be
corrected by a circuit representing their characteristic function.

This yields a different optimization problem from minimizing |H| alone. A
witness set can have many members but few distinct restrictions at costly gates.

## Occupancy and alteration: master theorem

Assume n >= 2, and fix a verifier circuit with s gates. Let mu be any probability
distribution on witnesses, and let t >= 0 be an integer. For each gate v and
support assignment alpha, put p(v,alpha)=Pr_mu[y restricted to Y_v = alpha].
Define

    B_mu(t) = sum over gates v and alpha of (1-(1-p(v,alpha))^t)
              + sum over output-support alpha of (1-(1-p(out,alpha))^t),
    E_mu(t) = sum over x with g(x)=1 of (1-mu(W_x))^t.

All powers with exponent zero are one, including 0^0 in these sampling formulas.
The first conclusion is the exact convenient upper bound

    C(g) <= B_mu(t) + n E_mu(t).

Proof: sample t independent witnesses; share identical restrictions at each
gate; OR their outputs; add point indicators for all uncovered positive inputs.
An indicator of one n-bit input takes n-1 gates over B2, and its final OR costs
at most one additional gate. Hence the actual cost is bounded by
sum_v p_v(H)+p_out(H)+n times the number of missed positives. Taking expectation
and choosing a sample no worse than average proves the bound. For t=0, use only
the correction circuit. No algorithm for finding the sample is asserted.

### Sparse correction refinement

Any n-input function supported on R points has size

    O(n R / log_2(R+2)),

with the expression zero for R=0. For R>=2, partition inputs into blocks of
k=min(n,floor(log_2 R)) bits. Precompute every assignment indicator on each
block in O(2^k) gates, then form each of the R full point indicators as an AND
of the appropriate block indicators, and OR them. Total cost is
O(ceil(n/k) (2^k+R))=O(n R/log(R+2)). R=1 is direct.

The function z/log(z+2) is concave on z>=0. In natural logarithms its second
derivative has the sign of 2z-(z+4)ln(z+2), which is negative: the opposite
quantity is positive at zero and has positive derivative.
Jensen's inequality therefore gives the stronger theorem

    C(g) <= B_mu(t) + O(n E_mu(t)/log_2(E_mu(t)+2)).

The implied constant is absolute. This adds a classic block-synthesis argument
to the occupancy/alteration construction. Its novelty as a combined statement
is not established.

## Proved consequences and examples

The [manuscript coverage section](../../sections/coverage.tex) gives complete proofs of:

1. A covering sample whose cost is at most twice the expected occupancy, with
   min{support states, n/rho} charged separately at each gate.
2. C(g)=O(n(s+1)2^m/K + n R_K/log(R_K+2)), where R_K counts positive
   inputs with fewer than K witnesses.
3. If C(g)>=T=2^m(s+1)/L, at least c' (T/n)log(2+T/n) positive inputs
   have at most c n L witnesses, for absolute positive constants c,c'.

The [worked examples](../examples/index.md) analytically separate the coupled
bound from the original estimates and exhibit distribution tradeoffs, including
exponentially different occupancy costs for identical row-mass vectors.
The [independent audit](../../review-notes/coverage-audit.md) found no Error or
Gap in the analytical claims. Finite checks validate implementations only.

## Required literature comparisons

The construction uses standard occupancy, the probabilistic method with
alteration, and circuit restriction caching. Compare precise prior results on
knowledge compilation, set cover with submodular costs, and sparse Boolean
function synthesis when interpreting originality. Parent sources:
[source record](../sources.md).
