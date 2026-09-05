# From consistency to exact counting: a worked circuit

The circuit `f(x,y,z) = (x OR y) AND (y XOR z)` has three accepting inputs.
Turning it into local tables preserves that count only if repeated uses of
`y` agree. This page calculates the tables and shows exactly what a contraction
remembers. The general [structural argument](../structural/index.md) supplies
the graph reductions and width bounds; this finite example supplies no
asymptotic evidence. Return to the [research index](../index.md).

## Gate equations and unique extensions

Write the three gate values as

```text
v_1 = x OR y
v_2 = y XOR z
v_3 = v_1 AND v_2
```

Using brackets for zero-one indicators, define

```text
G1(x,y,a) = [a = x OR y]
G2(y,z,c) = [c = y XOR z]
G3(a,c,d) = [d = a AND c]
O(d)      = [d = 1]
```

Here `a,c,d` range over possible values of `v_1,v_2,v_3`. We reserve `b` for
the manuscript's number of retained circuit inputs.

The sum of `G1*G2*G3*O` over all six bits is the number of accepting inputs.
To prove this, fix `x,y,z`. The first two equations force `a,c`, and the third
forces `d`. Exactly one gate-value extension satisfies all gate equations.
The last factor keeps that extension if and only if the output is one.
This is a bijection, which is stronger than just preserving satisfiability.

| x | y | z | v_1 | v_2 | v_3 |
|---|---|---|-----|-----|-----|
| 0 | 0 | 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 0 | 1 | 0 |
| 0 | 1 | 0 | 1 | 1 | 1 |
| 0 | 1 | 1 | 1 | 0 | 0 |
| 1 | 0 | 0 | 1 | 0 | 0 |
| 1 | 0 | 1 | 1 | 1 | 1 |
| 1 | 1 | 0 | 1 | 1 | 1 |
| 1 | 1 | 1 | 1 | 0 | 0 |

The three accepting triples are `010,101,110`. With `x` ordinary and `(y,z)`
witnesses, the two witness fibers have sizes one and two. Both ordinary
inputs are positive in the projection, so its number of positive inputs is
two. Witness-pair counting and projected-input counting differ already here.

## Ten incidences and one consistency cycle

The bipartite incidence graph has six value vertices and four factor vertices.
Each gate factor has three incidences and the output factor has one. Thus it
has ten vertices and ten edges. It is connected and has cycle rank
`edges - vertices + 1 = 1`.

```text
x ---- [G1] ---- v_1
         |         \
         y          [G3] ---- v_3 ---- [O]
         |         /
z ---- [G2] ---- v_2
```

The cycle is `y, G1, v_1, G3, v_2, G2, y`. This is an undirected
consistency cycle, even though the original circuit has no directed cycle.
The two uses of `y` diverge and later recombine. In the structural budget,
`q=b=3`, `h=0`, and `ell=6`, giving `kappa=ell-q-b+1=1`.

Give each edge its own binary index. Every value vertex receives a table
that is one precisely when its incident edge indices agree. A degree-one
value vertex has table `(1,1)`, since either value is allowed. Every factor
vertex receives its gate or output indicator. Sum the product over the ten
edge indices. Each satisfying input gives exactly one nonzero edge assignment:
all occurrences carry their forced value. Conversely every nonzero assignment
has agreeing occurrences and satisfies the gate equations.

These arrays are tensors, and summing products over shared indices is
contraction. Tensor-network circuit simulation is an established framework
[markov-tensor][markov-tensor]. The proof above independently supplies this
example's particular count-preserving correspondence.

## Eliminate x and z, retaining shared y

The first intermediate tables are

```text
A(y,a) = sum_x [a = x OR y]      B(y,c) = sum_z [c = y XOR z]

          a=0 a=1                         c=0 c=1
    y=0    1   1                   y=0    1   1
    y=1    0   2                   y=1    1   1
```

At `y=1`, both possible `x` values produce OR output one. That is why `A(1,1)`
equals two. Fixing `y` and the XOR output uniquely determines `z`, so every
entry of `B` equals one.

Summing `G3(a,c,d)*O(d)` over `d` gives `[a=c=1]`. Consequently

```text
Z = sum_y A(y,1) B(y,1) = 1*1 + 2*1 = 3.
```

The `y=0` term counts `101`; the `y=1` term counts `010` and `110`.
If the two branches chose their own independent copies of `y`, the result
would be `(1+2)*(1+1)=6`. The three extra combinations disagree on `y`.
This changes the circuit being counted, rather than providing a shortcut.

