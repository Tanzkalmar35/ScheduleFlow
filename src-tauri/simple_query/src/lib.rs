use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType, Type};

#[proc_macro_attribute]
pub fn log_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis; // Get the visibility of the function
    let block = &input.block;

    // Check if the first parameter is actually the PgDriver
    let tks = check_function_params(&input);

    let _expanded = quote! {
        #fn_vis fn #fn_name() {
            println!("Function {} is called", stringify!(#fn_name));
            #block
        }
    };

    TokenStream::from(tks)
}

fn check_function_params(input: &ItemFn) -> TokenStream {
    if let Some(FnArg::Typed(PatType { ty, .. })) = input.sig.inputs.iter().next() {
        if let Type::Path(type_path) = &**ty {
            // Error at compiletime if the first arg is not 'PgDriver'
            if !type_path.path.is_ident("PgDriver") {
                return TokenStream::from(quote! {
                    compile_error!("Function's first argument must be of type PgDriver.");
                });
            }
        } else {
            return TokenStream::from(quote! {
                compile_error!("Yeah don't even have a clue of what actually happened here...");
            });
        }
    } else {
        return TokenStream::from(quote! {
            compile_error!("Function must have at least 1 arg.");
        });
    }

    TokenStream::new()
}

