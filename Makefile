.ONESHELL:
SHELL := /bin/bash

.SILENT:

.PHONY: test-code
test-code:
	cargo test -- --nocapture --color=always

.PHONY: test-format
test-format:
	cargo +nightly fmt --all -- --check

.PHONY: test-clippy
test-clippy:
	cargo +nightly clippy --all

.PHONY: test-coverage-get
test-coverage-get:
	coverage=$$(cargo llvm-cov -- --nocapture  --quiet 2>/dev/null | grep '^TOTAL' | awk '{print $$10}');

	if [ -z "$$coverage" ]
	then
		echo "Tests failed.";
		exit 1;
	fi

	echo "$${coverage/%\%/ }";

.PHONY: test-coverage-export
test-coverage-export:
	if [ -z "$(export)" ]
	then
		EXPORT_PATH="./coverage.lcov";
	else
		EXPORT_PATH="$(export)";
	fi;

	cargo llvm-cov --lcov -- --nocapture > $$EXPORT_PATH 2>/dev/null;
