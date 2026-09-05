# Foundations source verification and integration notes

Verified on 2026-09-05. Sources were fetched in this authoring turn, not inferred
from memory. No root build was run while other authors owned concurrent edits.

## Prospective BibTeX entries

```bibtex
@inproceedings{FoundationsCook1971,
  author = {Cook, Stephen A.},
  title = {The Complexity of Theorem-Proving Procedures},
  booktitle = {Proceedings of the Third Annual ACM Symposium on Theory of Computing},
  year = {1971},
  pages = {151--158},
  publisher = {Association for Computing Machinery},
  doi = {10.1145/800157.805047}
}

@article{FoundationsLevin1973,
  author = {Levin, Leonid A.},
  title = {Universal Sequential Search Problems},
  journal = {Problems of Information Transmission},
  volume = {9},
  number = {3},
  pages = {265--266},
  year = {1973},
  note = {English translation; Russian original in Problemy Peredachi Informatsii 9(3), 115--116},
  url = {https://www.mathnet.ru/eng/ppi914}
}
```

Research keys are `foundcook71` and `foundlevin73`; canonicalize during assembly.

## Claim-to-source evidence

1. **Cook's Boolean encoding of accepting computations.** The primary paper
   transcription at
   https://www.cs.cmu.edu/~15455/resources/Cook1971-complx-thm-proof.pdf
   was freshly fetched and inspected. Theorem 1 proof begins on transcription
   p. 2 and constructs A(w) satisfiable iff M accepts w, with construction time
   polynomial in |w|. Transcription p. 3 lists configuration variables and
   consistency/initial/transition/acceptance conditions. The end of Section 2,
   on transcription pp. 4-5, describes bounded existential quantification over
   polynomial-time relations. The manuscript cites these specific constructions;
   it does not describe Cook's original statement as a modern many-one
   completeness formulation, since the theorem's stated target is DNF tautology
   under oracle-style reducibility.

2. **Cook identity and pagination.** The author's page
   https://www.cs.utoronto.ca/~sacook/ was freshly fetched; its historic
   publications list confirms title, venue, 1971, and original pp. 151-158.
   The primary PDF confirms title and author. Direct opening of the ACM DOI
   endpoint returned an internal fetch error; the DOI
   was independently corroborated by the freshly returned DBLP catalog record
   https://dblp.uni-trier.de/rec/conf/stoc/Cook71.html. The publisher endpoint
   was not treated as inspected article content.

3. **Levin's universal search formulation.** The primary journal page
   https://www.mathnet.ru/eng/ppi914 was fetched. Its title, author, abstract,
   volume, issue, and original/translation page ranges were checked. Its linked
   primary full text was also fetched successfully at
   https://www.mathnet.ru/php/getFT.phtml?jrnid=ppi&option_lang=eng&paperid=914&what=fullt
   (two pages, Russian). The citation supports only the narrow statement that
   the paper studies universal search problems, not the specific canonical
   padding scheme, count-preserving machine conversion, or modern circuit bound.

4. **Internal derivations.** The padding bijection, the single-witness uniform
   guessing bound, unique gate-value extension, and the running example are
   elementary derivations written in the text. They are not represented as
   novelty claims or attributed to the two historical papers. Counting paths
   and counting witness strings are explicitly separated.

## Local finite validation

A Python standard-library enumeration was run in this authoring turn and
confirmed the accepting triples `010,101,110`, fiber sizes `1,2`, and the
local-consistency failure at `x=0,z=1`. It also enumerated the canonical
length-prefixed zero-padded encodings for all length bounds from 0 through 8,
checking fixed encoded length and injectivity. Own-file LaTeX brace and
table/equation environment balances passed. These finite checks supplement
the written arguments; no root compilation or PDF layout claim is made here.

## Ownership and integration

- Added `sections/foundations.tex` (section label `sec:foundations`). Intended
  to precede the algorithm walkthrough and substantive projection arguments.
- Added `research/foundations/index.md` with a Local References section.
- Added this source-support note as requested. No edits to main.tex,
  references.bib, research/index.md, research/sources.md, or README.md.
- Root integrator should add the two BibTeX entries, include the section,
  link the corpus chapter, canonicalize its citation keys, and inspect layout.
- The vocabulary table uses only existing packages. Equations carry the
  `foundations-` label prefix; no new preamble macros or theorem environments.
