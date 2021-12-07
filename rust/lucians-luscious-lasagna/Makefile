
.PHONY: test help clean
.SUFFIXES: .md .pdf

test:
	cargo test

.md.pdf:
	pandoc -s -o $@ $<

help: HELP.pdf HINTS.pdf README.pdf

clean:
	cargo clean
	rm -f *.pdf
