use with_wasm::with_wasm;

macro_rules! with_wasm {
    ($name:ident {$field:ident: $t:ty}) => {
        #[with_wasm]
        pub struct $name {
            pub $field: $t
        }
    }
}
with_wasm!(AStruct {
    a_field: bool
});
