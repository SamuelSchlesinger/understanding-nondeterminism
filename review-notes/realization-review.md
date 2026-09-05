# Independent analytical review of cubic-kernel realization

Reviewed: [realization.md](../research/structural/realization.md), against the
actual consistency-graph construction and allowed reductions in
[structure.tex](../sections/structure.tex).

Date: 2026-09-05 UTC. This review uses analytical arguments. I did not use
the realization checker's counts, implementation, or output as evidence.

## Verdict

**Pass: no proof-breaking error or required mathematical correction found.**
The attachment budget, circuit construction, exact parameter counts, and
zero-loss retained-kernel claim are supported by the stated proof. The theorem
is honestly positioned as a theorem about a supplied representation. It does
not establish structural criticality of a minimum circuit or hardness of its
projection.

The useful distinction is precise: every input is essential, but the supplied
AND circuit has exactly `k` more gates than the minimum circuit for its
function. Essentiality prevents unused-input padding; it does not prevent
redundant shared computation. The document explicitly says both.

## 1. Conflict count and attachment supply

The source/fork induced subgraph is a forest: any acyclic orientation of an
undirected cycle has a vertex receiving two cycle edges, whereas a fork has
total indegree one and the source has indegree zero.

A component of `m` forks outside the source component has `m-1` internal
edges. Summing fork indegrees shows that it has exactly one incoming edge,
necessarily from a join; summing outdegrees then gives `m+1` outgoing edges.
It carries one formal wire name. A repeated-input join consumes two distinct
outgoing edges from exactly one such component.

For `m=1`, simplicity rules out a repeated-input join. For `m=2`, the upper
bound is one conflict and `1 <= 4/3`. For `m>=3`,
`floor((m+1)/2) <= (m+1)/2 <= 2m/3`. Components with no forks and a direct
join-to-join wire have only one use and contribute no conflict; their omission
from the induced forest count is harmless.

The source component has `m+3` outgoing edges. Its `m=0,1,2` bounds are
respectively `0,2,2`; for larger `m`, the outgoing-edge bound proves
`r_source <= 2m/3+4/3`. With `F=k-2`, summation gives
`r <= floor(2k/3)`.

In a triangle-free graph, a source component with one fork cannot supply a
conflict. Two distinct edges supplying that join must come from the source
and its fork, which would complete a triangle. The strengthened source bound
`r_source <= 2m/3+1` holds for `m=0,1,2` and for `m>=3` by the displayed
inequality. Thus `r <= floor((2k-1)/3)` follows.

The fresh-input supply is sufficient in every residue class:

| `k` | General supply `2t-1` | General conflict bound | Triangle-free supply | Triangle-free conflict bound |
| --- | --- | --- | --- | --- |
| `3a` | `2a+1` | `2a` | `2a-1` | `2a-1` |
| `3a+1` | `2a+1` | `2a` | `2a+1` | `2a` |
| `3a+2` | `2a+1` | `2a+1` | `2a+1` | `2a+1` |

Inserting the repair directly on one incoming edge of each conflicting join
is important. Its fresh output wire reaches only that input pin before the
join, so it cannot equal the other input wire. Later insertions split existing
copied-wire classes; they never merge distinct classes. Thus repairs do not
create new conflicts. The remaining inputs may safely be attached in a serial
chain on one original edge.

## 2. Sink inputs

The sink-distinctness claim is correct. Here is a shorter direct argument,
which may be used when integrating the proof.

A simple cubic graph has at least four vertices, so `k>=3` and there is at
least one join. Choose a topologically last join `v`. All its descendants
other than the sink are forks, forming a directed tree because their
indegrees are one. If this tree contains a fork, take a last fork. Both of
its outgoing edges must reach the sink, contradicting simplicity. Therefore
`v` points directly to the sink. Its fresh formal output name occurs on only
that one edge, so the three sink inputs cannot all have the same name.

The first sink AND can consequently take two distinct formal wires. Its
output is a new wire, distinct from the remaining original sink input used
at the second AND. Attachments preserve this distinction. This proof concerns
wire identity, as required by the stated normalization, rather than semantic
inequality of the signals.

## 3. Circuit, exact budgets, and minimum sizes

The bipolar orientation gives `F=J=k-2`. Joins account for `k-2` gates and
the sink for two, so the skeleton has `k` gates. There are exactly `2t-1`
attachments, giving `q=k+2t-1`.

