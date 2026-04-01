#![cfg(feature = "model103")]

use sunspec::{
    models::model103::{Model103, St},
    Model,
};

#[test]
fn test_model103() {
    // 1:1 register dump from a SolarEdge SE25K device.
    // This Model 103 payload includes mandatory fields that use invalid sentinel values.
    #[rustfmt::skip]
    let data = [
        0,              // a
        0,              // aph_a
        0,              // aph_b
        0,              // aph_c
        65534,          // a_sf
        3965,           // pp_vph_ab
        3953,           // pp_vph_bc
        3963,           // pp_vph_ca
        2288,           // ph_vph_a
        2284,           // ph_vph_b
        2285,           // ph_vph_c
        65535,          // v_sf
        0,              // w
        0,              // w_sf
        4998,           // hz
        65534,          // hz_sf
        0,              // va
        0,              // va_sf
        0,              // v_ar
        0,              // v_ar_sf
        0,              // pf
        0,              // pf_sf
        60, 8980,       // wh
        0,              // wh_sf
        0,              // dca
        0,              // dca_sf
        16,             // dcv
        65535,          // dcv_sf
        0,              // dcw
        0,              // dcw_sf
        32768,          // tmp_cab
        3927,           // tmp_snk
        32768,          // tmp_trns
        32768,          // tmp_ot
        65534,          // tmp_sf
        2,              // st
        0,              // st_vnd
        65535, 65535,   // evt1
        65535, 65535,   // evt2
        0, 0,           // evt_vnd1
        65535, 65535,   // evt_vnd2
        65535, 65535,   // evt_vnd3
        0, 0,           // evt_vnd4
    ];
    let model = Model103::parse(&data).unwrap();
    assert_eq!(model.a, 0);
    assert_eq!(model.aph_a, 0);
    assert_eq!(model.aph_b, 0);
    assert_eq!(model.aph_c, 0);
    assert_eq!(model.a_sf, -2);
    assert_eq!(model.pp_vph_ab, Some(3965));
    assert_eq!(model.pp_vph_bc, Some(3953));
    assert_eq!(model.pp_vph_ca, Some(3963));
    assert_eq!(model.ph_vph_a, 2288);
    assert_eq!(model.ph_vph_b, 2284);
    assert_eq!(model.ph_vph_c, 2285);
    assert_eq!(model.v_sf, -1);
    assert_eq!(model.w, 0);
    assert_eq!(model.w_sf, 0);
    assert_eq!(model.hz, 4998);
    assert_eq!(model.hz_sf, -2);
    assert_eq!(model.va, Some(0));
    assert_eq!(model.va_sf, Some(0));
    assert_eq!(model.v_ar, Some(0));
    assert_eq!(model.v_ar_sf, Some(0));
    assert_eq!(model.pf, Some(0));
    assert_eq!(model.pf_sf, Some(0));
    assert_eq!(model.wh, 3_941_140);
    assert_eq!(model.wh_sf, 0);
    assert_eq!(model.dca, Some(0));
    assert_eq!(model.dca_sf, Some(0));
    assert_eq!(model.dcv, Some(16));
    assert_eq!(model.dcv_sf, Some(-1));
    assert_eq!(model.dcw, Some(0));
    assert_eq!(model.dcw_sf, Some(0));
    assert_eq!(model.tmp_cab, i16::MIN);
    assert_eq!(model.tmp_snk, Some(3927));
    assert_eq!(model.tmp_trns, None);
    assert_eq!(model.tmp_ot, None);
    assert_eq!(model.tmp_sf, -2);
    assert_eq!(model.st, St::Sleeping);
    assert_eq!(model.st_vnd, Some(0));
    assert_eq!(model.evt1.bits(), u32::MAX);
    assert_eq!(model.evt2.bits(), u32::MAX);
    assert_eq!(model.evt_vnd1.map(|value| value.bits()), Some(0));
    assert_eq!(model.evt_vnd2, None);
    assert_eq!(model.evt_vnd3, None);
    assert_eq!(model.evt_vnd4.map(|value| value.bits()), Some(0));
}
