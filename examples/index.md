# Explicit examples and adversarial comparisons

Parent: [research record](../index.md). These are complete analytic
constructions; priority is unestablished. The finite checks below verify the
specified circuits and numerical formulas, not asymptotic claims or circuit
optimality. [Stress tests](stress-tests.md) contain the sparse-correction audit
and counterexamples to stronger conjectures.

The first family makes the coupled guarantee exponentially smaller than every
applicable original-note structural, full-support, and coverage expression.
The second shows that the unique best distribution for minimum row mass can
give a strictly worse compilation objective. Both projections are explicitly
easy. These are comparisons of upper-bound guarantees, not new circuit lower
bounds or evidence for hard projections.

## Cost convention

Fix a supplied B2 circuit with gates V and witness support Y_v at each gate.
For a nonempty selected witness set H, write

    A(H) = sum_(v in V) |projection_(Y_v)(H)|
           + |projection_(Y_out)(H)| - 1.

The shared-restriction compiler has size at most A(H) when H covers every
positive input. The full-support expression substitutes 2^|Y_v| at every
gate and output. For sampling, use B_mu(t) and E_mu(t) from the
[occupancy theorem](../coverage/index.md), so that

    F_mu(t) = B_mu(t) + n E_mu(t)

is the stated convenient bound. B counts one extra output term: the actual
OR of a nonempty set of output copies requires one fewer gate. All exact
counts below concern the specified unsimplified compilers; simplification can
only improve them.

## A cyclic family separating all the original compiler bounds

Let N >= 2, k >= 3, ell >= 2. Ordinary inputs are z in B^N and u in B^k;
witnesses are a in B^k and b in B^ell. Indices on a are cyclic. Set

    T(u,a) = AND_(i=1)^k [u_i = a_i XOR a_(i+1)],
    f(z,u;a,b) = PAR_N(z) AND T(u,a) AND PAR_ell(b).

The supplied circuit uses an XOR chain for each parity, one XOR followed
by an XNOR for each equation, a prefix AND chain for the k equations, and
then AND(PAR_N(z),T), followed by its AND with PAR_ell(b). Hence

    n = N+k,                 m = k+ell,
    s = (N-1)+2k+(k-1)+(ell-1)+2 = N+3k+ell-1.

Every gate lies in the output cone. Every input is essential: start with an
accepting assignment; changing one z or b bit toggles the required parity,
changing one u bit violates one equation, and changing one a bit violates
two equations. Therefore the essential-input lower bound gives

    N+2k+ell-1 <= C(f) <= N+3k+ell-1.

No exact optimality claim is made for this supplied verifier. Its size is
within k gates of this elementary lower bound; there is no padding or dead
subcircuit.

### Projection and cover geometry

XORing the k equations shows that T can hold only when PAR_k(u)=0.
Conversely, after fixing a_1, the first k-1 equations uniquely determine
a_2,...,a_k. The last equation holds exactly when PAR_k(u)=0. Consequently

    g(z,u) = PAR_N(z) AND NOT PAR_k(u).

Both values of a_1 give witnesses, and they are bitwise complements. Each
positive fiber therefore has size 2 * 2^(ell-1) = 2^ell. The number of
positive ordinary inputs is 2^(N-1) 2^(k-1) = 2^(n-2).

Distinct even-parity u values have disjoint witness fibers. There are
2^(k-1) such u, so every cover has at least that many witnesses. Fix one
odd-parity b0 and set

    H = {(a,b0): a_1=0},       h = |H| = 2^(k-1).

For each even u, exactly one member of H satisfies T. Thus tau(f)=h.

All n ordinary inputs are essential in g. Its two parity chains followed
by the B2 operation (p,q) -> p AND NOT q use N+k-1 gates. Thus, in this
case, the elementary input bound proves the exact value

    C(g) = N+k-1.

This easy projection is part of the construction, not an inferred lower
bound from the cost of any compiler.

### Exact support counts

The following table charges the chosen prefix association. For its j-th
conjunction gate, 2 <= j <= k, the support is {a_1,...,a_(j+1)} when j<k,
and all k a bits when j=k.

