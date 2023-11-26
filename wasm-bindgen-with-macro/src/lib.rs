macro_rules! test_macro {
    ($name:ident {$field:ident: $t:ty}) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub struct $name {
            pub $field: $t
        }
    }
}

// wasm-pack build reports an error:
//   error[E0425]: cannot find value `js` in this scope
test_macro!(AStruct {
    a_field: u8
});
/*
The above should be expanded as below:

  #[wasm_bindgen::prelude::wasm_bindgen]
  pub struct AStruct {
      pub a_field: u8,
  }

It's weird that running `wasm-pack build` against the above (expanded code)
does not raise any errors.
*/
