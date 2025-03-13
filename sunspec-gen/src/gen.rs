use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use thiserror::Error;

use crate::json::{Group, GroupCount, Model, Point, PointAccess, PointMandatory, PointType};

/// This fixes sunspec identifiers which contains things like
/// `SoC` and `SoH` which causes heck to transform them to `so_c`
/// and `so_h` which is not what we want.
fn normalize_ident(s: &impl AsRef<str>) -> String {
    s.as_ref().replace("SoC", "Soc").replace("SoH", "Soh")
}

fn snake_case(s: &impl AsRef<str>) -> String {
    use heck::ToSnakeCase;
    normalize_ident(s).to_snake_case()
}

fn shouty_snake_case(s: &impl AsRef<str>) -> String {
    use heck::ToShoutySnakeCase;
    normalize_ident(s).to_shouty_snake_case()
}

fn upper_camel_case(s: &impl AsRef<str>) -> String {
    use heck::ToUpperCamelCase;
    normalize_ident(s).to_upper_camel_case()
}

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

pub fn gen_models_struct(models: &[Model]) -> Result<TokenStream, GenModelError> {
    let modules = models.iter().map(|model| {
        let module_identifier = format_ident!("model{}", model.id);
        quote! {
            pub mod #module_identifier;
        }
    });
    let models_fields = models.iter().map(|model| {
        let field_name = format_ident!("m{}", model.id);
        let model_module = format_ident!("model{}", model.id);
        let model_struct = format_ident!("Model{}", model.id);
        let model_doc = doc_to_ts(model.group.doc.label.as_deref().unwrap_or_default());
        quote! {
            #model_doc
            pub #field_name: crate::ModelAddr<#model_module::#model_struct>,
        }
    });
    let models_struct = quote! {
        /// This struct contains the addresses of all discovered models.
        #[derive(Debug, Default)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct Models {
            #(#models_fields)*
        }
    };
    let models_len = models.len();
    let supported_model_ids_code = models.iter().map(|model| {
        let field_name = format_ident!("m{}", model.id);
        let model_id = Literal::u16_unsuffixed(model.id);
        quote! {
            if self.#field_name.addr != 0 {
                v.push(#model_id);
            }
        }
    });
    let set_addr_code = models.iter().map(|model| {
        let field_name = format_ident!("m{}", model.id);
        let model_id = Literal::u16_unsuffixed(model.id);
        quote! {
            #model_id => self.#field_name.set_addr(addr, len),
        }
    });
    let models_impl = quote! {
        impl Models {
            /// Returns a list of all supported model ids
            pub fn supported_model_ids(&self) -> Vec<u16> {
                let mut v = Vec::with_capacity(#models_len);
                #(#supported_model_ids_code)*
                v
            }
            /// Set address and length of the given model.
            ///
            /// This method is used by the model discovery.
            pub fn set_addr(&mut self, model_id: u16, addr: u16, len: u16) -> bool {
                match model_id {
                    #(#set_addr_code)*
                    _ => { return false; }
                }
                true
            }
        }
    };
    Ok(quote! {
        #(#modules)*
        #models_struct
        #models_impl
    })
}