| Circuit part | Selected H copies | Full-support copies |
| --- | ---: | ---: |
| PAR_N(z) | N-1 | N-1 |
| k XORs and k XNORs | 8k-8 | 8k |
| Prefix conjunction of equations | 3*2^(k-1)-4 | 3*2^k-8 |
| PAR_ell(b) | ell-1 | 2^(ell+1)-4 |
| AND(PAR_N(z),T) | 2^(k-1) | 2^k |
| Final AND | 2^(k-1) | 2^(k+ell) |
| Final OR of restrictions | 2^(k-1)-1 | 2^(k+ell)-1 |

For the second row, two cycle edges touch the fixed bit a_1. Each of their
XOR and XNOR gates has two distinct assignments under H; each remaining
edge has four. This gives 2*(2*2+(k-2)*4)=8k-8. For the third row the
selected count is sum_(j=2)^(k-1) 2^j + 2^(k-1); the full count is
sum_(j=2)^(k-1) 2^(j+1) + 2^k. The parity-chain counts follow by summing
the support sizes 2^2,...,2^ell. Adding the rows proves

    A(H) = N+ell+8k+3*2^k-15,
    A(full) = N+8k+4*2^k+2^(ell+1)+2^(k+ell+1)-14.

Exactly k vertices counted by the original shared-vertex theorem have
fan-out at least two: the a_i witness inputs, each used by two XORs.
All gate outputs have fan-out one. Thus r=k, and the excess inequality is
tight: s+1-n-m=k.

### An exponential comparison of guarantees

Now set N=2^k and ell=k. The explicitly calculated expressions are

| Quantity or original guarantee | Value or asymptotic expression |
| --- | --- |
| Supplied verifier size s | 2^k+4k-1 |
| Actual projected complexity C(g) | 2^k+k-1 |
| Coupled selected-cover count A(H) | 4*2^k+9k-15 |
| Independent optimal-cover count h(s+1)-1 | (1/2)*4^k+2k*2^k-1 |
| Full-support count A(full) | 2*4^k+7*2^k+8k-14 |
| Original shared-vertex expression (s+1)2^r | Theta(4^k) |
| Uniform-density expression (s+1) min(2^m,n/rho) | Theta(8^k), rho=2^-k |
| Exhaustive expansion expression (s+1)2^m | Theta(8^k) |
| Universal synthesis expression 2^n/n | 2^(2^k+k)/(2^k+k) |

The original size-dependent exponent and its three-way refinement do not
give a smaller order than the displayed shared-vertex expression here:
min(n,m,s+1-n-m)=k. Thus the coupled expression improves on the minimum
of the original guarantees by a factor Theta(2^k).

This statement compares expressions in proved upper bounds. It does not
assert that executing an original compiler and then simplifying it requires
Theta(4^k) gates. The new quantified-core or elimination bounds may also
capture this example efficiently. The example establishes what simultaneous
selection and support sharing add to the original list of guarantees.

### Exact occupancy realization

Let mu be uniform on H and, for t>=1, write

    d_j(t) = 2^j (1-(1-2^-j)^t).

The same support table, now using expected occupancy, gives exactly

    B_mu(t) = N+ell-2 + 4d_1(t) + 2(k-2)d_2(t)
              + sum_(j=2)^(k-1) d_j(t) + 4d_(k-1)(t),
    E_mu(t) = 2^(n-2) (1-1/h)^t.

In particular B_mu(t)<=A(H)+1. Choosing

    t = ceil(h*((n-2) ln 2 + ln n + 1))

gives n E_mu(t)<=exp(-1), and hence F_mu(t)<=A(H)+1+exp(-1).
Although t is Theta(4^k) on the diagonal N=2^k, the circuit expression is
O(2^k), because repeated support assignments incur no further copies.

