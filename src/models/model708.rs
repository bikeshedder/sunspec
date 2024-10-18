//! DER Trip HV
/// DER Trip HV
///
/// DER high voltage trip model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model708 {
    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    pub ena: Ena,
    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    pub adpt_crv_req: u16,
    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    pub adpt_crv_rslt: AdptCrvRslt,
    /// Number Of Points
    ///
    /// Number of curve points supported.
    pub n_pt: u16,
    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    pub n_crv_set: u16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    pub v_sf: i16,
    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    pub tms_sf: i16,
}
#[allow(missing_docs)]
impl Model708 {
    pub const ENA: crate::Point<Self, Ena> = crate::Point::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::Point<Self, AdptCrvRslt> = crate::Point::new(2, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const N_CRV_SET: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
}
impl crate::Model for Model708 {
    const ID: u16 = 708;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adpt_crv_req: Self::ADPT_CRV_REQ.from_data(data)?,
            adpt_crv_rslt: Self::ADPT_CRV_RSLT.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            n_crv_set: Self::N_CRV_SET.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m708
    }
}
/// DER Trip HV Module Enable
///
/// DER high voltage trip control enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Ena {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for Ena {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Ena> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Ena::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
/// Adopt Curve Result
///
/// Result of last adopt curve operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum AdptCrvRslt {
    /// Update In Progress
    ///
    /// Curve update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Curve update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Curve update failed.
    Failed = 2,
}
impl crate::Value for AdptCrvRslt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<AdptCrvRslt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                AdptCrvRslt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
