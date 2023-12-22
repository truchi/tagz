use crate::{text, AttributeType, Element};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(element: &Element) -> TokenStream {
    let name = text::pascal(&element.name);
    let builder = text::builder(&element.name);
    let children = element.has_children()
            .then_some(text::child(&element.name))
            .map(|child| quote! {
                pub fn child<T: Into<children::#child>>(mut self, child: T) -> Self {
                    self.element.children.push(child.into());
                    self
                }

                pub fn children<T: Into<children::#child>, I: IntoIterator<Item = T>>(mut self, children: I) -> Self {
                    self.element.children.extend(children.into_iter().map(|child| child.into()));
                    self
                }
            });
    let attributes = element
        .attributes
        .iter()
        .map(|(name, ty)| (text::attribute(name), ty))
        .map(|(name, ty)| match ty {
            AttributeType::Bool | AttributeType::BoolOrF64OrString => quote! {
                pub fn #name<T: Into<#ty>>(mut self, #name: T) -> Self {
                    self.element.#name = #name.into();
                    self
                }
            },
            _ => quote! {
                pub fn #name<T: Into<#ty>>(mut self, #name: T) -> Self {
                    self.element.#name = Some(#name.into());
                    self
                }
            },
        });
    let description = format!(" The `<{}>` element's builder.", element.name);

    quote! {
        use crate::*;

        #[doc = #description]
        #[derive(Clone, Default, Debug)]
        pub struct #builder {
            pub element: #name,
        }

        impl #builder {
            pub fn build(self) -> #name {
                self.element
            }

            pub fn id<T: Into<CowStr>>(mut self, id: T) -> Self {
                self.element.id = Some(id.into());
                self
            }

            pub fn class<T: Into<CowStr>>(mut self, class: T) -> Self {
                self.element.classes.insert(class.into());
                self
            }

            pub fn classes<T: Into<CowStr>, I: IntoIterator<Item = T>>(mut self, classes: I) -> Self {
                self.element.classes.extend(classes.into_iter().map(|class| class.into()));
                self
            }

            pub fn data<K: Into<CowStr>, V: Into<AttributeType>>(mut self, key: K, value: V) -> Self {
                self.element.datas.insert(key.into(), value.into());
                self
            }

            pub fn datas<K: Into<CowStr>, V: Into<AttributeType>, I: IntoIterator<Item = (K, V)>>(mut self, datas: I) -> Self {
                self.element.datas.extend(datas.into_iter().map(|(key, value)| (key.into(), value.into())));
                self
            }

            #children
            #(#attributes)*
        }

        impl From<#name> for #builder {
            fn from(element: #name) -> Self {
                Self { element }
            }
        }

        impl std::fmt::Display for #builder {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.element)
            }
        }
    }
}
