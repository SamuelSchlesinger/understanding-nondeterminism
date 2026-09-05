# Exact-counting and knowledge-compilation connections

Bounded expert-persona author note, 2026-09-05. Inspected the current plan,
manuscript interfaces, and counting proof. The four connections below should be
explained explicitly. Primary documents were freshly fetched; bibliographic
identities and page locations refer to those documents, not recollection. This
note does not certify novelty, optimality, or the full manuscript's correctness.

## 1. Satisfiability preservation, count preservation, and recoverable solutions

**Recommendation.** Put a short three-level distinction before the integer table
proof. Preserving whether a solution exists is weaker than preserving its count;
preserving a total count alone is weaker than retaining conditional counts and
the identity of solutions. The manuscript's gate equations offer the useful
stronger fact: each input has exactly one internal extension. Equality-tree
expansion has the same property. Later contractions preserve the conditional
count on every boundary assignment, which is why pinning can be supported.

For an elementary warning, `x` and `x OR z` are both satisfiable but have different
model counts. More sharply, on the same named universe `(x,z)`, `x AND NOT z`
and `NOT x AND z` each have one model but disagree after pinning `x=1`. Thus
unconditional count preservation alone cannot justify the later search proof.
The example is a direct derivation, not a quoted source claim.

**Primary evidence.** Lagniez--Marquis [lm17][lm17], introduction, author-PDF
p. 3, expressly separates satisfiability-preserving preprocessing from counting
and lists variable elimination and pure-literal elimination among techniques
that need not preserve counts. The same page discusses why a representation
intended for subsequent conditioning needs more than just its initial count.
Section 3.2, author-PDF pp. 19-20, discusses count-preserving replacement of
defined gate variables. These are author-PDF page numbers: that 76-page version
does not have the final journal's 413-481 pagination.

**Boundary.** Cite this as motivation for the invariant, not as a theorem proving
the manuscript's equality expansion or cubic reductions. Keep their complete
local proof. Do not describe arbitrary SAT encodings as count-preserving.

## 2. Existential projection counts fibers once, rather than counting their mass

**Recommendation.** Add the two formulas side by side:

```text
ordinary model count  = sum_x |{y : f(x,y)=1}|
projected model count = sum_x 1[there exists y with f(x,y)=1]
```

In the shared three-gate example these are three and two. The nonuniform
projection theorem concerns a circuit representing the existential Boolean
function; the uniform exact-counting theorem counts accepting assignments to
all circuit inputs. Neither statement automatically supplies an equally fast
algorithm to count distinct positive ordinary inputs. This is the most important
terminology guardrail in a paper that combines projection and counting.

**Primary evidence.** Fichte--Hecher--Morak--Woltran [fhmw18][fhmw18], inspected
arXiv PDF, Section 2, p. 2, defines projected model counting by the number of
assignments to the projection variables that have a satisfying extension.
Example 1, p. 3, exhibits six full models but four projected models. The
definition of `pmc` in Definition 5, p. 7, uses a union of projected extension
sets, and Definition 9, p. 8, explicitly accounts for overlaps. This is direct
evidence that replacing existential aggregation with integer summation changes
the counted object.

**Boundary.** No transfer of their treewidth exponent is asserted: their primal
CNF graph and this manuscript's binary-index consistency graph are different
objects. The fetched arXiv metadata puts Morak before Hecher, but the actual
PDF title and the publisher record both order the authors Hecher then Morak;
use the publisher order in the bibliography.

## 3. Prefix counts are an index into the solution set

**Recommendation.** Present extraction, ranking, unranking, and exact sampling as
uses of one prefix-count interface. A prefix with `C` completions partitions into
a zero branch with `C0` and a one branch with `C-C0`. In lexicographic unranking,
an integer rank below `C0` enters the zero branch; otherwise subtract `C0` and
enter the one branch. This maintains a bijection from `[0,C)` to completions.
Rank zero extracts one witness. A uniform rank produces a uniform completion.
These integer-interval facts should receive a short local induction proof.

