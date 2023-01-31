//! Quote compatibility module for aggregate types.

use crate::*;

use quote::quote;
use quote::ToTokens;
use quote::__private::TokenStream;

impl ToTokens for Fields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keys = self.0.keys();
        let values = self.0.values();

        tokens.extend(quote! {{
            let map = aggregate::types::FieldMap::from([
                #((#keys.into(), #values)),*
            ]);
            aggregate::types::Fields(map)
        }});
    }
}

impl ToTokens for Descendants {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keys = self.map.keys();
        let values = self.map.values();

        match &self.variation {
            Some(variation) => {
                let variation = variation;
                tokens.extend(quote! {{
                    #(
                        let variation = amalgamate.fields.0.get_mut(#variation).unwrap();
                        let field = variation.inner.as_mut().unwrap().fields.0.get_mut(#keys).unwrap();
                        field.inner = Some(<#values as aggregate::Aggregate>::aggregate());
                    )*
                }});
            }
            None => {
                tokens.extend(quote! {{
                    #(
                        let field = amalgamate.fields.0.get_mut(#keys).unwrap();
                        field.inner = Some(<#values as aggregate::Aggregate>::aggregate());
                    )*
                }});
            }
        }
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.0;

        tokens.extend(quote! {
            aggregate::types::Attributes(Vec::from([
                #(aggregate::syn::parse_quote!(#attrs)),*
            ]))
        });
    }
}

impl ToTokens for Amalgamate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let fields = &self.fields;

        tokens.extend(quote! {
            aggregate::types::Amalgamate {
                attrs: #attrs,
                fields: #fields
            }
        });
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;

        match &self.inner {
            Some(inner) => {
                tokens.extend(quote! {
                    aggregate::types::Field {
                        attrs: #attrs,
                        inner: Some(#inner),
                    }
                });
            }

            None => {
                tokens.extend(quote! {
                    aggregate::types::Field {
                        attrs: #attrs,
                        inner: None,
                    }
                });
            }
        }
    }
}
