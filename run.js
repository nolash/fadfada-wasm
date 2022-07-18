import init, { from_yaml } from "./w/fadafada_wasm.js";

async function run() {
	await init();
	const s = `delay: 200
timeout: 4000
sources:
  - engine: foo
    endpoints:
      - url: http://localhost:8080
      - url: http://localhost/~lash/tmp/fadafada/a
  - engine: bar
      - url: http://localhost:8080
`;
	const v = from_yaml(s);
	console.debug(v);

}

run();
