# Proof of quantified-only tensor elimination

[Return to structural results](index.md).
All circuit costs below count gates over the full binary basis B2.
Inputs, constants, wiring, and unrestricted fan-out are free.
This is an analytic proof. The accompanying finite compiler checks are auxiliary.

## 1. The circuit and its quantified graph

Prune to the output cone and propagate constants without increasing size.
If the output has no syntactic witness dependency, retain its existing circuit.
If it is a witness input, its existential projection is the constant one.
Hence assume that the output is a witness-dependent gate.

Partition gates by syntactic witness dependency: there are p independent gates
and q dependent gates, with q >= 1. Compute the p independent gates once.
Let h_1(x),...,h_h(x) be the distinct independent signals entering dependent
gates, including direct ordinary inputs. Let b be the used witness count.
Let ell count dependent-gate input pins supplied by witnesses or dependent
gates. Every dependent gate has at least one such pin. In particular,
each has at most one independent input pin and ell+h <= 2q.

Give every witness and dependent gate output one consistency variable.
For each dependent gate v, include a factor

    A_v(z_v, quantified predecessor values; x)
      = [z_v = op_v(predecessor values)].

Include a unary factor [z_out=1]. Ordinary signals appear as coefficients;
they are not consistency variables or factor-graph vertices. Connect each
factor to its consistency variables, retaining separate occurrences if needed.
This bipartite incidence multigraph H has

    |V(H)| = 2q+b+1,    |E(H)| = q+ell+1.

It is connected: every dependent gate and used witness has a directed path
through dependent gates to the output. Subdividing the corresponding directed
wires through defining factors and adding the output factor preserves that
connectivity. Consequently its cycle rank is

    kappa = |E|-|V|+1 = ell-q-b+1 >= 0,
    h+b+kappa = h+ell-q+1 <= q+1.                 (1)

Its factor conjunction, existentially quantified over all consistency
variables, equals the original projection. A witness assignment extends
uniquely to actual gate values. Conversely every satisfying consistency
assignment has those actual values, by topological induction.

For a full normalized cone with s=p+q gates, a ordinary inputs and b witnesses,
deleting the independent part gives the connected underlying quantified
graph with q+b vertices and ell edges. Deleting vertices and edges cannot
increase cycle rank: a spanning forest, or the binary cycle space, proves this.
Thus, also using the full cone's wire bound E_full <= 2s,

    kappa <= E_full-(s+a+b)+1 <= s+1-a-b.        (2)

## 2. Turn consistency into binary edge indices

Replace every incidence by its own binary edge index. At each variable vertex
put the equality tensor, whose entries are one exactly when all its incident
indices agree. A degree-one equality tensor has both entries one.
Gate factors have rank at most three. Fixing their quantified indices leaves
either a constant, h_i(x), or its complement, because at most one input is
independent. At most h complement gates therefore prepare all their entries.
The output factor has the constant entries zero and one.

All entries throughout the proof are wires of ordinary Boolean circuits.
They are never arbitrary predicates supplied by an oracle.
Tensor contraction means OR over a shared binary index of the AND of entries.
Distributivity proves the contraction identity pointwise for each fixed x.

Replace an equality node of degree D>=4 by a tree of D-2 ternary equality
nodes, with the original incidences as its leaves. The internal edge values
are uniquely determined whenever the external indices agree. This adds D-3
vertices and D-3 edges, so preserves connectivity and cycle rank.
Keep equality nodes of degree at most three.
Writing N0 for the resulting vertex count, the sum of the old variable
degrees is q+ell+1. The number of new equality nodes is at most that sum.
Hence

    N0 <= q+1+(q+ell+1) <= 4q+2.                (3)

The resulting tensor graph has maximum degree three, counting each loop twice.

## 3. Low-degree elimination and multigraph issues

Maintain a scalar accumulator, initially one. Repeatedly apply these rules.

* A rank-zero tensor multiplies the accumulator and is removed: one AND gate.
* A loop at a tensor identifies two ports with the same bit. Replace it by
  the OR of the two diagonal entries. There are at most two remaining entries,
  so this costs at most two gates. Summing all four entries would be wrong.
