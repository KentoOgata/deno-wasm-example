pkg: ./src
	wasm-pack build --target web

.PHONY: run
run: pkg
	deno run --allow-read main.ts

.PHONY: test
test: pkg
	cargo test
	fd test -e ts | xargs -r -d '\n' deno test --allow-read

.PHONY: setup
setup:
	wasm-pack --version || cargo install wasm-pack
