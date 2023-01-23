extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;

#[proc_macro_derive(Aggregate, attributes(aggregate))]
pub fn derive_aggregate(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_aggregate(&ast)
}

fn impl_aggregate(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let attrs = &ast.attrs;

    let mut field_idents = Vec::new();
    let mut field_attrs = Vec::new();

    let mut fields_to_aggregate = Vec::new();
    let mut types_to_aggregate = Vec::new();

    match &ast.data {
        syn::Data::Enum(inner) => {
            for variant in &inner.variants {
                let ident = variant.ident.to_string();

                let attrs = {
                    let attrs = {
                        let mut vec = Vec::new();
                        for attr in &variant.attrs {
                            vec.push(attr.to_token_stream());
                        }
                        vec
                    };

                    quote! {
                        {
                            let mut vec = Vec::<aggregate::syn::Attribute>::new();
                            #(vec.push(aggregate::syn::parse_quote!(#attrs));)*
                            vec
                        }
                    }
                };

                field_idents.push(ident);
                field_attrs.push(attrs);
            }
        }

        syn::Data::Struct(inner) => {
            for field in &inner.fields {
                let ident = field
                    .ident
                    .clone()
                    .expect("Unnamed structs are not supported")
                    .to_string();

                let attrs = {
                    let attrs = {
                        let mut vec = Vec::new();
                        for attr in &field.attrs {
                            vec.push(attr.to_token_stream());
                        }
                        vec
                    };

                    quote! {
                        {
                            let mut vec = Vec::<aggregate::syn::Attribute>::new();
                            #(vec.push(aggregate::syn::parse_quote!(#attrs));)*
                            vec
                        }
                    }
                };

                for attr in &field.attrs {
                    if let Ok(meta) = attr.parse_meta() {
                        if let syn::Meta::Path(path) = meta {
                            if path.is_ident("aggregate") {
                                fields_to_aggregate.push(ident.clone());
                                types_to_aggregate.push(&field.ty);
                                break;
                            }
                        }
                    }
                }

                field_idents.push(ident);
                field_attrs.push(attrs);
            }
        }

        syn::Data::Union(inner) => {
            for field in &inner.fields.named {
                let ident = field.ident.clone().unwrap().to_string();

                let attrs = {
                    let attrs = {
                        let mut vec = Vec::new();
                        for attr in &field.attrs {
                            vec.push(attr.to_token_stream());
                        }
                        vec
                    };

                    quote! {
                        {
                            let mut vec = Vec::<aggregate::syn::Attribute>::new();
                            #(vec.push(aggregate::syn::parse_quote!(#attrs));)*
                            vec
                        }
                    }
                };

                field_idents.push(ident);
                field_attrs.push(attrs);
            }
        }
    }

    let expanded = quote! {
        impl aggregate::Aggregate for #ident {
            fn aggregate() -> aggregate::types::Struct {
                let attrs = {
                    let mut vec = Vec::new();
                    #(
                        let attr: aggregate::syn::Attribute = aggregate::syn::parse_quote!(#attrs);
                        vec.push(attr);
                    )*
                    vec
                };

                let fields = {
                    let mut map = {
                        let mut map = aggregate::types::Fields::new();

                        let idents = {
                            let mut vec = Vec::<String>::new();
                            #(
                                vec.push(#field_idents.to_string());
                            )*
                            vec
                        };

                        let attrs = {
                            let mut vec = Vec::<Vec<aggregate::syn::Attribute>>::new();
                            #(
                                vec.push(#field_attrs);
                            )*
                            vec
                        };

                        for (ident, attrs) in idents.into_iter().zip(attrs) {
                            map.insert(ident, aggregate::types::Field {attrs, inner: None});
                        }

                        map
                    };

                    #({
                        let field = map.get_mut(#fields_to_aggregate).expect("Field has no inner to aggrgeate");
                        field.inner = Some(<#types_to_aggregate as aggregate::Aggregate>::aggregate());
                    })*

                    map
                };

                aggregate::types::Struct { attrs, fields }
            }
        }
    };

    expanded.into()
}
