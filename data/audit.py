#!/usr/bin/env python3
"""Validates: finite compiler records, corpus links, and citation consistency.

This checks document structure and reruns the finite-domain verifications.
It does not check an analytical theorem or establish literature priority.
"""

from pathlib import Path
import difflib
import re
import subprocess
import sys
from urllib.parse import unquote

RESEARCH = Path(__file__).resolve().parents[1]
PROJECT = RESEARCH.parent
CHECKS = (
    (PROJECT / "scripts/check_claims.py", RESEARCH / "data/forest_expected.txt"),
    (PROJECT / "scripts/check_coverage.py", RESEARCH / "data/coverage_expected.txt"),
    (RESEARCH / "structural/data/check_tensor_compiler.py",
     RESEARCH / "structural/data/expected_output.txt"),
    (RESEARCH / "examples/data/check_examples.py",
     RESEARCH / "examples/data/expected_output.txt"),
)


def prose(text):
    """Remove fenced code and inline code before checking Markdown syntax."""
    lines = []
    fence = None
    for line in text.splitlines():
        marker = re.match(r"\s*(`{3,}|~{3,})", line)
        if marker:
            token = marker.group(1)
            if fence is None:
                fence = token
            elif token[0] == fence[0] and len(token) >= len(fence):
                fence = None
            continue
        if fence is None:
            lines.append(re.sub(r"(`+).*?\1", "", line))
    return "\n".join(lines)


def check_documents():
    docs = {p.resolve(): prose(p.read_text()) for p in RESEARCH.rglob("*.md")}
    sources = (RESEARCH / "sources.md").resolve()
    anchors = set(re.findall(r'<a id="([^"]+)"></a>', docs[sources]))
    used = set()
    edges = {p: set() for p in docs}
    errors = []
    for path, text in docs.items():
        definitions = dict(re.findall(r"^\[([^]]+)\]:\s+(\S+)", text, re.M))
        for display, key in re.findall(r"\[([^]\n]+)\]\[([^]\n]+)\]", text):
            if display != key:
                errors.append(f"{path}: citation display drift {display!r}/{key!r}")
            target = definitions.get(key)
            if target is None:
                errors.append(f"{path}: undefined citation {key}")
                continue
            file, _, anchor = target.partition("#")
            if (path.parent / file).resolve() != sources or anchor != key:
                errors.append(f"{path}: noncanonical citation {key}: {target}")
            if key not in anchors:
                errors.append(f"{path}: missing source anchor {key}")
            used.add(key)
        links = re.findall(r"\[[^]\n]+\]\(([^\s)]+)\)", text)
        links += list(definitions.values())
        for target in links:
            if re.match(r"[a-z]+:", target):
                continue
            file, _, anchor = target.partition("#")
            destination = (path.parent / unquote(file)).resolve() if file else path
            if not destination.exists():
                errors.append(f"{path}: broken local link {target}")
                continue
            if destination in docs:
                edges[path].add(destination)
                if anchor:
                    body = docs[destination]
                    explicit = set(re.findall(r'<a id="([^"]+)"></a>', body))
                    headings = re.findall(r"^#+\s+(.+)$", body, re.M)
                    slugs = {re.sub(r"[^\w -]", "", h.lower()).replace(" ", "-")
                             for h in headings}
                    if unquote(anchor) not in explicit | slugs:
                        errors.append(f"{path}: broken anchor {target}")
    if anchors - used:
        errors.append(f"Unused source anchors: {sorted(anchors-used)}")
    reachable, todo = set(), [(RESEARCH / "index.md").resolve()]
    while todo:
        current = todo.pop()
        if current not in reachable:
            reachable.add(current)
            todo.extend(edges[current] - reachable)
    if docs.keys() - reachable:
        errors.append(f"Unreachable documents: {sorted(docs.keys()-reachable)}")

    tex = "\n".join(p.read_text() for p in [PROJECT / "main.tex"]
                    + sorted((PROJECT / "sections").glob("*.tex")))
    labels = re.findall(r"\\label\{([^}]+)\}", tex)
    if len(labels) != len(set(labels)):
        errors.append("Duplicate LaTeX labels")
    refs = set(re.findall(r"\\(?:eqref|ref)\{([^}]+)\}", tex))
    if refs - set(labels):
        errors.append(f"Undefined LaTeX references: {sorted(refs-set(labels))}")
    bib = set(re.findall(r"@\w+\{([^,]+),", (PROJECT / "references.bib").read_text()))
    cites = set()
    for group in re.findall(r"\\cite\w*(?:\[[^]]*\])*\{([^}]+)\}", tex):
        cites.update(key.strip() for key in group.split(","))
    if cites - bib:
        errors.append(f"Undefined BibTeX keys: {sorted(cites-bib)}")
    if errors:
        raise AssertionError("\n".join(errors))
    print(f"Documents: {len(docs)} reachable; {len(anchors)} canonical sources; "
          f"{len(labels)} LaTeX labels; {len(cites)} cited BibTeX entries: OK")


def main():
    check_documents()
    for script, expected in CHECKS:
        actual = subprocess.check_output([sys.executable, str(script)],
                                         cwd=PROJECT, text=True, timeout=120)
        reference = expected.read_text()
        if actual != reference:
            diff = "".join(difflib.unified_diff(reference.splitlines(True),
                                              actual.splitlines(True),
                                              fromfile=str(expected),
                                              tofile="current output"))
            raise AssertionError(diff)
        print(f"{script.relative_to(PROJECT)}: finite checks and recorded output OK")
    print("Corpus audit: OK. General mathematical claims require the written proofs.")


if __name__ == "__main__":
    main()
