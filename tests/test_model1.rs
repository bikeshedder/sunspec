#![cfg(feature = "model1")]

use sunspec::{models::model1::Model1, Model};

#[test]
fn test_model1() {
    // 1:1 register dump from a SolarEdge SE25K device.
    // This Model 1 payload omits the final pad register.
    #[rustfmt::skip]
    let data = [
        16707, 19781, 8275, 20300, 16722, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,      // mn
        19791, 17477, 19488, 23128, 11571, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,     // md
        0, 0, 0, 0, 0, 0, 0, 0,                                                 // opt
        30256, 11824, 11636, 25971, 29696, 0, 0, 0,                             // vr
        21326, 11604, 17747, 21549, 12336, 12337, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // sn
        1,                                                                      // da
    ];

    let model = Model1::parse(&data).unwrap();
    assert_eq!(model.mn, "ACME SOLAR");
    assert_eq!(model.md, "MODEL ZX-3");
    assert_eq!(model.opt, None);
    assert_eq!(model.vr.as_deref(), Some("v0.0-test"));
    assert_eq!(model.sn, "SN-TEST-0001");
    assert_eq!(model.da, Some(1));
}
