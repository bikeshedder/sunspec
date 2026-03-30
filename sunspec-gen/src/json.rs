use std::{fs, io};

use serde::{Deserialize, Serialize};

pub fn read_dir(json_dir: &str) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let dir = fs::read_dir(json_dir)?;

    let mut models = vec![];

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        if !(file_name.starts_with("model_") && file_name.ends_with(".json")) {
            continue;
        }
        let file = fs::File::open(path)?;
        let rdr = io::BufReader::new(&file);
        let model: Model = serde_json::from_reader(rdr)?;
        models.push(model);
    }

    Ok(models)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: u16, // TODO minimum 1
    pub group: Group,
    #[serde(flatten)]
    pub doc: Documentation,
}

impl Model {
    pub fn size(&self) -> u16 {
        self.group.points.iter().map(|p| p.size).sum()
    }
}

fn group_count_default() -> GroupCount {
    GroupCount::Integer(1)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub r#type: GroupType,
    #[serde(default = "group_count_default")]
    pub count: GroupCount,
    #[serde(default)]
    pub points: Vec<Point>,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(flatten)]
    pub doc: Documentation,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupType {
    Group,
    Sync,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum GroupCount {
    Integer(u32),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub name: String,
    pub r#type: PointType,
    pub value: Option<PointValue>,
    pub count: Option<u32>,
    pub size: u16,
    pub sf: Option<Sf>,
    pub units: Option<String>,
    #[serde(default)]
    pub access: PointAccess,
    #[serde(default)]
    pub mandatory: PointMandatory,
    #[serde(default)]
    pub r#static: PointStatic,
    #[serde(flatten)]
    pub doc: Documentation,
    #[serde(default)]
    pub symbols: Vec<Symbol>,
}

impl Point {
    pub fn is_padding(&self) -> bool {
        self.r#type == PointType::Pad
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PointType {
    Int16,
    Int32,
    Int64,
    Raw16,
    Uint16,
    Uint32,
    Uint64,
    Acc16,
    Acc32,
    Acc64,
    Bitfield16,
    Bitfield32,
    Bitfield64,
    Enum16,
    Enum32,
    Float32,
    Float64,
    String,
    Sf,
    Pad,
    Ipaddr,
    Ipv6addr,
    Eui48,
    Sunssf,
    Count,
}

impl PointType {
    pub fn size(&self) -> Option<u16> {
        Some(match self {
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
            PointType::Sf => 2,
            PointType::Pad => 1,
            PointType::Ipaddr => 2,
            PointType::Ipv6addr => 8,
            PointType::Eui48 => return None,
            PointType::Sunssf => 1,
            PointType::Count => 1,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PointValue {
    Integer(i64),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Sf {
    Integer(i32),
    String(String),
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq)]
pub enum PointAccess {
    #[default]
    R,
    RW,
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq)]
pub enum PointMandatory {
    M,
    #[default]
    O,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum PointStatic {
    #[default]
    D,
    S,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub name: String,
    pub value: serde_json::Value,
    #[serde(flatten)]
    pub doc: Documentation,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    pub label: Option<String>,
    pub desc: Option<String>,
    pub notes: Option<String>,
    pub detail: Option<String>,
    #[serde(default)]
    pub comments: Vec<String>,
}

impl Documentation {
    pub fn to_doc_string(&self) -> String {
        let comments = (!self.comments.is_empty()).then_some(self.comments.join(", "));
        let parts = [
            ("", &self.label),
            ("", &self.desc),
            ("Notes: ", &self.notes),
            ("Detail: ", &self.detail),
            ("Comments: ", &comments),
        ];
        let parts = parts
            .iter()
            .filter_map(|(label, text)| {
                text.as_ref().and_then(|text| {
                    let text = text.trim();
                    if text.is_empty() {
                        None
                    } else {
                        Some((label, text))
                    }
                })
            })
            .map(|(label, text)| format!("{}{}", label, text))
            .collect::<Vec<_>>();
        parts.join("\n\n")
    }
}
