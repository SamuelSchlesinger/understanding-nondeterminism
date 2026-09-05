# Consequences of exact counting

Parent: [research index](../index.md).

The complete statements and proofs are in
[algorithm-consequences.tex](../../sections/algorithm-consequences.tex).
They are derived consequences of the existing consistency construction,
using classical self-reduction principles; no historical priority is asserted.

## Low excess and the used-input profile

For a normalized nontrivial cone with at most s supplied gates and b used
inputs, define d = s + 1 - b. The consistency rank kappa is at most d.
For fixed positive delta, counting takes polynomial time times
`2^((1/3 + delta)kappa)`. Thus logarithmic kappa, or logarithmic d,
gives polynomial time. This is representation-sensitive: extra syntax need
not make the function harder.

At s <= cu, the gate-only limiting input coefficient c/4 covers every b.
If all u inputs survive normalization, the sharper coefficient is (c-1)/3
for 1 <= c < 4. For c = 2, 3, 7/2, the respective pairs are
(1/2, 1/3), (3/4, 2/3), and (7/8, 5/6). All require fixed positive slack
and suppress a polynomial prefactor. These are analytical coefficients,
not experimental timings or hardness bounds.

## Pinning, indexing, and generation

Changing an input equality table to require a fixed bit leaves the graph
unchanged. The same equality expansion, graph reductions, and frontier
layout therefore compute every conditional count with the original exponent
`min{b, (1/3 + delta)kappa}`. Originally unused but unpinned inputs contribute
a final power of two. The proof preserves the whole boundary-conditioned
count function, which is stronger than preserving only the unconditioned total.

With Z(p) the count of satisfying extensions of a prefix p, the identity
Z(p) = Z(p0) + Z(p1) supports lexicographic rank and unrank in at most u+1
queries. Search is unranking zero. These deterministic tasks keep the original
exponent because the queries run sequentially.

Uniform generation from counts is classical. Jerrum, Valiant, and Vazirani
give the count-oracle connection in Theorem 3.3 and discuss the fair-coin
obstruction for three solutions in Section 2. Their generator model allows
a failure outcome. Our construction draws a uniform integer rank by rejection and then
un-ranks it. It returns a solution with probability one and has an expected-time
guarantee; a measure-zero random stream can reject forever. [jvv-generation][jvv-generation]

The three-gate circuit `(x OR y) AND (y XOR z)` has solutions 010, 101, 110.
Its first-bit branch counts are one and two. Equal probabilities for nonempty
branches do not give uniform solutions. Conversely, uniformly sampling these
three full assignments induces x probabilities 1/3 and 2/3; it does not
uniformly sample the two distinct positive projected inputs.

## Finite evidence and boundary

[check_queries.py](data/check_queries.py) independently tabulates every
Boolean relation on zero through three variables, then verifies rank/unrank,
prefix-count partitions, invalid-rank handling, and exact one-trial rejection
probabilities. The [expected output](data/expected_output.txt) records its
finite scope. This validator uses an explicit counting oracle. It does not
implement the asymptotic circuit-counting algorithm, test the graph theorem,
or prove the general pinning lemma.

[jvv-generation]: ../sources.md#jvv-generation
