import init, { from_yaml } from "./w/fadafada_wasm.js";

async function run() {
	await init();
	const s = `delay: 200
timeout: 4000
sources:
  - engine: foo
    endpoints:
      - url: file:///tmp/fadafada_curl/a
  - engine: bar
    endpoints:
      - url: file:///tmp/fadafada_curl/b
`;
	const v = from_yaml(s);
	console.debug(v);

}

run();
