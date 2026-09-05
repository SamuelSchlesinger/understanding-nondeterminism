# Coverage and examples: independent Accuracy / Completeness review

Reviewed 2026-09-04 under the author-review-revise skill. Scope: the joint
occupancy/alteration argument, sparse correction, density/support and sparse-tail
corollaries, and the explicit examples and stress tests. No manuscript edits
were made. The finite checks below are implementation checks; every asymptotic
conclusion assessed here has a separate analytical argument.

## Findings

No Error-severity or substantive Gap-severity finding was identified in the
reviewed mathematical claims for n >= 2. In particular, no counterexample was
found to the joint theorem or either principal example family.

### P1: Make the ordinary-input domain local to the whole coverage block

- Location: `sections/coverage.tex:50-76`, `:146-216` (selected restrictions and
  subsequent corollaries); compare the explicit n >= 2 assumptions at `:15`
  and `:104`, and `main.tex:98-99`.
- Severity: Polish (hypothesis precision, not a counterexample to the stated
  conclusions on their intended domain).
- Finding: The sparse lemma and master theorem explicitly assume n >= 2, but
  the selected-restriction lemma and subsequent corollaries do not repeat or
  inherit a section-wide assumption. Their proofs invoke those results, and the
  last corollary contains T/n. The global model allows nonnegative input counts;
  its n >= 2 convention is specifically attached to comparisons with 2^n/n.
  For example, the proof's n-1 point-indicator construction does not apply to
  the one-input point {0}, whose indicator requires one gate. This does not
  refute the selected-restriction inequality: the small-input cases can be
  handled directly.
- Suggested fix: State once that this coverage block assumes n >= 2, or add
  that hypothesis to the selected lemma and the three corollaries. Separate
  treatment of n <= 1 is unnecessary for the main results.

## Validated proof steps

### Selected restrictions and correction

`sections/coverage.tex:40-77` is sound on n >= 2.

For a predecessor u of v, its syntactic witness support is a subset of Y_v.
Consequently a restriction occurring at v has a well-defined occurring
restriction at u. Copying each gate once for every restriction occurring in H
therefore computes all selected evaluations by induction through the DAG.
Constants, ordinary-input outputs, witness-input outputs, irrelevant witness
bits, and empty witness supports cause no inconsistency.

For nonempty H, put S = sum_v N_v(H) and q = N_out(H) >= 1. The output OR
costs at most q-1. Each exceptional point costs n-1 gates, and adjoining its
indicator to the current output costs at most one further OR. Thus the linear
construction uses at most S+q-1+nR, below the displayed S+q+nR. If H is empty,
only the exceptional-set circuit is needed; no negative output-OR count is
used. Every selected evaluation implies g=1, so correcting the exact set of
missed positive inputs introduces no false positives.

Replacing the point list by a sparse circuit adds at most one final OR. For
R>0, nR/log_2(R+2) is bounded below by an absolute positive constant, so this
one gate is absorbed by an absolute implied constant. At R=0, the final OR is
unnecessary. Counting a gate output once in the gate sum and again in the
output term is intentional.

### Sparse synthesis, monotonicity, and Jensen

`sections/coverage.tex:14-38,127-137` and
`research/examples/stress-tests.md:51-85` contain a complete argument.

For 2 <= R <= 2^n, let k=floor(log_2 R) and B=ceil(n/k). On a block of b
bits, b=1 requires at most one negation, and b>=2 permits all four two-bit
indicators followed by prefix extensions. The latter cost is
sum_{j=2}^b 2^j = 2^(b+1)-4, hence at most 2^(b+1). Total library cost is at
most 2BR. Point conjunctions and their final OR cost R(B-1)+(R-1)=RB-1.
As B<=2n/k and log_2(R+2)<=3k, the stated loose absolute constant 18 is valid.
The cases R=0,1 are handled separately, so there is no division by zero at k=0.
At R=2^n the estimate is still valid, though it is deliberately suboptimal.

