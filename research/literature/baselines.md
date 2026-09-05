# Baselines and nonuniform hardness

Parent: [literature comparison](index.md).

The manuscript's classical ceiling is the Shannon-Lupanov asymptotic
maximum B2 circuit size, (1+o(1))2^n/n. Lupanov supplies the synthesis
upper bound, and Wegener gives the circuit-counting and synthesis treatment.
[lupanov58][lupanov58] [wegener87][wegener87]

The resulting first crossover is s+1 approximately 2^{n-m}/n. It comes
from the n-input ceiling on the projected function, not the larger
(n+m)-input ceiling on the verifier. It is an upper-bound comparison,
not an assertion that worst-case projections attain the crossover.

Nonuniform lower-bound evidence needs a nonuniform assumption. The
fixed-clause-mask encoding in the manuscript invokes nonuniform SETH
as explicitly defined by Aggarwal and collaborators, Definition 2.9.
[cvp-nuseth][cvp-nuseth]
The hypothesis says that for every fixed exponent improvement there
is a clause width for which the corresponding SAT family has no such
small circuit family. Uniform SETH alone is not used to infer this.

Paturi and Pudlak investigate circuit-satisfiability resource tradeoffs
under stated assumptions. Williams connects certain uniform SAT-algorithm
improvements to circuit lower bounds. Neither supplies the unrestricted
exponential projection lower bound that motivates this manuscript.
[paturi10][paturi10] [williams10][williams10]

The separately checked Morizumi papers and their exact U2 basis scope
appear in the [statement matrix](statement-matrix.md).

[cvp-nuseth]: ../sources.md#cvp-nuseth
[lupanov58]: ../sources.md#lupanov58
[paturi10]: ../sources.md#paturi10
[wegener87]: ../sources.md#wegener87
[williams10]: ../sources.md#williams10