pub fn gen_model_struct(model: &Model) -> Result<TokenStream, GenModelError> {
    let module_doc = format!(" {}", model.group.doc.label.as_ref().unwrap());
    let model_name = format_ident!("Model{}", model.id);
    let model_doc = doc_to_ts(&model.group.doc.to_doc_string());
    let mut points = model.group.points.iter();
    if let Some(Point { name, .. }) = points.next() {
        if name != "ID" {
            return Err(GenModelError::MissingIdPoint {
                model: format!("Model{}", model.id),
                point: name.clone(),
            });
        }
    }
    if let Some(Point { name, .. }) = points.next() {
        if name != "L" {
            return Err(GenModelError::MissingLPoint {
                model: format!("Model{}", model.id),
                point: name.clone(),
            });
        }
    }
    let points = &model.group.points[2..];
    let point_fields = points
        .iter()
        .filter(|point| !point.is_padding())
        .map(|point| {
            let point_name = format_ident!("{}", snake_case(&point.name));
            let point_type = rust_type(point, "");
            let point_doc = doc_to_ts(&point.doc.to_doc_string());
            // FIXME add #[allow[missing_docs)] if the doc is empty
            quote! {
                #point_doc
                pub #point_name: #point_type,
            }
        });

    let groups = model
        .group
        .groups
        .iter()
        // XXX It's quite unfortunate but groups with a count of 0
        // aren't defined properly in the sunspec JSON files.
        .filter(|group| group.count != GroupCount::Integer(0))
        .collect::<Vec<_>>();

    let group_fields = groups.iter().map(|group| {
        let field_name = format_ident!("{}", snake_case(&group.name));
        let group_type = format_ident!("{}", upper_camel_case(&group.name));
        let group_doc = doc_to_ts(&group.doc.to_doc_string());
        quote! {
            #group_doc
            pub #field_name: Vec<#group_type>,
        }
    });

    // FIXME do not add empty model docs
    let model_struct = quote! {
        #model_doc
        #[derive(Debug)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #model_name {
            #(#point_fields)*
            #(#group_fields)*
        }
    };

    let model_impl_consts = points.iter()
        .scan(0, |offset, point| {
            let point_offset = *offset;
            *offset += point.size;
            Some((point_offset, point))
        })
        .filter(|(_, point)| !point.is_padding())
        .map(|(offset, point)| {
            let point_name = format_ident!("{}", shouty_snake_case(&point.name));
            let point_type = rust_type(point, "");
            let len = Literal::u16_unsuffixed(point.size);
            let offset = Literal::u16_unsuffixed(offset);
            let writable = point.access == PointAccess::RW;
            let code = quote! {
                pub const #point_name: crate::Point<Self, #point_type> = crate::Point::new(#offset, #len, #writable);
            };
            Some(code)
        });

    let model_impl = quote! {
        #[allow(missing_docs)]
        impl #model_name {
            #(#model_impl_consts)*
        }
    };

    let from_data_fields = points
        .iter()
        .filter(|point| !point.is_padding())
        .map(|point| {
            let field_name = format_ident!("{}", snake_case(&point.name));
            let const_name = format_ident!("{}", shouty_snake_case(&point.name));
            quote! {
                #field_name: Self::#const_name.from_data(data)?,
            }
        });

    let from_data_groups_default = groups.iter().map(|group| {
        let field_name = format_ident!("{}", snake_case(&group.name));
        quote! {
            #field_name: Vec::new(),
        }
    });

    let from_data_groups = groups.iter().map(|group| {
        let group_type = format_ident!("{}", upper_camel_case(&group.name));
        quote! {
            let data = #group_type::load(data, &mut model)?;
        }
    });

    let model_id = Literal::u16_unsuffixed(model.id);
    let model_len = Literal::u16_unsuffixed(points.iter().map(|point| point.size).sum());
    let allow_unused = points.is_empty().then(|| quote! { #[allow(unused)] });
    let m_name = format_ident!("m{}", model.id);
    let trait_impl = quote! {
        impl crate::Model for #model_name {
            const ID: u16 = #model_id;
            const LEN: u16 = #model_len;
            fn from_data(#allow_unused data: &[u16]) -> Result<Self, crate::DecodeError> {
                let mut model = Self {
                    #(#from_data_fields)*
                    #(#from_data_groups_default)*
                };
                #( #from_data_groups )*
                Ok(model)
            }
            fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
                models.#m_name
            }
        }
    };

    let mut extra = TokenStream::new();

    for point in points {
        match point.r#type {
            PointType::Enum16 | PointType::Enum32 if !point.symbols.is_empty() => {
                extra.extend(gen_enum(point, ""));
            }
            PointType::Bitfield16 | PointType::Bitfield32 | PointType::Bitfield64 => {
                extra.extend(gen_bitfield(point, ""));
            }
            _ => {}
        }
    }

    for group in groups {
        extra.extend(gen_group(model, group, ""));
    }

    Ok(quote! {
        #![doc = #module_doc]
        #model_struct
        #model_impl
        #trait_impl
        #extra
    })
}

fn gen_group(model: &Model, group: &Group, prefix: &str) -> TokenStream {
    let mut extra = TokenStream::new();
    // group structure
    let identifier = format_ident!(
        "{}{}",
        upper_camel_case(&prefix),
        upper_camel_case(&group.name)
    );
    let prefix = format!("{}{}", prefix, group.name);
    let points = group
        .points
        .iter()
        .filter(|point| !point.is_padding())
        .map(|point| {
            let point_name = format_ident!("{}", snake_case(&point.name));
            let point_type = rust_type(point, &group.name);
            let point_doc = doc_to_ts(&point.doc.to_doc_string());
            quote! {
                #point_doc
                pub #point_name: #point_type,
            }
        });
    let group_doc = doc_to_ts(&group.doc.to_doc_string());
    let group_struct = quote! {
        #group_doc
        #[derive(Debug)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #identifier {
            #(#points)*
        }
    };

    let model_type = format_ident!("Model{}", model.id);
    /*
    let GroupCount::String(count_field) = &group.count else {
        todo!(
            "Non string counts are unsupported: {} {} {:?}",
            model.group.name,
            group.name,
            group.count
        );
    };
    let count_field = format_ident!("{}", count_field.to_snake_case());
    */
    let group_impl = quote! {
        impl #identifier {
            fn load<'a>(data: &'a [u16], _model: &mut #model_type) -> Result<&'a [u16], crate::DecodeError> {
                //let length = model.#count_field;
                // FIXME implement actual loading
                Ok(data)
            }
        }
    };

    // Generate enums and bitfields
    for point in &group.points {
        match point.r#type {
            PointType::Enum16 | PointType::Enum32 if !point.symbols.is_empty() => {
                extra.extend(gen_enum(point, &prefix));
            }
            PointType::Bitfield16 | PointType::Bitfield32 | PointType::Bitfield64 => {
                extra.extend(gen_bitfield(point, &prefix));
            }
            _ => {}
        }
    }

    for group in &group.groups {
        extra.extend(gen_group(
            model,
            group,
            &format!("{}{}", &prefix, group.name),
        ));
    }

    quote! {
        #group_struct
        #group_impl
        #extra
    }
}

