# Final comparison of the uniform counting consequence

Parent: [literature comparison](index.md).
The full retrieval and model audit is in
[the final source report](../../review-notes/sat-final-resolution.md).
This document updates the earlier access limits; it does not establish priority.

## What the manuscript proves

For every fixed epsilon>0, the count-preserving consistency construction gives
deterministic exact counting for a single-output B2 circuit with s gates and u
declared inputs in time poly_epsilon(s+u+1) 2^((1/4+epsilon)(s+1)), allowing
exponential space. Its proof is complete in the
[counting subsection](../../sections/counting-algorithm.tex).

The cubic layout and tensor contraction ingredients are established methods.
The quantitative accounting to assess is kappa<=s+1-b, reduction to at most
2(kappa-1) cubic vertices, explicit contraction time per frontier state,
and balancing with enumeration of the b used inputs. A source search that finds
no identical displayed formula does not establish a new algorithmic record.

## Specific access gaps narrowed

Nurk's complete Russian preprint was retrieved from the institute's linked
compressed PDF. The full source defines acyclic arbitrary binary-function
circuits with internal-gate size. Theorem 3.1, printed p.8, gives the
0.4058 gate exponent by branching. The theorem page was rendered and checked.
The algorithm's stated contract is decision, and Reduce step 7 is not
count-preserving: a private two-input OR may be replaced by a fresh input.
This changes three satisfying assignments into one. The later description
of this result as a #SAT bound does not supply a counting proof in the
inspected original. [nurk-sat][nurk-sat]

Broering--Lokam's chapter text was obtained through printed p.168 in a public
reproduction of the primary book, in addition to the publisher's pp.162-163
preview. Lemma 4 displays a circuit-cutwidth exponent 2W; Theorem 2 displays
a CNF-primal-pathwidth exponent 2pw. These particular bounds therefore do
not supply the exact coefficient in the manuscript. The reproduced formulas
were not independently checked against page images, and pp.169-171 remain
unretrieved. The circuit lemma also cannot be applied directly to a reduced
tensor that no longer represents a deterministic gate. [broering-width][broering-width]

Savinov's thesis remains unavailable. Lialina's complete primary presentation
does establish the same-model 0.389667 gate exponent for decision. A difference
between initials in two institutional bibliography records was retained as an
unresolved metadata discrepancy, not silently corrected. [lialina-sat][lialina-sat]

## The additional tensor antecedent

Dudek, Duenas-Osorio, and Vardi give exact weighted-CNF tensor representations,
connect maximum intermediate rank to carving width, and factor suitable
high-degree tensors along trees. Theorems 1, 3, and 6 of arXiv version 2 were
inspected. Thus exact tensor counting and equality-tree factoring are
established. Their maximum-rank guarantee must not be substituted for a
time bound without charging the arithmetic operations in each contraction.
The manuscript's sequential cubic frontier proof supplies that explicit
time accounting. [dudek-contraction][dudek-contraction]

## Final scope

The displayed counting exponent is arithmetically smaller than the specified
branching decision exponents, for a sufficiently small fixed epsilon. This
comparison does not certify the best unrestricted-space running time, historical
priority, or novelty of the combined numerical consequence. The finished paper
presents a proved quantitative application and retains those priority limits.
An unresolved historical comparison is separate from a gap in the displayed proof.

[broering-width]: ../sources.md#broering-width
[lialina-sat]: ../sources.md#lialina-sat
[nurk-sat]: ../sources.md#nurk-sat
[dudek-contraction]: ../sources.md#dudek-contraction