**Primary evidence.** Jerrum--Valiant--Vazirani [jvv86][jvv86], Theorem 3.3(1),
p. 174, proves uniform generation with a counting oracle. Its proof on
pp. 174-175 describes steering choices using the number of accepting extensions.
Their fair-coin model and generator definition, pp. 172-173, permit failure;
the discussion explicitly explains why three outputs obstruct bounded fair-bit
generation that always returns a solution. Cite these pages for the classical
counting-to-generation mechanism and the model distinction.

**Boundary.** The proposed uniform-rank implementation is a direct elementary
specialization; do not attribute this exact rank/unrank algorithm to a theorem
that only states generation. Its fair-bit rejection stage has expected time.
Preserving the manuscript's original `b`, cycle rank, and layout under every
prefix query requires the root author's pinning lemma. Neither a generic
counting oracle reduction nor re-normalization supplies that numerical bound.

## 4. Tractable queries require a representation with the right invariants

**Recommendation.** A short connection to knowledge compilation explains why
factoring a Boolean function and counting its models need different care.
Decomposable conjunctions permit independent choices to combine; deterministic
disjunctions avoid counting the same assignment twice. Equal variable universes
or explicit factors for missing variables are needed when adding counts.
The frontier invariant is the manuscript's concrete way of retaining the
necessary compatibility information during contraction.

**Primary evidence.** Darwiche--Marquis [dm02][dm02], definitions in Section 2,
pp. 231-232, specify decomposability, determinism, and smoothness. Proposition
4.1/Table 5, p. 239, records polynomial-time counting for d-DNNF; Section 4,
p. 240, explains the added role of determinism. Definitions 5.4-5.7, p. 241,
distinguish conditioning from forgetting. Lemma A.3, pp. 244-245, proves that
efficient conditioning plus consistency supports model enumeration, with runtime
polynomial in representation size and output size. These are printed journal
page numbers in the inspected arXiv-hosted article.

**Boundary.** Treat this as a conceptual connection. The current proof has not
established that the resulting symbolic circuit is d-DNNF, nor that compiling
once yields every conditional query in polynomial time in the original circuit
size. Enumeration of exponentially many models still takes exponential output
time. Do not convert the paper's nonuniform existence bound into a uniform
compilation guarantee through terminology.

## Local References

- [lm17] Jean-Marie Lagniez and Pierre Marquis. *On Preprocessing Techniques and
  Their Impact on Propositional Model Counting*. Journal of Automated Reasoning
  58 (2017), 413-481. DOI: 10.1007/s10817-016-9370-8.
  Inspected author version:
  https://www.cril.univ-artois.fr/~lagniez/papers/LagniezM17b.pdf
- [fhmw18] Johannes K. Fichte, Markus Hecher, Michael Morak, and Stefan Woltran.
  *Exploiting Treewidth for Projected Model Counting and Its Limits*. SAT 2018,
  LNCS 10929, 165-184. DOI: 10.1007/978-3-319-94144-8_11.
  Publisher identity verified at
  https://link.springer.com/chapter/10.1007/978-3-319-94144-8_11;
  inspected author PDF: https://arxiv.org/pdf/1805.05445.
- [jvv86] Mark R. Jerrum, Leslie G. Valiant, and Vijay V. Vazirani. *Random
  generation of combinatorial structures from a uniform distribution*.
  Theoretical Computer Science 43 (1986), 169-188.
  DOI: 10.1016/0304-3975(86)90174-X. Inspected article:
  https://www2.stat.duke.edu/~scs/Courses/Stat376/Papers/ConvergeRates/RandomizedAlgs/JerrumValiantVaziraniTCS1986.pdf
- [dm02] Adnan Darwiche and Pierre Marquis. *A Knowledge Compilation Map*.
  Journal of Artificial Intelligence Research 17 (2002), 229-264.
  DOI: 10.1613/jair.989. Inspected article: https://arxiv.org/pdf/1106.1819.

Access note: the DOI redirects for LM17 and DM02 failed in the web tool; their
full primary PDFs were accessible. No claim of successful DOI resolution is
made for those two references. All evidence above was inspected on 2026-09-05.

[lm17]: https://www.cril.univ-artois.fr/~lagniez/papers/LagniezM17b.pdf
[fhmw18]: https://link.springer.com/chapter/10.1007/978-3-319-94144-8_11
[jvv86]: https://doi.org/10.1016/0304-3975(86)90174-X
[dm02]: https://arxiv.org/pdf/1106.1819
