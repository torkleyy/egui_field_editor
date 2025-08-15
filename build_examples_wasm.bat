cargo build --target wasm32-unknown-unknown --example simple_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/simple_web.wasm --out-dir docs/advanced --target web
cargo build --target wasm32-unknown-unknown --example advanced_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/advanced_web.wasm --out-dir docs/advanced --target web
cargo build --target wasm32-unknown-unknown --example manual_implement_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/manual_implement_web.wasm --out-dir docs/manual_implement --target web
cargo build --target wasm32-unknown-unknown --example shared_data_web --release
wasm-bindgen target/wasm32-unknown-unknown/release/examples/shared_data_web.wasm --out-dir docs/shared_data --target web
cargo build --target wasm32-unknown-unknown --example datepicker_web --release --features="datepicker"
wasm-bindgen target/wasm32-unknown-unknown/release/examples/datepicker_web.wasm --out-dir docs/datepicker --target web
cargo build --target wasm32-unknown-unknown --example nalgebra_glm_web --release --features="nalgebra_glm"
wasm-bindgen target/wasm32-unknown-unknown/release/examples/nalgebra_glm_web.wasm --out-dir docs/nalgebra_glm --target web
