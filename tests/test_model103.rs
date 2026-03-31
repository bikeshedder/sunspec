use sunspec::{models::model103::Model103, Model};

#[test]
fn parse_model103_with_all_ones_event_bitfields() {
    let data = [
        0, 0, 0, 0, 65534, 3965, 3953, 3963, 2288, 2284, 2285, 65535, 0, 0, 4998, 65534, 0, 0, 0,
        0, 0, 0, 60, 8980, 0, 0, 0, 16, 65535, 0, 0, 32768, 3927, 32768, 32768, 65534, 2, 0, 65535,
        65535, 65535, 65535, 0, 0, 65535, 65535, 65535, 65535, 0, 0,
    ];

    let model = Model103::parse(&data).expect("model 103 should parse successfully");
    assert_eq!(model.evt1.bits(), u32::MAX);
    assert_eq!(model.evt2.bits(), u32::MAX);
}