For q(z)=z/ln(z+2), direct differentiation gives

    q''(z) = [2z-(z+4)ln(z+2)] / [(z+2)^2 ln(z+2)^3].

Let D(z)=(z+4)ln(z+2)-2z. Then D(0)=4ln(2)>0,
D'(0)=ln(2)>0, and D''(z)=z/(z+2)^2>=0. Thus q is strictly concave on
[0,infinity). Multiplying by ln(2) proves the required base-two statement.
The monotonicity numerator ln(z+2)-z/(z+2) is positive at zero and has
derivative z/(z+2)^2>=0. Both facts used in the manuscript are valid globally,
including at zero.

### Joint occupancy/alteration theorem

`sections/coverage.tex:79-144` is sound.

For each gate restriction, the indicator that it appears in t independent
samples has mean 1-(1-p)^t. The number of distinct sampled witnesses is
irrelevant to this identity. For each positive input, the miss indicator has
mean (1-p_x)^t. Linearity of expectation gives exactly B_mu(t) and E_mu(t);
no independence between different restrictions, gates, or inputs is needed.

For a fixed absolute synthesis constant a, the sample-dependent upper cost is
S(H)+N_out(H)+a*n*phi(R(H)). Its expectation is at most
B_mu(t)+a*n*phi(E_mu(t)) by concavity. A sample no worse than its mean exists
because the sample space is finite. The same argument with linear correction
gives the coefficient-one linear bound. Sampling cost and a description of mu
are not circuit costs: only the eventual finite choices are hardwired.

The t=0 convention gives H empty, B=0, and E equal to the number of positive
inputs. Zero atom masses cause no problem. If E=0, nonnegativity of the integer
miss count forces R=0 almost surely. Direct input outputs and m=0 are also
consistent with the definitions.

### Density/support corollary

`sections/coverage.tex:146-171` is sound on n >= 2.

The specified sample size makes the probability of any missed positive input
at most 1/4. Markov bounds the event that total occupancy exceeds 2B by at most
1/2. The union of these failures has probability below one, providing one
sample with both properties. This does not select unrelated samples for the
different gates. A nonempty projection implies rho<=1 and, for t>=1, output
occupancy has expectation at least one. The empty projection is directly free.

At each gate the same sample has occupancy at most min(t,2^|Y_v|).
The chosen t is O(n/rho), proving the sum of per-gate minima. This particular
upper estimate also holds for nonuniform mu if its positive-row masses satisfy
the same rho assumption; restricting the displayed consequence to uniform
witnesses is conservative, not incorrect.

### Fiber profile and sparse-population inversion

`sections/coverage.tex:175-215` is sound on n >= 2.

A uniform sample of O(n*2^m/K) witnesses hits every fiber of size at least K
with positive probability. Any remaining positive inputs belong to the R_K
set. Monotonicity of phi bounds the cost of their actual missed subset by the
cost expression for R_K. This covers K=1 (there are no positive sparse fibers)
and K=2^m, as well as the absence of any dense fibers.

Write the profile bound with fixed constants a,b. Choose c large enough that
K=ceil(c*n*L) makes a*n*(s+1)*2^m/K <= T/2 whenever K<=2^m. Then

    R_K/log_2(R_K+2) >= (1/(2b))*T/n.

With q=T/n and d=1/(2b), this first gives R_K>=d*q because the logarithm is
at least one. Substituting back gives

    R_K >= d*q*log_2(2+d*q)
        >= d*min(1,d)*q*log_2(2+q).

The last inequality follows directly from concavity of the logarithm for
d<=1, and monotonicity for d>=1. This proves an absolute c' without hidden
growth assumptions on q. Since witness counts are integers, |W_x|<ceil(c*n*L)
implies |W_x|<=c*n*L; the ceiling is harmless. If K>2^m, then c*n*L>2^m,
so every positive input qualifies and direct sparse synthesis gives the same
inversion. The conclusion is necessary for hardness and supplies no converse.

