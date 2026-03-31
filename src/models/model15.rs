//! Interface Counters Model
/// Interface Counters Model
///
/// Interface counters
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
    pub const CLR: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const IN_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(1, 2, false);
    pub const IN_UC_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(3, 2, false);
    pub const IN_N_UC_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(5, 2, false);
    pub const IN_DSC_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(7, 2, false);
    pub const IN_ERR_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(9, 2, false);
    pub const IN_UNK_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(11, 2, false);
    pub const OUT_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(13, 2, false);
    pub const OUT_UC_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(15, 2, false);
    pub const OUT_N_UC_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(17, 2, false);
    pub const OUT_DSC_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(19, 2, false);
    pub const OUT_ERR_CNT: crate::Point<Self, Option<u32>> = crate::Point::new(21, 2, false);
}
static MODEL15_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "clr",
        label: "Clear",
        description: "Write a \"1\" to clear all counters",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_cnt",
        label: "Input Count",
        description: "Number of bytes received",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_uc_cnt",
        label: "Input Unicast Count",
        description: "Number of Unicast packets received",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_n_uc_cnt",
        label: "Input Non-Unicast Count",
        description: "Number of non-Unicast packets received",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dsc_cnt",
        label: "Input Discarded Count",
        description: "Number of inbound packets received on the interface but discarded",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_err_cnt",
        label: "Input Error Count",
        description: "Number of inbound packets that contain errors (excluding discards)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_unk_cnt",
        label: "Input Unknown Count",
        description: "Number of inbound packets with unknown protocol",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "out_cnt",
        label: "Output Count",
        description: "Total number of bytes transmitted on this interface",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "out_uc_cnt",
        label: "Output Unicast Count",
        description: "Number of Unicast packets transmitted",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "out_n_uc_cnt",
        label: "Output Non-Unicast Count",
        description: "Number of Non-Unicast packets transmitted",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "out_dsc_cnt",
        label: "Output Discarded Count",
        description: "Number of Discarded output packets",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "out_err_cnt",
        label: "Output Error Count",
        description: "Number of outbound error packets",
        kind: crate::FieldKind::Point,
    },
];
static MODEL15_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_15",
    label: "Interface Counters Model",
    description: "Interface counters",
    fields: MODEL15_FIELDS,
};
impl crate::GroupMeta for Model15 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL15_GROUP_INFO
    }
}
impl crate::Group for Model15 {
    const LEN: u16 = 24;
}
impl Model15 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
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
            },
        ))
    }
}
impl crate::Model for Model15 {
    const ID: u16 = 15;
    const NAME: &'static str = "model_15";
    const LABEL: &'static str = "Interface Counters Model";
    const DESCRIPTION: &'static str = "Interface counters";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m15
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
