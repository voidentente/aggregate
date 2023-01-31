extern crate proc_macro;

use aggregate_types::*;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Aggregate, attributes(aggregate))]
pub fn derive_aggregate(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_aggregate(&ast)
}

fn impl_aggregate(ast: &syn::DeriveInput) -> TokenStream {
    match &ast.data {
        syn::Data::Enum(data) => impl_aggregate_enum(ast, data),
        syn::Data::Union(data) => impl_aggregate_union(ast, data),
        syn::Data::Struct(data) => impl_aggregate_struct(ast, data),
    }
}

/*
 *
 */

fn impl_aggregate_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let ident = &ast.ident;
    let attrs = Attributes(ast.attrs.to_owned());
    let (fields, descendants) = parse_variants(&data.variants);
    let amalgamate = Amalgamate { attrs, fields };

    let expanded = quote! {
        impl #impl_generics aggregate::Aggregate for #ident #ty_generics {
            fn aggregate() -> aggregate::types::Amalgamate {
                let mut amalgamate = #amalgamate;
                #(#descendants)*
                amalgamate
            }
        }
    };

    expanded.into()
}

fn impl_aggregate_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let ident = &ast.ident;
    let attrs = Attributes(ast.attrs.to_owned());
    let (fields, descendants) = parse_fields(&data.fields);
    let amalgamate = Amalgamate { attrs, fields };

    let expanded = quote! {
        impl #impl_generics aggregate::Aggregate for #ident #ty_generics {
            fn aggregate() -> aggregate::types::Amalgamate {
                let mut amalgamate = #amalgamate;
                #descendants
                amalgamate
            }
        }
    };

    expanded.into()
}

fn impl_aggregate_union(ast: &syn::DeriveInput, data: &syn::DataUnion) -> TokenStream {
    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let ident = &ast.ident;
    let attrs = Attributes(ast.attrs.to_owned());
    let (fields, descendants) = parse_fields_named(&data.fields);
    let amalgamate = Amalgamate { attrs, fields };

    let expanded = quote! {
        impl #impl_generics aggregate::Aggregate for #ident #ty_generics {
            fn aggregate() -> aggregate::types::Amalgamate {
                let mut amalgamate = #amalgamate;
                #descendants
                amalgamate
            }
        }
    };

    expanded.into()
}

/*
 *
 */

fn parse_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> (Fields, Vec<Descendants>) {
    let mut variant_map = FieldMap::new();
    let mut descendant_vec = Vec::new();

    for variant in variants {
        let ident = variant.ident.to_string();
        let attrs = Attributes(variant.attrs.to_owned());

        let inner = if variant.fields.is_empty() {
            None
        } else {
            let (fields, mut descendants) = parse_fields(&variant.fields);
            descendants.variation = Some(ident.to_owned());
            descendant_vec.push(descendants);

            Some(Amalgamate {
                attrs: Attributes(Vec::new()),
                fields,
            })
        };

        variant_map.insert(ident, Field { attrs, inner });
    }

    (Fields(variant_map), descendant_vec)
}

fn parse_fields_named(fields: &syn::FieldsNamed) -> (Fields, Descendants) {
    parse_fields(&syn::Fields::Named(fields.to_owned()))
}

fn parse_fields(fields: &syn::Fields) -> (Fields, Descendants) {
    let mut field_map = FieldMap::new();
    let mut descendant_map = DescendantMap::new();

    for (i, field) in fields.iter().enumerate() {
        let ident = match &field.ident {
            Some(ident) => ident.to_string(),
            None => i.to_string(),
        };

        let mut attrs = Vec::new();

        for attr in &field.attrs {
            if let Ok(syn::Meta::Path(path)) = attr.parse_meta() {
                if path.is_ident("aggregate") {
                    descendant_map.insert(ident.to_owned(), field.ty.to_owned());
                    continue;
                }
            }

            attrs.push(attr.to_owned());
        }

        field_map.insert(
            ident,
            Field {
                attrs: Attributes(attrs),
                inner: None,
            },
        );
    }

    (
        Fields(field_map),
        Descendants {
            map: descendant_map,
            variation: None,
        },
    )
}
