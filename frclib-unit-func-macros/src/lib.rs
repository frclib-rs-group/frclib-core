use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{QSelf, Token, Fields, Ident, Variant, Attribute, MetaList, Meta};

#[proc_macro_attribute]
pub fn unit_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // validate that the item is a function
    let raw_item: syn::Item = syn::parse(item)
        .expect("Not a supported item");

    let item_fn = match raw_item {
        syn::Item::Fn(item_fn) => item_fn,
        _ => panic!("Not a function"),
    };

    

    item_fn.into_token_stream().into()
}