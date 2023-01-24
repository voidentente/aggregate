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
    match &ast.data {
        syn::Data::Enum(inner) => impl_aggregate_enum(&ast, inner),
        syn::Data::Union(inner) => impl_aggregate_union(&ast, inner),
        syn::Data::Struct(inner) => impl_aggregate_struct(&ast, inner),
    }
}

#[allow(unused_variables)]
fn impl_aggregate_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
    unimplemented!("enum")
}

#[allow(unused_variables)]
fn impl_aggregate_union(ast: &syn::DeriveInput, data: &syn::DataUnion) -> TokenStream {
    unimplemented!("union")
}

#[allow(unused_variables)]
fn impl_aggregate_struct(ast: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let struct_ident = &ast.ident;
    let struct_attrs = &ast.attrs;

    let mut fields_ident = Vec::new();
    let mut fields_attrs = Vec::new();

    let mut fields_to_aggregate = Vec::new();
    let mut types_to_aggregate = Vec::new();

    for field in &data.fields {
        let ident = field
            .ident
            .clone()
            .expect("Unnamed structs cannot be aggregated")
            .to_string();

        let attrs = {
            let mut vec = Vec::new();

            for attr in &field.attrs {
                if let Ok(meta) = attr.parse_meta() {
                    if let syn::Meta::Path(path) = meta {
                        if path.is_ident("aggregate") {
                            fields_to_aggregate.push(ident.clone());
                            types_to_aggregate.push(&field.ty);
                            continue;
                        }
                    }
                }

                vec.push(attr.to_token_stream());
            }

            quote! {{
                let mut vec = Vec::<aggregate::syn::Attribute>::new();
                #(vec.push(aggregate::syn::parse_quote!(#vec));)*
                vec
            }}
        };

        fields_ident.push(ident);
        fields_attrs.push(attrs);
    }

    quote! {

    impl aggregate::Aggregate for #struct_ident {
        fn aggregate() -> aggregate::types::Struct {
            let attrs = {
                let mut vec = Vec::new();
                #(
                    let attr: aggregate::syn::Attribute = aggregate::syn::parse_quote!(#struct_attrs);
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
                            vec.push(#fields_ident.to_string());
                        )*
                        vec
                    };

                    let attrs = {
                        let mut vec = Vec::<Vec<aggregate::syn::Attribute>>::new();
                        #(
                            vec.push(#fields_attrs);
                        )*
                        vec
                    };

                    for (ident, attrs) in idents.into_iter().zip(attrs) {
                        map.insert(ident, aggregate::types::Field {attrs, inner: None});
                    }

                    map
                };

                #({
                    let field = map.get_mut(#fields_to_aggregate).expect("Field has no inner to aggregate");
                    field.inner = Some(<#types_to_aggregate as aggregate::Aggregate>::aggregate());
                })*

                map
            };

            aggregate::types::Struct { attrs, fields }
        }
    }

    }.into()
}
