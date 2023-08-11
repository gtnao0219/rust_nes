build-wasm:
	wasm-pack build --target web --out-dir ./static/pkg

host: build-wasm
	http-server ./static -c-1

.PHONY: build-wasm host
