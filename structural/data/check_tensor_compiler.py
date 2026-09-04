#!/usr/bin/env python3
"""Validates: symbolic Boolean tensor elimination preserves existential projection.

Also checks all charged contraction costs, cubic-kernel cycle rank, and the
median-time pathwidth-to-cutwidth conversion. Only Python's standard library
is used. The Fomin--Hoie asymptotic theorem is NOT tested or implemented here.
"""

from collections import Counter, defaultdict
from fractions import Fraction
from itertools import product
import random


def bits(n):
    return product((0, 1), repeat=n)


class Circuit:
    """An actual B2 straight-line circuit, with truth masks for finite checking."""

    def __init__(self, n):
        self.n = n
        self.full = (1 << (1 << n)) - 1
        self.values = [0, self.full]
        for i in range(n):
            self.values.append(sum(((a >> i) & 1) << a for a in range(1 << n)))
        self.gates = []
        self.negations = {0: 1, 1: 0}

    def gate(self, op, a, b):
        value = 0
        for x, y in bits(2):
            if (op >> (2 * x + y)) & 1:
                value |= (self.values[a] if x else self.full ^ self.values[a]) & (
                    self.values[b] if y else self.full ^ self.values[b]
                )
        self.gates.append((op, a, b))
        self.values.append(value)
        return len(self.values) - 1

    def neg(self, a):
        if a not in self.negations:
            self.negations[a] = self.gate(6, a, 1)
        return self.negations[a]

    def conjunction(self, a, b):
        return self.gate(8, a, b)

    def disjunction(self, xs):
        xs = list(xs)
        if not xs:
            return 0
        result = xs[0]
        for x in xs[1:]:
            result = self.gate(14, result, x)
        return result


