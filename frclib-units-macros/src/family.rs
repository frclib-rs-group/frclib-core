use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn family(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    //expect (ident colon ident...)
    let mut iter = input.into_iter();
    let family_name = iter.next().unwrap();
    match iter.next().unwrap() {
        proc_macro2::TokenTree::Punct(punct) => {
            assert_eq!(punct.as_char(), ':', "expected colon");
        },
        _ => panic!("expected colon")
    };
    let mut units = Vec::new();
    let mut func_names = Vec::new();
    for unit in iter {
        let unit = match unit {
            proc_macro2::TokenTree::Ident(ident) => ident,
            _ => panic!("expected ident"),
        };
        func_names.push(proc_macro2::Ident::new(("to_".to_owned() +&unit.to_string().to_lowercase()).as_str(), unit.span()));
        units.push(unit);
    }

    let doc = format!("Trait for converting between units in the {} family.", family_name);
    let trait_block = quote! {
        #[doc = #doc]
        pub trait #family_name: Sized + Copy + Clone + PartialEq + PartialOrd + std::fmt::Debug +
            std::ops::Add<Self> + std::ops::Sub<Self> + std::ops::Mul<Self> + std::ops::Div<Self> + std::ops::Neg<Output = Self> {
            #(fn #func_names(self) -> #units;)*
        }
    };
    output.extend(trait_block);

    for unit in units.iter() {
        let unit_block = quote! {
            impl #family_name for #unit {
                #(fn #func_names(self) -> #units {
                    #units::from(self)
                })*
            }
        };
        output.extend(unit_block);
    }

    output
}