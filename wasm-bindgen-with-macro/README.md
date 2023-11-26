# Summary
https://github.com/rustwasm/wasm-bindgen/issues/3707

I defined a simple macro rule as below, which just adds a line `#[wasm-bindgen]` to
a struct definition.

```rust
macro_rules! with_wasm {
    ($name:ident {$field:ident: $t:ty}) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub struct $name {
            pub $field: $t
        }
    }
}
```

This macro simply converts
```rust
with_wasm!(AStruct {
    a_field: u8
});
```
to
```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct AStruct {
    pub a_field: u8,
}
```

While the expanded code (the below) passes `wasm-pack build` without error,
the original code with macro (the above) fails with error:
`error[E0425]: cannot find value `js` in this scope`
