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

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析类名
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !content.is_empty() {
            if content.peek(Token![fn]) {
                // 解析方法
                let method: ImplItem = content.parse()?;
                methods.push(method);
            } else {
                // 解析字段
                let ident: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let ty: Type = content.parse()?;
                // 处理可选的逗号或分号
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

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    let class = parse_macro_input!(input as Class);

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
            // 检查方法签名，添加 &self 参数
            if !method_fn.sig.inputs.iter().any(|arg| matches!(arg, FnArg::Receiver(_))) {
                method_fn.sig.inputs.insert(0, syn::parse_quote!( &self ));
            }
            quote! {
                #method_fn
            }
        } else {
            quote! {}
        }
    });

    let expanded = quote! {
        pub struct #name {
            #(#field_defs)*
        }

        impl #name {
            #(#method_defs)*
        }
    };

    TokenStream::from(expanded)
}
