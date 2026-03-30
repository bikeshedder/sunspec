use sunspec::{
    models::model712::{self, Model712},
    Model,
};

#[test]
fn test_model712() {
    #[rustfmt::skip]
    let data = &[
        0,              // Ena
        0,              // adpt_crv_req
        1,              // adpt_crv_rslt
        6,              // n_pt
        2,              // n_crv
        65535, 65535,   // rvrt_tms
        65535, 65535,   // rvrt_rem
        65535,          // rvrt_crv
        65534,          // w_sf
        65534,          // dept_ref_sf
        6, 0, 65535, 1, // crv[0]
        0, 0,           // crv[0].pt[0]
        5000, 0,        // crv[0].pt[1]
        6000, 0,        // crv[0].pt[2]
        8000, 0,        // crv[0].pt[3]
        9000, 0,        // crv[0].pt[4]
        10000, 0,       // crv[0].pt[5]
        0, 0, 65535, 0, // crv[1]
        0, 0,           // crv[1].pt[0]
        0, 0,           // crv[1].pt[1]
        0, 0,           // crv[1].pt[2]
        0, 0,           // crv[1].pt[3]
        0, 0,           // crv[1].pt[4]
        0, 0,           // crv[1].pt[5]
    ];
    let m = Model712::parse(data).unwrap();
    assert_eq!(m.ena, model712::Ena::Disabled);
    assert_eq!(m.adpt_crv_req, 0);
    assert_eq!(m.adpt_crv_rslt, model712::AdptCrvRslt::Completed);
    assert_eq!(m.n_pt, 6);
    assert_eq!(m.n_crv, 2);
    assert_eq!(m.rvrt_tms, None);
    assert_eq!(m.rvrt_rem, None);
    assert_eq!(m.rvrt_crv, None);
    assert_eq!(m.w_sf, -2);
    assert_eq!(m.dept_ref_sf, -2);
    assert_eq!(m.crv.len(), 2);
    assert_eq!(m.crv[0].act_pt, 6);
    assert_eq!(m.crv[0].dept_ref, model712::CrvDeptRef::WMaxPct);
    assert_eq!(m.crv[0].pri, None);
    assert_eq!(m.crv[0].read_only, model712::CrvReadOnly::R);
    assert_eq!(m.crv[0].pt.len(), 6);
    assert_eq!(m.crv[0].pt[0].w, Some(0));
    assert_eq!(m.crv[0].pt[0].var, Some(0));
    assert_eq!(m.crv[0].pt[1].w, Some(5000));
    assert_eq!(m.crv[0].pt[1].var, Some(0));
    assert_eq!(m.crv[0].pt[2].w, Some(6000));
    assert_eq!(m.crv[0].pt[2].var, Some(0));
    assert_eq!(m.crv[0].pt[3].w, Some(8000));
    assert_eq!(m.crv[0].pt[3].var, Some(0));
    assert_eq!(m.crv[0].pt[4].w, Some(9000));
    assert_eq!(m.crv[0].pt[4].var, Some(0));
    assert_eq!(m.crv[0].pt[5].w, Some(10000));
    assert_eq!(m.crv[0].pt[5].var, Some(0));
    assert_eq!(m.crv[1].act_pt, 0);
    assert_eq!(m.crv[1].dept_ref, model712::CrvDeptRef::WMaxPct);
    assert_eq!(m.crv[1].pri, None);
    assert_eq!(m.crv[1].read_only, model712::CrvReadOnly::Rw);
    assert_eq!(m.crv[1].pt.len(), 6);
    for pt in &m.crv[1].pt {
        assert_eq!(pt.w, Some(0));
        assert_eq!(pt.var, Some(0));
    }
}