* At a loop-free vertex of degree one or two, contract one incident edge into
  its neighbor. The new rank is at most three. Each of at most eight entries
  uses two ANDs and one OR, for at most 24 gates. If both vertices had another
  common edge, that other edge becomes a loop; process it by the preceding rule.
* If two remaining cubic vertices share two edges, contract both simultaneously.
  The result has rank two. Its four entries each use four ANDs and three ORs,
  for 28 gates. If they share three edges, the scalar uses eight ANDs and seven
  ORs, for 15 gates. These rules remove parallel edges without subdivision loss.

Each operation reduces |V|+|E|, costs at most 32 gates, and never increases
degree. Initially |E| <= 3N0/2, so all reductions cost at most 80N0 gates.
This deliberately loose constant includes scalar accumulation.
No factor table grows beyond eight entries during these reductions.

Contracting a non-loop edge preserves cycle rank. Deleting a loop decreases
it by one; contracting k parallel edges decreases it by k-1. Removing an
isolated scalar preserves the rank when components are counted. Connectivity
of a remaining nonempty component is preserved by each operation.
Thus the original connected graph reduces either to no vertices, or to a
connected simple cubic graph K. In the latter case, with N=|V(K)|,

    |E(K)|=3N/2,
    N=2(kappa(K)-1) <= 2(kappa-1).              (4)

This is why charging all degree-two subdivisions to a cubic vertex count is
unnecessary. It is also why loops and parallel edges must be contracted
semantically rather than silently removed from an unweighted graph.

## 4. A path decomposition yields cutwidth at most its width plus two

Suppose a simple cubic graph has path decomposition B_1,...,B_L of width k.
Represent a vertex v by the interval

    I_v=[first(v)-1/4, last(v)+1/4].

At most k+1 such intervals contain any real point. For each edge e=uv choose
a distinct timestamp t_e in the interior of I_u intersect I_v, which is
nonempty because u and v occur in a common bag. Rational timestamps with
polynomial bit length suffice. Put v at the median t_v of its three incident
edge timestamps, and order vertices by t_v, breaking ties arbitrarily.

Consider first a cut at a point t different from all vertex medians.
For each crossing edge e=uv with t_u<t<t_v, assign it to u if t_e>t and
to v if t_e<t. Choose t away from the edge timestamps as well.
The assigned vertex's interval contains t, because it contains both its
median and its incident edge timestamp. Each vertex receives at most one
edge: a cubic vertex has only one incident timestamp strictly above its
median, and only one strictly below it. The edges therefore inject into
at most k+1 active intervals.

For a cut between tied medians, at most two vertices can be tied. Indeed,
each median is one of the distinct edge timestamps, and that edge has only
two endpoints. Write t for the common timestamp. Every other crossing edge
is assigned by the same strict rule, still injectively to intervals containing
t. Only the edge whose timestamp is t can remain unassigned. There is at
most one such edge. Every cut consequently has at most k+2 crossing edges:

    cw(K) <= k+2.                              (5)

The construction is explicit from the path decomposition. It needs no
assumption that the median order is topological for the original circuit.
Gate equations are factors, so any contraction order is sound.

## 5. The frontier circuit and its exact gate cost

Fix an order v_1,...,v_N of cutwidth w. Let F_i be the edges with exactly
one endpoint among the first i vertices. For every alpha in {0,1}^{F_i},
maintain a circuit wire P_i(alpha;x). Its meaning is the OR, over all
internal edge assignments in this prefix, of the AND of its tensor entries,
with the crossing edge values fixed to alpha. Initially P_0(empty;x)=1.

At vertex v_i, let J be its j edges into the processed prefix. Its other
3-j edges join the new frontier. For an assignment alpha to F_i set

    P_i(alpha;x) = OR_(beta in {0,1}^J)
      (P_(i-1)(alpha,beta restricted to F_(i-1);x)
       AND A_(v_i)(alpha,beta restricted to its ports;x)).   (6)

This is a circuit construction: each conjunction costs one gate and the
OR of 2^j terms costs 2^j-1 gates. Consequently this step costs

    2^|F_i| (2^(j+1)-1) < 2^(|F_i|+j+1).

