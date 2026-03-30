//! DER Trip HF
/// Type alias for [`DerTripHf`].
pub type Model710 = DerTripHf;
struct Counts {
    n_pt: u16,
    n_crv_set: u16,
}
/// DER Trip HF
///
/// DER high frequency trip model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerTripHf {
    /// DER Trip HF Module Enable
    ///
    /// DER high frequency trip control enable.
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
    /// Frequency Scale Factor
    ///
    /// Scale factor for curve frequency points.
    pub hz_sf: i16,
    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    pub tms_sf: i16,
    /// Stored Curves
    ///
    /// Stored curve sets.
    ///
    /// Comments: Stored curve sets - Number of curve sets contained in NCrvSet - The first set is read-only and indicates the current settings.
    pub crv: Vec<Crv>,
}
#[allow(missing_docs)]
impl DerTripHf {
    pub const ENA: crate::Point<Self, Ena> = crate::Point::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::Point<Self, AdptCrvRslt> = crate::Point::new(2, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const N_CRV_SET: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const HZ_SF: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
}
impl crate::Group for DerTripHf {
    const LEN: u16 = 7;
}
impl DerTripHf {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let counts = Counts {
            n_pt: Self::N_PT.from_data(data)?,
            n_crv_set: Self::N_CRV_SET.from_data(data)?,
        };
        let (nested_data, crv) = Crv::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                ena: Self::ENA.from_data(data)?,
                adpt_crv_req: Self::ADPT_CRV_REQ.from_data(data)?,
                adpt_crv_rslt: Self::ADPT_CRV_RSLT.from_data(data)?,
                n_pt: Self::N_PT.from_data(data)?,
                n_crv_set: Self::N_CRV_SET.from_data(data)?,
                hz_sf: Self::HZ_SF.from_data(data)?,
                tms_sf: Self::TMS_SF.from_data(data)?,
                crv,
            },
        ))
    }
}
/// DER Trip HF Module Enable
///
/// DER high frequency trip control enable.
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
/// Stored Curves
///
/// Stored curve sets.
///
/// Comments: Stored curve sets - Number of curve sets contained in NCrvSet - The first set is read-only and indicates the current settings.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Crv {
    /// Curve Access
    ///
    /// Curve read-write access.
    pub read_only: CrvReadOnly,
    /// Must Trip Curve
    ///
    /// Stored must trip curve.
    ///
    /// Comments: Stored curve set containing a Must Trip, May Trip, and Momentary Cessation Curve - Number of curve points contained in NPt
    pub must_trip: MustTrip,
    /// May Trip Curve
    ///
    /// Stored may trip curve.
    pub may_trip: MayTrip,
    /// Momentary Cessation Curve
    ///
    /// Stored momentary cessation curve.
    pub mom_cess: MomCess,
}
#[allow(missing_docs)]
impl Crv {
    pub const READ_ONLY: crate::Point<Self, CrvReadOnly> = crate::Point::new(0, 1, false);
}
impl crate::Group for Crv {
    const LEN: u16 = 1;
}
impl Crv {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, must_trip) = MustTrip::parse_group(nested_data, counts)?;
        let (nested_data, may_trip) = MayTrip::parse_group(nested_data, counts)?;
        let (nested_data, mom_cess) = MomCess::parse_group(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                read_only: Self::READ_ONLY.from_data(data)?,
                must_trip,
                may_trip,
                mom_cess,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_crv_set).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Crv::parse_group(data, counts)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// Curve Access
///
/// Curve read-write access.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CrvReadOnly {
    /// Read-Write Access
    ///
    /// Curve has read-write access.
    Rw = 0,
    /// Read-Only Access
    ///
    /// Curve has read-only access.
    R = 1,
}
impl crate::Value for CrvReadOnly {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CrvReadOnly> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CrvReadOnly::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Must Trip Curve
///
/// Stored must trip curve.
///
/// Comments: Stored curve set containing a Must Trip, May Trip, and Momentary Cessation Curve - Number of curve points contained in NPt
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MustTrip {
    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    pub act_pt: Option<u16>,
    /// Must Trip Curve Points
    ///
    /// Must trip curve points.
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl MustTrip {
    pub const ACT_PT: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
}
impl crate::Group for MustTrip {
    const LEN: u16 = 1;
}
impl MustTrip {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, pt) = Pt::parse_multiple(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                pt,
            },
        ))
    }
}
/// Must Trip Curve Points
///
/// Must trip curve points.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pt {
    /// Frequency Point
    ///
    /// Curve frequency point.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    pub hz: Option<u32>,
    /// Time Point
    ///
    /// Curve time point in seconds.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    pub tms: Option<u32>,
}
#[allow(missing_docs)]
impl Pt {
    pub const HZ: crate::Point<Self, Option<u32>> = crate::Point::new(0, 2, true);
    pub const TMS: crate::Point<Self, Option<u32>> = crate::Point::new(2, 2, true);
}
impl crate::Group for Pt {
    const LEN: u16 = 4;
}
impl Pt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                hz: Self::HZ.from_data(data)?,
                tms: Self::TMS.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_pt).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Pt::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// May Trip Curve
///
/// Stored may trip curve.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MayTrip {
    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    pub act_pt: Option<u16>,
    /// May Trip Curve Points
    ///
    /// May trip curve points.
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl MayTrip {
    pub const ACT_PT: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
}
impl crate::Group for MayTrip {
    const LEN: u16 = 1;
}
impl MayTrip {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, pt) = Pt::parse_multiple(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                pt,
            },
        ))
    }
}
/// Momentary Cessation Curve
///
/// Stored momentary cessation curve.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MomCess {
    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    pub act_pt: Option<u16>,
    /// Mom Cessation Curve Points
    ///
    /// Momentary cessation curve points.
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl MomCess {
    pub const ACT_PT: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
}
impl crate::Group for MomCess {
    const LEN: u16 = 1;
}
impl MomCess {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, pt) = Pt::parse_multiple(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                pt,
            },
        ))
    }
}
impl crate::Model for DerTripHf {
    const ID: u16 = 710;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m710
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
