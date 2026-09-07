PYTHON ?= python3
LATEXMK ?= latexmk

.PHONY: all pdf sat-note check clean

all: pdf

pdf:
	$(LATEXMK) -pdf -silent -interaction=nonstopmode -halt-on-error main.tex

sat-note:
	mkdir -p output/pdf
	$(LATEXMK) -pdf -silent -interaction=nonstopmode -halt-on-error -outdir=output/pdf sat-note.tex

check:
	$(PYTHON) research/data/audit.py

clean:
	$(LATEXMK) -c -silent main.tex
	$(LATEXMK) -c -silent -outdir=output/pdf sat-note.tex