Every skeleton signal descends from `y_0`: in a finite DAG with one source,
following predecessors reaches that source. Every attachment's old input is
therefore witness-dependent, even when its fresh input is ordinary. All
gates are witness-dependent and all vertices reach the unique sink. Hence
`p=0`, `h=b=t`; each ordinary input contributes exactly one independent pin.
This proves

    ell = 2q-t = 2k+3t-2,
    kappa = ell-q-b+1 = k,
    h+b+kappa = q+1.

Subdivision by attachments and the two-gate expansion at the sink preserve
acyclicity and give an actual topological gate list. Each repaired gate has
two distinct formal input wires, and AND depends on both formal arguments.
There are no constants, unary gates, unused inputs, or off-cone gates.
Identities involving multiple gates can still simplify this circuit; the
document's explicit syntactic-normal-form qualification is necessary and
sufficient for its normalization claim.

Induction on gate order identifies each signal with the conjunction of its
ancestor inputs. Every input reaches the output, so `f` is the conjunction
of all `2t` inputs. Setting all other inputs to one proves essentiality.
Existential quantification of the witnesses gives the conjunction of the `t`
ordinary inputs.

For any fanin-two circuit with `d` essential inputs, its connected output
cone has at least `s+d-1` edges and at most `2s` edges. Thus `s>=d-1`.
AND chains achieve equality here, proving `C(f)=2t-1` and `C(g)=t-1` over
the full binary basis. The difference `q-C(f)=k` is exact, not merely a
lower bound on excess gates.

## 4. Exact retained graph and cycle preservation

The equality-splitting specification matches actual circuit wires.

- A join or attachment output copied through `m` forks has `m+1` uses
  plus its defining incidence. Its equality degree is `D=m+2`, and the
  specified `m=D-2` ternary vertices are precisely the fork tree. For
  `m=0`, the equality has degree two.
- The source wire has no defining incidence. With `m` forks, its `m+3`
  uses require `m+1=D-2` ternary vertices, precisely the source and those
  forks. The degree-three case needs only relabeling the existing equality.
- An attachment on an edge inside a copying tree cuts it into rooted
  copying trees for the old and new wire names. Each new root has the
  attachment's defining incidence, so the same count applies to every
  piece. Each fresh witness has one use and gives a leaf equality.

After these choices, the incidence graph really is the marked target `K`
with subdivided edges, the sink replaced by the specified small tree, and
the output assertion and fresh-witness leaves attached. This is stronger
than merely having the same cycle rank.

The sink contraction has the exact local identity

    OR_w ([w = a AND b] AND [w AND c = 1]) = a AND b AND c.

It can be executed by first contracting the assertion and output equality
into the second sink gate, then its connection to the first gate. All
contracted vertices have degree one or two at the relevant step, and the
first sink gate retains its three original target ports.

For a fresh witness attachment, summing its leaf produces the binary relation
`OR_y [v = u AND y]`, equivalently `v <= u`. It need not be an equality
relation. This is no problem: contracting that degree-two table into a
neighbor preserves its semantic effect while restoring the original edge
topologically. Ordinary attachments similarly give degree-two tables with
ordinary inputs as coefficients.

Choose each edge contraction to remove a subdivision vertex while retaining
the marked target endpoint. This never identifies two target vertices.
Since `K` is simple, no loop or parallel edge is introduced by these
contractions. Each is an ordinary single-edge contraction and preserves
cycle rank. At the end the marked source, forks, joins, and first sink gate
have exactly the target incidences. All are degree three, so the stated
syntactic reduction rules stop there. Every local tensor rank remains at
most three and the number of operations is linear in the supplied circuit.

Semantic simplification of those final tables could reveal the easy
conjunction and permit a different representation. The manuscript correctly
does not exclude such simplification or claim that every legal reduction
order must retain `K`.

## 5. Graph class and precise scope

The ear-insertion existence proof for the bipolar ordering is valid. An
outside component must attach to two distinct old vertices because the
old set contains a cycle and the original graph has no cut vertex. Inserting
an ear immediately after its earlier endpoint preserves every old vertex's
earlier/later neighbors and supplies such neighbors for each new internal
vertex. The two designated endpoints remain the unique source and sink.

The extension from bridgeless connected cubic graphs is also valid: a cut
vertex partitions its three incident edges among at least two components,
so one component has a unique incident edge and that edge is a bridge.

The theorem proves coexistence of the critical supplied budgets, any graph
in the stated cubic class, essential inputs, and an easy nonconstant
projection. It therefore supplies an obstruction to a converse based only
on these supplied structural data. It does not show that such budgets occur
for minimum verifiers, that every cubic graph admits a bipolar orientation,
or that large retained width forces high minimum projection complexity.
Those limitations are all stated correctly in the reviewed document.
