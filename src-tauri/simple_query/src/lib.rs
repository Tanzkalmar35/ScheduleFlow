use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis; // Get the visibility of the function
    let block = &input.block;

    let expanded = quote! {
        #fn_vis fn #fn_name() {
            println!("Function {} is called", stringify!(#fn_name));
            #block
        }
    };

    TokenStream::from(expanded)
}