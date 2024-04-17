use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{Attribute, Fields, Ident, Meta, MetaList, QSelf, Token, Variant};

enum Parameter {
    Normal(syn::FnArg),
    Unit{
        name: Ident,
        family_ty: syn::Type,
        variant_ty: syn::Type,
    },
}

fn get_parameters(item_fn: &syn::ItemFn) -> Vec<Parameter> {
    let mut parameters = Vec::new();

    for input in &item_fn.sig.inputs {
        match input {
            syn::FnArg::Typed(pat_type) => {
                parameters.push(Parameter::Normal(input.clone()));
            }
            syn::FnArg::Receiver(_) => {
                parameters.push(Parameter::Normal(input.clone()));
            }
        }
    }

    parameters
}

#[proc_macro_attribute]
pub fn unit_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // validate that the item is a function
    let raw_item: syn::Item = syn::parse(item).expect("Not a supported item");

    let item_fn = match raw_item {
        syn::Item::Fn(item_fn) => item_fn,
        _ => panic!("Not a function"),
    };

    item_fn.into_token_stream().into()
}

