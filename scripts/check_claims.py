#!/usr/bin/env python3
"""Validates: forest compilation and projection counts on stated finite domains.

This checks an actual circuit-producing implementation of the forest-cut
simulation against direct existential evaluation. It does not estimate
minimum circuit complexity or replace the proof in the note.
"""

from collections import Counter
from dataclasses import dataclass
from itertools import product
from math import comb
from random import Random


def bits(width):
    return product((0, 1), repeat=width)


def truth(table, values):
    index = 0
    for value in values:
        index = 2 * index + value
    return (table >> index) & 1


@dataclass(frozen=True)
class Gate:
    parents: tuple
    table: int


class Circuit:
    """Input IDs are 0..n+m-1; constants use -1 and -2."""

    def __init__(self, n, m=0):
        self.n = n
        self.m = m
        self.gates = {}
        self.next_id = n + m

    @staticmethod
    def const(value):
        return -1 - value

    def add(self, parents, table):
        """Propagate constants, merge repeated pins, and drop unused pins."""
        unique = tuple(dict.fromkeys(v for v in parents if v >= 0))
        outputs = {}
        for assignment in bits(len(unique)):
            env = dict(zip(unique, assignment))
            values = tuple(env[v] if v >= 0 else -1 - v for v in parents)
            outputs[assignment] = truth(table, values)
        active = []
        for i in range(len(unique)):
            if any(outputs[a] != outputs[a[:i] + (1 - a[i],) + a[i + 1:]]
                   for a in outputs):
                active.append(i)
        kept = tuple(unique[i] for i in active)
        reduced = 0
        for index, assignment in enumerate(bits(len(kept))):
            full = [0] * len(unique)
            for i, value in zip(active, assignment):
                full[i] = value
            reduced |= outputs[tuple(full)] << index
        if not kept:
            return self.const(reduced)
        if len(kept) == 1 and reduced == 2:
            return kept[0]
        node = self.next_id
        self.next_id += 1
        self.gates[node] = Gate(kept, reduced)
        return node

    def neg(self, value):
        return self.add((value,), 1)

    def conjunction(self, values):
        out = self.const(1)
        for value in values:
            out = self.add((out, value), 8)
        return out

    def disjunction(self, values):
        out = self.const(0)
        for value in values:
            out = self.add((out, value), 14)
        return out

    def cone(self, output):
        seen = set()

        def visit(node):
            if node < 0 or node in seen:
                return
            seen.add(node)
            if node in self.gates:
                for parent in self.gates[node].parents:
                    visit(parent)

        visit(output)
        return seen

    def evaluate(self, output, x, y=()):
        values = dict(enumerate(x + y))
        values[-1], values[-2] = 0, 1
        for node, gate in self.gates.items():
            values[node] = truth(gate.table, tuple(values[p] for p in gate.parents))
        return values[output]


def compile_projection(source, output):
    target = Circuit(source.n)
    if output < 0:
        return target, output, 0, 0, 0, 0
    cone = source.cone(output)
    gates = sorted(cone.intersection(source.gates))
    ordinary = {v for v in cone if v < source.n}
    witnesses = {v for v in cone if source.n <= v < source.n + source.m}
    uses = Counter(p for v in gates for p in source.gates[v].parents)
    cut = sorted(v for v in cone if v >= source.n and uses[v] >= 2)
    size, a, b, r = len(gates), len(ordinary), len(witnesses), len(cut)
    assert a + b <= size + 1
    assert r <= size + 1 - a - b
    if not gates:
        if output < source.n:
            return target, output, size, a, b, r
        return target, target.const(1), size, a, b, r

    branches = []
    for assignment in bits(r):
        fixed = dict(zip(cut, assignment))
        cache = {}

        def possible(node, defining=False):
            # A cut gate is treated as its guessed value at its uses,
            # but its defining equation is evaluated at its own root.
            key = (node, defining)
            if key in cache:
                return cache[key]
            if node < 0 or (node in fixed and not defining):
                value = -1 - node if node < 0 else fixed[node]
                out = (target.const(1 - value), target.const(value))
            elif node < source.n:
                out = (target.neg(node), node)
            elif node < source.n + source.m:
                out = (target.const(1), target.const(1))
            else:
                gate = source.gates[node]
                pairs = [possible(p) for p in gate.parents]
                alternatives = [[], []]
                for values in bits(len(pairs)):
                    feasible = target.conjunction(pair[value]
                                                  for pair, value in zip(pairs, values))
                    alternatives[truth(gate.table, values)].append(feasible)
                out = tuple(target.disjunction(terms) for terms in alternatives)
            cache[key] = out
            return out

        requirements = [possible(output, defining=True)[1]]
        for node in cut:
            if node in source.gates:
                requirements.append(possible(node, defining=True)[fixed[node]])
        branches.append(target.conjunction(requirements))
    compiled = target.disjunction(branches)
    # A deliberately loose explicit constant checks the claimed O(s 2^r)
    # construction, including gates that later become unused.
    assert len(target.gates) <= 32 * (size + 1) * (1 << r)
    return target, compiled, size, a, b, r


