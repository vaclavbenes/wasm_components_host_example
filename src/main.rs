// src/main.rs
use anyhow::{Context, Result};
use std::path::PathBuf;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi;
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

// Import the bindings generated from the world.wit file
mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "adder",
        async: false,
    });
}

pub struct States {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl States {
    pub fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().build();
        Self { table, ctx }
    }
}

impl IoView for States {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for States {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn main() -> Result<()> {
    // Load the component from the file
    println!("Compiling module...");
    let engine = Engine::default();


    let path = PathBuf::from("target/wasm32-wasip1/release/adder.wasm");
    println!("Loading component from: {:?}", path);
    let component = Component::from_file(&engine, path).context("Failed to load component")?;

    let wasi_view = States::new();
    // Create a store with the WASI view
    let mut store = Store::new(&engine, wasi_view);

    // Create a linker
    let mut linker = Linker::new(&engine);
    // Bind the component to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).expect("Could not add wasi to linker");

    println!("Instantiating the component...");
    let instance = bindings::Adder::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate the example world")?;

    let x = 10;
    let y = 20;

    // Call the add function on instance
    println!("Calling add function with x = {} and y = {}...", x, y);
    let result = instance
        .docs_adder_add()
        .call_add(store, x, y)
        .context("Failed to call add function")?;

    println!("Result: {}", result);

    Ok(())
}
