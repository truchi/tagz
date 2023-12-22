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
                    let value = #name.into();
                    debug_assert!(check_attribute_value(&AttributeType::from(value.clone())), "Invalid attribute value: {value:?}");
                    self.element.#name = Some(value);
                    self
                }
            },
        });
    let derives = if element.custom {
        quote! { #[derive(Clone, Debug)] }
    } else {
        quote! { #[derive(Clone, Default, Debug)] }
    };
    let description = format!(" The `<{}>` element's builder.", element.name);

    quote! {
        use crate::*;

        #[doc = #description]
        #derives
        pub struct #builder {
            pub element: #name,
        }

        impl #builder {
            pub fn build(self) -> #name {
                self.element
            }

            pub fn id<T: Into<CowStr>>(mut self, id: T) -> Self {
                let id = id.into();
                debug_assert!(check_id(&id), "Invalid id: {id:?}");
                self.element.id = Some(id);
                self
            }

            pub fn class<T: Into<CowStr>>(mut self, class: T) -> Self {
                let class = class.into();
                debug_assert!(check_class(&class), "Invalid class: {class:?}");
                self.element.classes.insert(class);
                self
            }

            pub fn classes<T: Into<CowStr>, I: IntoIterator<Item = T>>(mut self, classes: I) -> Self {
                self.element
                    .classes
                    .extend(classes.into_iter().map(|class| {
                        let class = class.into();
                        debug_assert!(check_class(&class), "Invalid class: {class:?}");
                        class
                    }));
                self
            }

            pub fn data<K: Into<CowStr>, V: Into<AttributeType>>(mut self, key: K, value: V) -> Self {
                let key = key.into();
                let value = value.into();
                debug_assert!(check_attribute_name(&key), "Invalid attribute name: {key:?}");
                debug_assert!(check_attribute_value(&value), "Invalid attribute value: {value:?}");
                self.element.datas.insert(key.into(), value.into());
                self
            }

            pub fn datas<K: Into<CowStr>, V: Into<AttributeType>, I: IntoIterator<Item = (K, V)>>(
                mut self,
                datas: I,
            ) -> Self {
                self.element
                    .datas
                    .extend(datas.into_iter().map(|(key, value)| {
                        let key = key.into();
                        let value = value.into();
                        debug_assert!(check_attribute_name(&key), "Invalid attribute name: {key:?}");
                        debug_assert!(check_attribute_value(&value), "Invalid attribute value: {value:?}");

                        (key, value)
                    }));
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
