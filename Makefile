all:
	cargo build --all-features --target=wasm32-unknown-unknown
	/home/lash/.cargo/bin/wasm-bindgen --target web --out-dir example target/wasm32-unknown-unknown/debug/fadafada_wasm.wasm
	sed -i -e s#fadafada_wasm_bg.wasm#http://localhost:8000/fadafada_wasm_bg.wasm#g example/fadafada_wasm.js 

run: all
	cd example && python -mRangeHTTPServer
