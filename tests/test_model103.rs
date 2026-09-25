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
        0,              // a_ph_a
        0,              // a_ph_b
        0,              // a_ph_c
        65534,          // a_sf
        3965,           // pp_v_ph_ab
        3953,           // pp_v_ph_bc
        3963,           // pp_v_ph_ca
        2288,           // ph_v_ph_a
        2284,           // ph_v_ph_b
        2285,           // ph_v_ph_c
        65535,          // v_sf
        0,              // w
        0,              // w_sf
        4998,           // hz
        65534,          // hz_sf
        0,              // va
        0,              // va_sf
        0,              // var
        0,              // var_sf
        0,              // pf
        0,              // pf_sf
        60, 8980,       // wh
        0,              // wh_sf
        0,              // dc_a
        0,              // dc_a_sf
        16,             // dc_v
        65535,          // dc_v_sf
        0,              // dc_w
        0,              // dc_w_sf
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
    assert_eq!(model.a_ph_a, 0);
    assert_eq!(model.a_ph_b, 0);
    assert_eq!(model.a_ph_c, 0);
    assert_eq!(model.a_sf, -2);
    assert_eq!(model.pp_v_ph_ab, Some(3965));
    assert_eq!(model.pp_v_ph_bc, Some(3953));
    assert_eq!(model.pp_v_ph_ca, Some(3963));
    assert_eq!(model.ph_v_ph_a, 2288);
    assert_eq!(model.ph_v_ph_b, 2284);
    assert_eq!(model.ph_v_ph_c, 2285);
    assert_eq!(model.v_sf, -1);
    assert_eq!(model.w, 0);
    assert_eq!(model.w_sf, 0);
    assert_eq!(model.hz, 4998);
    assert_eq!(model.hz_sf, -2);
    assert_eq!(model.va, Some(0));
    assert_eq!(model.va_sf, Some(0));
    assert_eq!(model.var, Some(0));
    assert_eq!(model.var_sf, Some(0));
    assert_eq!(model.pf, Some(0));
    assert_eq!(model.pf_sf, Some(0));
    assert_eq!(model.wh, 3_941_140);
    assert_eq!(model.wh_sf, 0);
    assert_eq!(model.dc_a, Some(0));
    assert_eq!(model.dc_a_sf, Some(0));
    assert_eq!(model.dc_v, Some(16));
    assert_eq!(model.dc_v_sf, Some(-1));
    assert_eq!(model.dc_w, Some(0));
    assert_eq!(model.dc_w_sf, Some(0));
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