### Cyclic family: exact structure and counts

`research/examples/index.md:38-173` is correct for N>=2, k>=3, ell>=2.

The cyclic difference map a -> (a_i XOR a_(i+1)) has precisely the even-parity
strings in its image and a kernel of two complementary strings. This follows
by propagating a_2,...,a_k from a_1 and checking the closing edge. It proves the
projection, every positive fiber size 2^ell, and 2^(n-2) positive inputs.
Distinct admissible u have disjoint fibers, and the proposed H picks one
representative in each. Therefore tau(f)=2^(k-1).

All stated essential-input witnesses are valid. The supplied verifier uses
N+3k+ell-1 gates; its lower bound N+2k+ell-1 follows from its essential inputs.
The projected function has all n=N+k ordinary inputs essential and a circuit
of exactly n-1 gates, giving C(g)=N+k-1. No exact optimality claim is made for
the supplied verifier.

The two cycle edges touching a_1 give four gates with two selected support
assignments each; the other 2(k-2) gates have four each. This gives 8k-8.
For prefix conjunctions, the selected sum is
sum_{j=2}^{k-1}2^j + 2^(k-1) = 3*2^(k-1)-4; the full sum is 3*2^k-8.
All remaining rows in the support table have their claimed supports. Adding
them, including the output OR, gives exactly

    A(H)    = N+ell+8k+3*2^k-15,
    A(full) = N+8k+4*2^k+2^(ell+1)+2^(k+ell+1)-14.

Exactly the k witness vertices a_i have fan-out two; all other counted vertices
have fan-out at most one. Hence r=k and s+1-n-m=k, with no dead gates or inputs.

On N=2^k, ell=k, A(H)=Theta(2^k), while the minimum of the named original
guarantee expressions is Theta(4^k): independent optimal-cover, full-support,
and shared-vertex expressions all have that latter order, while exhaustive
expansion and uniform density give Theta(8^k). Universal synthesis is larger.
Thus the expression ratio is Theta(2^k), exponential in k (and in witness
count m=2k), not a lower bound on any optimally simplified compiler or on C(g).
The actual projection already has complexity 2^k+k-1. The text makes this
limitation explicit, including the possibility that the newer structural
bounds also handle this family efficiently.

Replacing each selected support count 2^j by
d_j(t)=2^j*(1-(1-2^-j)^t) reproduces the exact B_mu formula. The four extra
d_(k-1) terms are the final equation conjunction, two final ANDs, and output
term. Positive-row mass is 1/h, h=2^(k-1), giving the stated E_mu exactly.
The displayed t gives n*E_mu<=exp(-1), while B_mu<=A(H)+1. Consequently the
occupancy guarantee is O(2^k) even though t=Theta(4^k).

### Equal row masses, different occupancy objectives

`research/examples/stress-tests.md:6-31` proves its asymptotic claim uniformly
over every integer t, rather than inferring it from finite checks.

The h positive fibers partition the witnesses with odd-parity b. Both
distributions have mass 1/h in every such row, the largest possible minimum.
For w, there are M=2^(2k-1) equally probable output restrictions. If t>=M/8,
output occupancy alone is at least M*(1-exp(-1/8))=Omega(4^k). If t<M/8,
using ln(1-p)>=-p/(1-p), p=1/h<=1/4, yields

    E_w(t) >= exp((N+k-2)*ln(2)-N/6).

After division by 4^k, the logarithm of this lower bound is
N*(ln(2)-1/6)-(k+2)*ln(2), which tends to infinity since N=2^k. Thus the low-t
case is uniformly larger than the desired order. Finally, bounded occupancy
and E_w(t)->0 give the matching upper bound for the infimum. This validates
inf_t F_w(t)=Theta(4^k), versus O(2^k) on H, despite identical row-mass vectors.

