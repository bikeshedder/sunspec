use codegen::Scope;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::Literal;
use quote::{format_ident, quote, ToTokens};
use thiserror::Error;

use crate::json::{Model, Point, PointAccess, PointMandatory, PointType};

#[derive(Debug, Error)]
pub enum GenModelError {
    #[error("Missing ID point")]
    MissingIdPoint { model: String, point: String },
    #[error("Missing L point")]
    MissingLPoint { model: String, point: String },
    #[error("Missing type")]
    MissingType { model: String, point: String },
    #[error("Missing offset")]
    MissingOffset { model: String, point: String },
    #[error("Missing length")]
    MissingLength { model: String, point: String },
}

pub fn gen_models_struct(models: &[Model]) -> Result<String, GenModelError> {
    let mut scope = Scope::new();
    for model in models {
        let model_id = model.id;
        scope.raw(&format!("pub mod model{model_id};"));
        /*
        scope
            .import(&format!("model{model_id}"), &format!("Model{model_id}"))
            .vis("pub");
        */
    }
    let models_struct = scope
        .new_struct("Models")
        .vis("pub")
        .derive("Debug")
        .derive("Default")
        .doc("This struct contains the addresses of all discovered models.");
    for model in models {
        let model_id = model.id;
        let field = models_struct
            .new_field(
                &format!("m{}", model_id),
                &format!("crate::ModelAddr<model{model_id}::Model{model_id}>"),
            )
            .vis("pub");
        if let Some(label) = &model.group.doc.label {
            field.doc(label);
        }
    }
    let mimpl = scope.new_impl("Models");
    // supported_ids() function
    let fn_supported_ids = mimpl
        .new_fn("supported_model_ids")
        .vis("pub")
        .arg_ref_self()
        .ret("Vec<u16>")
        .doc("Returns a list of all supported model ids");
    fn_supported_ids.line(format!("let mut v = Vec::with_capacity({});", models.len()));
    for model in models {
        fn_supported_ids.line(format!("if self.m{}.addr != 0 {{", model.id));
        fn_supported_ids.line(format!("    v.push({});", model.id));
        fn_supported_ids.line("}");
    }
    fn_supported_ids.line("v");
    // set_addr() function
    let fn_set_addr = mimpl
        .new_fn("set_addr")
        .vis("pub")
        .arg_mut_self()
        .arg("model_id", "u16")
        .arg("addr", "u16")
        .arg("len", "u16")
        .ret("bool")
        .doc("Set address and length of the given model.\n\nThis method is used by the model discovery.");
    fn_set_addr.line("match model_id {");
    for model in models {
        let model_id = model.id;
        fn_set_addr.line(format!(
            "    {model_id} => self.m{model_id}.set_addr(addr, len),"
        ));
    }
    fn_set_addr.line("    _ => { return false; }");
    fn_set_addr.line("}");
    fn_set_addr.line("true");
    Ok(scope.to_string())
}

// See https://doc.rust-lang.org/reference/keywords.html
#[rustfmt::skip]
const RUST_KEYWORDS: &[&str] = &[
    // strict
    "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
    // reserved
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try",
    // weak keywords
    "macro_rules", "union",
];

fn safe_identifier(s: String) -> String {
    if RUST_KEYWORDS.contains(&s.as_str()) {
        format!("r#{s}")
    } else {
        s
    }
}

