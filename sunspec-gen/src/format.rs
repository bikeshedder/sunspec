//! Formatting of generated code.
//!
//! `prettyplease` does not format the bodies of arbitrary macro invocations
//! and prints them as a single stream of tokens. This module formats
//! `bitflags!` invocations by hand so every flag ends up on its own line.

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_quote, Attribute, Expr, Ident, Item, Token, Type, Visibility,
};

const INDENT: &str = "    ";

/// Format a token stream as a Rust source file.
pub fn format_file(stream: TokenStream) -> String {
    let mut file = syn::parse_file(&stream.to_string()).unwrap();
    let mut bitflags = Vec::new();
    for item in &mut file.items {
        let Item::Macro(item_macro) = item else {
            continue;
        };
        if !item_macro
            .mac
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "bitflags")
        {
            continue;
        }
        let path = &item_macro.mac.path;
        let body = syn::parse2::<Bitflags>(item_macro.mac.tokens.clone()).unwrap();
        let index = Literal::usize_unsuffixed(bitflags.len());
        bitflags.push(body.format(&quote!(#path).to_string().replace(' ', "")));
        *item = parse_quote!(__bitflags_placeholder!(#index););
    }
    let mut code = prettyplease::unparse(&file);
    for (index, text) in bitflags.iter().enumerate() {
        code = code.replace(&format!("__bitflags_placeholder!({index});\n"), text);
    }
    code
}

/// Body of a `bitflags!` invocation.
struct Bitflags {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    repr: Type,
    flags: Vec<Flag>,
}

/// A single `const Name = value;` inside a `bitflags!` invocation.
struct Flag {
    attrs: Vec<Attribute>,
    name: Ident,
    value: Expr,
}

impl Parse for Bitflags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let repr = input.parse()?;
        let content;
        braced!(content in input);
        let mut flags = Vec::new();
        while !content.is_empty() {
            flags.push(content.parse()?);
        }
        Ok(Self {
            attrs,
            vis,
            name,
            repr,
            flags,
        })
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        input.parse::<Token![const]>()?;
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(Self { attrs, name, value })
    }
}

impl Bitflags {
    fn format(&self, path: &str) -> String {
        let Self {
            attrs,
            vis,
            name,
            repr,
            flags,
        } = self;
        let mut out = format!("{path}! {{\n");
        // Let prettyplease format the attributes and struct header by
        // printing a unit struct with the same attributes.
        let header = unparse_item(parse_quote! {
            #(#attrs)*
            #vis struct #name;
        });
        let repr = quote!(#repr).to_string();
        push_indented(
            &mut out,
            &header.replacen(
                &format!("struct {name};"),
                &format!("struct {name}: {repr} {{"),
                1,
            ),
            1,
        );
        for Flag { attrs, name, value } in flags {
            let flag = unparse_item(parse_quote! {
                #(#attrs)*
                const #name: _ = #value;
            });
            push_indented(
                &mut out,
                &flag.replacen(&format!("const {name}: _ ="), &format!("const {name} ="), 1),
                2,
            );
        }
        out.push_str(INDENT);
        out.push_str("}\n}\n");
        out
    }
}

fn unparse_item(item: Item) -> String {
    prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: vec![item],
    })
}

fn push_indented(out: &mut String, text: &str, level: usize) {
    for line in text.lines() {
        if !line.is_empty() {
            out.push_str(&INDENT.repeat(level));
            out.push_str(line);
        }
        out.push('\n');
    }
}
