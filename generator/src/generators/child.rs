use crate::{text, Element};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(element: &Element) -> TokenStream {
    let child = text::child(&element.name);
    let name = element.children.iter().map(|name| text::pascal(name));
    let text = element.text.then_some(quote! { Text(CowStr) });
    let str_froms = element.text.then_some(quote! {
        impl From<&'static str> for #child {
            fn from(s: &'static str) -> Self {
                #child::Text(s.into())
            }
        }

        impl From<String> for #child {
            fn from(s: String) -> Self {
                #child::Text(s.into())
            }
        }

        impl From<CowStr> for #child {
            fn from(s: CowStr) -> Self {
                #child::Text(s)
            }
        }
    });
    let froms = element
        .children
        .iter()
        .map(|name| (text::pascal(name), text::builder(name)))
        .map(|(name, builder)| {
            quote! {
                impl From<#name> for #child {
                    fn from(child: #name) -> Self {
                        #child::#name(child)
                    }
                }

                impl From<builders::#builder> for #child {
                    fn from(builder: builders::#builder) -> Self {
                        #child::#name(builder.build())
                    }
                }
            }
        });
    let debug = {
        let name = name.clone();
        let text = element
            .text
            .then_some(quote! { #child::Text(text) => std::fmt::Debug::fmt(text, f), });
        quote! {
            impl std::fmt::Debug for #child {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#child::#name(child) => std::fmt::Debug::fmt(child, f),)*
                        #text
                    }
                }
            }
        }
    };
    let display = {
        let name = name.clone();
        let text = element
            .text
            .then_some(quote! { #child::Text(text) => std::fmt::Display::fmt(text, f), });
        quote! {
            impl std::fmt::Display for #child {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#child::#name(child) => std::fmt::Display::fmt(child, f),)*
                        #text

                    }
                }
            }
        }
    };
    let description = format!(" The `<{}>` element's children.", element.tag);

    quote! {
        use crate::*;

        #[doc = #description]
        #[derive(Clone)]
        pub enum #child {
            #(#name(#name),)*
            #text
        }

        #(#froms)*
        #str_froms
        #debug
        #display
    }
}
