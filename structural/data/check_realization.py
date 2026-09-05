#!/usr/bin/env python3
"""Validates: the cubic-kernel realization's conflicts, budgets, and zero loss.

Constructs actual AND gate lists and a chosen equality splitting of their
incidence graphs. Checks every contraction's legality and the exact labeled
terminal graph. Exhaustive graph families are finite implementation checks;
the general theorem is proved analytically in ../realization.md.
"""

from collections import Counter, defaultdict, deque
from itertools import product
import random


def adjacency(n, edges):
    adj = [set() for _ in range(n)]
    for u, v in edges:
        assert u != v and v not in adj[u]
        adj[u].add(v)
        adj[v].add(u)
    assert all(len(ns) == 3 for ns in adj)
    return adj


def path(adj, start, target, allowed, banned=None):
    previous = {start: None}
    queue = deque([start])
    while queue:
        u = queue.popleft()
        if u == target:
            result = []
            while u is not None:
                result.append(u)
                u = previous[u]
            return result[::-1]
        for v in sorted(adj[u]):
            if v in allowed and v not in previous and {u, v} != banned:
                previous[v] = u
                queue.append(v)
    return None


def ear_order(adj):
    """The elementary ear-insertion proof, for a 2-connected input graph."""
    n = len(adj)
    start, end = 0, min(adj[0])
    order = path(adj, start, end, set(range(n)), {start, end})
    assert order is not None
    while len(order) < n:
        old = set(order)
        component = {min(set(range(n)) - old)}
        todo = list(component)
        while todo:
            u = todo.pop()
            for v in adj[u] - old - component:
                component.add(v)
                todo.append(v)
        attachments = [(v, min(adj[v] & component)) for v in order
                       if adj[v] & component]
        assert len(attachments) >= 2
        (a, av), (b, bv) = attachments[:2]
        middle = path(adj, av, bv, component)
        assert middle is not None and order.index(a) < order.index(b)
        index = order.index(a) + 1
        order[index:index] = middle
    return order


class SplitGraph:
    def __init__(self):
        self.nodes = {}
        self.edges = {}

    def node(self, kind, wire=None, mark=None):
        node = len(self.nodes)
        self.nodes[node] = (kind, wire, mark)
        return node

    def edge(self, u, v, wire):
        self.edges[len(self.edges)] = (u, v, wire)

    def ports(self):
        result = defaultdict(list)
        for edge, (u, v, wire) in self.edges.items():
            assert u != v
            result[u].append((edge, v, wire))
            result[v].append((edge, u, wire))
        return result

    def certify_incidence(self, t, gates):
        expected = defaultdict(Counter)
        for offset, (a, b) in enumerate(gates):
            output = 2 * t + offset
            expected[output][output] += 1
            for wire in (a, b):
                if wire >= t:
                    expected[wire][output] += 1
        expected[2 * t + len(gates) - 1]["assert"] += 1
        ports = self.ports()
        groups = defaultdict(set)
        for v, (kind, wire, _) in self.nodes.items():
            assert 1 <= len(ports[v]) <= 3
            if kind == "eq":
                groups[wire].add(v)
                assert all(ref == wire for _, _, ref in ports[v])
            elif kind == "gate":
                a, b = gates[wire - 2 * t]
                assert sorted(ref for _, _, ref in ports[v]) == sorted(
                    [wire] + [ref for ref in (a, b) if ref >= t])
        assert set(groups) == set(expected)
        for wire, vertices in groups.items():
            seen, todo = {min(vertices)}, [min(vertices)]
            external = Counter()
            internal = 0
            for v in vertices:
                for _, u, ref in ports[v]:
                    assert ref == wire
                    if u in vertices:
                        internal += 1
                    else:
                        kind, gate, _ = self.nodes[u]
                        assert kind in {"gate", "assert"}
                        external[gate if kind == "gate" else "assert"] += 1
            while todo:
                for _, u, _ in ports[todo.pop()]:
                    if u in vertices and u not in seen:
                        seen.add(u)
                        todo.append(u)
            assert seen == vertices and internal == 2 * (len(vertices) - 1)
            assert external == expected[wire]
            degree = sum(external.values())
            assert len(vertices) == max(1, degree - 2)
            if len(vertices) > 1:
                assert all(len(ports[v]) == 3 for v in vertices)

    def retain(self, target_edges, k):
        assert len(self.edges) - len(self.nodes) + 1 == k
        steps = 0
        while any(mark is None for _, _, mark in self.nodes.values()):
            ports = self.ports()
            eligible = [v for v, (_, _, mark) in self.nodes.items()
                        if mark is None and len(ports[v]) <= 2]
            assert eligible, "The chosen splitting failed to expose a legal reduction."
            u = min(eligible, key=lambda v: (len(ports[v]), v))
            edge, v, _ = ports[u][0]
            assert 1 <= len(ports[u]) <= 2
            assert len(ports[u]) + len(ports[v]) - 2 <= 3
            assert sum(w == v for _, w, _ in ports[u]) == 1
            del self.edges[edge]
            del self.nodes[u]
            for other, (a, b, wire) in list(self.edges.items()):
                self.edges[other] = (v if a == u else a, v if b == u else b, wire)
            assert len(self.edges) - len(self.nodes) + 1 == k
            steps += 1
        ports = self.ports()
        assert all(len(ports[v]) == 3 for v in self.nodes)
        actual = Counter(tuple(sorted((self.nodes[u][2], self.nodes[v][2])))
                         for u, v, _ in self.edges.values())
        assert actual == Counter(tuple(sorted(e)) for e in target_edges)
        return steps


