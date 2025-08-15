cargo build --target wasm32-unknown-unknown --example advanced_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/advanced_web.wasm --out-dir docs --target web