pub fn gen_model_struct(model: &Model) -> Result<String, GenModelError> {
    let mut scope = Scope::new();
    scope.raw(format!("//! {}", model.group.doc.label.as_ref().unwrap()));
    let model_id = model.id;
    let model_name = format!("Model{}", model_id);
    let model_struct = scope.new_struct(&model_name).vis("pub").derive("Debug");
    let model_doc = model.group.doc.to_doc_string();
    if !model_doc.is_empty() {
        model_struct.doc(&model_doc);
    }
    if let Some(Point { name, .. }) = model.group.points.get(0) {
        if name != "ID" {
            return Err(GenModelError::MissingIdPoint {
                model: model_name,
                point: name.clone(),
            });
        }
    }
    if let Some(Point { name, .. }) = model.group.points.get(1) {
        if name != "L" {
            return Err(GenModelError::MissingLPoint {
                model: model_name,
                point: name.clone(),
            });
        }
    }
    let points = &model.group.points[2..];
    for point in points {
        if point.r#type == PointType::Pad {
            continue;
        }
        let field = model_struct
            .new_field(safe_identifier(point.name.to_snake_case()), {
                rust_type(point)
            })
            .vis("pub");
        let field_doc = point.doc.to_doc_string();
        if !field_doc.is_empty() {
            field.doc(field_doc);
        } else {
            field.annotation("#[allow(missing_docs)]");
        }
    }
    scope.raw("#[allow(missing_docs)]");
    let model_impl = scope.new_impl(&model_name);
    let mut offset = 0;
    for point in points {
        let writable = point.access == PointAccess::RW;
        if point.r#type != PointType::Pad {
            model_impl.associate_const(
                &point.name.to_shouty_snake_case(),
                &format!("crate::PointDef<Self, {}>", rust_type(point)),
                &format!(
                    "crate::PointDef::new({}, {}, {})",
                    offset, point.size, writable
                ),
                "pub",
            );
        }
        offset += point.size;
    }
    let trait_impl = scope.new_impl(&model_name).impl_trait("crate::Model");
    trait_impl.associate_const("ID", "u16", model_id.to_string(), "");
    //trait_impl.associate_const("LENGTH", "u16", format!("{}", offset), "");
    let fn_from_data = trait_impl
        .new_fn("from_data")
        .arg(if points.is_empty() { "_data" } else { "data" }, "&[u16]")
        .ret("Result<Self, crate::ReadModelError>");
    fn_from_data.line("Ok(Self {");
    for point in points {
        if point.r#type == PointType::Pad {
            continue;
        }
        fn_from_data.line(format!(
            "    {}: Self::{}.from_data(data)?,",
            safe_identifier(point.name.to_snake_case()),
            safe_identifier(point.name.to_shouty_snake_case()),
        ));
    }
    fn_from_data.line("})");
    for point in points {
        match point.r#type {
            PointType::Enum16 | PointType::Enum32 if !point.symbols.is_empty() => {
                scope.raw(gen_enum(point));
            }
            PointType::Bitfield16 | PointType::Bitfield32 | PointType::Bitfield64 => {
                scope.raw(gen_bitfield(point));
            }
            _ => {}
        }
    }
    Ok(scope.to_string())
}

fn gen_enum(point: &Point) -> String {
    let size = point.r#type.size().unwrap();
    let repr = format_ident!("u{}", size * 16);
    let name = format_ident!("{}", point.name.to_upper_camel_case());
    let invalid = match point.r#type {
        PointType::Enum16 => Literal::u16_unsuffixed(u16::MAX),
        PointType::Enum32 => Literal::u32_unsuffixed(u32::MAX),
        _ => unimplemented!(),
    };
    let variants = point.symbols.iter().map(|symbol| {
        let name = format_ident!("{}", symbol.name.to_upper_camel_case());
        let value = match point.r#type {
            PointType::Enum16 => {
                Literal::u16_unsuffixed(symbol.value.as_u64().unwrap().try_into().unwrap())
            }
            PointType::Enum32 => {
                Literal::u32_unsuffixed(symbol.value.as_u64().unwrap().try_into().unwrap())
            }
            _ => unimplemented!(),
        };
        let variant_doc = symbol.doc.to_doc_string();
        quote! {
            #[doc = #variant_doc]
            #name = #value
        }
        .into_token_stream()
    });
    let doc = point.doc.to_doc_string();
    let code = quote!(
        #[doc = #doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
        #[repr(#repr)]
        pub enum #name {
            #(#variants),*
        }
        impl crate::Value for #name {
            fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
                let value = #repr::decode(data)?;
                Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
            }
            fn encode(self) -> Box<[u16]> {
                (self as #repr).encode()
            }
        }
        impl crate::Value for Option<#name> {
            fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
                let value = #repr::decode(data)?;
                if value != #invalid {
                    Ok(Some(#name::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?))
                } else {
                    Ok(None)
                }
            }
            fn encode(self) -> Box<[u16]> {
                if let Some(value) = self {
                    value.encode()
                } else {
                    #invalid.encode()
                }
            }
        }
    );
    code.to_string()
}