class Network:
    """Each edge occurs twice among ports; repeated ports are self-loops."""

    def __init__(self, circuit):
        self.circuit = circuit
        self.nodes = {}
        self.next_node = 0
        self.next_edge = 0
        self.scalar = 1
        self.stats = Counter()

    def edge(self):
        e = self.next_edge
        self.next_edge += 1
        return e

    def node(self, ports, table):
        v = self.next_node
        self.next_node += 1
        self.nodes[v] = (tuple(ports), table)
        return v

    def ends(self):
        endpoints = defaultdict(list)
        for v, (ports, _) in self.nodes.items():
            for e in ports:
                endpoints[e].append(v)
        assert all(len(vs) == 2 for vs in endpoints.values())
        return endpoints

    def rank(self):
        adj = defaultdict(set)
        endpoints = self.ends()
        for u, v in endpoints.values():
            adj[u].add(v)
            adj[v].add(u)
        remaining = set(self.nodes)
        components = 0
        while remaining:
            components += 1
            todo = [remaining.pop()]
            while todo:
                for v in adj[todo.pop()]:
                    if v in remaining:
                        remaining.remove(v)
                        todo.append(v)
        return len(endpoints) - len(self.nodes) + components

    def equality(self, ports):
        ports = list(ports)
        if len(ports) <= 3:
            self.node(ports, {a: int(len(set(a)) <= 1) for a in bits(len(ports))})
            return
        self.stats["equality_split"] += 1
        carry = self.edge()
        self.equality([ports[0], ports[1], carry])
        for e in ports[2:-2]:
            fresh = self.edge()
            self.equality([carry, e, fresh])
            carry = fresh
        self.equality([carry, ports[-2], ports[-1]])

    def contract(self, u, v, shared):
        """Contract precisely the named edges, retaining other ports separately."""
        ap, at = self.nodes.pop(u)
        bp, bt = self.nodes.pop(v)
        shared = tuple(shared)
        aset = set(shared)
        outer = [(0, i, e) for i, e in enumerate(ap) if e not in aset]
        outer += [(1, i, e) for i, e in enumerate(bp) if e not in aset]
        result = {}
        before = len(self.circuit.gates)
        for alpha in bits(len(outer)):
            av, bv = [None] * len(ap), [None] * len(bp)
            for (side, i, _), bit in zip(outer, alpha):
                (av if side == 0 else bv)[i] = bit
            terms = []
            for beta in bits(len(shared)):
                assignment = dict(zip(shared, beta))
                for i, e in enumerate(ap):
                    if e in aset:
                        av[i] = assignment[e]
                for i, e in enumerate(bp):
                    if e in aset:
                        bv[i] = assignment[e]
                terms.append(self.circuit.conjunction(at[tuple(av)], bt[tuple(bv)]))
            result[alpha] = self.circuit.disjunction(terms)
        self.node([e for _, _, e in outer], result)
        cost = len(self.circuit.gates) - before
        assert cost <= 28, (len(ap), len(bp), len(shared), cost)

    def reduce(self):
        original_rank = self.rank()
        initial_vertices = len(self.nodes)
        initial_measure = initial_vertices + len(self.ends())
        start_cost = len(self.circuit.gates)
        while self.nodes:
            measure = len(self.nodes) + len(self.ends())
            done = False
            for v, (ports, table) in list(self.nodes.items()):
                if not ports:
                    self.scalar = self.circuit.conjunction(self.scalar, table[()])
                    del self.nodes[v]
                    self.stats["scalar"] += 1
                    done = True
                    break
                repeated = [e for e, count in Counter(ports).items() if count == 2]
                if repeated:
                    e = repeated[0]
                    keep = [i for i, f in enumerate(ports) if f != e]
                    reduced = {}
                    for alpha in bits(len(keep)):
                        row = [None] * len(ports)
                        for i, bit in zip(keep, alpha):
                            row[i] = bit
                        terms = []
                        for b in (0, 1):
                            for i, f in enumerate(ports):
                                if f == e:
                                    row[i] = b
                            terms.append(table[tuple(row)])
                        reduced[alpha] = self.circuit.disjunction(terms)
                    self.nodes[v] = (tuple(ports[i] for i in keep), reduced)
                    self.stats["loop"] += 1
                    done = True
                    break
            if not done:
                endpoints = self.ends()
                for v, (ports, _) in list(self.nodes.items()):
                    if len(ports) <= 2:
                        e = ports[0]
                        u = next(u for u in endpoints[e] if u != v)
                        self.stats["degree_" + str(len(ports))] += 1
                        self.contract(u, v, [e])
                        done = True
                        break
            if not done:
                pairs = defaultdict(list)
                for e, endpoints in self.ends().items():
                    pairs[tuple(sorted(endpoints))].append(e)
                for (u, v), shared in pairs.items():
                    if len(shared) >= 2:
                        self.stats["parallel_" + str(len(shared))] += 1
                        self.contract(u, v, shared)
                        done = True
                        break
            if not done:
                break
            assert len(self.nodes) + len(self.ends()) < measure
            assert self.rank() <= original_rank
        assert len(self.circuit.gates) - start_cost <= 32 * initial_measure
        assert len(self.circuit.gates) - start_cost <= 80 * initial_vertices
        assert all(len(ports) == 3 for ports, _ in self.nodes.values())
        assert len(self.nodes) <= 2 * original_rank

    def median_order(self):
        if not self.nodes:
            return [], 0
        endpoints = self.ends()
        initial_order = sorted(self.nodes)
        introduced = set()
        bags = []
        for v in initial_order:
            boundary = set()
            for u, w in endpoints.values():
                if u in introduced and w not in introduced:
                    boundary.add(u)
                if w in introduced and u not in introduced:
                    boundary.add(w)
            bags.append(boundary | {v})
            introduced.add(v)
        width = max(map(len, bags)) - 1
        times = {}
        for number, (e, (u, v)) in enumerate(sorted(endpoints.items())):
            common = next(i for i, bag in enumerate(bags) if u in bag and v in bag)
            times[e] = common + Fraction(number + 1, 4 * (len(endpoints) + 1))
        medians = {v: sorted(times[e] for e in ports)[1]
                   for v, (ports, _) in self.nodes.items()}
        order = sorted(self.nodes, key=lambda v: (medians[v], v))
        frontier = set()
        cutwidth = 0
        for v in order:
            frontier.symmetric_difference_update(self.nodes[v][0])
            cutwidth = max(cutwidth, len(frontier))
        assert cutwidth <= width + 2, (cutwidth, width)
        self.stats["median_ties"] += sum(count > 1 for count in Counter(medians.values()).values())
        return order, cutwidth

    def compile(self):
        initial_vertices = len(self.nodes)
        self.reduce()
        order, width = self.median_order()
        if order:
            self.stats["cubic_kernel"] += 1
        frontier = ()
        table = {(): 1}
        for v in order:
            ports, local = self.nodes[v]
            inbound = tuple(sorted(set(frontier) & set(ports)))
            new_frontier = tuple(sorted(set(frontier) ^ set(ports)))
            new_table = {}
            before = len(self.circuit.gates)
            for alpha in bits(len(new_frontier)):
                assignment = dict(zip(new_frontier, alpha))
                terms = []
                for beta in bits(len(inbound)):
                    assignment.update(zip(inbound, beta))
                    old_row = tuple(assignment[e] for e in frontier)
                    local_row = tuple(assignment[e] for e in ports)
                    terms.append(self.circuit.conjunction(table[old_row], local[local_row]))
                new_table[alpha] = self.circuit.disjunction(terms)
            assert len(new_frontier) + len(inbound) <= width + 1
            assert len(self.circuit.gates) - before < 2 ** (width + 2)
            frontier, table = new_frontier, new_table
        assert not frontier
        output = self.circuit.conjunction(self.scalar, table[()])
        assert len(order) <= initial_vertices
        return output, width


