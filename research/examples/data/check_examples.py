#!/usr/bin/env python3
"""Validates: exact verifier, cover, support-cost, and distribution calculations.

Also checks the sparse correction construction on finite truth tables. These
finite checks do not establish asymptotic claims or search for optimal circuits.
The proofs and precisely named compiler comparisons are in ../index.md and
../stress-tests.md. Only the Python standard library is required.
"""

from collections import Counter
from dataclasses import dataclass
from fractions import Fraction
from itertools import product
from math import ceil, log2
from random import Random


def bit_vectors(width):
    return product((0, 1), repeat=width)


@dataclass
class Circuit:
    n: int
    m: int

    def __post_init__(self):
        self.gates = []

    def gate(self, table, left, right):
        wire = self.n + self.m + len(self.gates)
        self.gates.append((table, left, right))
        return wire

    def fold(self, table, wires):
        wires = list(wires)
        result = wires[0]
        for wire in wires[1:]:
            result = self.gate(table, result, wire)
        return result

    def evaluate(self, output, ordinary, witness=()):
        values = list(ordinary) + list(witness)

        def read(wire):
            return values[wire] if wire >= 0 else -1 - wire

        for table, left, right in self.gates:
            values.append((table >> (2 * read(left) + read(right))) & 1)
        return read(output)

    def supports(self):
        supports = [0] * self.n + [1 << j for j in range(self.m)]

        def support(wire):
            return supports[wire] if wire >= 0 else 0

        for _, left, right in self.gates:
            supports.append(support(left) | support(right))
        return supports


XOR, XNOR, AND, OR = 6, 9, 8, 14


def cyclic_verifier(N, k, ell):
    assert N >= 2 and k >= 3 and ell >= 2
    circuit = Circuit(N + k, k + ell)
    z = list(range(N))
    u = list(range(N, N + k))
    a = list(range(circuit.n, circuit.n + k))
    b = list(range(circuit.n + k, circuit.n + circuit.m))
    parity_z = circuit.fold(XOR, z)
    differences = [circuit.gate(XOR, a[i], a[(i + 1) % k])
                   for i in range(k)]
    equations = [circuit.gate(XNOR, u[i], differences[i])
                 for i in range(k)]
    consistency = circuit.fold(AND, equations)
    parity_b = circuit.fold(XOR, b)
    ordinary_and_consistency = circuit.gate(AND, parity_z, consistency)
    output = circuit.gate(AND, ordinary_and_consistency, parity_b)
    return circuit, output


def selected_witnesses(k, ell):
    # Witness integers have a_1 in bit zero. Fix a_1=0 and b_1=1,
    # b_2=...=b_ell=0, and enumerate a_2,...,a_k.
    return [(tail << 1) | (1 << k) for tail in range(1 << (k - 1))]


def integer_bits(value, width):
    return tuple((value >> bit) & 1 for bit in range(width))


def support_cost(circuit, output, witnesses=None):
    supports = circuit.supports()
    if witnesses is None:
        occupancy = lambda mask: 1 << mask.bit_count()
    else:
        occupancy = lambda mask: len({witness & mask for witness in witnesses})
    gate_cost = sum(occupancy(mask) for mask in supports[circuit.n + circuit.m:])
    return gate_cost + max(occupancy(supports[output]) - 1, 0)


def check_cyclic_family():
    instances = 0
    truth_table_pairs = 0
    for N, k, ell in ((2, 3, 2), (2, 4, 2), (2, 5, 2), (3, 3, 3)):
        circuit, output = cyclic_verifier(N, k, ell)
        assert len(circuit.gates) == N + 3 * k + ell - 1
        H = selected_witnesses(k, ell)
        selected = {integer_bits(witness, circuit.m) for witness in H}
        sensitivity = [False] * (circuit.n + circuit.m)
        fibers = []
        table = {}
        for ordinary in bit_vectors(circuit.n):
            fiber = set()
            for witness in bit_vectors(circuit.m):
                actual = circuit.evaluate(output, ordinary, witness)
                a, b = witness[:k], witness[k:]
                expected = ((sum(ordinary[:N]) % 2 == 1)
                            and all(ordinary[N + i] == (a[i] ^ a[(i + 1) % k])
                                    for i in range(k))
                            and sum(b) % 2 == 1)
                assert actual == expected
                table[ordinary + witness] = actual
                truth_table_pairs += 1
                if actual:
                    fiber.add(witness)
            expected_projection = (sum(ordinary[:N]) % 2 == 1
                                   and sum(ordinary[N:]) % 2 == 0)
            assert bool(fiber) == expected_projection
            if fiber:
                assert len(fiber) == 1 << ell
                assert len(fiber & selected) == 1
                fibers.append(frozenset(fiber))
        distinct_fibers = set(fibers)
        assert len(distinct_fibers) == 1 << (k - 1)
        assert sum(map(len, distinct_fibers)) == len(set().union(*distinct_fibers))
        for point, value in table.items():
            for variable in range(len(point)):
                flipped = point[:variable] + (1 - point[variable],) + point[variable + 1:]
                sensitivity[variable] |= value != table[flipped]
        assert all(sensitivity)
        instances += 1

    for k in range(3, 11):
        N, ell = 1 << k, k
        circuit, output = cyclic_verifier(N, k, ell)
        uses = Counter(wire for _, left, right in circuit.gates
                       for wire in (left, right))
        shared = [wire for wire, count in uses.items()
                  if wire >= circuit.n and count >= 2]
        assert len(shared) == k
        H = selected_witnesses(k, ell)
        assert support_cost(circuit, output, H) == N + ell + 8 * k + 3 * (1 << k) - 15
        assert support_cost(circuit, output) == (
            N + 8 * k + 4 * (1 << k) + (1 << (ell + 1))
            + (1 << (k + ell + 1)) - 14)
    print(f"Cyclic verifier truth tables: {instances} instances, {truth_table_pairs} input pairs passed.")
    print("Cyclic exact gate/support/shared-vertex formulas: k=3,...,10 passed.")


