
.PHONY: test help clean bench
.SUFFIXES: .md .pdf

test:
	cargo test

.md.pdf:
	pandoc -s -o $@ $<

help: HELP.pdf README.pdf

clean:
	cargo clean
	rm -f *.pdf

bench:
	cargo bench
