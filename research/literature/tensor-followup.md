# COPY expansion, tensor rewriting, and contraction guarantees

Parent: [literature comparison](index.md).
The detailed retrieval record is in
[the focused comparison](../../review-notes/sat-comparison.md).

## Direct antecedent for the forest-cut mechanism

Biamonte, Morton, and Turner give a counting algorithm with cost
poly(g+c*d) 2^c in their canonical representation: g gates, c COPY tensors,
and maximum COPY degree d. Theorem 17 of arXiv version 2 expands each COPY
tensor into its all-zero/all-one choices, then contracts the resulting trees.
Remark 18 expressly limits that theorem to the canonical representation of
Section 2.1. Corollary 19 uses O(log n) COPY tensors for polynomial time;
the abstract's O(log c) wording should not be repeated. [biamonte-copy][biamonte-copy]

This is a direct antecedent for the forest-cut mechanism. The manuscript's
ordinary-input symbolic lift and B2 bound r<=s+1-a-b are applications and
bookkeeping refinements. They do not make COPY expansion original.
The inspected theorem states neither the later cycle coefficient 1/3 nor
the balanced projection and counting coefficients 1/5 and 1/4.

## Decision, counting, and exact rewrites

De Beaudrap, Kissinger, and Meichanetzidis, Theorem 2.1, encode a CNF counting
instance as a closed ZH diagram. Section 3.2 explicitly switches its semiring
from natural numbers to Boolean values for decision. Exact rewriting is an
established algorithmic approach, with different behavior for tractable and
general classes. The paper supplies no gate-minus-input, cubic-kernel, or
Fomin-Hoie coefficient. [beaudrap-rewriting][beaudrap-rewriting]

## Analytical planar guarantees versus cubic experiments

Kourtis and collaborators prove planar contraction bounds in Section 2.2,
Corollaries 1-2: for N tensors of bounded initial rank Delta and index dimension
D, time D^O(sqrt(Delta*N)). General cubic-graph contraction orders are studied
empirically using heuristic methods. These experiments are not a worst-case
exponential coefficient. Section 3.1.1 allows exponential space; Section 3.2
reports approximate rather than exact counts for large fixed-precision instances.
[kourtis-counting][kourtis-counting]

Their Section 4 describes COPY decomposition into rank-three rings. Such a ring
adds one cycle to the incidence graph; a tree decomposition adds equal numbers
of vertices and edges and preserves cycle rank. The manuscript needs this exact
accounting for its numerical bound. The general technique of rank reduction is
established.

These comparisons improve attribution and delimit the proposed quantitative
contribution. They do not establish historical priority by exclusion.

[biamonte-copy]: ../sources.md#biamonte-copy
[beaudrap-rewriting]: ../sources.md#beaudrap-rewriting
[kourtis-counting]: ../sources.md#kourtis-counting
