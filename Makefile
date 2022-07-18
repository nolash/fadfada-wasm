all:
	cargo build --all-features --target=wasm32-unknown-unknown
	/home/lash/.cargo/bin/wasm-bindgen --target web --out-dir example target/wasm32-unknown-unknown/debug/fadfada_wasm.wasm
	sed -i -e s#fadfada_wasm_bg.wasm#http://localhost:8000/fadfada_wasm_bg.wasm#g example/fadfada_wasm.js 

run: all
	cd example && python -mRangeHTTPServer
