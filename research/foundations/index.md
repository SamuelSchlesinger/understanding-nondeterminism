# From computation paths to shared witnesses

Nondeterminism expresses the existence of a successful computation. For the
circuit questions in this project, the useful representation is a deterministic
verifier with an additional witness input. Every use of that input must agree
with every other use. The algorithmic problem is to exploit the resulting
shared structure while preserving existence or exact multiplicity.

This chapter supplies the foundation for the [structural construction](../structural/index.md)
and the [counting algorithm](../../sections/counting-algorithm.tex). Its manuscript
counterpart is [sections/foundations.tex](../../sections/foundations.tex).

## A path is one possible computation

A deterministic machine has one next step at each nonhalting configuration.
A nondeterministic machine may have several legal next steps. Unfolding the
choices gives a computation tree. A path is one possible computation; the input
is accepted if some path accepts. In the time-bounded setting, all paths halt
within the bound, and nondeterministic running time measures the longest path.
It does not add the lengths of all paths in the tree.

The choices along a path can be written as a finite string. A deterministic
verifier can read that string, simulate the chosen transitions, reject illegal
transition codes, and check acceptance. Conversely, a machine can guess the
string and run the verifier. This is the path-to-certificate translation.
A certificate can instead be a convenient mathematical object, such as a graph
path or a satisfying assignment, provided that a deterministic checker verifies it.

Checking a proposed graph path means checking its listed vertices and edges.
Finding such a path is a separate task. Existential acceptance specifies what
makes the answer yes; it does not provide a procedure that discovers the
successful proposal. Likewise, interpreting a computation tree as an array of
physical processors would require accounting for their number, total work,
and storage. Path length alone provides none of those resources.

## Witnesses form fibers of a relation

Fix ordinary-input length n and witness length m. Let

```text
f : {0,1}^n x {0,1}^m -> {0,1}
W_x = { y : f(x,y) = 1 }
g(x) = exists y f(x,y) = OR_y f(x,y).
```

The ordinary input x specifies the instance; y is a proposed solution. W_x is
the fiber of successful proposals above x. The accepted pairs form a relation.
Projection onto x forgets which witness worked and remembers whether W_x is
nonempty. Several accepting pairs can therefore represent the same projected
input. The operation deliberately discards fiber multiplicity.

| Object | Meaning |
| --- | --- |
| x | The ordinary input, fixed while proposals are checked. |
| y | One complete witness shared by all checks. |
| f(x,y) | Deterministic verification of that pair. |
| W_x | The set of successful witnesses above x. |
| g(x) | The indicator that W_x is nonempty. |
| s | Number of gates in a supplied verifier circuit. |
| C(g) | Minimum number of gates in any circuit for g. |

The distinction between supplied and minimum size matters even in a tiny example.
Use the three gates

```text
v1 = x OR y
v2 = y XOR z
f  = v1 AND v2.
```

Treat x as ordinary and the ordered pair (y,z) as the witness. When x=0, the
first gate requires y=1 and the second then requires z=0. When x=1, the first
gate already outputs one, and either pair with y different from z works. Thus

```text
W_0 = {(1,0)}
W_1 = {(0,1), (1,0)}
accepting triples xyz = 010, 101, 110
g(0) = g(1) = 1.
```

The checker has three gates over the full binary basis. Its projection is
constant and has zero gates when constants are free. A large space of possible
witnesses does not by itself force a complicated projected function.

## A witness is shared, not chosen afresh at each check

The smallest consistency warning is a quantifier-scope calculation:

```text
(exists y, y=1) AND (exists y, y=0) = true
exists y, ((y=1) AND (y=0))        = false.
```

The left quantifiers bind different local variables, even though both are
printed as y. Rename them y1 and y2 to expose the independence. The right
expression requires one value to satisfy both conditions. A common witness
implies separate witnesses; separate witnesses need not have a common value.

Inside the running example, hold x=0 and z=1. The local relation x OR y = 1
can use y=1, while y XOR z = 1 can use y=0. Both local satisfiability tests
succeed, but their witnesses disagree. No shared y makes the output one.
Splitting occurrences into temporary variables is legitimate only when an
equality constraint reunites their values.

The circuit has no directed cycle: every gate can be evaluated after its
predecessors. Its underlying undirected graph does have the cycle
`y -- v1 -- f -- v2 -- y`. This records information that branches and is reused;
it is not a feedback loop. The consistency construction turns this sharing
into explicit equality constraints. Width then measures the unresolved
boundary information that an elimination order retains, rather than the
number of directed evaluation steps.

## Existential acceptance does not give a sampling guarantee

For a uniformly random m-bit witness Y,

```text
Pr[f(x,Y)=1] = |W_x| / 2^m
g(x)=1 iff this probability is positive.
```

Positivity need not mean a substantial chance of success. If a checker accepts
only 0^m, one uniform guess succeeds with probability 2^(-m). With r independent
guesses the success probability is `1-(1-2^(-m))^r <= r*2^(-m)`. Polynomially
many uniform guesses therefore do not give a constant success probability for
large m. This is a statement about uniform guessing, not a search lower bound:
the accepted witness in this example is obvious from the checker.