fn gen_bitfield(point: &Point) -> String {
    let size = point.r#type.size().unwrap();
    let repr = format_ident!("u{}", size * 16);
    let name = format_ident!("{}", point.name.to_upper_camel_case());
    let invalid = match point.r#type {
        PointType::Bitfield16 => Literal::u16_suffixed(u16::MAX),
        PointType::Bitfield32 => Literal::u32_suffixed(u32::MAX),
        PointType::Bitfield64 => Literal::u64_suffixed(u64::MAX),
        _ => unimplemented!(),
    };
    let doc = point.doc.to_doc_string();
    let fields = point.symbols.iter().map(|symbol| {
        let field_name = format_ident!("{}", symbol.name.to_upper_camel_case());
        let bit = Literal::u64_unsuffixed(1 << symbol.value.as_u64().unwrap());
        let field_doc = symbol.doc.to_doc_string();
        quote! {
            #[doc = #field_doc]
            const #field_name = #bit;
        }
        .into_token_stream()
    });
    let code = quote! {
        bitflags::bitflags! {
            #[doc = #doc]
            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub struct #name: #repr {
                #(#fields)*
            }
        }
        impl crate::Value for #name {
            fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
                let value = #repr::decode(data)?;
                Ok(Self::from_bits_retain(value))
            }
            fn encode(self) -> Box<[u16]> {
                self.bits().encode()
            }
        }
        impl crate::Value for Option<#name> {
            fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
                let value = #repr::decode(data)?;
                if value != #invalid {
                    Ok(Some(#name::from_bits_retain(value)))
                } else {
                    Ok(None)
                }
            }
            fn encode(self) -> Box<[u16]> {
                if let Some(value) = self {
                    value.encode()
                } else {
                    #invalid.encode()
                }
            }
        }
    };
    code.to_string()
}

fn rust_type(point: &Point) -> String {
    let rty: String = match point.r#type {
        PointType::Int16 => "i16".into(),
        PointType::Int32 => "i32".into(),
        PointType::Int64 => "i64".into(),
        PointType::Raw16 => "u16".into(),
        PointType::Uint16 => "u16".into(),
        PointType::Uint32 => "u32".into(),
        PointType::Uint64 => "u64".into(),
        PointType::Acc16 => "u16".into(),
        PointType::Acc32 => "u32".into(),
        PointType::Acc64 => "u64".into(),
        PointType::Bitfield16 => point.name.to_upper_camel_case(),
        PointType::Bitfield32 => point.name.to_upper_camel_case(),
        PointType::Bitfield64 => point.name.to_upper_camel_case(),
        PointType::Enum16 if point.symbols.is_empty() => "u16".into(),
        PointType::Enum16 => point.name.to_upper_camel_case(),
        PointType::Enum32 if point.symbols.is_empty() => "u32".into(),
        PointType::Enum32 => point.name.to_upper_camel_case(),
        PointType::Float32 => "f32".into(),
        PointType::Float64 => "f64".into(),
        PointType::String => "String".into(),
        PointType::Sf => "i16".into(), // FIXME is this type correct?
        PointType::Pad => unimplemented!(),
        PointType::Ipaddr => "std::net::Ipv4Addr".into(),
        PointType::Ipv6addr => "std::net::Ipv6Addr".into(),
        PointType::Eui48 => "String".into(),
        PointType::Sunssf => "i16".into(),
        PointType::Count => "u16".into(),
    };
    if point.mandatory == PointMandatory::M {
        rty
    } else {
        format!("Option<{rty}>")
    }
}
