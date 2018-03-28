//!
#![feature(proc_macro, proc_macro_lib)]
#![allow(unused_imports, unused_variables)]

extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use std::collections::HashSet as Set;
use syn::LitStr;
use syn::fold::{self, Fold};
use syn::punctuated::Punctuated;
use syn::synom::Synom;
use syn::{Expr, Ident, ImplItem, ImplItemMethod, Item, ItemImpl, ItemStatic, Pat, Stmt};

///
#[proc_macro_attribute]
pub fn thunderclap(_args: TokenStream, input: TokenStream) -> TokenStream {
    let i: ItemImpl = match syn::parse(input.clone()) {
        Ok(input) => input,
        Err(e) => panic!("Error: '{}'", e),
    };

    let orignal = quote!(#i);

    let mut app = quote! {
        App::new("Whatever")
    };

    for item in &i.items {
        match item {
            &ImplItem::Method(ref i) => {
                let name = LitStr::new(&i.sig.ident.to_string(), i.sig.ident.span);
                let args = i.sig.decl.inputs.iter().fold(quote!{}, |acc, arg| {
                    quote! { #acc.arg(Arg::with_name("foo")) }
                });

                app = quote! {
                    #app.subcommand(
                        SubCommand::with_name(#name)#args
                    )
                }
            }
            _ => {}
        }
    }

    let tokens = quote! {
        #orignal

        /// This block was generated by thunder v0.0.0
        impl MyApp {

            /// Starts the CLI parsing and calls whichever function handles the input
            fn start() {
                // let generated = quote! {
                //     #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                //     const #dummy_const: () = {
                //         extern crate serde as _serde;
                //         #impl_block
                //     };
                // };

                use clap::{App, SubCommand};

                let mut app = #app;
                let _args = app.print_help();
            }
        }
    };

    // for item in &i.items {
    //     match item {
    //         &ImplItem::Method(ref i) => {
    //             let name = quote!(#i.sig.ident);
    //             let foo = quote! {

    //             }

    //             // println!("{:#?}", i.sig.ident)
    //         },
    //         _ => continue,
    //     }
    // }

    // Parse the list of variables the user wanted to print.
    // let mut args: ItemStatic = syn::parse(args).unwrap();

    // Hand the resulting function body back to the compiler.
    // quote!(args).into()

    // println!("{:#?}", tokens);

    tokens.into()
}