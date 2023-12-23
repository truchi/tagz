use crate::{text, AttributeType, Element, MDN};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(element: &Element) -> TokenStream {
    let name = text::pascal(&element.name);
    let child = text::child(&element.name);
    let builder = text::builder(&element.name);
    let attributes = element
        .attributes
        .iter()
        .map(|(name, ty)| (text::attribute(name), ty))
        .map(|(name, ty)| match ty {
            AttributeType::Bool => quote! { pub #name: bool, },
            AttributeType::BoolOrF64OrString => quote! { pub #name: BoolOrF64OrString, },
            _ => quote! { pub #name: std::option::Option<#ty>, },
        });
    let children = element.has_children().then_some(quote! {
        pub children: Vec<children::#child>,
    });
    let children_builder = element.has_children().then_some(quote! {
            pub fn child<T: Into<children::#child>>(child: T) -> builders::#builder {
                <builders::#builder as Default>::default().child(child)
            }

            pub fn children<T: Into<children::#child>, I: IntoIterator<Item = T>>(children: I) -> builders::#builder {
                <builders::#builder as Default>::default().children(children)
            }
        });
    let attribute_builders = element
        .attributes
        .iter()
        .map(|(name, ty)| (text::attribute(name), ty))
        .map(|(name, ty)| {
            quote! {
                pub fn #name<T: Into<#ty>>(#name: T) -> builders::#builder {
                    <builders::#builder as Default>::default().#name(#name)
                }
            }
        });
    let from_builder = quote! {
        impl From<builders::#builder> for #name {
            fn from(builder: builders::#builder) -> Self {
                builder.element
            }
        }
    };
    let debug = {
        let tag = &element.tag;
        let tag = if element.custom {
            quote! { self.tag }
        } else {
            quote! { #tag }
        };
        let attributes = element.attributes.iter().map(|(name, ty)| {
            let attribute = text::attribute(name);
            match ty {
                AttributeType::Bool => quote! {
                    if self.#attribute {
                        f.field(#name, &true);
                    }
                },
                AttributeType::BoolOrF64OrString => quote! {
                    match &self.#attribute {
                        BoolOrF64OrString::Bool(false) => &mut f,
                        BoolOrF64OrString::Bool(true) => f.field(#name, &true),
                        BoolOrF64OrString::F64(value) => f.field(#name, &value),
                        BoolOrF64OrString::String(value) => f.field(#name, &value),
                    };
                },
                _ => quote! {
                    if let Some(value) = &self.#attribute {
                        f.field(#name, &value);
                    }
                },
            }
        });
        let children = element.has_children().then_some(quote! {
            if !self.children.is_empty() {
                f.field("children", &self.children);
            }
        });

        quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut f = f.debug_struct(&format!("<{}>", #tag));

                    if let Some(id) = &self.id {
                        f.field("id", &id);
                    }

                    if !self.classes.is_empty() {
                        f.field("classes", &self.classes);
                    }

                    if !self.datas.is_empty() {
                        f.field(
                            "datas",
                            &self
                                .datas
                                .iter()
                                .map(|(key, value)| {
                                    (
                                        key,
                                        match value {
                                            AttributeType::String(value) => Box::new(value) as Box<dyn std::fmt::Debug>,
                                            AttributeType::Bool(value) => Box::new(value),
                                            AttributeType::I16(value) => Box::new(value),
                                            AttributeType::U16(value) => Box::new(value),
                                            AttributeType::I32(value) => Box::new(value),
                                            AttributeType::U32(value) => Box::new(value),
                                            AttributeType::F32(value) => Box::new(value),
                                            AttributeType::I64(value) => Box::new(value),
                                            AttributeType::U64(value) => Box::new(value),
                                            AttributeType::F64(value) => Box::new(value),
                                        },
                                    )
                                })
                                .collect::<HashMap<_, _>>(),
                        );
                    }

                    #(#attributes)*
                    #children

                    f.finish()
                }
            }
        }
    };
    let display = {
        let tag = &element.tag;
        let doctype = (tag == "html").then_some(quote! { write!(f, "<!DOCTYPE html>")?; });
        let tag = if element.custom {
            quote! { self.tag }
        } else {
            quote! { #tag }
        };
        let open = {
            let attributes = element.attributes.iter().map(|(name, ty)| {
                let attribute = text::attribute(name);
                match ty {
                    AttributeType::Bool => quote! {
                        if self.#attribute {
                            write!(f, " {}", #name)?;
                        }
                    },
                    AttributeType::BoolOrF64OrString => quote! {
                        match &self.#attribute {
                            BoolOrF64OrString::Bool(false) => {}
                            BoolOrF64OrString::Bool(true) => write!(f, " {}", #name)?,
                            BoolOrF64OrString::F64(value) => write!(f, " {}={value}", #name)?,
                            BoolOrF64OrString::String(value) => write!(f, " {}='{value}'", #name)?,
                        }
                    },
                    AttributeType::String => quote! {
                        if let Some(value) = &self.#attribute {
                            write!(f, " {}='{value}'", #name)?;
                        }
                    },
                    _ => quote! {
                        if let Some(value) = &self.#attribute {
                            write!(f, " {}={value}", #name)?;
                        }
                    },
                }
            });
            quote! {
                write!(f, "<{}", #tag)?;

                if let Some(id) = &self.id {
                    write!(f, " id=\"{id}\"")?;
                }

                let mut classes = self.classes.iter();
                if let Some(class) = classes.next() {
                    write!(f, " class=\"{class}")?;
                    for class in classes {
                        write!(f, " {class}")?;
                    }
                    write!(f, "\"")?;
                }

                for (key, value) in &self.datas {
                    match value {
                        AttributeType::Bool(false) => {}
                        AttributeType::Bool(true) => write!(f, " {key}")?,
                        AttributeType::I16(value) => write!(f, " {key}={value}")?,
                        AttributeType::U16(value) => write!(f, " {key}={value}")?,
                        AttributeType::I32(value) => write!(f, " {key}={value}")?,
                        AttributeType::U32(value) => write!(f, " {key}={value}")?,
                        AttributeType::F32(value) => write!(f, " {key}={value}")?,
                        AttributeType::I64(value) => write!(f, " {key}={value}")?,
                        AttributeType::U64(value) => write!(f, " {key}={value}")?,
                        AttributeType::F64(value) => write!(f, " {key}={value}")?,
                        AttributeType::String(value) => write!(f, " {key}=\"{value}\"")?,
                    }
                }

                #(#attributes)*
                write!(f, ">")?;
            }
        };
        let children = element.has_children().then_some(quote! {
            for child in &self.children {
                write!(f, "{child}")?;
            }
        });
        let end = element
            .end_tag
            .then_some(quote! { write!(f, "</{}>", #tag)?; });

        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    #doctype
                    #open
                    #children
                    #end
                    Ok(())
                }
            }
        }
    };

    if element.custom {
        assert!(element.has_children());

        let new = {
            let attributes = element
                .attributes
                .iter()
                .map(|(name, _)| text::attribute(name))
                .map(|name| quote! { #name: Default::default(), });

            quote! {
                pub fn new<T: Into<CowStr>>(tag: T) -> builders::#builder {
                    builders::#builder {
                        element: #name {
                            tag: tag.into(),
                            id: None,
                            classes: HashSet::new(),
                            datas: HashMap::new(),
                            #(#attributes)*
                            children: Vec::new(),
                        },
                    }
                }
            }
        };

        quote! {
            use crate::*;

            /// An autonomous custom element.
            ///
            /// [`MDN`](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
            #[derive(Clone)]
            pub struct #name {
                pub tag: CowStr,
                pub id: StdOption<CowStr>,
                pub classes: HashSet<CowStr>,
                pub datas: HashMap<CowStr, AttributeType>,
                #(#attributes)*
                #children
            }

            impl #name {
                #new
            }

            #from_builder
            #debug
            #display
        }
    } else {
        let tag = &element.tag;
        let description = format!(" The `<{tag}>` element.");
        let link = format!(" [`MDN`]({MDN}/{tag})");

        quote! {
            use crate::*;

            #[doc = #description]
            ///
            #[doc = #link]
            #[doc(alias = #tag)]
            #[derive(Clone, Default)]
            pub struct #name {
                pub id: StdOption<CowStr>,
                pub classes: HashSet<CowStr>,
                pub datas: HashMap<CowStr, AttributeType>,
                #children
                #(#attributes)*
            }

            impl #name {
                pub fn new() -> Self {
                    <Self as Default>::default()
                }

                pub fn id<T: Into<CowStr>>(id: T) -> builders::#builder {
                    <builders::#builder as Default>::default().id(id)
                }

                pub fn class<T: Into<CowStr>>(class: T) -> builders::#builder {
                    <builders::#builder as Default>::default().class(class)
                }

                pub fn classes<T: Into<CowStr>, I: IntoIterator<Item = T>>(classes: I) -> builders::#builder {
                    <builders::#builder as Default>::default().classes(classes)
                }

                pub fn data<K: Into<CowStr>, V: Into<AttributeType>>(key: K, value: V) -> builders::#builder {
                    <builders::#builder as Default>::default().data(key, value)
                }

                pub fn datas<K: Into<CowStr>, V: Into<AttributeType>, I: IntoIterator<Item = (K, V)>>(datas: I) -> builders::#builder {
                    <builders::#builder as Default>::default().datas(datas)
                }

                #children_builder
                #(#attribute_builders)*
            }

            #from_builder
            #debug
            #display
        }
    }
}
