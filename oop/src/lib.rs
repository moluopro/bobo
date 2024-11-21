/*
 * Copyright (C) 2024-2025 moluopro. All rights reserved.
 * Github: https://github.com/moluopro
 */

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, Ident, ImplItem, Token, Type,
};

struct Class {
    name: Ident,
    fields: Vec<(Ident, Type)>,
    methods: Vec<ImplItem>,
}

struct Classes {
    classes: Vec<Class>,
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !content.is_empty() {
            if content.peek(Token![fn]) {
                let method: ImplItem = content.parse()?;
                methods.push(method);
            } else {
                let ident: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let ty: Type = content.parse()?;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    content.parse::<Token![;]>().ok();
                }
                fields.push((ident, ty));
            }
        }

        Ok(Class { name, fields, methods })
    }
}

impl Parse for Classes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes = Vec::new();

        while !input.is_empty() {
            let class: Class = input.parse()?;
            classes.push(class);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Classes { classes })
    }
}

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    let classes = parse_macro_input!(input as Classes);

    let class_defs = classes.classes.iter().map(|class| {
        let name = &class.name;
        let fields = &class.fields;
        let methods = &class.methods;

        let field_defs = fields.iter().map(|(ident, ty)| {
            quote! {
                pub #ident: #ty,
            }
        });

        let method_defs = methods.iter().map(|method| {
            if let ImplItem::Fn(mut method_fn) = method.clone() {
                let method_name = &method_fn.sig.ident;

                if method_name.to_string().to_lowercase() != "new" {
                    if !method_fn.sig.inputs.iter().any(|arg| matches!(arg, FnArg::Receiver(_))) {
                        method_fn.sig.inputs.insert(0, syn::parse_quote!( &self ));
                    }
                }

                quote! {
                    #method_fn
                }
            } else {
                quote! {}
            }
        });

        quote! {
            pub struct #name {
                #(#field_defs)*
            }

            impl #name {
                #(#method_defs)*
            }
        }
    });

    let expanded = quote! {
        #(#class_defs)*
    };

    TokenStream::from(expanded)
}
