use wasmtime::{
    Config,
    Engine,
    Module,
    Store,
    Linker,
};
use anyhow::Result;
use wasmtime_wasi::{WasiCtxBuilder};
use wasmtime_wasi::p1::WasiP1Ctx;
use wasmtime_wasi::p1::wasi_snapshot_preview1::add_to_linker as add_wasi_to_linker;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Agent: Starting up...");

    // 1. Create a Config and enable async support
    let mut config = Config::new();
    config.async_support(true);

    // 2. Create an Engine from the Config
    let engine = Engine::new(&config)?;

    // 3. Create a WasiP1Ctx directly
    let wasi_p1_ctx = WasiCtxBuilder::new()
        .inherit_stdio() // Inherit standard I/O from the host
        .build_p1();

    // 4. Create a Store with the WasiP1Ctx
    let mut store = Store::new(&engine, wasi_p1_ctx);

    // 5. Create a Linker
    let mut linker = Linker::<WasiP1Ctx>::new(&engine);

    // 6. Add WASI to the linker (synchronous version for p1)
    add_wasi_to_linker(&mut linker, |s: &mut WasiP1Ctx| s)?;

    // 7. Load the WASM module from file
    let wasm_bytes = std::fs::read("target/wasm32-wasip1/debug/hello_world.wasm")?;
    let module = Module::new(&engine, wasm_bytes)?;

    // 8. Instantiate the module
    let instance = linker.instantiate_async(&mut store, &module).await?;

    // 9. Get the `run` function from the WASM module
    let run_func = instance.get_func(&mut store, "run")
        .expect("Failed to find 'run' function in WASM module");

    println!("Agent: Calling WASM module...");

    // 10. Call the `run` function (async version)
    run_func.call_async(&mut store, &[], &mut []).await?;

    println!("Agent: WASM module call finished.");

    Ok(())
}