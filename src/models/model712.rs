/// DER Watt-Var
///
/// DER Watt-Var model.
#[derive(Debug)]
pub struct Model712 {
    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    pub ena: u16,
    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    pub adptcrvreq: u16,
    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    pub adptcrvrslt: u16,
    /// Number Of Points
    ///
    /// Number of curve points supported.
    pub npt: u16,
    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    pub ncrv: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds.  0 = No reversion time.
    pub rvrttms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    pub rvrtrem: Option<u32>,
    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    pub rvrtcrv: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    pub w_sf: i16,
    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    pub deptref_sf: i16,
}

#[allow(missing_docs)]

impl Model712 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPTCRVREQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPTCRVRSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const RVRTTMS: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, true);
    pub const RVRTREM: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const RVRTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const DEPTREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
}

impl crate::Model for Model712 {
    const ID: u16 = 712;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            adptcrvreq: Self::ADPTCRVREQ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            adptcrvrslt: Self::ADPTCRVRSLT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rvrtrem: Self::RVRTREM.from_data(data)?,
            rvrtcrv: Self::RVRTCRV.from_data(data)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            deptref_sf: Self::DEPTREF_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
