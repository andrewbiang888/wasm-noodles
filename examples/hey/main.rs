use wasmtime::*;

struct MyState {
    name: String,
    count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an engine and a store
    let engine = Engine::default();
    // Load a Wasm module
    let mut store = Store::new(
        &engine,
        MyState {
            name: "hello, world!".to_string(),
            count: 0,
        },
    );
    let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
        println!("Calling back...");
        println!("> {}", caller.data().name);
        caller.data_mut().count += 1;
    });
    let module = Module::from_file(&engine, "examples/hey/example.wasm")?;

    // Create an instance of the module
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    // Call an exported function (e.g., "hello")
    let hello = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    hello.call(&mut store, ())?;

    Ok(())
}
