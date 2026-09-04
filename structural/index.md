# Quantified-only elimination and the one-fifth exponent

[Research overview](../index.md) | [Full constructive proof](proof.md)

The proved quantitative contribution is a stronger universal upper bound:

    C(exists y f(x,y)) <= 2^(s/5+o(s))

for a verifier with s gates over the full binary basis B2.
More precisely, for every epsilon>0 the bound is
O_epsilon((s+1)2^((1/5+epsilon)(s+1))).
It improves the existing note's exponential rate s/3.
The proof combines a quantified-only circuit construction with the cubic
pathwidth theorem of Fomin and Hoie [fomin-pathwidth][fomin-pathwidth].
It is a quantitative application of established elimination and graph methods;
priority for this circuit-specific consequence remains unestablished.

## Parameters and statements

Normalize to the nontrivial output cone. Gates and wires may be shared.
Mark a gate dependent when a witness occurs in its syntactic input cone.
Compute every other gate once before the projection circuit.
No semantic independence test or minimal-circuit oracle is required.

| Symbol | Exact meaning |
| --- | --- |
| p | Number of witness-independent gates in the supplied cone |
| q | Number of witness-dependent gates; s=p+q |
| h | Distinct independent signals entering the dependent part |
| b | Used witness inputs |
| ell | Dependent-gate input pins from witnesses or dependent gates |
| kappa | ell-q-b+1, the quantified incidence graph's cycle rank |
| N0 | Tensor vertices after equality splitting; N0<=4q+2 |
| N | Vertices in the final simple cubic kernel; N<=2(kappa-1) if nonempty |
| w | Cutwidth of an order of that kernel; zero for an empty kernel |

An independent signal can be a direct ordinary input or the output of a
witness-independent subcircuit. Its Boolean circuit is shared by every
table entry and contributes no boundary bit. Every dependent gate has at
most one such input. The fundamental accounting identity is

    h+b+kappa <= q+1.                           (1)

For the full cone's a ordinary inputs, the second useful inequality is

    kappa <= s+1-a-b.                           (2)

Both use incidence multiplicity correctly; they do not simply delete
ordinary vertices and disregard the gate equations.

**Theorem S1 (explicit quantified-only compiler).**
There is a deterministic B2 circuit for the existential projection of size

    p+h+80N0+4N*2^w+1 <= p+400(q+1)*2^w.       (3)

The construction is explicit from a supplied path decomposition of the
kernel: a width-k decomposition gives an order with w<=k+2.
The bound also applies when ordinary signal circuits have large treewidth.

**Theorem S2 (cycle-rank rate).** Fix delta>0 and alpha=1/3+delta.
There is a constant A_delta, independent of every circuit parameter, such that

    C(g) <= p+A_delta(q+1)*2^(alpha*kappa).     (4)

One may take A_delta=400*2^(n_(delta/2)+2), where n_eta is a threshold
in Fomin--Hoie's theorem. This displays the dependence on the cited result;
it is not a claim that A_delta is a practically small constant.

**Corollary S3 (witness-independent core).** With
beta=alpha/(1+2alpha), one has

    C(g) <= p+O_delta((q+1)*2^(beta*(q+1))).    (5)

In particular the exponent approaches q/5, even if p is much larger than q.
If q=0, retain the ordinary circuit directly. If the output is a witness
wire, its projection is one. These degenerate cases need no tensor graph.

**Corollary S4 (universal rate).** For every epsilon>0,

    C(g) = O_epsilon((s+1)*2^min{m,n,(1/5+epsilon)(s+1)}).

This can be intersected with the sharper independent ceiling M_n,
the exact witness-enumeration bound, and all coverage bounds in the note.
It is an upper bound and does not locate the worst-case lower-bound frontier.

## Proof route and the constants that matter

The [full proof](proof.md) specifies every factor, graph operation and gate.
The steps below explain why the numerical improvement survives conversion.

1. Introduce variables for witnesses and dependent gate values, and retain
   all defining equations plus the output-one requirement. Ordinary signals
   remain symbolic coefficients. This gives the connected incidence graph
   with 2q+b+1 vertices and q+ell+1 edges, hence cycle rank kappa.
2. Replace high-degree consistency nodes by ternary equality trees. This
   preserves cycle rank and uses at most 4q+2 tensor vertices. Initial tensor
   entries are constants or independent signals and their complements.
