import init, {
  wasm_main,
} from "./pkg/phasmo_rs.js";

async function run() {
  await init();
  wasm_main();
}
run();
