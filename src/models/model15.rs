//! Interface Counters Model

/// Interface Counters Model
///
/// Interface counters
#[derive(Debug)]
pub struct Model15 {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    pub clr: Option<u16>,
    /// Input Count
    ///
    /// Number of bytes received
    pub in_cnt: Option<u32>,
    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    pub in_uc_cnt: Option<u32>,
    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    pub in_n_uc_cnt: Option<u32>,
    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    pub in_dsc_cnt: Option<u32>,
    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    pub in_err_cnt: Option<u32>,
    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    pub in_unk_cnt: Option<u32>,
    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    pub out_cnt: Option<u32>,
    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    pub out_uc_cnt: Option<u32>,
    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    pub out_n_uc_cnt: Option<u32>,
    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    pub out_dsc_cnt: Option<u32>,
    /// Output Error Count
    ///
    /// Number of outbound error packets
    pub out_err_cnt: Option<u32>,
}

#[allow(missing_docs)]

impl Model15 {
    pub const CLR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const IN_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(1, 2, false);
    pub const IN_UC_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(3, 2, false);
    pub const IN_N_UC_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(5, 2, false);
    pub const IN_DSC_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, false);
    pub const IN_ERR_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(9, 2, false);
    pub const IN_UNK_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, false);
    pub const OUT_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(13, 2, false);
    pub const OUT_UC_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(15, 2, false);
    pub const OUT_N_UC_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(17, 2, false);
    pub const OUT_DSC_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(19, 2, false);
    pub const OUT_ERR_CNT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(21, 2, false);
}

impl crate::Model for Model15 {
    const ID: u16 = 15;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            clr: Self::CLR.from_data(data)?,
            in_cnt: Self::IN_CNT.from_data(data)?,
            in_uc_cnt: Self::IN_UC_CNT.from_data(data)?,
            in_n_uc_cnt: Self::IN_N_UC_CNT.from_data(data)?,
            in_dsc_cnt: Self::IN_DSC_CNT.from_data(data)?,
            in_err_cnt: Self::IN_ERR_CNT.from_data(data)?,
            in_unk_cnt: Self::IN_UNK_CNT.from_data(data)?,
            out_cnt: Self::OUT_CNT.from_data(data)?,
            out_uc_cnt: Self::OUT_UC_CNT.from_data(data)?,
            out_n_uc_cnt: Self::OUT_N_UC_CNT.from_data(data)?,
            out_dsc_cnt: Self::OUT_DSC_CNT.from_data(data)?,
            out_err_cnt: Self::OUT_ERR_CNT.from_data(data)?,
        })
    }
}