### Unique row-mass optimizer versus compilation cost

`research/examples/index.md:175-225` and
`research/examples/stress-tests.md:33-49` are correct.

Every ordinary input excludes exactly one witness label: its pair of block
parities. Every label occurs 2^(2L-2) times. Therefore minimum row mass equals
1-max_a mu(a), whose unique maximizer is the uniform distribution. Essential
input counting also proves C(f_L)=2L+1. Projection is the free constant one.

For uniform mu, each of the 2L singleton-support gates has expected occupancy
2*(1-2^-t); the output gate and output term each have occupancy
4*(1-(3/4)^t). This proves B_u, and every row has miss probability 4^-t,
proving E_u. For integer t<L, nE_u>=8L; for t>=L,
B_u>=4L*(1-2^-L). Hence the lower bound holds for every sample count.

For equal mass on 00 and 01, y_1 is fixed and only two output labels occur.
For t>=1, this gives B_v=L+(2L+4)*(1-2^-t). Exactly half the ordinary inputs
can be missed, each with probability 2^-t, giving E_v=2^(2L-1-t). The stated
t=2L+ceil(log_2(2L)) makes nE_v<=1/2. For L>=8,
3L+4.5 < 4L*(1-2^-L), so this distribution beats every uniform sample count.
The special t=0 formula is correctly separated in the implementation.

For every fixed sparse-correction constant c>0, low sample counts t<L/2 have
E_u>=2^L and hence a correction term growing faster than L. At t>=L/2,
occupancy is at least 4L-o(1). The uniform infimum is therefore 4L+O(1), while
letting t grow for the pair distribution gives at most 3L+4+o(1). Thus the
sparse-objective gap is analytical and does not depend on assigning an
unjustified numerical value to a big-O constant. All of these are statements
about the supplied representation and cost objectives, not its constant
projection's actual complexity.

### Additional stress tests

The equal-profile A and Q constructions have the stated two-element rows,
essential inputs, linear-size supplied circuits, and covers 1 versus 2^k.
Their uniform row masses agree, while their optimized minimum masses do not.

The high-average-density example has density
1-2^-r+2^(-r-m) and requires every witness because of its z=0 singleton rows.
Its r+2m-1 gate circuit matches the essential-input lower bound; projection
remains constant one.

The two-coordinate incompatible-cover example correctly gives a sum of
separate minima equal to 2 and minimum of the common sum equal to 3.
The entropy example's masses sum to one, entropy tends to zero, and expected
rare-state occupancy at t=q^3 is at least (1-exp(-1))*q^2. A proper subset of
cycle constraints forms paths and is satisfiable even when the full odd-parity
cycle is inconsistent. These examples refute exactly the stronger claims they
identify, without claiming circuit hardness.

## Implementation checks actually run

Both commands returned exit code 0:

    python3 scripts/check_coverage.py
    python3 research/examples/data/check_examples.py

First script: 362 sparse support sets; 896 shared-restriction/correction
circuit/sample pairs; 140 exact rational occupancy/miss cases including biased
distributions, zero masses, t=0, and numerical Jensen checks.

Second script: 4 cyclic truth-table instances and 25,600 input/witness pairs;
exact cyclic support and fan-out counts for k=3,...,10; equal-profile rows for
k=2,...,6; rational distribution checks for L=8,16,32,64; 141 sparse-circuit
truth tables; and the incompatible-cover example. The relevant script bodies
and their claim headers were also read. They do not search for minimal
circuits and are not used here as asymptotic evidence.

## Scope and attribution limits

The proof checks above establish the displayed constructions and comparisons.
They do not establish literature priority. The example documents explicitly
leave priority unestablished, identify their projections as easy, and avoid
turning large compiler expressions into lower bounds. No obvious novelty or
intrinsic-hardness overclaim was found within this review's assigned scope.