A randomized algorithm must specify a probability law and an error or success
guarantee. Existential acceptance asks whether any legal choice succeeds.
These may describe the same tree but use different acceptance criteria.
A good randomized search algorithm may use structure in the verifier; the
existence of one accepting path alone supplies no such guarantee.

## Polynomial verification and exact encodings

A language is a set of finite strings, interpreted as its yes-instances.
NP consists of languages with a single deterministic polynomial-time verifier V
and a polynomially bounded, polynomial-time computable witness length p(n) such that

```text
x belongs to L iff some y of length p(|x|) satisfies V(x,y)=1.
```

One may choose p to be an integer polynomial bound. Guessing p(|x|) bits and
checking them gives a polynomial-time nondeterministic machine. A record of
choices from a polynomial-time nondeterministic machine can conversely be
verified by simulation. Illegal transition codes are rejected.

The exact-length convention needs more care for counting than for acceptance.
Suppose an initial verifier allows lengths up to p(n). Use h bits to encode
the length ell, where `h=ceil(log2(p(n)+1))`, followed by p(n) data bits. Require
`0 <= ell <= p(n)`, require the data after ell to be zero, and run the initial
verifier on the first ell bits. Every original witness has exactly one valid
encoding. If arbitrary suffix bits were allowed, a length-ell witness would
instead have `2^(p(n)-ell)` representations. Decision survives that change;
counts do not.

Path records also require encoding discipline. Multiple codes for one
transition or arbitrary bits after early halting can give several records
for one accepting path. Canonical encodings and padding remove this ambiguity.
Acceptance equivalence should not be described as count preservation without
a bijection between the objects being counted.

For each fixed input length, a polynomial-time verifier can be unrolled into
a polynomial-size Boolean circuit. Encode its successive configurations and
connect copies of a circuit implementing one transition. A polynomial time
bound limits both the number of layers and the tape region that can be visited.
The machine description fixes the wiring; x and y remain circuit inputs.
This construction is effective across lengths.

Cook's accepting-computation construction gives a Boolean formula satisfiable
exactly when the bounded nondeterministic computation accepts. His paper also
discusses bounded existential quantification over deterministic polynomial-time
relations [cook-sat][cook-sat]. Levin's formulation studies universal
search problems [levin-search][levin-search]. These sources motivate the
verification viewpoint; the explicit witness encoding above is supplied here
to make the counting convention unambiguous.

With x fixed, asking whether the circuit accepts some y is circuit SAT.
Introduce one value for each internal gate and enforce all gate relations.
For a fixed input assignment, deterministic evaluation provides exactly one
assignment to these gate values, by induction in topological order. Therefore
this particular extension preserves the number of accepted input assignments.

## Decision, search, witness counting, and projected model counting

For fixed x, decision asks whether W_x is empty, search returns one member
when possible, and exact witness counting returns |W_x|. For the whole relation,
there are two different totals:

```text
N_pairs = sum_x |W_x| = sum_(x,y) f(x,y)
N_proj  = sum_x [W_x is nonempty] = sum_x g(x).
```

The bracket is an indicator. N_pairs counts accepting pairs; projected model
counting returns N_proj, counting each successful ordinary input once. They
agree if every nonempty fiber contains exactly one witness. In the running
example N_pairs=3 and N_proj=2.

Exact counting decides existence by checking whether the count is positive.
It also supports search: try a next witness bit, count its completions, and
choose a nonempty branch. The invariant that the chosen prefix has an accepting
extension proves that the final string is a witness. This logical reduction
does not by itself prove that an instance-sensitive exponent survives every
restriction; that requires preserving the appropriate graph representation.

Uniform accepting-pair sampling and uniform projected-input sampling also
differ. The first induces mass `|W_x|/N_pairs` on x. In the example it gives
x=1 probability 2/3; uniform choice from successful projected inputs gives
probability 1/2. Discarding the witness biases toward larger fibers.

The manuscript's all-input counting algorithm computes N_pairs for a circuit
whose named inputs are partitioned into ordinary and witness bits. It does
not thereby compute N_proj. Fixing an ordinary input first gives the witness
count for that input.

## Uniform algorithms and nonuniform circuit existence

A circuit-size bound says that a circuit for g exists. A uniform SAT algorithm
receives a circuit description and computes an answer. A family might admit
small circuits at every length without an efficient procedure generating them.
A projection that is constant has a zero-gate circuit, even when discovering
that constancy from the supplied verifier is difficult.

P/poly is the class of languages with polynomial-size circuit families,
without an efficient-generation requirement. An arbitrary polynomial-size
verifier family therefore does not automatically give the uniform verifier
required for NP. Starting from one polynomial-time algorithm and unrolling it
does give an effective family. These quantifiers must remain distinct when
interpreting projection bounds and their consequences.

A uniform algorithm's input includes the circuit's gate operations, wiring,
output, and named inputs. Its polynomial factors must account for this
description, including inputs that normalization later removes. The size of
a circuit is instead its gate count under a stated basis and wiring convention.
The two measures answer different questions.

Alternation adds universal choices to existential choices; nondeterministic
space studies memory along computations. Those topics are beyond this
chapter's finite existential circuit scope. The next step here is specific:
one path gives one witness, a fixed witness gives deterministic gate values,
shared values need equality, and exact local tables can count globally
consistent assignments.

[cook-sat]: ../sources.md#cook-sat
[levin-search]: ../sources.md#levin-search
