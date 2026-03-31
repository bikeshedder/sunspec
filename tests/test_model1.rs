use sunspec::{models::model1::Model1, Model};

#[test]
fn parse_model1_without_trailing_pad_register() {
    let data = [
        21359, 27745, 29253, 25703, 25888, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21317, 12853, 19245,
        21079, 12336, 18754, 20045, 13312, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12336,
        12340, 11824, 12338, 13102, 12341, 12857, 0, 14146, 12357, 17989, 13382, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1,
    ];

    let model = Model1::parse(&data).expect("model 1 should parse without the trailing pad");
    assert_eq!(model.da, Some(1));
}
