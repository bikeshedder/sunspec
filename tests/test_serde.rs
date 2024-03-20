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
