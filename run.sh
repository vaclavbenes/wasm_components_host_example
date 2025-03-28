(cd adder && cargo component build --release)
(cd cmd && cargo component build --release)

wac plug target/wasm32-wasip1/release/cmd.wasm \
    --plug target/wasm32-wasip1/release/adder.wasm  \
    -o main.wasm


wasmtime run main.wasm 