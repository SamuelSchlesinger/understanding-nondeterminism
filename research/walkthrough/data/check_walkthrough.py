#!/usr/bin/env python3
"""Validates: the worked circuit's unique extensions, count 3, fibers 1 and 2,
incidence cycle rank 1, pinned counts, and local exact-contraction identities.

Exhaustive finite checks only; this is not the full counting/layout algorithm.
Run with ordinary Python (assertions enabled); no third-party dependencies.
"""

from itertools import product
from math import prod


BITS = (0, 1)
TRIPLES = tuple(product(BITS, repeat=3))
EDGES = (
    ("x", "G1"), ("y", "G1"), ("a", "G1"),
    ("y", "G2"), ("z", "G2"), ("c", "G2"),
    ("a", "G3"), ("c", "G3"), ("d", "G3"), ("d", "O"),
)


def evaluate(x, y, z):
    a, c = x | y, y ^ z
    return a, c, a & c


def accepts(x, y, z):
    return evaluate(x, y, z)[-1]


def network_weight(values, evidence):
    """Evaluate factors on independently enumerated incidence-edge bits."""
    for variable in "xyzacd":
        occurrences = [values[i] for i, edge in enumerate(EDGES)
                       if edge[0] == variable]
        if len(set(occurrences)) != 1:
            return 0
        if variable in evidence and occurrences[0] != evidence[variable]:
            return 0
    x1, y1, a1, y2, z2, c2, a3, c3, d3, dout = values
    return int(a1 == (x1 | y1) and c2 == (y2 ^ z2)
               and d3 == (a3 & c3) and dout == 1)


def main():
    if not __debug__:
        raise RuntimeError("Run without -O: this validator requires assertions")
    solutions = [t for t in TRIPLES if accepts(*t)]
    assert solutions == [(0, 1, 0), (1, 0, 1), (1, 1, 0)]
    for x, y, z in TRIPLES:
        extensions = [(a, c, d) for a, c, d in TRIPLES
                      if a == (x | y) and c == (y ^ z) and d == (a & c)]
        assert extensions == [evaluate(x, y, z)]

    # Guard the uniqueness argument across all 16^3 binary gate choices.
    for g1, g2, g3 in product(range(16), repeat=3):
        for x, y, z in TRIPLES:
            extensions = [(a, c, d) for a, c, d in TRIPLES
                          if a == ((g1 >> (2 * x + y)) & 1)
                          and c == ((g2 >> (2 * y + z)) & 1)
                          and d == ((g3 >> (2 * a + c)) & 1)]
            assert len(extensions) == 1
    print("Unique gate extensions: 8 toy inputs; 4096 gate triples x 8 inputs")
    print("Accepting triples: 010 101 110; total=3; x-fibers=1,2")

    vertices = {v for edge in EDGES for v in edge}
    reached = {"x"}
    while True:
        expanded = reached | {v for edge in EDGES if reached.intersection(edge)
                              for v in edge}
        if expanded == reached:
            break
        reached = expanded
    assert reached == vertices and len(vertices) == len(EDGES) == 10
    assert len(EDGES) - len(vertices) + 1 == 1
    print("Incidence graph: vertices=10 edges=10 connected=yes cycle-rank=1")

    # All partial evidence on all three inputs, including contradictory branches.
    for pins in product((None, 0, 1), repeat=3):
        evidence = {name: bit for name, bit in zip("xyz", pins) if bit is not None}
        direct = sum(accepts(*t) for t in TRIPLES
                     if all(t["xyz".index(name)] == bit
                            for name, bit in evidence.items()))
        network = sum(network_weight(values, evidence)
                      for values in product(BITS, repeat=len(EDGES)))
        assert network == direct
    # Per-input edge enumeration additionally verifies unique incidence extension.
    for triple in TRIPLES:
        valid = [values for values in product(BITS, repeat=len(EDGES))
                 if network_weight(values, dict(zip("xyz", triple)))]
        assert len(valid) == accepts(*triple)
    print("Pinned incidence counts: all 27 partial input assignments agree")

    a = [[sum(out == (x | y) for x in BITS) for out in BITS] for y in BITS]
    b = [[sum(out == (y ^ z) for z in BITS) for out in BITS] for y in BITS]
    assert a == [[1, 1], [0, 2]] and b == [[1, 1], [1, 1]]
    assert sum(a[y][1] * b[y][1] for y in BITS) == len(solutions)
    assert sum(row[1] for row in a) * sum(row[1] for row in b) == 6
    pinned_a = [[[int(out == (x | y)) for out in BITS] for y in BITS] for x in BITS]
    assert pinned_a == [[[1, 0], [0, 1]], [[0, 1], [0, 1]]]
    assert [sum(t[y][1] * b[y][1] for y in BITS) for t in pinned_a] == [1, 2]
    print("Elimination: A=[[1,1],[0,2]] B=[[1,1],[1,1]]; shared=3 independent=6")

    # Ternary equality chains have exactly one extension for equal boundary bits.
    for degree in range(3, 7):
        for external in product(BITS, repeat=degree):
            extensions = 0
            for internal in product(BITS, repeat=degree - 3):
                if degree == 3:
                    factors = [external]
                else:
                    factors = [(external[0], external[1], internal[0])]
                    factors += [(internal[j - 1], external[j + 1], internal[j])
                                for j in range(1, degree - 3)]
                    factors += [(internal[-1], external[-2], external[-1])]
                extensions += prod(int(len(set(factor)) == 1) for factor in factors)
            assert extensions == int(len(set(external)) == 1)
    print("Equality expansions: all external assignments at degrees 3 through 6")

    loop = [[2, 3], [5, 7]]
    assert sum(loop[r][r] for r in BITS) == 9
    assert sum(map(sum, loop)) == 17
    u, v = [[1, 2], [3, 4]], [[5, 6], [7, 8]]
    w = [[sum(u[r][s] * v[r][t] for r in BITS) for t in BITS] for s in BITS]
    assert w == [[26, 30], [38, 44]]
    assert sum(w[s][s] for s in BITS) == sum(u[r][s] * v[r][s] for r, s in product(BITS, repeat=2)) == 70
    assert sum(map(sum, w)) == 138
    # Every pair of binary 2x2 tables checks joint vs sequential contraction.
    matrices = tuple(product(BITS, repeat=4))
    for left, right in product(matrices, repeat=2):
        joint = sum(left[i] * right[i] for i in range(4))
        intermediate = [[sum(left[2*r+s] * right[2*r+t] for r in BITS)
                         for t in BITS] for s in BITS]
        assert joint == sum(intermediate[s][s] for s in BITS)
    print("Loop: diagonal=9 full-sum=17; parallel: joint=70 trace=70 full-sum=138")
    print("Parallel contraction guard: all 256 pairs of binary 2x2 tables")

    frontier = [sum(u[p][q] * int(r == (p ^ q)) for p, q in product(BITS, repeat=2)) for r in BITS]
    assert frontier == [5, 5]
    for r, s in product(range(9), repeat=2):
        assert bool(r + s) == (bool(r) or bool(s))
        assert bool(r * s) == (bool(r) and bool(s))
    print("Frontier XOR transition: [5,5]; positivity identities: all r,s in 0..8")
    print("PASS: finite local mechanisms only; no full solver or asymptotic layout check")


if __name__ == "__main__":
    main()
