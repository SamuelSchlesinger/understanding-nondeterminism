#!/usr/bin/env python3
"""Validates: shared sampled restrictions plus sparse correction compute exists-y f.

Also checks the exact finite occupancy/expected-miss identities and the concavity
step used in research/coverage/index.md. These are construction checks, not
minimum-circuit-size measurements or evidence of literature novelty.
"""

from fractions import Fraction
from itertools import product
from math import log2
from random import Random

from check_claims import Circuit, bits, from_raw


def supports(source):
    result = {-1: (), -2: ()}
    for v in range(source.n + source.m):
        result[v] = () if v < source.n else (v - source.n,)
    for v, gate in source.gates.items():
        result[v] = tuple(sorted(set().union(*(result[p] for p in gate.parents))))
    return result


def point_indicator(target, point):
    """Exactly n-1 B2 gates suffice when n >= 2."""
    if not point:
        return target.const(1)
    if len(point) == 1:
        return 0 if point[0] else target.neg(0)
    out = target.add((0, 1), 1 << (2 * point[0] + point[1]))
    for i in range(2, len(point)):
        out = target.add((out, i), 8 if point[i] else 4)
    return out


def sparse_circuit(n, points):
    """Block synthesis with all block assignments computed by prefix sharing."""
    target = Circuit(n)
    points = tuple(dict.fromkeys(points))
    r = len(points)
    if r == 0:
        return target, target.const(0)
    if r == 1:
        return target, point_indicator(target, points[0])
    k = min(n, r.bit_length() - 1)
    libraries = []
    for start in range(0, n, k):
        library = {(): target.const(1)}
        end = min(start + k, n)
        for coordinate in range(start, end):
            extended = {}
            for prefix, wire in library.items():
                extended[prefix + (0,)] = target.add((wire, coordinate), 4)
                extended[prefix + (1,)] = target.add((wire, coordinate), 8)
            library = extended
        libraries.append((start, end, library))
    indicators = [target.conjunction(library[p[start:end]]
                                    for start, end, library in libraries)
                  for p in points]
    out = target.disjunction(indicators)
    assert len(target.gates) <= 12 * n * r / log2(r + 2)
    return target, out


def raw_positives(n, m, raw):
    positive = set()
    for x in bits(n):
        for y in bits(m):
            values = list(x + y)
            for table, left, right in raw:
                values.append((table >> (2 * values[left] + values[right])) & 1)
            if values[-1]:
                positive.add(x)
                break
    return positive


def compile_sample(source, output, sample, positives):
    target = Circuit(source.n)
    support = supports(source)
    sample = tuple(dict.fromkeys(sample))
    cache = {}

    def restricted(v, witness):
        if v < 0:
            return v
        if v < source.n:
            return v
        if v < source.n + source.m:
            return target.const(witness[v - source.n])
        key = (v, tuple(witness[i] for i in support[v]))
        if key not in cache:
            gate = source.gates[v]
            cache[key] = target.add(tuple(restricted(p, witness) for p in gate.parents),
                                    gate.table)
        return cache[key]

    # Different full witnesses can induce the same output-support restriction.
    # Deduplicate before ORing; general circuit simplification need not recognize
    # non-adjacent repetitions by absorption.
    outputs = tuple(dict.fromkeys(restricted(output, y) for y in sample))
    approximant = target.disjunction(outputs)
    missed = [x for x in positives if not target.evaluate(approximant, x)]
    terms = [approximant] + [point_indicator(target, x) for x in missed]
    out = target.disjunction(terms)
    vertices = list(source.gates) + [output]
    occupied = sum(len({tuple(y[i] for i in support[v]) for y in sample})
                   for v in vertices)
    assert len(target.gates) <= occupied + source.n * len(missed)
    for x in bits(source.n):
        assert target.evaluate(out, x) == (x in positives)
    return occupied, len(missed)


