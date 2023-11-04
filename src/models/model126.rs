/// Static Volt-VAR
///
/// Static Volt-VAR Arrays
///
/// Notes: Ref 3: 8.8.1.2
#[derive(Debug)]
pub struct Model126 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// Is Volt-VAR control active.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for volt-VAR change.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    /// DeptRef_SF
    ///
    /// scale factor for dependent variable.
    pub deptref_sf: i16,
    #[allow(missing_docs)]
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model126 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const DEPTREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model126 {
    const ID: u16 = 126;
    const LENGTH: u16 = 64;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            deptref_sf: Self::DEPTREF_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}
