use std::{fs, io};

use serde::{Deserialize, Serialize};

fn default_version() -> String {
    "1".to_owned()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    #[serde(default = "default_version")]
    pub v: String,
    #[serde(rename = "$value")]
    pub content: Vec<ModelOrStrings>,
}

impl Document {
    pub fn into_model_groups(self) -> Vec<ModelGroup> {
        let mut group: Option<ModelGroup> = None;
        let mut groups: Vec<ModelGroup> = vec![];
        for x in self.content.into_iter() {
            match x {
                ModelOrStrings::Model(model) => {
                    if let Some(group) = group.take() {
                        groups.push(group);
                    }
                    group = Some(ModelGroup {
                        model,
                        strings: vec![],
                    })
                }
                ModelOrStrings::Strings(strings) => {
                    if let Some(group) = &mut group {
                        group.strings.push(strings);
                    }
                }
            }
        }
        if let Some(group) = group.take() {
            groups.push(group);
        }
        groups
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelOrStrings {
    Model(Model),
    Strings(Strings),
}

// This structure can't be deserialized directly using serde due to the
// lack of proper xsd:sequence support.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModelGroup {
    pub model: Model,
    pub strings: Vec<Strings>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    #[serde(rename = "$value")]
    pub blocks: Vec<Block>, // max len 2
    pub id: u16,
    pub len: Option<u16>,
    pub name: Option<String>,
    #[serde(default)]
    pub status: LifecycleStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub len: u16,
    #[serde(default)]
    pub r#type: BlockType,
    pub name: Option<String>,
    #[serde(rename = "point")]
    #[serde(default)]
    pub points: Vec<Point>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    #[serde(rename = "symbol")]
    #[serde(default)]
    pub symbol: Vec<Symbol>,
    pub id: String,
    pub len: Option<u16>,
    pub offset: Option<u16>,
    pub r#type: PointType,
    pub sf: Option<String>,
    pub units: Option<String>,
    #[serde(default)]
    pub access: PointAccess,
    #[serde(default = "default_mandatory")]
    pub mandatory: bool,
    #[serde(default)]
    pub category: Category,
}

fn default_mandatory() -> bool {
    false
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Strings {
    pub model: Option<StringsModel>,
    #[serde(rename = "point")]
    #[serde(default)]
    pub points: Vec<StringsPoint>,
    pub id: u16,
    pub locale: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StringsModel {
    pub label: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StringsPoint {
    pub label: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    #[serde(rename = "symbol")]
    pub symbols: Vec<StringsSymbol>,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StringsSymbol {
    pub label: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PointType {
    Int16,
    Uint16,
    Count,
    Acc16,
    Int32,
    Uint32,
    Float32,
    Acc32,
    Int64,
    Uint64,
    Float64,
    Acc64,
    Enum16,
    Enum32,
    Bitfield16,
    Bitfield32,
    Sunssf,
    String,
    Pad,
    Ipaddr,
    Ipv6addr,
    Eui48,
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PointAccess {
    RW,
    #[default]
    R,
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    None,
    #[default]
    Measurement,
    Metered,
    Status,
    Event,
    Setting,
    Control,
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BlockType {
    #[default]
    Fixed,
    Repeating,
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LifecycleStatus {
    Draft,
    Test,
    #[default]
    Approved,
    Superseded,
}

pub fn read_dir(smdx_dir: &str) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
    let dir = fs::read_dir(smdx_dir)?;
    let mut docs = vec![];
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        if !(file_name.starts_with("smdx_") && file_name.ends_with(".xml")) {
            continue;
        }
        let file = fs::File::open(path)?;
        let rdr = io::BufReader::new(&file);
        let doc: Document = serde_xml_rs::from_reader(rdr)?;
        docs.push(doc);
    }
    Ok(docs)
}