def check_sparse():
    cases = 0
    for n in (2, 3):
        universe = list(bits(n))
        for mask in range(1 << len(universe)):
            points = {x for i, x in enumerate(universe) if mask >> i & 1}
            target, out = sparse_circuit(n, points)
            assert all(target.evaluate(out, x) == (x in points) for x in universe)
            cases += 1
    rng = Random(202609041)
    for n in (4, 5, 6):
        universe = list(bits(n))
        for _ in range(30):
            points = set(rng.sample(universe, rng.randrange(len(universe) + 1)))
            target, out = sparse_circuit(n, points)
            assert all(target.evaluate(out, x) == (x in points) for x in universe)
            cases += 1
    print(f"Sparse block synthesis: {cases} support sets passed.")


def check_sampled_circuits():
    rng = Random(202609042)
    cases = 0
    for _ in range(80):
        n, m = rng.randrange(2, 5), rng.randrange(0, 4)
        raw = []
        for _ in range(rng.randrange(1, 13)):
            limit = n + m + len(raw)
            raw.append((rng.randrange(16), rng.randrange(limit), rng.randrange(limit)))
        source, output = from_raw(n, m, raw)
        positives = raw_positives(n, m, raw)
        universe = list(bits(m))
        masks = (range(1 << len(universe)) if m <= 2 else
                 [0, (1 << len(universe)) - 1] +
                 [rng.randrange(1 << len(universe)) for _ in range(20)])
        for mask in masks:
            sample = [y for i, y in enumerate(universe) if mask >> i & 1]
            compile_sample(source, output, sample, positives)
            cases += 1
    print(f"Shared restrictions plus exact correction: {cases} circuit/sample pairs passed.")


def check_occupancy():
    rng = Random(202609043)
    cases = 0
    for _ in range(35):
        n, m = 2, 2
        raw = []
        for _ in range(8):
            limit = n + m + len(raw)
            raw.append((rng.randrange(16), rng.randrange(limit), rng.randrange(limit)))
        source, output = from_raw(n, m, raw)
        support = supports(source)
        positives = raw_positives(n, m, raw)
        universe = list(bits(m))
        weights = [rng.randrange(0, 5) for _ in universe]
        weights[0] += 1
        probabilities = [Fraction(w, sum(weights)) for w in weights]
        masses = {x: sum(p for y, p in zip(universe, probabilities)
                         if source.evaluate(output, x, y)) for x in positives}
        vertices = list(source.gates) + [output]
        for t in range(4):
            expected_occupied = Fraction(0)
            for v in vertices:
                for alpha in bits(len(support[v])):
                    p = sum(p for y, p in zip(universe, probabilities)
                            if tuple(y[i] for i in support[v]) == alpha)
                    expected_occupied += 1 - (1 - p) ** t
            expected_missed = sum((1 - p) ** t for p in masses.values())
            actual_occupied, actual_missed = Fraction(0), Fraction(0)
            expected_sparse = 0.0
            for indices in product(range(len(universe)), repeat=t):
                probability = Fraction(1)
                for i in indices:
                    probability *= probabilities[i]
                if probability == 0:
                    continue
                sample = tuple(universe[i] for i in indices)
                occupied = sum(len({tuple(y[i] for i in support[v]) for y in sample})
                               for v in vertices)
                missed = sum(not any(source.evaluate(output, x, y) for y in sample)
                             for x in positives)
                actual_occupied += probability * occupied
                actual_missed += probability * missed
                expected_sparse += float(probability) * missed / log2(missed + 2)
            assert actual_occupied == expected_occupied
            assert actual_missed == expected_missed
            jensen_bound = float(expected_missed) / log2(float(expected_missed) + 2)
            assert expected_sparse <= jensen_bound + 1e-10
            cases += 1
    print(f"Exact rational occupancy/alteration identities: {cases} cases passed;")
    print("  includes biased distributions, zero masses, t=0, and sparse-correction Jensen checks.")


if __name__ == "__main__":
    check_sparse()
    check_sampled_circuits()
    check_occupancy()