def realize(n, edges, order, truth_check=False):
    adj = adjacency(n, edges)
    position = {v: i for i, v in enumerate(order)}
    assert set(position) == set(range(n))
    incoming = {v: sorted((u for u in adj[v] if position[u] < position[v]),
                          key=position.get) for v in order}
    source, sink = order[0], order[-1]
    assert len(incoming[source]) == 0 and len(incoming[sink]) == 3
    assert all(len(incoming[v]) in (1, 2) for v in order[1:-1])
    triangle_free = not any(adj[u] & adj[v] for u, v in edges)
    k = n // 2 + 1
    t = (k + 2) // 3 if triangle_free else k // 3 + 1
    names, conflicts = {source: source}, []
    for v in order[1:-1]:
        values = [names[u] for u in incoming[v]]
        if len(values) == 1:
            names[v] = values[0]
        else:
            if values[0] == values[1]:
                conflicts.append((incoming[v][0], v))
            names[v] = v
    assert len({names[u] for u in incoming[sink]}) >= 2
    bound = (2 * k - 1) // 3 if triangle_free else 2 * k // 3
    assert len(conflicts) <= bound <= 2 * t - 1
    fresh = list(range(t)) + list(range(t + 1, 2 * t))
    attachments = defaultdict(list)
    for edge, wire in zip(conflicts, fresh):
        attachments[edge].append(wire)
    first_edge = (source, min(adj[source], key=position.get))
    attachments[first_edge].extend(fresh[len(conflicts):])
    gates, values, chains, sink_pair = [], {source: t}, {}, None

    def gate(a, b):
        assert a != b
        ref = 2 * t + len(gates)
        assert a < ref and b < ref
        gates.append((a, b))
        return ref

    for v in order[1:]:
        ins = []
        for u in incoming[v]:
            wire, chain = values[u], []
            for new_input in attachments[(u, v)]:
                wire = gate(wire, new_input)
                chain.append(wire)
            chains[(u, v)] = chain
            ins.append(wire)
        if v == sink:
            second = next(i for i in (1, 2) if ins[i] != ins[0])
            third = 3 - second
            first_gate = gate(ins[0], ins[second])
            values[v] = gate(first_gate, ins[third])
            sink_pair = {incoming[v][0], incoming[v][second]}
        elif len(ins) == 1:
            values[v] = ins[0]
        else:
            values[v] = gate(*ins)
    q = len(gates)
    used = Counter(ref for gate_inputs in gates for ref in gate_inputs)
    assert q == k + 2 * t - 1
    assert all(used[x] == 1 for x in range(t))
    assert all(used[y] >= 1 for y in range(t, 2 * t))
    dependent = [False] * t + [True] * t
    support = [{i} for i in range(2 * t)]
    for a, b in gates:
        dependent.append(dependent[a] or dependent[b])
        support.append(support[a] | support[b])
    assert all(dependent[2 * t:]) and support[-1] == set(range(2 * t))
    ell = sum(dependent[ref] for gate_inputs in gates for ref in gate_inputs)
    assert ell == 2 * k + 3 * t - 2 and ell - q - t + 1 == k
    active, todo = set(), [2 * t + q - 1]
    while todo:
        ref = todo.pop()
        if ref not in active:
            active.add(ref)
            if ref >= 2 * t:
                todo.extend(gates[ref - 2 * t])
    assert active == set(range(2 * t + q))

    graph, core, gate_nodes = SplitGraph(), {}, {}
    for v in order:
        kind = "eq" if v == source or len(incoming[v]) == 1 else "gate"
        ref = values[v] if v != sink else 2 * t + q - 2
        core[v] = graph.node(kind, ref, v)
        if kind == "gate":
            gate_nodes[ref] = core[v]
    final_ref = 2 * t + q - 1
    final_node = graph.node("gate", final_ref)
    gate_nodes[final_ref] = final_node
    for (u, v), chain in chains.items():
        previous, wire = core[u], values[u]
        for ref in chain:
            node = graph.node("gate", ref)
            gate_nodes[ref] = node
            graph.edge(previous, node, wire)
            fresh_input = gates[ref - 2 * t][1]
            if fresh_input >= t:
                leaf = graph.node("eq", fresh_input)
                graph.edge(leaf, node, fresh_input)
            previous, wire = node, ref
        end = final_node if v == sink and u not in sink_pair else core[v]
        graph.edge(previous, end, wire)
    graph.edge(core[sink], final_node, 2 * t + q - 2)
    assertion = graph.node("assert")
    graph.edge(final_node, assertion, final_ref)
    # Insert the unsplit degree-two consistency variables on gate-to-gate wires.
    for edge, (u, v, wire) in list(graph.edges.items()):
        if graph.nodes[u][0] != "eq" and graph.nodes[v][0] != "eq":
            node = graph.node("eq", wire)
            graph.edges[edge] = (u, node, wire)
            graph.edge(node, v, wire)
    assert len(gate_nodes) == q and len(graph.nodes) <= 4 * q + 2
    graph.certify_incidence(t, gates)
    steps = graph.retain(edges, k)
    if truth_check:
        from check_tensor_compiler import compile_verifier, direct_projection
        for assignment in product((0, 1), repeat=2 * t):
            evaluated = list(assignment)
            for a, b in gates:
                evaluated.append(evaluated[a] & evaluated[b])
            assert evaluated[-1] == int(all(assignment))
        verifier = [(8, a, b) for a, b in gates]
        circuit, output, _, rank = compile_verifier(t, t, verifier)
        expected = 1 << ((1 << t) - 1)
        assert rank == k and circuit.values[output] == expected
        assert direct_projection(t, t, verifier) == expected
    return triangle_free, k % 3 == 0, len(conflicts), steps


