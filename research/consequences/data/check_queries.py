#!/usr/bin/env python3
"""Validates: exact prefix counts give lexicographic rank/unrank and uniform ranks.

Finite domain: all Boolean relations on zero through three input bits.
The oracle is an explicit relation table, not the asymptotic tensor algorithm.
The general layout-preserving pinning statement is proved in the manuscript.
"""

from fractions import Fraction
from itertools import product


def unrank(width, count, rank):
    total = count(())
    if not 0 <= rank < total:
        raise ValueError("rank is outside the solution set")
    prefix = ()
    for _ in range(width):
        left = count(prefix + (0,))
        if rank < left:
            prefix += (0,)
        else:
            prefix += (1,)
            rank -= left
    assert rank == 0 and count(prefix) == 1
    return prefix


def rank_of(assignment, count):
    if count(assignment) != 1:
        raise ValueError("assignment is not a solution")
    prefix, rank = (), 0
    for bit in assignment:
        if bit:
            rank += count(prefix + (0,))
        prefix += (bit,)
    return rank


def main():
    relations = solutions_checked = partitions = rejected = 0
    for width in range(4):
        assignments = list(product((0, 1), repeat=width))
        for mask in range(1 << len(assignments)):
            solutions = [a for i, a in enumerate(assignments) if (mask >> i) & 1]

            def count(prefix):
                # Independent table lookup, with no rank/unrank recursion.
                return sum(a[:len(prefix)] == prefix for a in solutions)

            for length in range(width):
                for prefix in product((0, 1), repeat=length):
                    assert count(prefix) == count(prefix + (0,)) + count(prefix + (1,))
                    partitions += 1
            for expected_rank, assignment in enumerate(solutions):
                assert unrank(width, count, expected_rank) == assignment
                assert rank_of(assignment, count) == expected_rank
                solutions_checked += 1
            for bad_rank in (-1, len(solutions)):
                try:
                    unrank(width, count, bad_rank)
                except ValueError:
                    rejected += 1
                else:
                    raise AssertionError("invalid rank accepted")
            for assignment in assignments:
                if assignment not in solutions:
                    try:
                        rank_of(assignment, count)
                    except ValueError:
                        rejected += 1
                    else:
                        raise AssertionError("nonsolution accepted")
            relations += 1

    # Every finite draw outcome is accounted for exactly; no statistical test.
    for total in range(1, 17):
        width = (total - 1).bit_length()
        denominator = 1 << width
        accepted = [rank for rank in range(denominator) if rank < total]
        probability = Fraction(len(accepted), denominator)
        assert probability > Fraction(1, 2)
        assert Fraction(1, denominator) / probability == Fraction(1, total)
        assert 1 / probability < 2

    running = [(0, 1, 0), (1, 0, 1), (1, 1, 0)]
    actual = [a for a in product((0, 1), repeat=3)
              if (a[0] or a[1]) and (a[1] != a[2])]
    assert actual == running
    assert [sum(a[0] == x for a in actual) for x in (0, 1)] == [1, 2]
    print(f"Explicit relations on 0..3 bits: {relations}; prefix partitions: {partitions}: OK")
    print(f"Satisfying assignments ranked and unranked: {solutions_checked}: OK")
    print(f"Invalid ranks and nonsolutions rejected: {rejected}: OK")
    print("Exact rejection probabilities for solution counts 1..16: OK")
    print("Running example: ranks 0,1,2 map to 010,101,110; first-bit counts 1,2: OK")
    print("Scope: finite explicit counting oracles; no asymptotic layout or solver benchmark.")


if __name__ == "__main__":
    main()
