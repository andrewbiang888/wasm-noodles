// You can execute this example with `cargo run --example gcd`

use wasmtime::*;

fn main() -> Result<()> {
    // Load our WebAssembly (parsed WAT in our case), and then load it into a
    // `Module` which is attached to a `Store` cache. After we've got that we
    // can instantiate it.
    let mut store = Store::<()>::default();
    let module = Module::from_file(store.engine(), "examples/gcd/gcd.wat")?;
    // let module = Module::from_file(store.engine(), "examples/gcd/gcd.wasm")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // Invoke `gcd` export
    let gcd = instance.get_typed_func::<(i32, i32), i32>(&mut store, "gcd")?;

    println!("gcd(6, 27) = {}", gcd.call(&mut store, (6, 27))?);
    Ok(())
}
