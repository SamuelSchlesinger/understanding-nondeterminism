PYTHON ?= python3
LATEXMK ?= latexmk

.PHONY: all pdf check clean

all: pdf

pdf:
	$(LATEXMK) -pdf -silent -interaction=nonstopmode -halt-on-error main.tex

check:
	$(PYTHON) research/data/audit.py

clean:
	$(LATEXMK) -c -silent main.tex