3. Contract leaves and degree-two vertices at constant circuit cost. Handle
   self-loops by diagonal summation and parallel edges by joint contraction.
   All residual tensor ranks stay at most three. At most 80N0 gates leave
   a simple cubic kernel with at most 2(kappa-1) vertices.
4. Apply the asymptotic cubic pathwidth bound [fomin-pathwidth][fomin-pathwidth]. Place each
   cubic vertex at the median timestamp of its incident edges in the path
   decomposition. Crossing edges inject into active intervals, with at most
   one extra edge at a median tie. Therefore w<=pathwidth+2.
5. Contract the kernel in that order. A frontier entry is a circuit wire,
   indexed only by binary values on crossing quantified edges. At a vertex
   with j old incident edges, the naive cost is
   2^|F_new|*(2^(j+1)-1). The identity
   |F_new|=|F_old|+3-2j bounds it by less than 2^(w+2).
6. Combine the resulting exponent alpha*kappa with witness enumeration
   exponent b and synthesis on the h independent placeholders. If t is
   their minimum, then (2+1/alpha)t<=h+b+kappa<=q+1.
   This yields beta=alpha/(1+2alpha), approaching one fifth.

The limiting coefficient arises from balancing h:b:kappa=1:1:3.
The original three-way argument balanced a:b:d=1:1:1 and paid exponent d.
The new graph argument pays approximately one third of the cyclic excess.
An uncontrolled factor in a generic 2^O(width) statement would lose this gain.

The median conversion is consistent with the known subcubic inequality
cw<=pw+2, recalled in the proof of Lemma 16 of [bodlaender-cubic][bodlaender-cubic].
The proof here includes an elementary construction and the tie case rather
than relying on an unspecified graph-search conversion.

## A worked separation with essential inputs

For t>=3, take 2t ordinary inputs and t witnesses. Define

    a_i(x) = x_(2i-1) AND x_(2i),
    H_t(x) = XOR_(1<=i<j<=t) (a_i(x) AND a_j(x)),
    f_t(x,y) = H_t(x) AND (y_1 OR ... OR y_t).

Use one gate for each a_i, one for each pair conjunction, an XOR chain
over pair conjunctions, and an OR chain over witnesses. The supplied cone has

    p = t + 2*binom(t,2)-1 = t^2-1,
    q = b = t,    h = 1,    ell = 2t-1,    kappa = 0,
    s = t^2+t-1.

The projection is the nonconstant function H_t, computed with p gates.
Every ordinary input is essential: fix another a_j to one, all a_k outside
{i,j} to zero, and the other member of input pair i to one. Flipping the
chosen ordinary input flips H_t. Each witness is essential by setting
H_t=1 and all other witnesses to zero. There are no unused inputs or dead gates.

Exactly t gate vertices a_i have fan-out at least two. The old shared-vertex
estimate therefore gives O((s+1)2^t). Raw witness enumeration has the same
rate, and the full-support construction includes an output term 2^t.
The new compiler has no cyclic kernel and gives O(t^2) gates directly.
This comparison is about the displayed guarantees, not lower bounds on them.
Coverage can also recognize that a single fixed witness suffices in this example.

The full underlying circuit graph contains a subdivision of K_t: use the
a_i as branch vertices and the pair-conjunction vertices on its edges.
Deleting the XOR-chain edges exposes that subdivision. Thus full-circuit
treewidth is at least t-1, while the quantified graph has cycle rank zero.
The ordinary/witness distinction therefore matters even for a nonconstant
projection with every input essential. An implementation may exploit the
ordinary part by preprocessing; doing so is exactly the boundary formalized here.

For t=3 the construction has p=8, q=b=3, s=11, and kappa=0.
It uses a_1,a_2,a_3, their three pair products, and two XOR gates;
two witness ORs and the final AND complete the verifier.

## Conditioning and comparison with prior methods

If S is any set of t witness or gate-value variables, substitute each of
its 2^t assignments into all equations. Let kappa_* be the maximum cycle
rank of a component of the remaining incidence graph. The same construction
gives p+O_delta((q+1)2^(t+alpha*kappa_*)). Components may share ordinary
signals; their quantified variables are disjoint, which is what correctness needs.

