use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn async_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let output = quote! {
        #[cfg_attr(
            not(all(target_arch = "wasm32", target_os = "unknown")),
            async_trait::async_trait
        )]
        #[cfg_attr(
            all(target_arch = "wasm32", target_os = "unknown"),
            async_trait::async_trait(?Send)
        )]
        #input
    };

    output.into()
}
