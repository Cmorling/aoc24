use std::fs;

use proc_macro::{TokenStream};
use proc_macro2::{Span};
use quote::quote;
use syn::{parse_macro_input, LitStr, Ident};

#[proc_macro]
pub fn pub_use_solutions(input: TokenStream) -> TokenStream {
    let inp = parse_macro_input!(input as LitStr);
    let paths = fs::read_dir(inp.value()).unwrap();

    let imports: Vec<Ident> = paths
        .map(|path| path.unwrap().path())
        .filter_map(|path_result| {
            path_result
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| Ident::new(s, Span::call_site()))
        })
        .collect();
    let expanded = quote! {
        #(
            pub use self::#imports::*;
        )*
    };

    TokenStream::from(expanded)
}
