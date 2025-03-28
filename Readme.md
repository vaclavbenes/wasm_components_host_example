# Play with wasm component model

Install components in rust
https://component-model.bytecodealliance.org/language-support/rust.html

Install wasmtime, wac.

```
run.sh
```

## From

[](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/tutorial/wit/calculator/world.wit)



## Compose components

```
wac plug target/wasm32-wasip1/release/cmd.wasm \
    --plug target/wasm32-wasip1/release/adder.wasm  \
    -o main.wasm
```

## Todo

add syscall