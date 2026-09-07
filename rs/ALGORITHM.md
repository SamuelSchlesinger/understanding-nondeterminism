# Implementation of the SAT note

The default `--layout paper --method auto` implements all four steps in
[the note](../sat-note.tex). `--layout auto` is an alias for `paper`.
For each fixed `--epsilon P/Q`, with `0 < P/Q <= 1`, the algorithm has time

```text
poly_epsilon(n+s+1) * 2^((1/4+epsilon)(s+1)).
```

The space bound is a polynomial times the same exponential factor. Configured
resource limits can stop execution with an error. They do not select a slower
exponential branch, return an approximate count, or report UNSAT.
Forced evaluation methods, experimental layouts, and the independent
`--verify` enumeration are outside this default running-time guarantee.

## Source-to-code correspondence

| Note or source | Implementation |
| --- | --- |
| Circuit normalization and unused-input factor | `src/circuit.rs`, `src/solver.rs` |
| Incidence cycle budget and count-preserving reductions | `src/network.rs` |
| Monien--Preis helpful-set bisection | `src/layout/bisection.rs` |
| Fomin--Hoie anchored decompositions and boundary bridge | `src/layout/paper.rs`: `anchored`, `construct` |
| Cubic median-timestamp lemma | `src/layout/paper.rs`: `median_order` |
| Frontier invariant and choice of exponent | `src/network.rs`, `src/solver.rs` |

The primary layout sources are:

