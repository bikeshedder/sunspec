use sunspec::{FieldKind, GroupInfo, ModelInfo, MODELS};

#[test]
fn model_info_matches_root_group() {
    for info in MODELS {
        assert_eq!(info.group.name, info.name);
        assert_eq!(info.group.label, info.label);
    }
}

fn field<'a>(group: &'a GroupInfo, name: &str) -> &'a FieldKind {
    &group
        .fields
        .iter()
        .find(|field| field.name == name)
        .unwrap_or_else(|| panic!("field {name:?} not found in group {:?}", group.name))
        .kind
}

#[cfg(feature = "model1")]
#[test]
fn points() {
    let group = ModelInfo::by_id(1).unwrap().group;
    assert!(!group.description.is_empty());
    assert!(matches!(field(group, "mn"), FieldKind::Point));
    // `ID` and `L` are not part of the generated struct
    assert!(group.fields.iter().all(|f| f.name != "id" && f.name != "l"));
}

#[cfg(feature = "model160")]
#[test]
fn repeating_group() {
    let group = ModelInfo::by_id(160).unwrap().group;
    let FieldKind::RepeatingGroup(module) = field(group, "module") else {
        panic!("module is not a repeating group");
    };
    assert_eq!(module.name, "module");
    assert!(matches!(field(module, "dc_a"), FieldKind::Point));
}
