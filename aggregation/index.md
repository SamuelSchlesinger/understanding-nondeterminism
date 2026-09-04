# Weaker aggregation and existential projection

Parent: [research overview](../index.md).
The integrated analytical arguments are in
[the aggregation subsection](../../sections/aggregation.tex).

## Why this is a bounded research avenue

The user suggested replacing exact counts with a cheaper aggregate that still
detects a witness. Merely using Boolean entries changes polynomial bit costs,
because the existing compiler already performs OR/AND contraction. A meaningful
improvement needs an aggregate with a different efficient evaluation method.
No improved universal exponent is claimed from this exploration.

## A standard fingerprint transfer

Over a characteristic-two field, form

    P_x(Z) = sum_y f(x,y) product_{i:y_i=1} Z_i.

Distinct witnesses give distinct monomials, so this polynomial is nonzero
exactly on positive ordinary inputs. Its degree is at most the number b of
used witness bits. Polynomial identity testing provides the zero-probability
bound b/Q for a field of size Q. [schwartz80][schwartz80]

For a verifier using a ordinary inputs, t independent evaluation points have
failure probability at most 2^a (b/Q)^t for the simultaneous all-input property.
If this is less than one, some fixed list is correct for every ordinary input.
An evaluator of Boolean circuit size T then yields an exact projection circuit
of size at most t(T+log_2 Q)-1. For Q>=2b, t=a+1 suffices. With h independent
formal signals, t=h+1 suffices under the same field-size condition and ordinary
preprocessing remains shared. Complete proofs and bit costs are in the manuscript.

These are applications of classical identity testing and nonuniform hardwiring,
not claims of a new identity-testing method. Generic witness isolation is also
classical. [valiant-vazirani86][valiant-vazirani86] Directly adding k dense
affine conditions can add O(kb) gates; a fine-grained use must charge that cost.

## Structural accounting and a precise limitation

A unary weight on each witness consistency vertex realizes P_x in the existing
tensor representation. Each attachment adds one vertex and one edge, preserving
cycle rank. The original exact reductions therefore remain applicable. Reusing
the old layout recovers the old exponent, rather than improving it.

For one fixed linear fingerprint w from b-bit witnesses to F_2^L to detect every
nonempty subset of the witness universe, its 2^b weight vectors must be linearly
independent. Therefore L>=2^b. This is a full analytical argument for all b, not
an empirical claim. It does not apply to lists tailored to one verifier's fibers,
nor establish a lower bound for small verifiers or arbitrary projection circuits.

## A classical positive example

For a bipartite adjacency matrix X, the characteristic-two determinant
det(x_ij Z_ij) sums distinct monomials for its perfect matchings. It is nonzero
as a polynomial exactly when a matching exists, and its evaluations are
polynomial-time computable. This is the classical algebraic matching phenomenon.
[mvv87][mvv87] The polynomial uses edge-selection tags, separately from the
binary-column encoding of witnesses; only its degree r matters for the zero bound.

Exact integer counting produces the zero-one permanent, whose computation is
#P-complete. [valiant79][valiant79] This example shows how a weaker aggregate
can change the algorithm, without asserting an unconditional complexity-class
separation or a new matching algorithm.

No extra experiment was performed for these statements. Their value is the
analytical transfer target, the exact graph accounting, and the obstruction to
a relation-independent universal linear fingerprint.

[schwartz80]: ../sources.md#schwartz80
[valiant-vazirani86]: ../sources.md#valiant-vazirani86
[mvv87]: ../sources.md#mvv87
[valiant79]: ../sources.md#valiant79
