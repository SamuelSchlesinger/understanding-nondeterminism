# Understanding nondeterminism

**Faster SAT and exact counting for circuits with fewer than four gates per input.**

This research manuscript derives an algorithm that beats exhaustive search on
single-output Boolean circuits with `u` inputs and `s <= (4 - gamma)u` gates,
for every fixed `0 < gamma < 4`. Gates may compute any Boolean function of at
most two inputs (the full `B2` basis), and fanout is unrestricted.

For every fixed `epsilon > 0`, it decides satisfiability **and counts all
satisfying assignments exactly** in time

```text
poly_epsilon(s + u + 1) * 2^((1/4 + epsilon)(s + 1)).
```

Choosing epsilon small enough for the fixed gap gamma gives
`poly(u) * 2^((1 - eta)u)` for some `eta > 0`: an exponential saving over
enumerating all `2^u` assignments. **The algorithm allows exponential space.**
This is a proved asymptotic construction, with finite checks of supporting
components; the repository does not implement or benchmark the full SAT solver.
Historical priority and best-known unrestricted-space status remain unestablished.

**[Read the paper](main.pdf)** · **[Algorithm and proof](sections/counting-algorithm.tex)** ·
**[Literature comparison](research/literature/final-comparison.md)** ·
**[Research record](research/index.md)**

## What the restriction buys

The headline bound gives these limiting coefficients in the exponent of `2^u`:

| Gate budget | Exhaustive search | Algorithm's limiting coefficient |
| --- | --- | --- |
| `s <= 2u` | `1` | `1/2` |
| `s <= 3u` | `1` | `3/4` |
| `s <= 3.5u` | `1` | `7/8` |
| `s <= (4 - gamma)u` | `1` | `1 - gamma/4` |

Each algorithm coefficient requires an arbitrarily small **fixed positive
slack** and suppresses a polynomial factor depending on that slack. These are
asymptotic guarantees, not measured speedups or exact endpoint bounds. The
restriction is on circuit size in the stated basis, not on CNF clause count.

## How the algorithm works

1. Prune the circuit to its output cone and simplify constants and unary gates.
   If `b` inputs remain, removed inputs contribute the count factor `2^(u-b)`.
2. Encode gate equations and input consistency as binary-index tables.
   The resulting connected graph has cycle rank at most `s + 1 - b`.
3. Split equality tables into trees and contract low-degree parts exactly.
   A nonempty residual simple cubic graph has at most twice the cycle budget
   in vertices. Every satisfying input still has exactly one internal extension.
4. Use the constructive [Fomin-Hoie pathwidth bound](https://doi.org/10.1016/j.ipl.2005.10.012)
   and the proved cutwidth conversion to evaluate frontier tables, using
   integer addition and multiplication to preserve counts.
5. Choose between this contraction and enumeration of the `b` remaining inputs.

The sharper, instance-sensitive time bound is

```text
poly_delta(s + u + 1) * 2^min{b, (1/3 + delta)(s + 1 - b)}
```

for every fixed `delta > 0`. Balancing the two terms yields the headline gate
coefficient approaching `1/4`. The paper proves the reductions, layout
conversion, operation count, and polynomial bit cost. Its width and tensor
methods build on established work; the [source comparison](research/literature/final-comparison.md)
records what has and has not been checked.

## The broader nondeterminism question

The same construction studies existential projection, `g(x) = exists y f(x,y)`,
when some inputs remain symbolic. The manuscript also proves:

- A nonuniform circuit-size upper bound with exponential rate approaching
  `1/5` in verifier gate count. This is separate from the uniform SAT algorithm.
- Necessary structural conditions for nearly full amplification at the
  critical gate budget, and exact realizations showing that complicated supplied
  circuits can still have easy projections.
- A joint witness-sampling and shared-computation bound, with analytical examples
  that improve on separate coverage and support estimates.

The paper does not establish an unrestricted circuit lower bound, optimality
of the exponents, or a resolution of P versus NP.

## Reproduce the checks and paper

```sh
git clone https://github.com/SamuelSchlesinger/understanding-nondeterminism.git
cd understanding-nondeterminism
make check
make pdf
```

The checks require Python 3.10 or newer and use only its standard library.
The PDF build requires `latexmk`, BibTeX, and the LaTeX packages declared in
[main.tex](main.tex), including `newpxtext`, `newpxmath`, and TikZ. A full
TeX Live or MacTeX installation supplies them. CI runs the same checks and
builds the PDF; its workflow records the Ubuntu package list.

`make check` validates corpus links and citation keys, reruns five finite
verification suites, and compares their outputs with retained records.
Those finite domains check the supporting constructions; the general results
depend on the written proofs and cited graph theorem. In particular, the
checker does not implement the asymptotic layout construction or the full
integer-counting algorithm. `make clean` removes LaTeX intermediates and keeps
the committed reading copy, `main.pdf`.

## Repository guide

| Path | Contents |
| --- | --- |
| [main.pdf](main.pdf) | Compiled manuscript, starting with the SAT algorithm |
| [main.tex](main.tex), [sections/](sections/) | Editable manuscript and proofs |
| [references.bib](references.bib) | Separate BibTeX entries for cited sources |
| [research/](research/index.md) | Supporting derivations, examples, source comparisons, and finite checks |
| [scripts/](scripts/) | Forest, coverage, and relation verification scripts |
| [review-notes/](review-notes/README.md) | Dated review history and publication validation |
| [CITATION.cff](CITATION.cff) | Repository citation metadata |

The [methods appendix](sections/methods.tex) describes the AI-assisted research
process and the limits of its verification. Earlier research commits are
preserved in Git history; the publication commit brings the manuscript and
supporting corpus together under one repository root.

## License and citation

Code and build automation are licensed under [MIT](LICENSES/MIT.txt).
The manuscript, research notes, and other documentation are licensed under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). See [LICENSE](LICENSE)
for the file-level scope and attribution instructions. Citation metadata is in
[CITATION.cff](CITATION.cff).
