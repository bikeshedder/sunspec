#![cfg(all(feature = "serde", feature = "model1", feature = "model2"))]

#[test]
fn test_serialize_model1() {
    let model = sunspec::models::model1::Model1 {
        mn: "manufacturer".into(),
        md: "model".into(),
        opt: None,
        vr: Some("version".into()),
        sn: "serial_number".into(),
        da: Some(42),
    };
    assert_eq!(
        serde_json::to_string(&model).unwrap(),
        r#"{"mn":"manufacturer","md":"model","opt":null,"vr":"version","sn":"serial_number","da":42}"#
    );
}

#[test]
fn test_serialize_model2() {
    let model = sunspec::models::model2::Model2 {
        aid: 0,
        n: 1,
        un: 2,
        st: sunspec::models::model2::St::Full,
        st_vnd: None,
        evt: sunspec::models::model2::Evt::ArcDetection | sunspec::models::model2::Evt::MemoryLoss,
        evt_vnd: None,
        ctl: Some(sunspec::models::model2::Ctl::Test),
        ctl_vnd: None,
        ctl_vl: None,
    };
    assert_eq!(
        serde_json::to_string(&model).unwrap(),
        r#"{"aid":0,"n":1,"un":2,"st":"Full","st_vnd":null,"evt":"MemoryLoss | ArcDetection","evt_vnd":null,"ctl":"Test","ctl_vnd":null,"ctl_vl":null}"#
    );
}

#[test]
fn test_serde_models() {
    let json = r#"[{"id":1,"addr":40004,"len":66},{"id":2,"addr":40072,"len":14}]"#;
    let models: sunspec::Models = serde_json::from_str(json).unwrap();
    let ids = models
        .iter()
        .map(|model| model.info().id)
        .collect::<Vec<_>>();
    assert_eq!(ids, [1, 2]);
    assert_eq!(serde_json::to_string(&models).unwrap(), json);

    // Unknown model id
    assert!(
        serde_json::from_str::<sunspec::Models>(r#"[{"id":0,"addr":40004,"len":66}]"#).is_err()
    );
    // Invalid address
    assert!(serde_json::from_str::<sunspec::Models>(r#"[{"id":1,"addr":0,"len":66}]"#).is_err());
}

#[test]
fn test_serde_discovery_result() {
    let json = r#"{"models":[{"id":1,"addr":40004,"len":66}],"unknown_models":[{"id":64999,"addr":40072,"len":10}]}"#;
    let discovery: sunspec::client::DiscoveryResult = serde_json::from_str(json).unwrap();
    assert_eq!(discovery.models.iter().count(), 1);
    assert_eq!(discovery.unknown_models[0].id, 64999);
    assert_eq!(serde_json::to_string(&discovery).unwrap(), json);
}

#[test]
fn test_serde_models_exceeding_address_space() {
    assert!(
        serde_json::from_str::<sunspec::Models>(r#"[{"id":1,"addr":65500,"len":66}]"#).is_err()
    );
    assert!(serde_json::from_str::<sunspec::Models>(r#"[{"id":1,"addr":65469,"len":66}]"#).is_ok());
}
