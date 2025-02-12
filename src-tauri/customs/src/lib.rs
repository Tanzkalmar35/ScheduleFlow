use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

//#[proc_macro_attribute]
//pub fn log_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
//    let input = parse_macro_input!(item as ItemFn);
//    let fn_name = &input.sig.ident;
//    let fn_vis = &input.vis; // Get the visibility of the function
//    let block = &input.block;
//
//    // Check if the first parameter is actually the PgDriver
//    if let Some(FnArg::Typed(PatType { ty, .. })) = input.sig.inputs.iter().next() {
//        if let Type::Path(type_path) = &**ty {
//            // Error at compiletime if the first arg is not 'PgDriver'
//            if !type_path.path.is_ident("PgDriver") {
//                return TokenStream::from(quote! {
//                    compile_error!("Function's first argument must be of type PgDriver.");
//                });
//            }
//        } else {
//            return TokenStream::from(quote! {
//                compile_error!("Yeah don't even have a clue of what actually happened here...");
//            });
//        }
//    } else {
//        return TokenStream::from(quote! {
//            compile_error!("Function must have at least 1 arg.");
//        });
//    }
//
//    let expanded = quote! {
//        #fn_vis fn #fn_name() {
//            println!("Function {} is called", stringify!(#fn_name));
//            #block
//        }
//    };
//
//    TokenStream::from(expanded)
//}

#[proc_macro_attribute]
pub fn benchmark(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute argument (the custom message)
    let message = parse_macro_input!(attr as LitStr);

    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function's name, inputs, body, and return type
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_vis = &input_fn.vis; // Get the visibility of the function
    let fn_block = &input_fn.block;

    // Extract the return type
    let return_type = match &input_fn.sig.output {
        syn::ReturnType::Default => quote! { () }, // Default return type is `()`
        syn::ReturnType::Type(_, ty) => quote! { #ty }, // Explicit return type
    };

    // Generate the new function with benchmarking logic
    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) -> #return_type {
            use std::time::Instant;
            use log::info;

            let start = Instant::now();
            println!("KSJdalksjdklaj");
            let result = #fn_block;
            let duration = start.elapsed();

            info!("{} {:?}", #message, duration);
            result
        }
    };

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}