def compile_verifier(n, m, gates):
    circuit = Circuit(n)
    if not gates:
        if n + m == 0:
            raise ValueError("An empty input list has no designated final output.")
        # The input convention designates the last input as the output.
        # Quantifying a witness output gives true; an ordinary output is retained.
        return circuit, 1 if m else n + 1, Counter(), 0
    output = n + m + len(gates) - 1
    active = set()

    def visit(ref):
        if ref >= n + m and ref not in active:
            active.add(ref)
            _, left, right = gates[ref - n - m]
            visit(left)
            visit(right)
    visit(output)
    dependent = set(range(n, n + m))
    pure = {i: i + 2 for i in range(n)}
    for ref in sorted(active):
        op, left, right = gates[ref - n - m]
        if left in dependent or right in dependent:
            dependent.add(ref)
        else:
            pure[ref] = circuit.gate(op, pure[left], pure[right])
    core = sorted(active & dependent)
    p, q = len(active) - len(core), len(core)
    if not core:
        return circuit, pure[output], Counter(), 0
    network = Network(circuit)
    occurrences = defaultdict(list)
    used_boundary = set()
    for ref in core:
        op, left, right = gates[ref - n - m]
        out_edge = network.edge()
        occurrences[ref].append(out_edge)
        ports = [out_edge]
        in_edges = {}
        for slot, source in enumerate((left, right)):
            if source in dependent:
                e = network.edge()
                occurrences[source].append(e)
                in_edges[slot] = e
                ports.append(e)
            else:
                used_boundary.add(source)
        symbolic = [pure[source] for slot, source in enumerate((left, right))
                    if slot not in in_edges]
        assert len(symbolic) <= 1
        table = {}
        for alpha in bits(len(ports)):
            assignment = dict(zip(ports, alpha))
            values = []
            for h in (0, 1):
                args = [assignment[in_edges[slot]] if slot in in_edges else h
                        for slot in range(2)]
                values.append(int(((op >> (2 * args[0] + args[1])) & 1) == alpha[0]))
            if values[0] == values[1]:
                table[alpha] = values[0]
            else:
                assert symbolic
                table[alpha] = symbolic[0] if values == [0, 1] else circuit.neg(symbolic[0])
        network.node(ports, table)
    out_edge = network.edge()
    occurrences[output].append(out_edge)
    network.node([out_edge], {(0,): 0, (1,): 1})
    for ports in occurrences.values():
        network.equality(ports)
    rank = network.rank()
    b = sum(n <= ref < n + m for ref in occurrences)
    ell = sum(left in dependent for _, left, _ in (gates[ref - n - m] for ref in core))
    ell += sum(right in dependent for _, _, right in (gates[ref - n - m] for ref in core))
    assert rank == ell - q - b + 1
    assert len(used_boundary) + b + rank <= q + 1
    assert len(network.nodes) <= 4 * q + 2
    assert len(circuit.gates) <= p + len(used_boundary)
    result, width = network.compile()
    assert len(circuit.gates) <= p + 400 * (q + 1) * 2 ** width
    return circuit, result, network.stats, rank


