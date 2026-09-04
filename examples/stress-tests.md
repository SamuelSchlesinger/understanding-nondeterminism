# Stress tests for occupancy, correction, and coverage

Parent: [examples and worked families](index.md). These are analytic checks
and counterexamples, not conjectured complexity lower bounds.

## Equal-coverage distributions can have an exponential cost gap

Use the cyclic family with N=2^k and ell=k, and h=2^(k-1). Its h distinct
positive fibers are disjoint, so their minimum mu-mass is at most 1/h.
Both uniform measure on H and uniform measure w on all witnesses with
odd-parity b achieve equality. The latter has M=2^(2k-1) equally probable
witnesses; all are distinguished by the syntactic output support. Therefore

    B_w(t) >= M(1-(1-1/M)^t),
    E_w(t) = 2^(n-2)(1-1/h)^t.

For t>=M/8 the first expression is at least M(1-exp(-1/8))=Omega(4^k).
For t<M/8, use k>=3 and ln(1-p)>=-p/(1-p), with p=1/h<=1/4. Then

    E_w(t) >= 2^(n-2) exp(-N/6),

because (M/8)/h=N/8. Since n=N+k and N=2^k, this quantity eventually
exceeds any constant multiple of 4^k. Thus F_w(t)=Omega(4^k) for every t.
Conversely, B_w(t) is bounded by the full-support expression O(4^k), and
E_w(t) tends to zero as t tends to infinity. This proves

    inf_t F_w(t) = Theta(4^k),

while uniform sampling on H gives O(2^k), despite exactly the same mass in
every positive row. The gain depends on support concentration beyond the
whole vector of row masses, not merely beyond its minimum.

## The distribution tradeoff survives sparse correction

For the two-chain family, let h(z)=z/log_2(z+2), and fix any constant c>0.
Consider F_mu,c(t)=B_mu(t)+c n h(E_mu(t)). For the uniform distribution,
if t<L/2 then E_u(t)>=2^L and n h(E_u(t)) grows faster than L. Monotonicity
of h follows from

    d/dz [z/ln(z+2)] = (ln(z+2)-z/(z+2))/ln(z+2)^2 > 0.

Indeed the numerator is ln(2)>0 at zero and its derivative is z/(z+2)^2.
If t>=L/2 then B_u(t)>=4L(1-2^(-L/2))=4L-o(1). Taking t large also gives
the upper bound 4L+8+o(1), so inf_t F_u,c(t)=4L+O(1).

For the pair distribution, taking t sufficiently large gives
F_v,c(t)<=3L+4+o(1). Thus for every fixed c>0 there is a strict gap for all
sufficiently large L. This conclusion does not depend on fixing an
unjustified numerical value for the sparse-synthesis big-O constant.

## Sparse correction and the Jensen step are valid

Let an n-input function, n>=2, have exactly R positive points. For R=0 it
is the zero constant. For R=1 a B2 point indicator costs n-1 gates. For
2<=R<=2^n choose k=floor(log_2 R), partition the n inputs into
B=ceil(n/k) nonempty blocks, and build every assignment indicator on each
block. A b-bit block needs at most 2^(b+1) gates: for b=1 a negation
suffices; for b>=2 compute all four two-bit indicators, then extend each
partial indicator by the positive and negative literal of the next bit.

Since k<=n and 2^k<=R, the total block-library cost is at most 2BR.
Each positive point is an AND of B library outputs. All point ANDs and
their final OR cost R(B-1)+(R-1)=RB-1. The total is at most 3BR-1.
Using B<=2n/k and log_2(R+2)<=3k gives the explicit, deliberately loose
bound 18nR/log_2(R+2). For R=1 the same constant also works.

The nonlinear expectation step requires concavity; it is not a consequence
of the synthesis bound alone. For q(z)=z/ln(z+2), direct differentiation
gives

    q''(z) = [2z-(z+4)ln(z+2)] / [(z+2)^2 ln(z+2)^3].

Put D(z)=(z+4)ln(z+2)-2z. Then D(0)=4ln 2>0,
D'(0)=ln 2>0, and D''(z)=z/(z+2)^2>=0. Thus D(z)>0 on z>=0, proving
q''(z)<0 there. Multiplication by ln 2 yields concavity of
h(z)=z/log_2(z+2), with h(0)=0. Therefore

    E[h(R_sample)] <= h(E[R_sample])

is justified, including samples with no missed positive inputs.

The finite script builds the actual block circuits for n=2,...,8 and checks
their truth tables. It does not replace this proof or establish optimality
of sparse synthesis. Universal synthesis can be substantially better when
R is large.

## Counterexample: high average density need not yield a small cover

Let ordinary inputs be v in B^m and z in B^r, m>=2, r>=1, and define

    f(v,z;y) = (OR_j z_j) OR [v=y].

For z!=0 every witness accepts; for z=0 only y=v accepts. The mean row
density is

    1-2^-r + 2^(-r-m),

which approaches one with r. Nevertheless the singleton rows force every
one of the 2^m witnesses into any cover, so tau(f)=2^m. All variables are
essential, and a circuit of r+2m-1 gates matches the essential-input lower
bound. The projection is the constant one. Thus average density alone
does not control cover size; this is not a hardness claim.

## Counterexample: gatewise optimal covers need not be compatible

For f(x_1,x_2;y_1,y_2)=[(x_1,x_2)!=(y_1,y_2)], every pair of distinct
witnesses is a cover, but no singleton is. For support {y_1}, the cover
{00,01} has occupancy one. For support {y_2}, {00,10} has occupancy one.
No cover has occupancy one on both supports, because both bits would then
be fixed throughout H. Consequently

    sum_(j=1)^2 min_(H a cover) |projection_(y_j)(H)| = 2,
    min_(H a cover) sum_(j=1)^2 |projection_(y_j)(H)| = 3.

One may bound each gate by min(t,2^|Y_v|) for the same sampled H. One may
not choose an unrelated optimal covering distribution or set separately
for each gate and assume these choices can be compiled together.

## Counterexample: entropy does not bound occupied states by 2^entropy

For an integer q>=2, put mass 1-1/q on one support assignment and mass
1/q^3 on each of q^2 other assignments. A witness support with at least
q^2+1 assignments suffices. Its Shannon entropy in bits is

    H = h_2(1/q)+(2 log_2 q)/q -> 0.

At t=q^3 samples, the expected number of occupied rare assignments is

    q^2(1-(1-1/q^3)^(q^3)) >= (1-exp(-1))q^2.

It tends to infinity although 2^H tends to one. Thus replacing support
occupancy by 2^Shannon-entropy, independently of t, is false. The example
does not refute an entropy bound that explicitly tracks t or a tail error.

## Counterexample: proper subfamilies can conceal a global inconsistency

In the cyclic verifier, take any odd-parity u. The full k equations have
no simultaneous solution, as their XOR requires PAR_k(u)=0. Every proper
subset of the equations is satisfiable: deleting an edge breaks the cycle
into paths, and along each path one can freely choose its first a bit and
propagate the required values. This explicitly shows why separately
projecting locally consistent witness constraints loses information.

## Local References

All claims here are derived from the displayed definitions or proved
directly. Related prior frameworks are compared in
[literature and scope](../literature/index.md); no literature novelty claim
is made for any elementary construction or counterexample.