def from_raw(n, m, raw):
    source = Circuit(n, m)
    wires = list(range(n + m))
    for table, left, right in raw:
        wires.append(source.add((wires[left], wires[right]), table))
    return source, wires[-1]


def check_raw(n, m, raw):
    source, output = from_raw(n, m, raw)
    target, compiled, size, a, b, r = compile_projection(source, output)
    for x in bits(n):
        expected = 0
        # This evaluator uses the original, unsimplified circuit, independently
        # of both source normalization and the forest recurrence.
        for y in bits(m):
            values = list(x + y)
            for table, left, right in raw:
                values.append((table >> (2 * values[left] + values[right])) & 1)
            assert source.evaluate(output, x, y) == values[-1]
            expected |= values[-1]
        actual = target.evaluate(compiled, x)
        assert actual == expected, (n, m, raw, x, expected, actual)
    if output >= 0:
        assert min(a, b, size + 1 - a - b) <= (size + 1) / 3
    return r


def check_forests():
    count = 0
    # All circuits with one ordinary input, one witness input, and one
    # or two binary gates, including all 16 gate truth tables and repeated pins.
    for first in product(range(16), range(2), range(2)):
        check_raw(1, 1, [first])
        count += 1
        for second in product(range(16), range(3), range(3)):
            check_raw(1, 1, [first, second])
            count += 1
    print(f"Exhaustive forest simulation: {count} circuits passed.")

    seed = 20260904
    rng = Random(seed)
    max_cut = 0
    nondegenerate = (1, 2, 4, 6, 7, 8, 9, 11, 13, 14)
    for _ in range(2000):
        n, m = rng.randrange(0, 5), rng.randrange(0, 5)
        if n + m == 0:
            n = 1
        raw = []
        for _ in range(rng.randrange(0, 13)):
            limit = n + m + len(raw)
            raw.append((rng.choice(nondegenerate), rng.randrange(limit), rng.randrange(limit)))
        max_cut = max(max_cut, check_raw(n, m, raw))
    # Cut-gate consistency and a contradictory shared witness.
    fixtures = [
        (1, 1, [(8, 0, 1), (14, 2, 1), (8, 2, 3)]),
        (1, 1, [(3, 1, 0), (8, 1, 2)]),
        (1, 2, [(6, 1, 2), (8, 0, 3), (14, 3, 4), (8, 4, 5)]),
    ]
    for n, m, raw in fixtures:
        max_cut = max(max_cut, check_raw(n, m, raw))
    print(f"Seeded forest simulation: 2000 random circuits and {len(fixtures)} fixtures passed;")
    print(f"  seed={seed}, maximum cut-set size exercised={max_cut}.")


def check_projection_distribution():
    n, m = 2, 2
    rows, columns = 1 << n, 1 << m
    counts = Counter()
    zeros = Counter()
    for table in range(1 << (rows * columns)):
        projected = 0
        for row in range(rows):
            if (table >> (row * columns)) & ((1 << columns) - 1):
                projected |= 1 << row
        counts[projected] += 1
        zeros[rows - projected.bit_count()] += 1
    for projected in range(1 << rows):
        assert counts[projected] == ((1 << columns) - 1) ** projected.bit_count()
    for z in range(rows + 1):
        assert zeros[z] == comb(rows, z) * ((1 << columns) - 1) ** (rows - z)
    print(f"Projection distribution: all {1 << (rows * columns)} truth tables passed (n={n}, m={m}).")


if __name__ == "__main__":
    check_forests()
    check_projection_distribution()
