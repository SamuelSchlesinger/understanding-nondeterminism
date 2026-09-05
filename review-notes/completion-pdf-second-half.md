# Final PDF visual review: pages 20–39

## Scope and method

The reviewer opened and visually inspected every rendered image for pages
20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
38, and 39 of `main.pdf`. The initial renders were
`/private/tmp/circuit-analysis-completion-qa/page-20.png` through
`page-39.png`, at 110 dpi. Text extraction was used only as a supplement
for the exact wording of a paragraph break and bibliography entries.

After intervening source edits, the reviewer compared the final renders
`final-20.png` through `final-39.png` against all twenty inspected images.
Pages 35–39 changed. The reviewer opened and visually inspected all five
final images again; pages 20–34 were pixel-identical. The final PDF is
39 pages and 387,559 bytes, with SHA-256
`c2899a9481d64d8ecc98b429532bb62a48b3fbbef66ca69ccb412b4e5c3ac84b`.

## Findings

**Pass. No outstanding visual corrections.** No clipping, overlapping text, equation-number
collisions, unreadable glyphs, or material margin defects were found.

| Pages | Material inspected | Result |
| --- | --- | --- |
| 20–21 | Occupancy theorem, boxed bounds, proofs, sparse-fiber corollaries | Displays and equation labels fit; proof continuation is clear. |
| 22–24 | Worked-family formulas, support-count table, asymptotic comparison table | Both tables are legible and aligned; formulas and subscripts remain readable. |
| 25–28 | Random-verifier propositions, unconditional and conditional lower bounds | No awkward heading isolation or formula collisions; mathematical text stays inside the margins. |
| 29–30 | Prior-work comparison and scope statements | Dense prose and inline rates are readable; Section 11 has introductory text on its opening page. |
| 31–32 | Combined bounds, boxed necessary window, concluding distinctions, transfer formulas | The multiline minimum and adjacent displays fit. The Appendix A heading is accompanied by introductory text. |
| 33–34 | Fingerprint propositions, determinant example, Appendix B opening | Long displays are clear; appendix transitions and theorem/proof boundaries are readable. |
| 35–36 | Methods, finite-check table, reproducibility commands | Table entries and command text are legible. The short encoding paragraph now stays together at the top of page 36. |
| 37–39 | End of reproducibility appendix and all 33 bibliography entries | Hanging indents, wrapped URLs, accents, mathematical titles, and numbering are legible. No entry is clipped or overlaps the footer. |

### Corrected during review

The initial page 35 ended with the lone first line of the paragraph beginning
“In the exhaustive circuit checks, an encoding consists of one or two
topologically ordered binary”; its remaining two lines opened page 36.
The author kept that paragraph together using `samepage` with an explicit
paragraph ending inside the environment. The final render was checked:
the complete paragraph opens page 36, the table remains legible, and the
reflowed bibliography still fits cleanly through page 39. No further
correction is requested.

No manuscript or bibliography source was edited during this review.
This is a visual QA result, not an additional proof or source-priority
certification.
