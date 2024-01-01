
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{QSelf, Token};

/// Derive macro generating an impl of the trait `FrcStructure`.
#[proc_macro_derive(FrcStructure)]
pub fn frc_structure(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_frc_struct(&ast).into()
}

/// rerurns `<typ as FrcStructure>`
fn type_as_frcstructure(typ: &syn::Type) -> syn::Type {
    let path = syn::parse_str::<syn::Path>("FrcStructure").expect("Failed to parse FrcStructure path");
    let typ = typ.clone();
    syn::Type::Path(syn::TypePath {
        qself: Some(QSelf {
            lt_token: Token![<](syn::spanned::Spanned::span(&typ)),
            position: 1,
            as_token: Some(Token![as](syn::spanned::Spanned::span(&typ))),
            gt_token: Token![>](syn::spanned::Spanned::span(&typ)),
            ty: Box::new(typ),
        }),
        path,
    })
}


fn impl_frc_struct(ast: &syn::DeriveInput) -> TokenStream2 {
    // every supported field type implements `FrcStructure`
    // so we can use it to generate the schema, size, pack, and unpack functions
    let name = &ast.ident;
    let mut field_types: Vec<syn::Type> = Vec::new();
    let mut field_strs: Vec<syn::LitStr> = Vec::new();
    let mut field_names: Vec<syn::Ident> = Vec::new();

    //filter out non structs
    let syn::Data::Struct(syn::DataStruct {
        fields,
        ..
    }) = &ast.data else {
        panic!("Only structs are supported");
    };

    match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            for field in named {
                let field_name = field.ident.as_ref().expect("Only named fields are supported");
                let field_type = &field.ty;
                field_names.push(field_name.clone());
                field_strs.push(syn::LitStr::new(field_name.to_string().as_str(), field_name.span()));
                field_types.push(field_type.clone());
            }
        }
        // syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
        //     for (i, field) in unnamed.iter().enumerate() {
        //         let field_name = syn::LitInt::new(&i.to_string(), syn::spanned::Spanned::span(&field));
        //         let field_type = &field.ty;
        //         field_names.push(syn::Ident::new(&i.to_string(), syn::spanned::Spanned::span(&field)));
        //         field_strs.push(syn::LitStr::new(field_name.to_string().as_str(), field_name.span()));
        //         field_types.push(field_type.clone());
        //     }
        // }
        _ => panic!("Unit structs are not supported")
    };
    field_types = field_types.iter().map(type_as_frcstructure).collect();

    //generate schema
    let schema = {
        let schema_template = "{}".repeat(field_types.len());
        let format_exprs = {
            field_types.iter().zip(field_strs.iter())
                .map(|(typ, name)| {
                    format!("{}::format_field({})", typ.into_token_stream(), name.into_token_stream())
                }).collect::<Vec<_>>()
        };
        syn::parse_str::<syn::Expr>(
            &format!("format!(\"{}\", {})", schema_template, format_exprs.join(", "))
        ).expect("Failed to parse format! call")
    };

    //generate size
    let size = {
        let mut size = syn::parse_str::<syn::Expr>("0usize").expect("Failed to parse 0usize");
        for typ in field_types.iter() {
            let size_expr = syn::parse_str::<syn::Expr>(format!("{}::SIZE", typ.into_token_stream()).as_str())
                .expect("Failed to parse size");
            size = syn::parse_str::<syn::Expr>(format!("{} + {}", size.into_token_stream(), size_expr.into_token_stream()).as_str())
                .expect("Failed to parse size addition");
        }
        size
    };

    //generate pack
    let pack = {
        let mut pack = "{".to_string();
        for (typ, name) in field_types.iter().zip(field_names.iter()) {
            pack = format!("{}{}::pack(&self.{}, buffer);", pack, typ.into_token_stream(), name.into_token_stream());
        }
        pack = format!("{}}}", pack);
        syn::parse_str::<syn::Stmt>(pack.as_str()).expect("Failed to parse pack statement")
    };

    //generate unpack
    let unpack = {
        let mut unpack = "Self {".to_string();
        for (typ, name) in field_types.iter().zip(field_names.iter()) {
            unpack = format!("{}{}: {}::unpack(buffer),", unpack, name.into_token_stream(), typ.into_token_stream());
        }
        unpack = format!("{}}}", unpack);
        syn::parse_str::<syn::ExprStruct>(unpack.as_str()).expect("Failed to parse unpack expression")
    };

    quote! {
        impl FrcStructure for #name {
            const SIZE: usize = #size;
            const TYPE: &'static str = stringify!(#name);
            const SCHEMA_SUPPLIER: fn() -> String = || #schema;

            fn pack(&self, buffer: &mut impl frclib_core::structure::bytes::BufMut) {
                #pack
            }

            fn unpack(buffer: &mut impl frclib_core::structure::bytes::Buf) -> Self {
                #unpack
            }
        }
        frclib_core::structure::inventory::submit! { <#name as FrcStructure>::DESCRIPTION }
        ///This isnt a generic impl for every struct because of primitive and unit types
        impl Into<frclib_core::value::FrcValue> for #name {
            fn into(self) -> frclib_core::value::FrcValue {
                let mut buffer = frclib_core::structure::bytes::BytesMut::with_capacity(Self::SIZE);
                self.pack(&mut buffer);
                frclib_core::value::FrcValue::Struct(
                    &Self::DESCRIPTION,
                    Box::new(buffer.freeze())
                )
            }
        }
    }
}
