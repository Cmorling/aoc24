use std::fs;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Data, parse_macro_input, Ident, LitStr, parse::Parse, parse::ParseStream, Token};

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

struct InitDayArgs {
    var: LitStr,
    path: LitStr,
    match_stmt: LitStr,
}

impl Parse for InitDayArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
         
        let var: LitStr = input.parse()?;
        let _: Token![,] = input.parse()?;
        let path: LitStr = input.parse()?;
        let _: Token![,] = input.parse()?;
        let match_stmt: LitStr = input.parse()?;
        Ok(InitDayArgs { var, path,  match_stmt})
    }
}
#[proc_macro]

pub fn init_day(input: TokenStream) -> TokenStream {
    let InitDayArgs { var, path, match_stmt, .. } = parse_macro_input!(input as InitDayArgs);

    let paths = fs::read_dir(path.value()).unwrap();

    let imports_s: Vec<String> = paths
        .map(|path| path.unwrap().path())
        .filter_map(|path_result| {
            path_result
                .file_stem()
                .and_then(|stem| Some(stem.to_str().unwrap().to_uppercase()))
        })
        .collect();

    let single_digits: Vec<u32> = imports_s
        .iter()
        .map(|s| {
            s[1..].parse::<u32>().expect("Not a valid digit")
        })
        .collect();

    let ident_imports: Vec<Ident> = imports_s
        .iter()
        .map(|s| Ident::new(s, Span::call_site()))
        .collect();

    let ident_var = Ident::new(&var.value(), Span::call_site());
    let ident_match = Ident::new(&match_stmt.value(), Span::call_site());

    let expanded = quote! {
        match #ident_match {
           #(
                #single_digits => #ident_var = DayEnum::#ident_imports(#ident_imports::new()),
           )*
            _ => #ident_var = DayEnum::new() 
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn init_day_enum_members(input: TokenStream) -> TokenStream {
    let inp = parse_macro_input!(input as LitStr);
    let paths = fs::read_dir(inp.value()).unwrap();

    let imports: Vec<Ident> = paths
        .map(|path| path.unwrap().path())
        .filter_map(|path_result| {
            path_result
                .file_stem()
                .and_then(|stem| Some(stem.to_str().unwrap().to_uppercase()))
                .map(|s| Ident::new(&s, Span::call_site()))
        })
        .collect();
    let variants = imports.iter().map(|ident| {
        quote!(#ident(days::#ident),)
    });

    let expanded = quote! {
        #[macro_export]
        macro_rules! test {
            () => {
                #(#variants)*
            };
        }
    };
    // let expanded = quote! {
    //     pub enum DayEnum {
    //         #(
    //             #imports(days::#imports),
    //         )*
    //     }
    // };

    TokenStream::from(expanded)

}

#[proc_macro_derive(DayEnum)]
pub fn derive_day_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(name, "DayEnum can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let arms_parse_input = variants.iter().map(|variant| {
        let v_name = &variant.ident;
        quote! {
            #name::#v_name(inner) => inner.parse_input(path),
        }
    });

    let arms_solve_part_one = variants.iter().map(|variant| {
        let v_name = &variant.ident;
        quote! {
            #name::#v_name(inner) => inner.solve_part_one(),
        }
    });

    let arms_solve_part_two = variants.iter().map(|variant| {
        let v_name = &variant.ident;
        quote! {
            #name::#v_name(inner) => inner.solve_part_two(),
        }
    });

    let arms_get_solution = variants.iter().map(|variant| {
        let v_name = &variant.ident;
        quote! {
            #name::#v_name(inner) => inner.get_solution(),
        }
    });

    let expanded = quote! {
         impl Default for #name {
            fn default() -> Self {
                #name::D1(Default::default())
            }
        }

        impl self::Day for #name {
            fn parse_input(&mut self, path: &str) -> std::io::Result<()> {
                match self {
                    #(#arms_parse_input)*
                }
            }

            fn solve_part_one(&mut self) -> std::io::Result<()> {
                    match self {
                        #(#arms_solve_part_one)*
                    }
                }

            fn solve_part_two(&mut self) -> std::io::Result<()> {
                match self {
                    #(#arms_solve_part_two)*
                }
            }

            fn get_solution(&self) -> String {
                match self {
                    #(#arms_get_solution)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
