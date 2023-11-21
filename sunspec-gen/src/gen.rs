use codegen::Scope;
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
        scope.raw(&format!("mod model{model_id};"));
        scope
            .import(&format!("model{model_id}"), &format!("Model{model_id}"))
            .vis("pub");
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
                &format!("crate::ModelAddr<Model{}>", model_id),
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
            .new_field(safe_identifier(point.name.to_lowercase()), {
                rust_type(point.r#type, point.mandatory == PointMandatory::M)
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
                &point.name.to_uppercase(),
                &format!(
                    "crate::PointDef<Self, {}>",
                    rust_type(point.r#type, point.mandatory == PointMandatory::M)
                ),
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
            safe_identifier(point.name.to_lowercase()),
            safe_identifier(point.name.to_uppercase()),
        ));
    }
    fn_from_data.line("})");
    Ok(scope.to_string())
}

fn rust_type(ty: PointType, mandatory: bool) -> String {
    let rty = match ty {
        PointType::Int16 => "i16",
        PointType::Int32 => "i32",
        PointType::Int64 => "i64",
        PointType::Raw16 => "u16",
        PointType::Uint16 => "u16",
        PointType::Uint32 => "u32",
        PointType::Uint64 => "u64",
        PointType::Acc16 => "u16",
        PointType::Acc32 => "u32",
        PointType::Acc64 => "u64",
        PointType::Bitfield16 => "u16",
        PointType::Bitfield32 => "u32",
        PointType::Bitfield64 => "u64",
        PointType::Enum16 => "u16",
        PointType::Enum32 => "u32",
        PointType::Float32 => "f32",
        PointType::Float64 => "f64",
        PointType::String => "String",
        PointType::Sf => "i16", // FIXME is this type correct?
        PointType::Pad => unimplemented!(),
        PointType::Ipaddr => "std::net::Ipv4Addr",
        PointType::Ipv6addr => "std::net::Ipv6Addr",
        PointType::Eui48 => "String",
        PointType::Sunssf => "i16",
        PointType::Count => "u16",
    };
    if mandatory {
        String::from(rty)
    } else {
        format!("Option<{rty}>")
    }
}

fn type_len(ty: PointType) -> Option<u16> {
    Some(match ty {
        PointType::Int16 => 1,
        PointType::Int32 => 2,
        PointType::Int64 => 4,
        PointType::Raw16 => 1,
        PointType::Uint16 => 1,
        PointType::Uint32 => 2,
        PointType::Uint64 => 4,
        PointType::Acc16 => 1,
        PointType::Acc32 => 2,
        PointType::Acc64 => 4,
        PointType::Bitfield16 => 1,
        PointType::Bitfield32 => 2,
        PointType::Bitfield64 => 4,
        PointType::Enum16 => 1,
        PointType::Enum32 => 2,
        PointType::Float32 => 2,
        PointType::Float64 => 4,
        PointType::String => return None,
        PointType::Sf => 1, // FIXME is this type correct?
        PointType::Pad => 1,
        PointType::Ipaddr => 2,
        PointType::Ipv6addr => 8,
        PointType::Eui48 => 3,
        PointType::Sunssf => 1,
        PointType::Count => 1,
    })
}
