//! DER DC Measurement
/// DER DC Measurement
///
/// DER DC measurement.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model714 {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    ///
    /// Comments: DC General
    pub prt_alrms: Option<PrtAlrms>,
    /// Number Of Ports
    ///
    /// Number of DC ports.
    pub n_prt: Option<u16>,
    /// DC Current
    ///
    /// Total DC current for all ports.
    pub dca: Option<i16>,
    /// DC Power
    ///
    /// Total DC power for all ports.
    pub dcw: Option<i16>,
    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    pub dc_wh_inj: Option<u64>,
    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    pub dc_wh_abs: Option<u64>,
    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    pub dca_sf: Option<i16>,
    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    pub dcv_sf: Option<i16>,
    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    pub dcw_sf: Option<i16>,
    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    pub dcwh_sf: Option<i16>,
    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    pub tmp_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model714 {
    pub const PRT_ALRMS: crate::PointDef<Self, Option<PrtAlrms>> =
        crate::PointDef::new(0, 2, false);
    pub const N_PRT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
    pub const DCA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const DCW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(4, 1, false);
    pub const DC_WH_INJ: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(5, 4, false);
    pub const DC_WH_ABS: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(9, 4, false);
    pub const DCA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(13, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(14, 1, false);
    pub const DCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(16, 1, false);
    pub const TMP_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(17, 1, false);
}
impl crate::Model for Model714 {
    const ID: u16 = 714;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            prt_alrms: Self::PRT_ALRMS.from_data(data)?,
            n_prt: Self::N_PRT.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dc_wh_inj: Self::DC_WH_INJ.from_data(data)?,
            dc_wh_abs: Self::DC_WH_ABS.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            tmp_sf: Self::TMP_SF.from_data(data)?,
        })
    }
}
bitflags::bitflags! {
    #[doc = " Port Alarms"] #[doc = " "] #[doc =
    " Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port."]
    #[doc = " "] #[doc = " Comments: DC General"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct PrtAlrms : u32 {}
}
impl crate::Value for PrtAlrms {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<PrtAlrms> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(PrtAlrms::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}
