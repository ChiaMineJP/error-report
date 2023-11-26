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
```
error[E0425]: cannot find value `js` in this scope
```

# Temporal solution
I found a pretty violent solution for this issue.
1. Create a crate for custom attribute which applies to a struct. (Let it be called as `with-wasm` crate)
2. Make the custom attribute append `#[wasm_bindgen::prelude::wasm_bindgen] ` to the head of input token stream  
   and name it like `with_wasm`.
3. Replace `#[wasm_bindgen]` custom attribute in your original file with `#[with_wasm]`.

This solution is just a wrapper for `wasm_bindgen` custom attribute but strangely this works well.