Minimum row mass alone does not even distinguish good and bad distributions
on this family. Uniform sampling on all witnesses with odd-parity b also
achieves the maximum minimum row mass 1/h, but its optimized F_mu is
Theta(4^k). Uniform sampling on H gives O(2^k). A full analytic proof of
the former claim is in [the stress tests](stress-tests.md#equal-coverage-distributions-can-have-an-exponential-cost-gap).

## A unique coverage optimizer that is worse for circuit cost

Let L>=8, n=2L, m=2, and build

    f_L(x;y) = (y_1 XOR x_1 XOR ... XOR x_L)
               OR (y_2 XOR x_(L+1) XOR ... XOR x_(2L)).

Each chain starts with its witness bit. There are L gates with support
{y_1}, L with support {y_2}, and one OR with support {y_1,y_2}. Every input
is essential, so the input bound proves C(f_L)=2L+1 exactly. Its projection
is nevertheless g=1, which has a zero-gate circuit.

For a fixed x, the fiber excludes exactly its two block parities. All four
excluded labels occur equally often. For any witness distribution mu,

    min_x mu(W_x) = 1-max_(a in B^2) mu(a).

The maximum is 3/4, and its unique optimizer is the uniform distribution u.
For every integer t>=0 the occupancy and miss expressions are

    B_u(t) = 4L(1-2^-t) + 8(1-(3/4)^t),
    E_u(t) = 2^(2L-2t).

If t<L, then n E_u(t)>=8L. If t>=L, then
B_u(t)>=4L(1-2^-L). Therefore, for all integer t>=0,

    F_u(t) >= 4L(1-2^-L).

Instead put equal mass on 00 and 01; call this distribution v. Its minimum
row mass is only 1/2. For t>=1, its first witness support has occupancy one,
and both its second support and output support have expected occupancy
2(1-2^-t). Only the two excluded labels in {00,01} can be missed. Hence

    B_v(t) = L+(2L+4)(1-2^-t),
    E_v(t) = 2^(2L-1-t).

At t=2L+ceil(log_2(2L)), n E_v(t)<=1/2 and B_v(t)<=3L+4. Thus

    F_v(t) <= 3L+4.5 < 4L(1-2^-L) <= inf_(j>=0) F_u(j).

The strict middle inequality holds for L>=8: 4L/2^L<=1/8 and
L-4.5-1/8>0. The unique optimizer of minimum row mass is therefore not an
optimizer of the compilation objective. Its best expression is 4L+O(1),
whereas the displayed alternative gives 3L+O(1). The gap survives the sparse
correction objective for every fixed positive implied constant; see the
[analytic argument](stress-tests.md#the-distribution-tradeoff-survives-sparse-correction).

These statements concern the supplied circuit. Moving each witness to the
end of its XOR chain changes the support profile without changing f_L or
its optimal verifier size. The example does not define an intrinsic cost
of a witness distribution independent of the verifier representation.

## Equal fiber profiles, exponentially different covers

For k>=2 take ordinary u in B^k and witnesses (c,v) in B x B^k. Define

    A(u;c,v) = [c=0 AND v=0^k] OR [c=1 AND v=u],
    Q(u;c,v) = [v_i = u_i XOR c for every i].

Their respective rows are

    W^A_u = {(0,0^k),(1,u)},
    W^Q_u = {(0,u),(1,NOT u)}.

Both have exactly two witnesses for every one of their 2^k ordinary inputs;
both projections are 1; all ordinary and witness variables are essential
in each verifier. B2 circuits of sizes at most 3k+1 and 3k-1, respectively,
follow from a zero-test, equality and a three-gate multiplexer for A, and
k XORs, k XNORs and k-1 ANDs for Q. No optimal-size assertion is needed.

All A rows contain (0,0^k), so tau(A)=1 and maximum minimum row mass is 1.
The Q rows partition the 2^(k+1) witnesses, so tau(Q)=2^k and maximum
minimum row mass is 2^-k. Under the uniform distribution their row masses
are identical, namely 2^-k. Thus even the entire fiber-size profile fails
to determine common-cover geometry or the optimized row mass.

## Verification

[check_examples.py](data/check_examples.py) checks the displayed cyclic
verifier against direct evaluation on four small instances (25,600 ordinary
input/witness pairs), exact support and fan-out counts for k=3,...,10,
the equal-profile constructions, rational distribution inequalities,
141 sparse-correction truth tables, and the gatewise incompatibility example.
Run `python3 research/examples/data/check_examples.py` from the project root.
The seed 20260904 is used only for finite sparse-set selection. The analytic
proofs above, rather than these finite checks, establish the asymptotic claims.

## Local References

The external literature comparison belongs to
[literature and scope](../literature/index.md). All example-specific claims
here are proved from their displayed definitions. No new external reference
or unverified priority attribution is used.
