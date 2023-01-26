//! Quote compatibility module for aggregate types.

use crate::*;

use quote::quote;
use quote::ToTokens;
use quote::__private::TokenStream;

impl ToTokens for Fields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keys: Vec<&String> = self.0.keys().collect();
        let values: Vec<&Field> = self.0.values().collect();
        tokens.extend(quote! {{
            let mut map = aggregate::types::FieldMap::new();
            #(map.insert(#keys.to_string(), #values);)*
            aggregate::types::Fields(map)
        }});
    }
}

impl ToTokens for Descendants {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keys: Vec<&String> = self.map.keys().collect();
        let values: Vec<&Type> = self.map.values().collect();

        match &self.variation {
            Some(variation) => {
                let variation = variation;
                tokens.extend(quote! {{
                    #(
                        let mut variation = amalgamate.fields.0.get_mut(#variation).unwrap();
                        if let Some(inner) = &mut variation.inner {
                            let field = inner.fields.0.get_mut(#keys).unwrap();
                            field.inner = Some(<#values as aggregate::Aggregate>::aggregate());
                        }
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
        tokens.extend(quote! {{
            let mut vec = Vec::new();
            #(vec.push(aggregate::syn::parse_quote!(#attrs));)*
            aggregate::types::Attributes(vec)
        }});
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
