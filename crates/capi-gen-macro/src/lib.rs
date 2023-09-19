use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn capi_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}