def direct_projection(n, m, gates):
    result = 0
    for x in range(1 << n):
        for y in range(1 << m):
            values = [(x >> i) & 1 for i in range(n)]
            values += [(y >> i) & 1 for i in range(m)]
            for op, left, right in gates:
                values.append((op >> (2 * values[left] + values[right])) & 1)
            if values[-1]:
                result |= 1 << x
                break
    return result


def check_verifier(n, m, gates, totals):
    circuit, output, stats, _ = compile_verifier(n, m, gates)
    assert circuit.values[output] == direct_projection(n, m, gates), (n, m, gates)
    totals.update(stats)


def check_network(edges, rng, totals):
    circuit = Circuit(2)
    network = Network(circuit)
    ports = defaultdict(list)
    for u, v in edges:
        e = network.edge()
        ports[u].append(e)
        ports[v].append(e)
    choices = [0, 1, 2, 3, circuit.neg(2), circuit.neg(3)]
    for ps in ports.values():
        network.node(ps, {row: rng.choice(choices) for row in bits(len(ps))})
    expected = 0
    for alpha in bits(len(edges)):
        term = circuit.full
        for ps, table in network.nodes.values():
            term &= circuit.values[table[tuple(alpha[e] for e in ps)]]
        expected |= term
    output, _ = network.compile()
    assert circuit.values[output] == expected, edges
    totals.update(network.stats)


def main():
    totals = Counter()
    trivial_cases = [(0, 1), (1, 0), (1, 1), (3, 0), (0, 3)]
    for n, m in trivial_cases:
        check_verifier(n, m, [], totals)
    print(f"Zero-gate designated input projections: {len(trivial_cases)} OK")
    count = 0
    for first in product(range(16), range(2), range(2)):
        check_verifier(1, 1, [first], totals)
        count += 1
        for second in product(range(16), range(3), range(3)):
            check_verifier(1, 1, [first, second], totals)
            count += 1
    rng = random.Random(20260904)
    for _ in range(600):
        n, m = rng.randrange(4), rng.randrange(1, 5)
        gates = []
        for _ in range(rng.randrange(1, 23)):
            predecessors = n + m + len(gates)
            gates.append((rng.randrange(16), rng.randrange(predecessors), rng.randrange(predecessors)))
        check_verifier(n, m, gates, totals)
    shapes = [
        [(0, 0)],
        [(0, 1), (0, 1)],
        [(0, 1)] * 3,
        [(0, 1), (0, 1), (0, 2), (1, 3), (2, 3), (2, 3)],
        [(0, 1), (1, 2), (2, 3)],
        [(u, v) for u in range(4) for v in range(u + 1, 4)],
        [(u, v) for u in range(3) for v in range(3, 6)],
    ]
    for shape in shapes:
        for _ in range(20):
            check_network(shape, rng, totals)
    required = ["degree_1", "degree_2", "loop", "parallel_2", "parallel_3",
                "equality_split", "cubic_kernel", "median_ties"]
    assert all(totals[key] for key in required), totals
    print(f"Exhaustive verifier projections: {count} OK")
    print("Seeded verifier projections: 600 OK (seed 20260904)")
    print(f"Adversarial symbolic tensor networks: {20 * len(shapes)} OK")
    print("Reduction and boundary checks: " + ", ".join(f"{k}={totals[k]}" for k in required))
    print("All generated circuits, cycle-rank inequalities and gate-cost checks: OK")


if __name__ == "__main__":
    main()
