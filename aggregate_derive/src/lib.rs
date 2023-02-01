extern crate proc_macro;

use aggregate_types::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;

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
    let (_, attrs) = attr_map(&ast.attrs);
    let variants = variant_map(&data.variants);

    let expanded = quote! {
        impl #impl_generics aggregate::Aggregate for #ident #ty_generics {
            fn aggregate() -> &'static aggregate::types::Amalgamate {
                use aggregate::types::*;
                use syn::parse_quote;
                use phf::phf_ordered_map;

                static mut AMALGAMATE: Amalgamate = Amalgamate {
                    attrs: #attrs,
                    fields: #variants,
                };

                unsafe { &AMALGAMATE }
            }
        }
    };

    expanded.into()
}

fn impl_aggregate_union(ast: &syn::DeriveInput, data: &syn::DataUnion) -> TokenStream {
    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let ident = &ast.ident;
    let (_, attrs) = attr_map(&ast.attrs);
    let fields = named_field_map(&data.fields);

    let expanded = quote! {
        impl #impl_generics aggregate::Aggregate for #ident #ty_generics {
            fn aggregate() -> &'static aggregate::types::Amalgamate {
                use aggregate::types::*;
                use syn::parse_quote;
                use phf::phf_ordered_map;

                static mut AMALGAMATE: Amalgamate = Amalgamate {
                    attrs: #attrs,
                    fields: #fields,
                };

                unsafe { &AMALGAMATE }
            }
        }
    };

    expanded.into()
}

fn impl_aggregate_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let ident = &ast.ident;
    let (_, attrs) = attr_map(&ast.attrs);
    let fields = field_map(&data.fields);

    let expanded = quote! {
        impl #impl_generics aggregate::Aggregate for #ident #ty_generics {
            fn aggregate() -> &'static aggregate::types::Amalgamate {
                use aggregate::types::*;
                use syn::parse_quote;
                use phf::phf_ordered_map;

                static mut AMALGAMATE: Amalgamate = Amalgamate {
                    attrs: #attrs,
                    fields: #fields,
                };

                unsafe { &AMALGAMATE }
            }
        }
    };

    expanded.into()
}

/*
 *
 */

fn variant_map(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> quote::__private::TokenStream {
    let mut streams = Vec::new();

    for variant in variants {
        let ident = variant.ident.to_string();
        let (_, attrs) = attr_map(&variant.attrs);

        let extended = if variant.fields.is_empty() {
            quote! {
                #ident => {
                    Field {
                        attrs: #attrs,
                        inner: None,
                    }
                }
            }
        } else {
            let fields = field_map(&variant.fields);
            let inner = quote! {
                {
                    static mut AMALGAMATE: Amalgamate = Amalgamate {
                        attrs: phf_ordered_map! {},
                        fields: #fields,
                    };
                    static mut INNER: LazyAmalgamate =
                        LazyAmalgamate::new(|| unsafe { &AMALGAMATE });
                    unsafe { &INNER }
                }
            };

            quote! {
                #ident => {
                    Field {
                        attrs: #attrs,
                        inner: Some(#inner),
                    }
                }
            }
        };

        streams.push(extended);
    }

    quote! {
        phf_ordered_map! {
            #(#streams),*
        }
    }
}

fn named_field_map(fields: &syn::FieldsNamed) -> quote::__private::TokenStream {
    field_map(&syn::Fields::Named(fields.clone()))
}

fn field_map(fields: &syn::Fields) -> quote::__private::TokenStream {
    let mut streams = Vec::new();

    for (i, field) in fields.iter().enumerate() {
        let ident = match &field.ident {
            Some(ident) => ident.to_string(),
            None => i.to_string(),
        };

        let (aggregate, attrs) = attr_map(&field.attrs);

        let extended = if aggregate {
            let ty = &field.ty;
            let inner = quote! {
                {
                    static mut INNER: LazyAmalgamate =
                        LazyAmalgamate::new(|| <#ty as aggregate::Aggregate>::aggregate());
                    unsafe { &INNER }
                }
            };

            quote! {
                #ident => {
                    Field {
                        attrs: #attrs,
                        inner: Some(#inner),
                    }
                }
            }
        } else {
            quote! {
                #ident => {
                    Field {
                        attrs: #attrs,
                        inner: None,
                    }
                }
            }
        };

        streams.push(extended);
    }

    quote! {
        phf_ordered_map! {
            #(#streams),*
        }
    }
}

fn attr_map(attrs: &[Attribute]) -> (bool, quote::__private::TokenStream) {
    let mut aggregate = false;

    let mut idents = Vec::new();
    let mut attributes = Vec::new();

    for (i, attr) in attrs.iter().enumerate() {
        if let Ok(syn::Meta::Path(path)) = attr.parse_meta() {
            if path.is_ident("aggregate") {
                aggregate = true;
                continue;
            }
        }
        idents.push(i as u16);
        attributes.push(attr);
    }

    let expanded = quote! {
        phf_ordered_map! {
            #(#idents => {
                static mut ATTR: LazyAttribute = LazyAttribute::new(|| parse_quote!(#attributes));
                unsafe { &ATTR }
            }),*
        }
    };

    (aggregate, expanded)
}