def check_same_profile():
    for k in range(2, 7):
        rows_A, rows_B = [], []
        for u in bit_vectors(k):
            rows_A.append({(0,) + (0,) * k, (1,) + u})
            rows_B.append({(0,) + u, (1,) + tuple(1 - bit for bit in u)})
        assert all(len(row) == 2 for row in rows_A + rows_B)
        assert len(set.intersection(*rows_A)) == 1
        assert sum(map(len, rows_B)) == len(set.union(*rows_B))
        assert len(rows_B) == 1 << k
    print("Identical fiber profiles with covers 1 versus 2^k: k=2,...,6 passed.")


def uniform_objective(L, t):
    occupancy = 4 * L * (1 - Fraction(1, 2) ** t)
    occupancy += 8 * (1 - Fraction(3, 4) ** t)
    missed = (1 << (2 * L)) * Fraction(1, 4) ** t
    return occupancy + 2 * L * missed


def pair_objective(L, t):
    if t == 0:
        return Fraction(2 * L * (1 << (2 * L)))
    occupancy = L + (2 * L + 4) * (1 - Fraction(1, 2) ** t)
    missed = (1 << (2 * L - 1)) * Fraction(1, 2) ** t
    return occupancy + 2 * L * missed


def check_distribution_tradeoff():
    for L in (8, 16, 32, 64):
        t_pair = 2 * L + ceil(log2(2 * L))
        pair_value = pair_objective(L, t_pair)
        pair_upper = Fraction(6 * L + 9, 2)
        uniform_lower = 4 * L * (1 - Fraction(1, 2) ** L)
        assert pair_value <= pair_upper < uniform_lower
        # The proof controls every t. This finite scan separately checks the
        # formula's endpoint t=0 and the proposed gap near its minimum.
        for t in range(4 * L + 1):
            assert uniform_objective(L, t) > pair_value
        print(f"Distribution tradeoff L={L}: pair value {float(pair_value):.6f}; "
              f"uniform lower bound {float(uniform_lower):.6f}.")


def point_gate_table(first, second):
    return 1 << (2 * first + second)


def block_indicators(circuit, block):
    if len(block) == 1:
        return {(0,): circuit.gate(3, block[0], -1), (1,): block[0]}
    library = {(a, b): circuit.gate(point_gate_table(a, b), block[0], block[1])
               for a, b in bit_vectors(2)}
    for wire in block[2:]:
        extended = {}
        for prefix, prior in library.items():
            for bit in (0, 1):
                extended[prefix + (bit,)] = circuit.gate(4 if bit == 0 else 8,
                                                        prior, wire)
        library = extended
    return library


def sparse_circuit(n, positive):
    circuit = Circuit(n, 0)
    R = len(positive)
    if R == 0:
        return circuit, -1
    if R == 1:
        point = next(iter(positive))
        output = circuit.gate(point_gate_table(*point[:2]), 0, 1)
        for variable in range(2, n):
            output = circuit.gate(4 if point[variable] == 0 else 8,
                                  output, variable)
        return circuit, output
    block_size = R.bit_length() - 1
    blocks = [list(range(start, min(n, start + block_size)))
              for start in range(0, n, block_size)]
    libraries = [block_indicators(circuit, block) for block in blocks]
    indicators = [circuit.fold(AND, (library[tuple(point[j] for j in block)]
                                    for library, block in zip(libraries, blocks)))
                  for point in sorted(positive)]
    output = circuit.fold(OR, indicators)
    assert len(circuit.gates) <= 3 * len(blocks) * R - 1
    assert len(circuit.gates) <= 18 * n * R / log2(R + 2)
    return circuit, output


def check_sparse_correction():
    rng = Random(20260904)
    instances = 0
    for n in range(2, 9):
        universe = list(bit_vectors(n))
        cardinalities = {0, 1, 2, 3, 1 << (n - 1), (1 << n) - 1, 1 << n}
        for R in sorted(cardinalities):
            for _ in range(3):
                positive = set(rng.sample(universe, R))
                circuit, output = sparse_circuit(n, positive)
                for point in universe:
                    assert circuit.evaluate(output, point) == (point in positive)
                instances += 1
    print(f"Sparse block correction: {instances} truth-table instances passed; seed=20260904.")


def check_gatewise_incompatibility():
    witnesses = list(bit_vectors(2))
    covers = []
    for mask in range(1 << len(witnesses)):
        H = [witnesses[j] for j in range(len(witnesses)) if mask & (1 << j)]
        if all(any(witness != label for witness in H) for label in witnesses):
            covers.append(tuple(len({witness[j] for witness in H}) for j in range(2)))
    assert sum(min(cost[j] for cost in covers) for j in range(2)) == 2
    assert min(sum(cost) for cost in covers) == 3
    print("Incompatible per-gate cover minimizers: sum of minima 2, minimum of sum 3 passed.")


if __name__ == "__main__":
    check_cyclic_family()
    check_same_profile()
    check_distribution_tradeoff()
    check_sparse_correction()
    check_gatewise_incompatibility()
