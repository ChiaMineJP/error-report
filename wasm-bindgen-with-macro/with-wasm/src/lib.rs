extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn with_wasm(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut t = item.to_string();
    t = format!("#[wasm_bindgen::prelude::wasm_bindgen] {}", t);
    t.parse().unwrap()
}
