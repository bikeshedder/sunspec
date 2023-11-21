/// Inverter (Single Phase) FLOAT
///
/// Include this model for single phase inverter monitoring using float values
#[derive(Debug)]
pub struct Model111 {
    /// Amps
    ///
    /// AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub aph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aph_b: Option<f32>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aph_c: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub pp_vph_ab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub pp_vph_bc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub pp_vph_ca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_vph_a: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_vph_b: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_vph_c: Option<f32>,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub v_ar: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmp_cab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmp_snk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmp_trns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmp_ot: Option<f32>,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub st_vnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evt_vnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evt_vnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evt_vnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evt_vnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model111 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APH_A: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APH_B: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(4, 2, false);
    pub const APH_C: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(6, 2, false);
    pub const PP_VPH_AB: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(8, 2, false);
    pub const PP_VPH_BC: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(10, 2, false);
    pub const PP_VPH_CA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(12, 2, false);
    pub const PH_VPH_A: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PH_VPH_B: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(16, 2, false);
    pub const PH_VPH_C: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(18, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const VA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(24, 2, false);
    pub const V_AR: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(26, 2, false);
    pub const PF: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(28, 2, false);
    pub const WH: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const DCA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(32, 2, false);
    pub const DCV: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(34, 2, false);
    pub const DCW: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(36, 2, false);
    pub const TMP_CAB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const TMP_SNK: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(40, 2, false);
    pub const TMP_TRNS: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(42, 2, false);
    pub const TMP_OT: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(44, 2, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const ST_VND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(47, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const EVT_VND1: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(52, 2, false);
    pub const EVT_VND2: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(54, 2, false);
    pub const EVT_VND3: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(56, 2, false);
    pub const EVT_VND4: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(58, 2, false);
}

impl crate::Model for Model111 {
    const ID: u16 = 111;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            aph_a: Self::APH_A.from_data(data)?,
            aph_b: Self::APH_B.from_data(data)?,
            aph_c: Self::APH_C.from_data(data)?,
            pp_vph_ab: Self::PP_VPH_AB.from_data(data)?,
            pp_vph_bc: Self::PP_VPH_BC.from_data(data)?,
            pp_vph_ca: Self::PP_VPH_CA.from_data(data)?,
            ph_vph_a: Self::PH_VPH_A.from_data(data)?,
            ph_vph_b: Self::PH_VPH_B.from_data(data)?,
            ph_vph_c: Self::PH_VPH_C.from_data(data)?,
            w: Self::W.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            va: Self::VA.from_data(data)?,
            v_ar: Self::V_AR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            wh: Self::WH.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            tmp_cab: Self::TMP_CAB.from_data(data)?,
            tmp_snk: Self::TMP_SNK.from_data(data)?,
            tmp_trns: Self::TMP_TRNS.from_data(data)?,
            tmp_ot: Self::TMP_OT.from_data(data)?,
            st: Self::ST.from_data(data)?,
            st_vnd: Self::ST_VND.from_data(data)?,
            evt1: Self::EVT1.from_data(data)?,
            evt2: Self::EVT2.from_data(data)?,
            evt_vnd1: Self::EVT_VND1.from_data(data)?,
            evt_vnd2: Self::EVT_VND2.from_data(data)?,
            evt_vnd3: Self::EVT_VND3.from_data(data)?,
            evt_vnd4: Self::EVT_VND4.from_data(data)?,
        })
    }
}
