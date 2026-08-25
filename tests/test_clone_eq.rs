//! Regression tests ensuring that parsed models can be cloned and compared.
//!
//! The exhaustive compile time checks for all models live in the generated
//! `sunspec::models` module. These tests additionally verify the runtime
//! behavior of `Clone` and `PartialEq` on actual model data.

#[cfg(feature = "model711")]
#[test]
fn test_model711_clone_and_eq() {
    use sunspec::{models::model711, Model};

    #[rustfmt::skip]
    let data = &[
        1,              // ena
        0,              // adpt_ctl_req
        1,              // adpt_ctl_rslt
        2,              // n_ctl
        65535, 65535,   // rvrt_tms
        65535, 65535,   // rvrt_rem
        65535,          // rvrt_ctl
        65534,          // db_sf
        65534,          // k_sf
        65534,          // rsp_tms_sf
        0, 300, 0, 300, 200, 200, 0, 50, 65535, 1, // ctl[0]
        0, 200, 0, 200, 100, 100, 0, 10, 0,     0, // ctl[1]
    ];

    let model = model711::Model711::parse(data).unwrap();
    let clone = model.clone();
    assert_eq!(model, clone);

    // The nested group struct must be cloneable and comparable as well.
    let ctl: model711::Ctl = model.ctl[0].clone();
    assert_eq!(model.ctl[0], ctl);
    assert_ne!(model.ctl[0], model.ctl[1]);

    let mut modified = model.clone();
    modified.ctl[0].k_of += 1;
    assert_ne!(model, modified);
}

#[cfg(feature = "model103")]
#[test]
fn test_model103_clone_and_partial_eq() {
    use sunspec::{models::model103, Model};

    #[rustfmt::skip]
    let data = &[
        1000,   // a
        333,    // aph_a
        333,    // aph_b
        334,    // aph_c
        65535,  // a_sf
        4000,   // pp_vph_ab
        4000,   // pp_vph_bc
        4000,   // pp_vph_ca
        2300,   // ph_vph_a
        2300,   // ph_vph_b
        2300,   // ph_vph_c
        65535,  // v_sf
        5000,   // w
        0,      // w_sf
        5000,   // hz
        65534,  // hz_sf
        5000,   // va
        0,      // va_sf
        0,      // var
        0,      // var_sf
        1000,   // pf
        65533,  // pf_sf
        0, 100, // wh
        0,      // wh_sf
        0,      // dca
        0,      // dca_sf
        0,      // dcv
        0,      // dcv_sf
        0,      // dcw
        0,      // dcw_sf
        250,    // tmp_cab
        65535,  // tmp_snk
        65535,  // tmp_trns
        65535,  // tmp_ot
        65535,  // tmp_sf
        4,      // st
        65535,  // st_vnd
        0, 0,   // evt1
        0, 0,   // evt2
        0, 0,   // evt_vnd1
        0, 0,   // evt_vnd2
        0, 0,   // evt_vnd3
        0, 0,   // evt_vnd4
    ];

    // Model 103 uses scale factors and therefore integer point types, but the
    // test also guards against accidental loss of `Clone`/`PartialEq`.
    let model = model103::Model103::parse(data).unwrap();
    let clone = model.clone();
    assert_eq!(model, clone);

    let mut modified = model.clone();
    modified.w += 1;
    assert_ne!(model, modified);
}