| Prior method | Exact comparison with these statements |
| --- | --- |
| Fix shared vertices in the initial note | Pays exponent r and includes independent gates in r. S1 excludes them and may contract cyclic local structure. The bounds are pointwise incomparable when one shared value has very large fan-out; keep both. |
| Dechter's elimination and conditioning | Theorem 14 charges conditioning count plus residual induced width, with exponential space in width. The conditional refinement above follows that architecture. The additional content is the B2 coefficient-level conversion through a cubic kernel [dechter-bucket][dechter-bucket]. |
| Compile the full circuit into a structured language | Amarilli et al., Theorem 5, gives O(|T|2^((4+eta)k)) from a width-k decomposition. Their source gates are AND/OR/NOT and inputs count as vertices; their output is d-SDNNF. This is a stronger representation requirement than an unrestricted B2 circuit, and uses a different width [amarilli-icdt][amarilli-icdt]. |
| Forget variables in DNNF | Forgetting is tractable in DNNF, while preserving the disjoint-OR property is an additional demand. Our OR gates need not have disjoint satisfying inputs, and repeated ordinary predicates need not be decomposable [darwiche-map][darwiche-map]. |
| Fomin--Hoie | Supplies the graph inequality, not a stated circuit projection theorem. S1's reductions and exact boundary costs are needed to turn it into S2 and the one-fifth rate [fomin-pathwidth][fomin-pathwidth]. |

The structural compiler is explicit and may retain exponentially many frontier
entries. It makes no polynomial-space claim. The universal core bound also
uses arbitrary truth-table synthesis and so concerns circuit existence,
not an equally fast algorithm for discovering that circuit.
For n=0, the minimum projected circuit is always a constant, which does not
make finding its value easy. These distinctions must survive manuscript integration.

## Adversarial boundaries

* Dropping equality factors permits incompatible uses of a witness. For
  y AND NOT y, separate existential feasibility is true on both sides,
  while the correct joint projection is false.
* Cutting a gate value without retaining its defining equation is unsound.
  S1 includes every equation before choosing an arbitrary elimination order.
* A loop represents one index appearing twice. Its contraction takes only
  diagonal entries. Replacing it by two independent indices changes the answer.
* Degree-two contraction can create a loop or parallel edges. The proof
  charges their actual tensor reductions; it never treats the graph as simple
  until those reductions finish.
* Each table entry is a constructed circuit. Allowing an unexplained
  realizability predicate as one free entry would hide the projection problem.
* Ordinary values are fixed pointwise in x. Their correlations may be ignored
  when synthesizing a core function on formal inputs and then substituting,
  but they may not be independently existentially guessed.
* The coefficient one fifth is specific to B2 gate count and binary indices.
  Unbounded fan-in or a basis conversion requires new quantitative accounting.
* Large width or cycle rank is an obstruction to a particular bound, not
  an unrestricted circuit lower bound. This result does not prove a hard family.

## Validation and limitations

Run [check_tensor_compiler.py](data/check_tensor_compiler.py) with Python 3.
It constructs actual B2 gate lists, retaining symbolic ordinary inputs.
It checked all 9,280 one- and two-gate circuits with one ordinary and one
witness input, 600 additional circuits with seed 20260904, and 140 targeted
symbolic tensor networks. All outputs agreed with direct existential evaluation.
The [expected output](data/expected_output.txt) records the exercised cases.

The checks include all local gate-cost inequalities, kernel vertex counts,
loops, double/triple parallel edges, equality splitting, surviving cubic
kernels, and median ties. They do not implement or test the asymptotic
Fomin--Hoie construction. The proofs establish the asymptotic results;
the finite checks help detect mistakes in a concrete implementation.

The remaining limitations are priority, efficient discovery of the best
projection circuit, and practical constants as delta tends to zero.
Neither exact endpoint O((s+1)2^(s/5)) nor optimality of one fifth is proved.
The [literature comparison](../literature/index.md) carries the wider priority audit.

[amarilli-icdt]: ../sources.md#amarilli-icdt
[bodlaender-cubic]: ../sources.md#bodlaender-cubic
[darwiche-map]: ../sources.md#darwiche-map
[dechter-bucket]: ../sources.md#dechter-bucket
[fomin-pathwidth]: ../sources.md#fomin-pathwidth
