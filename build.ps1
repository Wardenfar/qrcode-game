cd client
wasm-pack build --target web --out-dir ../assets/build --out-name bundle
rollup ./main.js --format iife --file ../assets/build/bundle.js