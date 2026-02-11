# touched

`touched` is a utility crate for writing fuzzing harnesses of callback-style and trait-style Rust crates.

To use `touched`, add it in your `Cargo.toml`:

```toml
[dependencies]
# Optionally, you can add a "derive" feature for derive macros
touched = "0.1"
```

## Background and Usage

```rust
use touched::Touchable;

// Target crate
pub trait Callbacks {
    fn callback(&self, s: &[u8]);
}
#[derive(Touchable)] // <-- Add this to enable touching this struct
pub struct Data {
    pub a: u8,
    pub b: [u8; 32]
}
pub fn callback_style_api(data: &[u8], f: impl Fn(&Data)) { /* .. */ }
pub fn trait_style_api(data: &[u8], handler: &impl Callbacks) { /* .. */ }

// Fuzzing harness
struct FuzzingCallbacks;
impl Callbacks for FuzzingCallbacks {
    fn callback(&self, s: &[u8]) {
        touched::touching(s);
    }
}
fn harness(data: &[u8]) {
    callback_style_api(data, touched::touching::<Data>);
    trait_style_api(data, &FuzzingCallbacks);
}
```

Rust has flexible patterns for providing callbacks in library APIs. In the above example, `callback_style_api` takes a closure `f` as callback, and may invoke this callback with prepared data. `trait_style_api` uses a more complicated interface, which takes a handler implementing `Callbacks`. This is usually designed for advanced callbacks, which may have persistent data or with multiple mutual-related callbacks.

However, when the target crate passes prepared data to the callbacks, such data may be created with unsafe code, and may have UBs. For now, most of Rust fuzzing works only integrated with basic address sanitizers, which can only detect violations when the memory is actually accessed instead of alerting at the construction point (e.g. `slice::from_raw_parts`). As a result, in the fuzzing harness, each argument should be checked in the callback.

`touched` provides the capability to access each valid memory unit in a reference, which can trigger the address sanitizer's check. For references of primitive types, such as `&usize`, `&(u8, u16)`, or `&[u8]`, you can use the function [`touching`][touching] to access it. For complex types including custom structures, you should implement [`Touchable`][Touchable-trait] trait for it. This trait can be directly derived if the "derive" feature is enabled.

[touching]: https://docs.rs/touched/latest/touched/fn.touching.html
[Touchable-trait]: https://docs.rs/touched/latest/touched/trait.Touchable.html
