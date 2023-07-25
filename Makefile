pkg: ./src
	wasm-pack build --target web

.PHONY: run
run: pkg
	deno run --allow-read main.ts

.PHONY: setup
setup:
	wasm-pack --version || cargo install wasm-pack