def hamiltonian_graphs(n):
    cycle = [(v, (v + 1) % n) for v in range(n)]

    def matching(remaining):
        if not remaining:
            yield []
            return
        u = remaining[0]
        for v in remaining[1:]:
            if (u - v) % n not in (1, n - 1):
                rest = [w for w in remaining[1:] if w != v]
                for tail in matching(rest):
                    yield [(u, v)] + tail
    for extra in matching(list(range(n))):
        yield cycle + extra


def bridged_chain(blocks, bipartite_blocks=False):
    # Terminal caps are cubic graphs with one edge subdivided; internal blocks
    # delete that edge, leaving two ports. Every joining edge is a bridge.
    base_n = 6 if bipartite_blocks else 4
    base = [(u, v) for u in range(base_n) for v in range(u + 1, base_n)
            if (u < 3 <= v if bipartite_blocks else True)]
    a, b = (0, 3) if bipartite_blocks else (0, 1)
    base.remove((a, b))
    cap = [3, 1, 4, 2, 5, 0, 6] if bipartite_blocks else [2, 0, 3, 1, 4]
    middle = [0, 4, 1, 5, 2, 3] if bipartite_blocks else [0, 2, 3, 1]
    edges, order, offset, previous = [], [], 0, None
    for index in range(blocks + 2):
        terminal = index in (0, blocks + 1)
        local_order = cap if index == 0 else cap[::-1] if terminal else middle
        local_edges = base + [(a, base_n), (base_n, b)] if terminal else base
        edges.extend((u + offset, v + offset) for u, v in local_edges)
        if previous is not None:
            edges.append((previous, local_order[0] + offset))
        order.extend(v + offset for v in local_order)
        previous = order[-1]
        offset += base_n + int(terminal)
    return offset, edges, order


def random_biconnected(n, rng):
    while True:
        stubs = list(range(n)) * 3
        rng.shuffle(stubs)
        edges = [tuple(sorted(stubs[i:i + 2])) for i in range(0, 3 * n, 2)]
        if len(set(edges)) != len(edges) or any(u == v for u, v in edges):
            continue
        adj = adjacency(n, edges)
        if all(all(path(adj, min(set(range(n)) - {cut}), v,
                        set(range(n)) - {cut}) is not None
                       for v in range(n) if v != cut) for cut in range(n)):
            return edges, ear_order(adj)


def main():
    stats = Counter()
    for n in (4, 6, 8, 10, 12):
        count = 0
        for edges in hamiltonian_graphs(n):
            tf, exact, conflicts, steps = realize(n, edges, list(range(n)), count == 0)
            stats["total"] += 1
            stats["triangle_free"] += tf
            stats["exact_critical"] += tf and exact
            stats["contractions"] += steps
            count += 1
        print(f"Hamiltonian-cycle plus matching graphs, n={n}: {count} OK")
    for bipartite in (False, True):
        for blocks in range(21):
            n, edges, order = bridged_chain(blocks, bipartite)
            tf, exact, _, steps = realize(n, edges, order, blocks == 0)
            assert tf == bipartite
            stats["total"] += 1
            stats["contractions"] += steps
        print(f"Bridged {'triangle-free' if bipartite else 'triangle'} block chains: 21 OK")
    rng = random.Random(20260904)
    for n in (14, 20, 30, 60):
        for _ in range(8):
            edges, order = random_biconnected(n, rng)
            _, _, _, steps = realize(n, edges, order)
            stats["total"] += 1
            stats["contractions"] += steps
        print(f"Seeded 2-connected cubic graphs with ear orders, n={n}: 8 OK")
    print(f"Triangle-free Hamiltonian cases: {stats['triangle_free']} OK")
    print(f"Exact critical Hamiltonian cases (k divisible by 3): {stats['exact_critical']} OK")
    print("Exhaustive verifier truth tables and symbolic projection checks: 7 OK")
    print(f"Legal degree-one/two contractions: {stats['contractions']} OK")
    print(f"Exact labeled kernels and zero cycle loss: {stats['total']} OK")


if __name__ == "__main__":
    main()
