# Necessary structure near the five-gates-per-witness boundary

Parent: [structural simulation](index.md).

This is a consequence of the existing compiler, not a lower-bound construction
or a claim that its numerical exponent is optimal. All statements concern the
same normalized supplied B2 circuit as the structural theorem, with q>=1
and a witness-dependent gate as output. Constant, ordinary-only, and
designated-input outputs have already been handled separately.

## Finite stability before taking an asymptotic limit

Fix delta>0, alpha=1/3+delta, and choose A_delta>=1 large enough that

    C(g) <= p + A_delta(q+1) 2^min{h,b,alpha*kappa}.

Let t>=0 and suppose C(g)>=p+A_delta(q+1)2^t. Define

    Delta = q+1 - (2+1/alpha)t.

Then Delta>=0 and there are nonnegative e_h,e_b,e_k such that

    h=t+e_h,  b=t+e_b,  kappa=t/alpha+e_k,
    e_h+e_b+e_k <= Delta.

Proof: comparing the lower and upper bounds forces each of h,b,alpha*kappa
to be at least t. Subtract these three lower bounds from
h+b+kappa<=q+1. This is the entire finite argument. In particular each
parameter's excess over its forced lower bound is at most Delta.

The constant A_delta depends on the fixed delta. This lemma does not replace
that constant with a uniform one or assert an endpoint exponent.

## Ordinary inputs and unused interface capacity

Let a be the number of used ordinary inputs. Then

    |a-h| <= p.

Indeed h<=a+p because interface signals are among those vertices. For the
opposite inequality consider the undirected graph on the a ordinary inputs
and p witness-independent gates, retaining their wires. Every connected
component contains an interface signal: all vertices are in the output cone,
and every directed path to a witness-dependent output eventually leaves this
independent part through the interface. Thus its number c of components is
at most h. It has at least a+p-c edges and at most 2p edges, so a<=p+h.

Let r count ordinary-signal input pins of the q dependent gates, with
multiplicity, and let U=2q-(ell+r) be the total unused fanin capacity. Then

    q+1-h-b-kappa = U+(r-h).

Both terms on the right are nonnegative. This is obtained by substituting
kappa=ell-q-b+1. Thus a small budget slack forces both nearly binary gates
and little repetition of ordinary signals at the interface. It says nothing
by itself about whether their values are semantically independent.

## Critical families must saturate several independent estimates

Consider a sequence with m declared witness bits tending to infinity and

    s=p+q,  s+1=(5+o(1))m,  log_2 C(g)>=m-o(m).

The amplification condition C(g)>=2^m(s+1)/L implies this logarithmic lower
bound whenever log L=o(m).
It is a hypothetical lower-bound condition, not an asserted example.
Then

    p=o(m),  q=(5+o(1))m,
    a=(1+o(1))m, h=(1+o(1))m, b=(1+o(1))m,
    kappa=(3+o(1))m,
    U+(r-h)=o(m).

Proof: p=O(m), so subtracting p from C(g) preserves the lower bound
log_2(C(g)-p)>=m-o(m). For each fixed delta, polynomial prefactors have
logarithm O_delta(log m), so the compiler gives

    h>=m-o(m),  b>=m-o(m),  kappa>=m/alpha-o(m).

Use p+h+b+kappa<=s+1. First fix delta, then let delta tend to zero.
It follows that limsup p/m<=3-1/alpha for every delta, hence p=o(m).
The same comparison bounds limsup h/m and b/m by 4-1/alpha, tending to one.
Their lower bounds give h/m,b/m->1. The budget then bounds limsup kappa/m
by three, while the fixed-delta lower bounds give liminf at least three.
The input/interface inequality proves a/m->1. Finally the slack identity
proves U+(r-h)=o(m). All limit passages hold delta fixed before taking m->infinity.

## Every legal residual kernel is large and near the width bound

For any legal equality splitting and reduction sequence of the manuscript,
let K be its resulting simple cubic kernel, N=|V(K)|, and let cw(K),pw(K)
be its minimum cutwidth and pathwidth. Under the same critical-family
hypotheses, for all sufficiently large indices K is nonempty, and

    N=(6+o(1))m,
    kappa(K)=(3+o(1))m,
    kappa-kappa(K)=o(m),
    cw(K)=(1+o(1))m,  pw(K)=(1+o(1))m.

Proof: an empty kernel would give C(g)=O(s), contradicting the hypothesis.
Using an order of minimum cutwidth in the finite compiler bound gives

    C(g)<=p+400(q+1)2^cw(K),

and therefore cw(K)>=m-o(m). On the other hand N<=2(kappa-1)<=6m+o(m).
The lower bound on cutwidth and cw(K)<=3N/2 imply N->infinity. The cited
cubic pathwidth theorem and the proved conversion give, for each
fixed eta>0 and all sufficiently large kernels,

    cw(K)<=pw(K)+2 <= (1/6+eta)N+2.

Hence N/m has liminf at least 1/(1/6+eta) for every fixed eta, and so tends
to six. The cubic cycle identity gives kappa(K)=N/2+1. Comparing with kappa
proves the sublinear cycle loss. The same inequalities sandwich both width
parameters between m-o(m) and m+o(m).

This holds for every selected sequence of legal reductions along the family:
if some sequence gave a smaller kernel or width, its valid circuit would
contradict the assumed C(g). It does not require finding an optimal order.

## Relation to witness multiplicities

Keep all critical-family hypotheses. If additionally
C(g)>=T=2^m(s+1)/L with L>=1, use the existing
sparse-population corollary on the original n ordinary inputs and m declared
witness bits. It forces at least

    Omega((T/n) log(2+T/n))

positive inputs with O(nL) declared witnesses. If unused witness bits are
removed, each positive fiber is divided by exactly 2^(m-b); the threshold
must be divided by the same factor. Declared and used witness multiplicities
must not be mixed silently. The critical-family theorem already gives
m-b=o(m); enumeration more directly gives m-b<=log L for this stronger
amplification hypothesis.

Together these are necessary conditions for a near-critical hard family.
These deductions establish no converse. In particular a graph realization with
these budgets is not an example satisfying the C(g) lower-bound hypothesis.

The external graph theorem is already attributed in the
[structural proof](proof.md); this document adds no new external claim.
