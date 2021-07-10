extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(TestProcMacroDerive)]
pub fn derive_test_proc_macro(_item: TokenStream) -> TokenStream {
    "fn derive_test_proc_macro() -> u32 { 42 }".parse().unwrap()
}
