use std::{fs, io};

use serde::{Deserialize, Serialize};

pub fn read_dir(json_dir: &str) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let dir = fs::read_dir(json_dir)?;

    let mut models = vec![];

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        println!("{path:?}");
        let file_name = path.file_name().unwrap().to_string_lossy();
        if !(file_name.starts_with("model_") && file_name.ends_with(".json")) {
            continue
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
    pub label: Option<String>,
    pub desc: Option<String>,
    pub detail: Option<String>,
    pub notes: Option<String>,
    pub comments: Option<Vec<String>>,
}

impl Model {
    pub fn size(&self) -> u32 {
        self.group.points.iter()
            .map(|p| p.size())
            .sum()
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
    pub label: Option<String>,
    pub desc: Option<String>,
    pub detail: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub comments: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupType {
    Group,
    Sync,
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub size: u32,
    pub sf: Option<Sf>,
    pub units: Option<String>,
    #[serde(default)]
    pub access: PointAccess,
    #[serde(default)]
    pub mandatory: PointMandatory,
    #[serde(default)]
    pub r#static: PointStatic,
    pub label: Option<String>,
    pub desc: Option<String>,
    pub detail: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub comments: Vec<String>,
    #[serde(default)]
    pub symbols: Vec<Symbol>,
}

impl Point {
    fn size(&self) -> u32 {
        self.size
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub fn size(&self) -> Option<u32> {
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

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum PointAccess {
    #[default]
    R,
    RW,
}

#[derive(Debug, Deserialize, Serialize, Default)]
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
    pub label: Option<String>,
    pub desc: Option<String>,
    pub detail: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub comments: Vec<String>,
}
