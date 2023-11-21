/// DER DC Measurement
///
/// DER DC measurement.
#[derive(Debug)]
pub struct Model714 {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    ///
    /// Comments: DC General
    pub prtalrms: Option<u32>,
    /// Number Of Ports
    ///
    /// Number of DC ports.
    pub nprt: Option<u16>,
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
    pub dcwhinj: Option<u64>,
    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    pub dcwhabs: Option<u64>,
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
    pub const PRTALRMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(0, 2, false);
    pub const NPRT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
    pub const DCA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const DCW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(4, 1, false);
    pub const DCWHINJ: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(5, 4, false);
    pub const DCWHABS: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(9, 4, false);
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
            prtalrms: Self::PRTALRMS.from_data(data)?,
            nprt: Self::NPRT.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcwhinj: Self::DCWHINJ.from_data(data)?,
            dcwhabs: Self::DCWHABS.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            tmp_sf: Self::TMP_SF.from_data(data)?,
        })
    }
}