Since |F_i|=|F_(i-1)|+3-2j and both frontier sizes are at most w,
one has |F_i|+j <= w+1. For j=0,3 it is at most w; for j=1 use
|F_i|+1, and for j=2 use |F_(i-1)|+1. Thus each vertex costs less
than 2^(w+2) gates. The final scalar is ANDed with the accumulator.
Induction using the displayed meaning proves (6) and correctness at i=N.

If K is empty, set N=w=0 and return the scalar accumulator directly.
Combining all charges gives the explicit bound

    C(g) <= p+h+80N0+4N*2^w+1
         <= p+400(q+1)*2^w.                   (7)

The additive p is computed once and is never replicated by table states.
The sharper first line is available when the reduced kernel is small.

## 6. The asymptotic graph theorem and the gate-size exponent

Fomin and Hoie's Theorem 5 states: for every eta>0 there is n_eta such
that every sufficiently large subcubic simple graph has pathwidth at most
(1/6+eta) times its vertex count. Their Section 4 states that the construction
can be carried out in polynomial time. The primary record is in the
[structural bibliography](../sources.md).

Fix delta>0 and put eta=delta/2. The finitely many graphs with N<=n_eta
are covered by pw(K)<=N-1. By (4) and (5), uniformly in all circuit sizes,

    w <= (1/3+delta)*kappa + n_(delta/2)+2.

Let alpha=1/3+delta and A_delta=400*2^(n_(delta/2)+2). Then (7) proves

    C(g) <= p+A_delta(q+1)*2^(alpha*kappa).     (8)

The threshold-dependent constant is explicit in terms of the cited theorem's
threshold; no small numerical constant or exact endpoint O(2^(kappa/3))
is asserted. For supplied decompositions, (7) is a finite exact bound.

There are two alternative bounds. Enumerating b witnesses costs at most
p+(q+1)2^b-1. Viewing the core as a circuit with h ordinary placeholder
inputs, universal synthesis gives p+M_h. Substituting the actual h_i(x)
after synthesis is valid even when their possible joint values are correlated.
An elementary bound M_h<=C*2^h suffices here: Shannon expansion using
constant-cost multiplexers gives it, including h=0,1.
After increasing the constant in (8), taking the minimum yields

    C(g) <= p+O_delta((q+1)*2^min{h,b,alpha*kappa}).

If t=min{h,b,alpha*kappa}, then by (1),

    (2+1/alpha)*t <= h+b+kappa <= q+1.

Thus, with beta=alpha/(1+2alpha),

    C(g) <= p+O_delta((q+1)*2^(beta*(q+1))).    (9)

As delta tends to zero, beta tends to 1/5. For every epsilon>0 this gives
p+O_epsilon((q+1)2^((1/5+epsilon)(q+1))). Equivalently the exponential
rate is at most q/5+o(q), allowing all polynomial factors in the o(q) term.
Applying (2), or setting p+q<=s in (9), gives the universal s/5+o(s) rate.
The use of truth-table synthesis makes the combined size claim nonuniform;
it does not promise an algorithm that discovers this circuit in its size bound.

## 7. Conditioning and disconnected residuals

One may first fix any t witness or gate-value variables, retaining every
gate equation with those values substituted. For each of the 2^t branches,
the remaining incidence graph is H-S. Its distinct components have disjoint
quantified edge variables, so their projections can be ANDed pointwise in x.
Run the construction on each component. If kappa_* is the maximum component
cycle rank (zero when no cyclic component remains), their total initial
vertex count is O(q+1), and the exponent is bounded by alpha*kappa_*.
ORing branches gives p+O_delta((q+1)2^(t+alpha*kappa_*)). Independent
signals and their complements remain shared across all branches.
This is a conditioning/elimination refinement of (8), with the architecture
already present in Dechter's framework; its role is a usable circuit bound.

## 8. Verification boundary

The [compiler](data/check_tensor_compiler.py) builds B2 gate lists and
compares their truth masks with direct existential evaluation. It separately
exercises symbolic cubic tensors, loops, parallel edges and median ties.
Its path decomposition is an elementary supplied decomposition, not the
Fomin--Hoie construction. No finite test establishes (8) or (9); their
justification is the displayed proof plus the cited asymptotic graph theorem.