fn gen_enum(point: &Point, prefix: &str) -> TokenStream {
    let size = point.r#type.size().unwrap();
    let repr = format_ident!("u{}", size * 16);
    let name = format_ident!(
        "{}{}",
        upper_camel_case(&prefix),
        upper_camel_case(&point.name)
    );
    let invalid = match point.r#type {
        PointType::Enum16 => Literal::u16_unsuffixed(u16::MAX),
        PointType::Enum32 => Literal::u32_unsuffixed(u32::MAX),
        _ => unimplemented!(),
    };
    let variants = point.symbols.iter().map(|symbol| {
        let variant_name = format_ident!("{}", upper_camel_case(&symbol.name));
        let variant_value = match point.r#type {
            PointType::Enum16 => {
                Literal::u16_unsuffixed(symbol.value.as_u64().unwrap().try_into().unwrap())
            }
            PointType::Enum32 => {
                Literal::u32_unsuffixed(symbol.value.as_u64().unwrap().try_into().unwrap())
            }
            _ => unimplemented!(),
        };
        let variant_doc = doc_to_ts(&symbol.doc.to_doc_string());
        quote! {
            #variant_doc
            #variant_name = #variant_value,
        }
        .into_token_stream()
    });
    let doc = doc_to_ts(&point.doc.to_doc_string());
    quote!(
        #doc
        #[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[repr(#repr)]
        pub enum #name {
            #(#variants)*
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
    )
}

fn gen_bitfield(point: &Point, prefix: &str) -> TokenStream {
    let size = point.r#type.size().unwrap();
    let repr = format_ident!("u{}", size * 16);
    let name = format_ident!(
        "{}{}",
        upper_camel_case(&prefix),
        upper_camel_case(&point.name)
    );
    let invalid = match point.r#type {
        PointType::Bitfield16 => Literal::u16_suffixed(u16::MAX),
        PointType::Bitfield32 => Literal::u32_suffixed(u32::MAX),
        PointType::Bitfield64 => Literal::u64_suffixed(u64::MAX),
        _ => unimplemented!(),
    };
    let doc = doc_to_ts(&point.doc.to_doc_string());
    let fields = point.symbols.iter().map(|symbol| {
        let symbol_name = symbol.name.clone();
        let field_name = format_ident!("{}", upper_camel_case(&symbol_name));
        let bit = Literal::u64_unsuffixed(1 << symbol.value.as_u64().unwrap());
        let field_doc = doc_to_ts(&symbol.doc.to_doc_string());
        quote! {
            #field_doc
            const #field_name = #bit;
        }
        .into_token_stream()
    });
    quote! {
        bitflags::bitflags! {
            #doc
            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
    }
}

fn rust_type(point: &Point, prefix: &str) -> TokenStream {
    let rty = match point.r#type {
        PointType::Int16 => quote! { i16 },
        PointType::Int32 => quote! { i32 },
        PointType::Int64 => quote! { i64 },
        PointType::Raw16 => quote! { u16 },
        PointType::Uint16 => quote! { u16 },
        PointType::Uint32 => quote! { u32 },
        PointType::Uint64 => quote! { u64 },
        PointType::Acc16 => quote! { u16 },
        PointType::Acc32 => quote! { u32 },
        PointType::Acc64 => quote! { u64 },
        PointType::Bitfield16 | PointType::Bitfield32 | PointType::Bitfield64 => {
            let ident = format_ident!(
                "{}",
                format!(
                    "{}{}",
                    upper_camel_case(&prefix),
                    upper_camel_case(&point.name)
                )
            );
            quote! { #ident }
        }
        PointType::Enum16 if point.symbols.is_empty() => quote! { u16 },
        PointType::Enum32 if point.symbols.is_empty() => quote! { u32 },
        PointType::Enum16 | PointType::Enum32 => {
            let ident = format_ident!(
                "{}",
                format!(
                    "{}{}",
                    upper_camel_case(&prefix),
                    upper_camel_case(&point.name)
                )
            );
            quote! { #ident }
        }
        PointType::Float32 => quote! { f32 },
        PointType::Float64 => quote! { f64 },
        PointType::String => quote! { String },
        PointType::Sf => quote! { i16 }, // FIXME is this type correct?
        PointType::Pad => unimplemented!(),
        PointType::Ipaddr => quote! { std::net::Ipv4Addr },
        PointType::Ipv6addr => quote! { std::net::Ipv6Addr },
        PointType::Eui48 => quote! { String },
        PointType::Sunssf => quote! { i16 },
        PointType::Count => quote! { u16 },
    };
    if point.mandatory == PointMandatory::M {
        rty
    } else {
        quote! { Option<#rty> }
    }
}

fn doc_to_ts(doc: &str) -> TokenStream {
    let lines = doc
        .lines()
        .map(|line| format!(" {}", line))
        .collect::<Vec<_>>();
    if lines.is_empty() {
        quote! {
            #[allow(missing_docs)]
        }
    } else {
        quote! {
            #( #[doc = #lines] )*
        }
    }
}