Pinning `x=0` replaces its input table by `(1,0)`; pinning `x=1` uses `(0,1)`.
The original graph stays in place. The resulting left tables are

```text
A_x=0 = [[1,0],[0,1]]       A_x=1 = [[0,1],[0,1]].
```

Using their column indexed by output one gives counts `0+1=1` and `1+1=2`.
The four two-bit pins `(x,y)=00,01,10,11` have counts `0,1,1,1`.
Thus a single topology can answer partial-evidence queries by changing input
tables. This fact is relevant to search and sampling through repeated counts.

## Existence and multiplicity use different arithmetic

OR and AND compute existence in the Boolean semiring. Addition and
multiplication compute multiplicity in the nonnegative integers. Both obey
distributivity. The positivity map respects both operations:

```text
[r+s > 0] = [r > 0] OR [s > 0]
[r*s > 0] = [r > 0] AND [s > 0].
```

For fixed `x`, Boolean contraction returns `g(x)=exists y,z f(x,y,z)`;
integer contraction returns `W(x)=sum_(y,z) f(x,y,z)`. These obey
`g(x)=[W(x)>0]`. Counting projected inputs requires `sum_x [W(x)>0]`,
which equals two here; ordinary all-input sum-product contraction returns
`sum_x W(x)=3`. The semiring substitution does not remove witness multiplicity.

Loops and parallel edges also retain multiplicity. A loop on the two ports of
`L=[[2,3],[5,7]]` has value `L(0,0)+L(1,1)=9`, not `17`: its two ends carry
the same bit. For two tables linked by two parallel edges, let

```text
U = [[1,2],[3,4]]      V = [[5,6],[7,8]].
```

Joint contraction is `sum_(r,s) U(r,s)V(r,s)=5+12+21+32=70`.
Contracting the `r` edge first gives
`W(s,t)=sum_r U(r,s)V(r,t)=[[26,30],[38,44]]`. The remaining edge becomes
a loop and its trace is again `70`. Summing all entries of `W` would give
`138`, which improperly treats the remaining edge's two ends independently.
These integer matrices are separate local fixtures, not gates of the example.

## The frontier is a table of conditional counts

The running example reduces completely using low-degree operations. A
nonempty connected cubic remainder would have cycle rank at least two,
whereas this graph starts with one. It therefore cannot illustrate the
asymptotic layout bound for a nonempty cubic kernel.

For a larger instance, fix a vertex order. A prefix's frontier is the set of
edges with one endpoint processed and the other still unprocessed. Its table
entry counts assignments to fully internal edges, conditioned on the frontier
bits. A positive entry need not extend through the unprocessed part: that
part's constraints have not been applied yet.

At the next vertex let `J` be its edges into the past, `R` its edges into the
future, and `S` the other frontier edges. With its local table `T`, the update is

```text
P_next(sigma,rho) = sum_eta P_old(sigma,eta) T(eta,rho),
```

where `sigma`, `eta`, and `rho` assign `S`, `J`, and `R`, respectively.
Each extension is classified once by its consumed-edge assignment `eta`,
proving the conditional-count invariant. Start with scalar one; finish with
an empty frontier whose scalar is the total count.

As a standalone cubic-vertex transition, take `P_old(p,q)=[[1,2],[3,4]]`
and `T(p,q,r)=[r=p XOR q]`, with no spectator edges. The new table is
`[1+4,2+3]=[5,5]`. This fixture explains the transition, without asserting
that the three-gate running example generates that old table.

The maximum frontier size is the order's cutwidth. A frontier of size `w`
stores `2^w` integers. Arithmetic operations and integer bit cost are separate:
the full construction has `O(s+1)` indices, so intermediate entries count at
most `2^O(s+1)` assignments and need `O(s+1)` bits. The manuscript accounts for
polynomial integer-arithmetic overhead and allows exponential table storage.

## Supplementary code and limits

Run [check_walkthrough.py](data/check_walkthrough.py) with Python 3 and compare
its output with [expected_output.txt](data/expected_output.txt). It checks all
input assignments; all 4096 triples of binary Boolean gates for unique gate
extension; all 27 partial pins by exhaustive enumeration of incidence bits;
the displayed contraction tables; ternary equality expansions of degrees
three through six; all 256 pairs of binary two-by-two tables for parallel
contraction; and the weighted loop and frontier fixtures.

These finite checks validate the local mechanisms and all displayed numerical
claims. They do not implement or benchmark the complete SAT solver, prove
asymptotic pathwidth, or establish a novelty claim.

[markov-tensor]: ../sources.md#markov-tensor
