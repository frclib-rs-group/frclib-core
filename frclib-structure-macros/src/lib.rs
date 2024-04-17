use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{Attribute, DeriveInput, Fields, Ident, Meta, MetaList, QSelf, Token, Variant};

/// Derive macro generating an impl of the trait `FrcStructure`.
#[proc_macro_derive(FrcStructure)]
pub fn frc_structure(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let attr_tokens = get_frcstructre_attr(&ast.attrs);

    match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => impl_frc_struct(name, fields).into(),
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let repr = ast
                .attrs
                .iter()
                .find_map(|attr| {
                    if attr.path().is_ident("repr") {
                        attr.parse_args::<Ident>().ok()
                    } else {
                        None
                    }
                })
                .expect("Failed to find repr attribute");
            let allow_fields = attr_tokens.into_iter().any(|token| {
                if let TokenTree::Ident(ident) = token {
                    ident == "allow_fields"
                } else {
                    false
                }
            });
            let variants = variants.iter().cloned().collect::<Vec<_>>();
            impl_frc_enum(name, variants, repr, allow_fields).into()
        }
        _ => panic!("Only known size structs and c-style enums are supported"),
    }
}

fn get_frcstructre_attr(attr: &[Attribute]) -> TokenStream2 {
    let frc_attrs: Vec<MetaList> = attr
        .iter()
        .filter_map(|attr| {
            if let Meta::List(maybe_frc_attr) = &attr.meta {
                if maybe_frc_attr.path.is_ident("FrcStructure") {
                    Some(maybe_frc_attr.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let mut out = TokenStream2::new();
    for frc_attr in frc_attrs {
        out.extend(frc_attr.tokens)
    }
    out
}

/// rerurns `<typ as FrcStructure>`
fn type_as_frcstructure(typ: &syn::Type) -> syn::Type {
    let path =
        syn::parse_str::<syn::Path>("FrcStructure").expect("Failed to parse FrcStructure path");
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

fn impl_frc_struct(name: &Ident, fields: &Fields) -> TokenStream2 {
    // every supported field type implements `FrcStructure`
    // so we can use it to generate the schema, size, pack, and unpack functions
    let mut field_types: Vec<syn::Type> = Vec::new();
    let mut field_strs: Vec<syn::LitStr> = Vec::new();
    let mut field_names: Vec<syn::Ident> = Vec::new();

    match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            for field in named {
                let field_name = field
                    .ident
                    .as_ref()
                    .expect("Only named fields are supported");
                let field_type = &field.ty;
                field_names.push(field_name.clone());
                field_strs.push(syn::LitStr::new(
                    field_name.to_string().as_str(),
                    field_name.span(),
                ));
                field_types.push(field_type.clone());
            }
        }
        _ => panic!("Unit structs are not supported"),
    };
    field_types = field_types.iter().map(type_as_frcstructure).collect();

    //generate schema
    let schema = {
        let schema_template = "{}".repeat(field_types.len());
        let format_exprs = {
            field_types
                .iter()
                .zip(field_strs.iter())
                .map(|(typ, name)| {
                    format!(
                        "{}::format_field({})",
                        typ.into_token_stream(),
                        name.into_token_stream()
                    )
                })
                .collect::<Vec<_>>()
        };
        syn::parse_str::<syn::Expr>(&format!(
            "format!(\"{}\", {})",
            schema_template,
            format_exprs.join(", ")
        ))
        .expect("Failed to parse format! call")
    };

    //generate size
    let size = {
        let mut size = syn::parse_str::<syn::Expr>("0usize").expect("Failed to parse 0usize");
        for typ in field_types.iter() {
            let size_expr =
                syn::parse_str::<syn::Expr>(format!("{}::SIZE", typ.into_token_stream()).as_str())
                    .expect("Failed to parse size");
            size = syn::parse_str::<syn::Expr>(
                format!(
                    "{} + {}",
                    size.into_token_stream(),
                    size_expr.into_token_stream()
                )
                .as_str(),
            )
            .expect("Failed to parse size addition");
        }
        size
    };

    //generate pack
    let pack = {
        let mut pack = "{".to_string();
        for (typ, name) in field_types.iter().zip(field_names.iter()) {
            pack = format!(
                "{}{}::pack(&self.{}, buffer);",
                pack,
                typ.into_token_stream(),
                name.into_token_stream()
            );
        }
        pack = format!("{}}}", pack);
        syn::parse_str::<syn::Stmt>(pack.as_str()).expect("Failed to parse pack statement")
    };

    //generate unpack
    let unpack = {
        let mut unpack = "Self {".to_string();
        for (typ, name) in field_types.iter().zip(field_names.iter()) {
            unpack = format!(
                "{}{}: {}::unpack(buffer),",
                unpack,
                name.into_token_stream(),
                typ.into_token_stream()
            );
        }
        unpack = format!("{}}}", unpack);
        syn::parse_str::<syn::ExprStruct>(unpack.as_str())
            .expect("Failed to parse unpack expression")
    };

    quote! {
        impl FrcStructure for #name {
            const SIZE: usize = #size;
            const TYPE: &'static str = stringify!(#name);
            const SCHEMA_SUPPLIER: fn() -> String = || #schema;

            fn pack(&self, buffer: &mut Vec<u8>) {
                #pack
            }

            fn unpack(buffer: &mut Cursor<&[u8]>) -> Self {
                #unpack
            }
        }
        frclib_core::structure::inventory::submit! { <#name as FrcStructure>::DESCRIPTION }
        ///This isnt a generic impl for every struct because of primitive and unit types
        impl Into<frclib_core::value::FrcValue> for #name {
            fn into(self) -> frclib_core::value::FrcValue {
                let mut buffer = Vec::with_capacity(Self::SIZE);
                self.pack(&mut buffer);
                frclib_core::value::FrcValue::Struct(
                    Box::new(
                        FrcStructureBytes::from_parts(
                            &Self::DESCRIPTION,
                            1,
                            buffer.into_boxed_slice()
                        )
                    )
                )
            }
        }
    }
}

fn impl_frc_enum(
    name: &Ident,
    variants: Vec<Variant>,
    repr: Ident,
    allow_fields: bool,
) -> TokenStream2 {
    let mut enum_variants: Vec<(syn::Ident, syn::LitInt)> = Vec::new();
    let mut last_value = 0;
    for variant in variants {
        let variant_name = variant.ident;
        //if the variant has fields print a warning
        if let syn::Fields::Named(_) | syn::Fields::Unnamed(_) = variant.fields {
            if !allow_fields {
                panic!(
                    "
                    Warning: Enum variant {} has fields, when serializing the fields will be ignored \n
                    If you would like to allow this, add `#[FrcStructure(allow_fields)]` to the enum definition
                    ",
                    variant_name
                );
            }
        }
        if let Some(variant_value) = variant.discriminant {
            let variant_value = match variant_value.1 {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) => lit_int,
                _ => panic!("Enum variants must have integer values"),
            };
            last_value = variant_value.base10_parse::<i64>().unwrap();
            enum_variants.push((variant_name, variant_value));
        } else {
            let span = variant_name.span();
            enum_variants.push((
                variant_name,
                syn::LitInt::new(last_value.to_string().as_str(), span),
            ));
            last_value += 1;
        }
    }

    let schema = {
        let enum_decl = {
            let mut ss = String::new();
            for enm in enum_variants.clone() {
                ss.push_str(&format!("{}={}, ", enm.0, enm.1))
            }
            ss.truncate(ss.len() - 2);
            ss
        };
        syn::parse_str::<syn::Expr>(&format!(
            "format!(\"enum {{{{{}}}}} {{}} variant\", {}::TYPE)",
            enum_decl, repr
        ))
        .expect("Failed to parse format! call")
    };

    let from_repr = {
        let impl_str = {
            let mut match_ = String::new();
            for enm in enum_variants {
                match_.push_str(&format!("{} => Some({}::{}), ", enm.1, name, enm.0))
            }
            match_.push_str("_ => None");
            match_ = format!("match repr {{ {} }}", match_);
            format!(
                "impl {} {{ fn from_repr(repr: {}) -> Option<{}> {{ {} }} }}",
                name, repr, name, match_
            )
        };
        syn::parse_str::<syn::Item>(impl_str.as_str()).expect("Failed to parse impl")
    };

    quote! {
        #from_repr
        impl FrcStructure for #name {
            const SIZE: usize = <#repr as FrcStructure>::SIZE;
            const TYPE: &'static str = stringify!(#name);
            const SCHEMA_SUPPLIER: fn() -> String = || #schema;

            fn pack(&self, buffer: &mut Vec<u8>) {
                let repr = *self as #repr;
                <#repr as FrcStructure>::pack(&repr, buffer);
            }

            fn unpack(buffer: &mut Cursor<&[u8]>) -> Self {
                let repr = <#repr as FrcStructure>::unpack(buffer);
                Self::from_repr(repr).unwrap_or_default()
            }
        }
        frclib_core::structure::inventory::submit! { <#name as FrcStructure>::DESCRIPTION }
        ///This isnt a generic impl for every struct because of primitive and unit types
        impl Into<frclib_core::value::FrcValue> for #name {
            fn into(self) -> frclib_core::value::FrcValue {
                let mut buffer = Vec::with_capacity(Self::SIZE);
                self.pack(&mut buffer);
                frclib_core::value::FrcValue::Struct(
                    Box::new(
                        FrcStructureBytes::from_parts(
                            &Self::DESCRIPTION,
                            1,
                            buffer.into_boxed_slice()
                        )
                    )
                )
            }
        }
    }
}
