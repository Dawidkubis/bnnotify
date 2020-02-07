def: src/*
	cargo build --release

dev: src/*
	cargo build

.PHONY: def dev