- Monien and Preis, *Upper Bounds on the Bisection Width of 3- and 4-regular
  Graphs*, [ALCOMFT-TR-01-116](https://cs.au.dk/~gerth/alcom-ft/TR/ALCOMFT-TR-01-116.html),
  Theorem 1 and Lemmas 2 and 5. The full proof is available as a linked
  compressed PostScript file. The later
  [journal article](https://doi.org/10.1016/j.jda.2005.12.009) numbers these
  helpful-set and balancing statements as Lemmas 3 and 4.
- Fomin and Hoie, [*Pathwidth of cubic graphs and exact algorithms*](https://fedorvf.github.io/articles/2006/2006b.pdf),
  Lemma 4, Theorem 5, and Section 4.

Two implementation choices need an explicit resource argument. We search for
the bounded-size witnesses in the Monien--Preis lemmas by increasing
cardinality, and use centroid decompositions of trees inside the Fomin--Hoie
construction. Both are concrete, terminating subroutines with the required
guarantees. We do not implement Monien--Preis's internal graph-rewriting
procedure or an optimal tree pathwidth algorithm. The choices increase the
polynomial overhead; they leave the note's fixed-epsilon bound intact.

## 1. The graph passed to the layout routine

Normalization leaves `b` used inputs and `q <= s` gates. Its accepting count
is multiplied by `2^(n-b)` at the end. Let `ell` count input pins. The
connected incidence graph has

```text
kappa = ell-q-b+1 <= s+1-b.
```

Equality expansion preserves the count and cycle rank. Loop, parallel-edge,
and low-degree contractions preserve the count and never increase cycle rank.
A nonempty remainder is simple, connected, and cubic, with

```text
N = 2*(kappa(K)-1) <= 2*(kappa-1).
```

The solver checks these identities. The layout API additionally checks its
graph's simplicity, symmetry, cubic degree, and connectivity.

## 2. A polynomial bisection procedure, including finite exceptions

Write `e = epsilon`. Start from the balanced partition given by vertex labels.
Let `c` be the number of crossing edges. Stop if

```text
c <= (1/6 + e/4)*N.                                  (target)
```

Otherwise start a round, saving its initial bisection and cut `c0`. Let `i`
be the number of vertices moved from the left side to the right side, and
let `g = c0-c` be the cut reduction so far. Before searching the current left
side `L`, check

```text
c > (1/3 + e/4)*|L|.                                 (density)
```

Monien--Preis Lemma 2, with its parameter `e/8`, supplies a nonempty set in
`L` with positive helpfulness and size at most some integer `h_e`, depending
only on `e`. Helpfulness is the exact cut reduction when moving the set.
The implementation enumerates cardinalities `1,2,...` and stops at the first
such set. Each successful search therefore examines only subsets of size at
most `h_e`; its time is polynomial in `N` for fixed `e`.

Continue these moves until `g > 1 + floor(log2(i))`. Check the balancing premise

```text
N/2 + i < 3*c.                                       (balance)
```

Under this premise, Lemma 5 supplies a set of exactly `i` vertices on the
right with helpfulness at least `-1-log2(i)`. Since helpfulness is integral,
we search for helpfulness at least `-1-floor(log2(i))`. Move that set back.
It may include vertices just moved to the right. The resulting partition is
balanced and its cut is strictly less than `c0`. There are at most `3N/2`
successful rounds.

If a density or balance premise fails, roll back that round and return the
saved balanced partition. This is the finite-size case allowed by the note,
not a heuristic failure. Here is why both its width and time are controlled.

Choose a uniform integer `h = max(1,h_e)`. If all density premises hold, at
most `t = 4h+4` helpful moves suffice: after `t` moves, `g >= t`, `i <= th`,
and `t > 1+log2(th)`. The latter follows from `th <= 8h^2` and
`log2(h) <= h`. Consequently in every round, including an aborted round,

```text
i <= I_e := 4h(h+1),          g <= 3i <= 3I_e.
```

If density fails, the returned cut obeys

```text
c0 = c+g <= (1/3+e/4)*(N/2-i)+g
         <= (1/6+e/8)*N + 3I_e.
```

If balance fails, it obeys

```text
c0 = c+g <= (N/2+i)/3+g <= N/6 + (10/3)*I_e.
```

Thus every return, including a finite-size exception, satisfies

```text
c <= (1/6+e/4)*N + 4I_e.                             (bisection)
```

Moreover, because a round starts above the target, either exception implies
`N < 24I_e/e`. Exceptions are confined to an epsilon-dependent finite range.
The report records the actual failed premise, `i`, and `g`, without inventing
a numerical value for the source's hidden constant `h_e`.

Balancing searches inspect subsets of size `i <= I_e`. The complete bisection
procedure therefore has a polynomial time bound for every fixed `e`; a
conservative bound is `O_e(N^(I_e+2))`. It stores only a few membership arrays
and the current combination, using `O(N)` working entries. No `2^N` subset
table is allocated. The degree of this polynomial can be very large.

All decisions use exact integer inequalities. The diagnostic count of examined
subsets saturates at `u64::MAX`; it has no role in termination or correctness.

## 3. From the bisection to bags

For each side, construct a path decomposition ending at its boundary `X`,
following Fomin--Hoie Lemma 4:

1. If a vertex of `X` has no neighbor outside `X`, delete it recursively and
   append the bag `X`.
2. If it has one outside neighbor `u`, recurse after deleting the vertex and
   replacing it in `X` by `u`; append `X+u` and `X`.
3. Otherwise, if `|X| <= floor(m/3)`, enlarge it to `floor(m/3)+1`, construct
   that decomposition, and append the old `X`.
4. Otherwise the graph outside `X` has fewer edges than vertices, hence a
   tree component. Recurse after deleting that component, then append its
   tree decomposition with `X` added to every bag, ending again at `X`.

Tree decompositions use centroids: keep the centroid in the bags and process
the components of its removal sequentially. Every component has at most half
the vertices, so bag size is at most `floor(log2(m))+1`. Induction through the
four cases gives anchored width at most

```text
max(|X|, floor(m/3)+1) + ceil(log2(m)) + 1.
```

The main recursion uses explicit continuations to avoid a linear-depth call
stack. There are `O(N)` bags, each with at most `N` vertices, so retaining the
decomposition requires `O(N^2)` entries and polynomial construction time.

The bridge between boundaries starts with the left boundary. Introduce each
left boundary vertex's right neighbors, then forget that left vertex. Every
bridge bag has at most `c+1` vertices: after forgetting a set `S` of left
vertices, the bag has size at most `|boundary_left|-|S|+sum_{v in S} d_cross(v)
<= c`; introducing the next neighbors adds at most one above this bound.
Append the reversed right-side decomposition. The resulting pathwidth `p`
therefore satisfies the explicit finite bound

```text
p <= max(c, floor(N/6)+1) + ceil(log2(N)) + 1.
```

The implementation checks vertex coverage, edge coverage, consecutive
occurrence, bag uniqueness, and this bound before using the decomposition.

## 4. Median ordering and the final exponent

For an edge `uv`, the first common bag is
`max(first_bag[u], first_bag[v])`. Order edges by `(common_bag,u,v)` and use
their distinct integer ranks as timestamps. These ranks represent distinct
small rational perturbations inside each common bag. Order vertices by their
median incident timestamp, breaking endpoint ties by vertex identifier.

The note's charging proof gives `w <= p+2`. The implementation measures every
prefix boundary and checks this inequality. Combining with the bisection
bound gives

```text
w <= (1/6+e/4)*N + ceil(log2(N)) + O_e(1)
  <= (1/3+e/2)*kappa + O(log(kappa+1)) + O_e(1)
  <= (1/3+e)*kappa + O_e(1).
```

The final step uses the fact that logarithmic growth is bounded by
`(e/2)*kappa` plus an epsilon-dependent constant.

The automatic solver enumerates if `b <= w`, and otherwise contracts. It
does not switch to the larger exponent to evade a resource cap. Frontier
contraction uses `O(N*2^w)` arithmetic operations and two tables with
`O(2^w)` entries. Entries have polynomial bit length. With `alpha=1/3+e`,

```text
min(b,w) <= min(b,alpha*(s+1-b)) + O_e(1)
         <= (alpha/(1+alpha))*(s+1) + O_e(1),

alpha/(1+alpha) = 1/4 + 9e/(16+12e) <= 1/4+e.
```

This accounts for preprocessing, enumeration, contraction, and stored bags;
there is no unimplemented layout oracle in the bound. Small-instance tests
check the code and its certificates. The asymptotic argument additionally
uses the two published Monien--Preis lemmas, not an empirical scaling fit.
