# 🔮 Ritual Modules

This directory contains WASM ritual modules that can be loaded and executed by the Codex Control Engine.

## Creating a Ritual Module

Ritual modules are WASM binaries that export an `execute_ritual` function:

```rust
#[no_mangle]
pub extern "C" fn execute_ritual() -> i32 {
    // Ritual logic here
    // Return codes:
    // 0 = Success
    // 1 = Energy activation
    // 2 = Transformation cycle
    // Other = Custom meaning
    0
}
```

## Compilation

To compile a Rust ritual to WASM:

```bash
rustc --target wasm32-unknown-unknown -O ritual.rs -o ritual.wasm
```

Or use a proper Rust project with:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
```

Then compile with:

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Integration

Add the WASM module path to your ritual definition:

```rust
let ritual = RitualDefinition {
    name: "custom_ritual".to_string(),
    // ... other fields
    wasm_module_path: Some("rituals/custom_ritual.wasm".to_string()),
    native_handler: None,
    // ...
};
```

The engine will automatically load and execute the WASM module in a sandboxed environment.
