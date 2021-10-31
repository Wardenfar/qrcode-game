import init, { run_app } from '../assets/build/bundle.js';
async function main() {
   await init('/build/bundle_bg.wasm');
   run_app();
}
main()