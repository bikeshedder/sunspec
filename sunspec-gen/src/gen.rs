use std::collections::HashMap;

use codegen::Scope;
use thiserror::Error;

use crate::{
    doc::Documentation,
    smdx::{BlockType, ModelGroup, PointAccess, PointType},
};

#[derive(Debug, Error)]
pub enum GenModelError {
    #[error("Missing type")]
    MissingType { model: String, point: String },
    #[error("Missing offset")]
    MissingOffset { model: String, point: String },
    #[error("Missing length")]
    MissingLength { model: String, point: String },
}

pub fn gen_models_struct(model_groups: &[ModelGroup]) -> Result<String, GenModelError> {
    let mut scope = Scope::new();
    for model_group in model_groups {
        let model_id = model_group.model.id;
        scope.raw(&format!("mod model{};", model_id));
        scope
            .import(&format!("model{model_id}"), &format!("Model{model_id}"))
            .vis("pub");
    }
    let models = scope
        .new_struct("Models")
        .vis("pub")
        .derive("Debug")
        .derive("Default")
        .doc("This struct contains the addresses of all discovered models.");
    for model_group in model_groups {
        let model_id = model_group.model.id;
        let strings = model_group.strings.first().unwrap();
        let field = models
            .new_field(
                &format!("m{}", model_id),
                &format!("crate::ModelAddr<Model{}>", model_id),
            )
            .vis("pub");
        if let Some(strings_model) = &strings.model {
            if let Some(label) = &strings_model.label {
                field.doc(label);
            }
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
    fn_supported_ids.line(format!(
        "let mut v = Vec::with_capacity({});",
        model_groups.len()
    ));
    for model_group in model_groups {
        fn_supported_ids.line(format!("if self.m{}.addr != 0 {{", model_group.model.id));
        fn_supported_ids.line(format!("    v.push({});", model_group.model.id));
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
    for model_group in model_groups {
        let model_id = model_group.model.id;
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

pub fn gen_model_struct(model_group: &ModelGroup) -> Result<String, GenModelError> {
    let mut scope = Scope::new();
    let model_id = model_group.model.id;
    let model_name = format!("Model{}", model_id);
    let model_len = model_group.model.len.unwrap();
    let model_struct = scope.new_struct(&model_name).vis("pub").derive("Debug");
    let strings = model_group.strings.first().unwrap();
    if let Some(strings_model) = &strings.model {
        let doc = Documentation {
            label: strings_model.label.clone(),
            description: strings_model.description.clone(),
            notes: strings_model.notes.clone(),
        };
        model_struct.doc(&doc.to_string());
    }
    let mut point_docs: HashMap<String, Documentation> = HashMap::new();
    for point_doc in &strings.points {
        point_docs.insert(
            point_doc.id.to_owned(),
            Documentation {
                label: point_doc.label.clone(),
                description: point_doc.description.clone(),
                notes: point_doc.notes.clone(),
            },
        );
    }
    let points = model_group
        .model
        .blocks
        .iter()
        .filter(|b| b.r#type == BlockType::Fixed)
        .flat_map(|b| &b.points)
        // Remove all paddings from the structure as it is of
        // no practical value and just causes troubles with some
        // SunSpec devices (e.g. SolarEdge SE25K) which report
        // the model length without the last padding.
        .filter(|p| p.r#type != PointType::Pad)
        .collect::<Vec<_>>();
    for point in &points {
        let field = model_struct
            .new_field(safe_identifier(point.id.to_lowercase()), {
                if point.mandatory {
                    String::from(rust_type(point.r#type))
                } else {
                    format!("Option<{}>", rust_type(point.r#type))
                }
            })
            .vis("pub");
        if let Some(doc) = point_docs.get(&point.id) {
            field.doc(doc.to_string());
        } else {
            field.annotation("#[allow(missing_docs)]");
        }
    }
    scope.raw("#[allow(missing_docs)]");
    let model_impl = scope.new_impl(&model_name);
    for point in &points {
        let offset = point.offset.ok_or_else(|| GenModelError::MissingOffset {
            model: model_name.clone(),
            point: point.id.clone(),
        })?;
        let length = point
            .len
            .or_else(|| type_len(point.r#type))
            .ok_or_else(|| GenModelError::MissingLength {
                model: model_name.clone(),
                point: point.id.clone(),
            })?;
        let writable = point.access == PointAccess::RW;
        model_impl.associate_const(
            &point.id.to_uppercase(),
            &format!("crate::PointDef<Self, {}>", rust_type(point.r#type)),
            &format!("crate::PointDef::new({}, {}, {})", offset, length, writable),
            "pub",
        );
    }
    let trait_impl = scope.new_impl(&model_name).impl_trait("crate::Model");
    trait_impl.associate_const("ID", "u16", model_id.to_string(), "");
    trait_impl.associate_const("LENGTH", "u16", format!("{}", model_len), "");
    let fn_from_data = trait_impl
        .new_fn("from_data")
        .arg(if points.is_empty() { "_data" } else { "data" }, "&[u16]")
        .ret("Result<Self, crate::ReadModelError>");
    fn_from_data.line("Ok(Self {");
    for point in &points {
        fn_from_data.line(format!(
            "    {}: Self::{}.from_data(data)?{},",
            safe_identifier(point.id.to_lowercase()),
            safe_identifier(point.id.to_uppercase()),
            if point.mandatory {
                ".ok_or(crate::ReadPointError::MissingMandatoryValue)?"
            } else {
                ""
            }
        ));
    }
    fn_from_data.line("})");
    Ok(scope.to_string())
}

fn rust_type(ty: PointType) -> &'static str {
    match ty {
        PointType::Int16 => "i16",
        PointType::Uint16 => "u16",
        PointType::Count => "u16",
        PointType::Acc16 => "u16",
        PointType::Int32 => "i32",
        PointType::Uint32 => "u32",
        PointType::Float32 => "f32",
        PointType::Acc32 => "u32",
        PointType::Int64 => "i64",
        PointType::Uint64 => "u64",
        PointType::Float64 => "f64",
        PointType::Acc64 => "u64",
        PointType::Enum16 => "u16",
        PointType::Enum32 => "u32",
        PointType::Bitfield16 => "u16",
        PointType::Bitfield32 => "u32",
        PointType::Sunssf => "i16",
        PointType::String => "String",
        PointType::Pad => "u16",
        PointType::Ipaddr => "u32",
        PointType::Ipv6addr => "u128",
        PointType::Eui48 => "String",
    }
}

fn type_len(ty: PointType) -> Option<u16> {
    Some(match ty {
        PointType::Int16 => 1,
        PointType::Uint16 => 1,
        PointType::Count => 1,
        PointType::Acc16 => 1,
        PointType::Int32 => 2,
        PointType::Uint32 => 2,
        PointType::Float32 => 2,
        PointType::Acc32 => 2,
        PointType::Int64 => 4,
        PointType::Uint64 => 4,
        PointType::Float64 => 4,
        PointType::Acc64 => 4,
        PointType::Enum16 => 1,
        PointType::Enum32 => 2,
        PointType::Bitfield16 => 1,
        PointType::Bitfield32 => 2,
        PointType::Sunssf => 1,
        PointType::String => return None,
        PointType::Pad => 1,
        PointType::Ipaddr => 2,
        PointType::Ipv6addr => 8,
        PointType::Eui48 => 3, // FIXME is this correct?
    })
}
