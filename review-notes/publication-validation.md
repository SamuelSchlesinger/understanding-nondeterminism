# Repository publication validation

Date: September 5, 2026.

The repository edition is titled **Understanding Nondeterminism: Faster SAT
and Exact Counting for Small Circuits**. The README, abstract, and opening
section lead with the restricted-instance algorithm. The full proof remains
in `sections/counting-algorithm.tex`, supported by `sections/structure.tex`.

## Claim review

The headline concerns single-output circuits over the full binary basis B2,
with unrestricted fanout and internal-gate size. For every fixed positive
epsilon, time is polynomial times `2^((1/4 + epsilon)(s + 1))`, with
exponential space allowed. For fixed `0 < gamma < 4` and
`s <= (4 - gamma)u`, choosing `epsilon = gamma / (8(4 - gamma))` gives
an input exponent at most `(1 - gamma/8)u` plus a constant. This is the
explicit exhaustive-search comparison used in the introduction.

The instance-sensitive bound, count-preserving local identities, unique
internal extensions, polynomial bit costs, and separation from nonuniform
synthesis were checked against the existing written proof. The constructive
layout claim was rechecked in Theorem 5 and Section 4 of the
[Fomin-Hoie author PDF](https://fedorvf.github.io/articles/2006/2006b.pdf).
The paper and README retain the fixed positive slack, exponential-space
cost, unestablished priority, and absence of a full solver implementation
or benchmark. The publication pass does not establish a current best-known
algorithm or close the remaining literature-access gaps.

## Repository and provenance

The four original research commits are retained as ancestors of the root
repository. The integration commit moves their paths under `research/` and
adds the manuscript, compiled reading copy, scripts, and historical reviews.
LaTeX intermediates and Python caches are ignored. The original nested Git
metadata was backed up locally outside the publication tree.

Code and build automation use MIT; the manuscript and research notes use
CC BY 4.0, as requested by the author. `LICENSE` specifies the file scope.
The full CC BY text was retrieved from Creative Commons. `CITATION.cff`
records the manuscript title, author, repository, and publication date.

GitHub Actions runs the finite checks on Python 3.10 and 3.14 and builds the
PDF separately. Actions are pinned to verified upstream commit IDs. Workflow
permissions are limited to reading repository contents.

## Validation

- `make check`: all five finite suites agree with their retained outputs.
- Document audit: 17 reachable research documents, 39 canonical sources,
  83 LaTeX labels, and 33 cited BibTeX entries; publication entry links also pass.
- `make pdf`: succeeds without warnings, unresolved references, or
  overfull/underfull boxes in the final LaTeX/BibTeX logs.
- `cffconvert --validate -i CITATION.cff`: passes CFF schema 1.2.0.
- Visual inspection: all rendered pages inspected in contact sheets; the
  bibliography spacing and a stranded command introduction were then corrected,
  and final pages 37-40 inspected individually. The final reading copy has
  40 pages, with no clipping or overlapping text found.
- CI results are available from the repository's Actions tab; these local
  observations do not claim a hosted run before publication.

Final reading copy: 394234 bytes. SHA-256:
`8044ec9d22b41384514a5ee62923a398c86bbce42fcff269fdf27945cad5f5d4`.
