extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn log_message(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = parse_macro_input!(attr as LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        fn #fn_name() {
            println!("{}", #message);
            #fn_block
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn bench_message(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message = parse_macro_input!(attr as LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;

    // Convert the LitStr into a &str
    let message_str = message.value();
    let msg = message_str.as_str();

    let expanded = quote! {
         #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            use crate::bencher::Bencher;
            let mut bencher = Bencher::new_msg(#msg);
            bencher.start();
            let result = (|| #fn_block)();
            bencher.stop();
            result
        }
    };

    TokenStream::from(expanded)
}
