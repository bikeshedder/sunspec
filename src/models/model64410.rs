//! DC Simulator Control Interface
pub type Model64410 = DcSimInterface;
/// DC Simulator Control Interface
///
/// A generic DC simulator/power supply control interface for DER electrical testing.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DcSimInterface {
    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    pub v_max_lim: Option<u16>,
    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    pub p_max_lim: Option<u16>,
    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    pub i_max_lim: Option<u16>,
    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    pub mode: Option<Mode>,
    /// Power On/Off
    ///
    /// Power On/Off
    pub ena: Option<Ena>,
    /// Reset Device
    ///
    /// Reset Device
    pub reset: Option<Reset>,
    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    pub v_set: Option<u16>,
    /// Power Setpoint
    ///
    /// Power Setpoint
    pub p_set: Option<u16>,
    /// Current Setpoint
    ///
    /// Current Setpoint
    pub i_set: Option<u16>,
    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    pub en50530: Option<En50530>,
    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    pub vmpp: Option<u16>,
    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    pub pmpp: Option<u16>,
    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    pub g_set: Option<u16>,
    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    pub v_slew_rate: Option<u16>,
    /// Power Slew Rate
    ///
    /// Power Slew Rate
    pub p_slew_rate: Option<u16>,
    /// Current Slew Rate
    ///
    /// Current Slew Rate
    pub i_slew_rate: Option<u16>,
    /// Enable Profile
    ///
    /// Start/Stop the Profile
    pub ena_prof: Option<EnaProf>,
    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    pub adpt_prof_req: Option<u16>,
    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    pub adpt_prof_rslt: AdptProfRslt,
    /// Measured Voltage
    ///
    /// Measured Voltage
    pub v: Option<i32>,
    /// Measured Power
    ///
    /// Measured Power
    pub p: Option<i32>,
    /// Measured Current
    ///
    /// Measured Current
    pub i: Option<i32>,
    /// Errors
    ///
    /// Error States
    pub errors: Option<String>,
    /// Number Of Points
    ///
    /// Number of profile points supported.
    pub n_pt: u16,
    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    pub n_prof: u16,
    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    pub w_sf: i16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    pub v_sf: i16,
    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    pub a_sf: i16,
    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    pub g_sf: i16,
    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    pub tms_sf: i16,
    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    pub v_slew_sf: i16,
    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    pub p_slew_sf: i16,
    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    pub i_slew_sf: i16,
    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    pub pct_sf: i16,
    /// Stored Profiles
    ///
    /// Stored profile sets.
    ///
    /// Comments: Stored Pofile Sets - Number of profile sets = NProf - The first set is read-only and indicates the current settings.
    pub prof: Vec<Prof>,
}
#[allow(missing_docs)]
impl DcSimInterface {
    pub const V_MAX_LIM: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const P_MAX_LIM: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const I_MAX_LIM: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const MODE: crate::Point<Self, Option<Mode>> = crate::Point::new(3, 1, true);
    pub const ENA: crate::Point<Self, Option<Ena>> = crate::Point::new(4, 1, true);
    pub const RESET: crate::Point<Self, Option<Reset>> = crate::Point::new(5, 1, true);
    pub const V_SET: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const P_SET: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const I_SET: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const EN50530: crate::Point<Self, Option<En50530>> = crate::Point::new(9, 1, true);
    pub const VMPP: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const PMPP: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const G_SET: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const V_SLEW_RATE: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const P_SLEW_RATE: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const I_SLEW_RATE: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, true);
    pub const ENA_PROF: crate::Point<Self, Option<EnaProf>> = crate::Point::new(16, 1, true);
    pub const ADPT_PROF_REQ: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const ADPT_PROF_RSLT: crate::Point<Self, AdptProfRslt> = crate::Point::new(18, 1, false);
    pub const V: crate::Point<Self, Option<i32>> = crate::Point::new(19, 2, false);
    pub const P: crate::Point<Self, Option<i32>> = crate::Point::new(21, 2, false);
    pub const I: crate::Point<Self, Option<i32>> = crate::Point::new(23, 2, false);
    pub const ERRORS: crate::Point<Self, Option<String>> = crate::Point::new(25, 32, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(57, 1, false);
    pub const N_PROF: crate::Point<Self, u16> = crate::Point::new(58, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(59, 1, true);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(60, 1, true);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(61, 1, true);
    pub const G_SF: crate::Point<Self, i16> = crate::Point::new(62, 1, true);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(63, 1, true);
    pub const V_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(64, 1, true);
    pub const P_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(65, 1, true);
    pub const I_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(66, 1, true);
    pub const PCT_SF: crate::Point<Self, i16> = crate::Point::new(67, 1, true);
}
impl crate::Group for DcSimInterface {
    const LEN: u16 = 68;
}
impl DcSimInterface {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                v_max_lim: Self::V_MAX_LIM.from_data(data)?,
                p_max_lim: Self::P_MAX_LIM.from_data(data)?,
                i_max_lim: Self::I_MAX_LIM.from_data(data)?,
                mode: Self::MODE.from_data(data)?,
                ena: Self::ENA.from_data(data)?,
                reset: Self::RESET.from_data(data)?,
                v_set: Self::V_SET.from_data(data)?,
                p_set: Self::P_SET.from_data(data)?,
                i_set: Self::I_SET.from_data(data)?,
                en50530: Self::EN50530.from_data(data)?,
                vmpp: Self::VMPP.from_data(data)?,
                pmpp: Self::PMPP.from_data(data)?,
                g_set: Self::G_SET.from_data(data)?,
                v_slew_rate: Self::V_SLEW_RATE.from_data(data)?,
                p_slew_rate: Self::P_SLEW_RATE.from_data(data)?,
                i_slew_rate: Self::I_SLEW_RATE.from_data(data)?,
                ena_prof: Self::ENA_PROF.from_data(data)?,
                adpt_prof_req: Self::ADPT_PROF_REQ.from_data(data)?,
                adpt_prof_rslt: Self::ADPT_PROF_RSLT.from_data(data)?,
                v: Self::V.from_data(data)?,
                p: Self::P.from_data(data)?,
                i: Self::I.from_data(data)?,
                errors: Self::ERRORS.from_data(data)?,
                n_pt: Self::N_PT.from_data(data)?,
                n_prof: Self::N_PROF.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                g_sf: Self::G_SF.from_data(data)?,
                tms_sf: Self::TMS_SF.from_data(data)?,
                v_slew_sf: Self::V_SLEW_SF.from_data(data)?,
                p_slew_sf: Self::P_SLEW_SF.from_data(data)?,
                i_slew_sf: Self::I_SLEW_SF.from_data(data)?,
                pct_sf: Self::PCT_SF.from_data(data)?,
                prof: Vec::new(),
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.prof) = Prof::parse_multiple(data, &group)?;
        Ok((data, group))
    }
}
/// CV or CC Mode
///
/// Constant Voltage (CV) or Constant Current (CC) Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Mode {
    /// CV Mode
    ///
    /// Constant Voltage (CV) Mode
    Cv = 0,
    /// CC Mode
    ///
    /// Constant Current (CC) Mode.
    Cc = 1,
}
impl crate::Value for Mode {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Mode> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Mode::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Power On/Off
///
/// Power On/Off
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Ena {
    /// Power On
    ///
    /// Power On
    On = 1,
    /// Power Off
    ///
    /// Power Off
    Off = 0,
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
/// Reset Device
///
/// Reset Device
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Reset {
    /// Reset Device
    ///
    /// Reset Device
    Reset = 1,
    /// Do Not Reset Device
    ///
    /// Do Not Reset Device
    DoNotReset = 0,
}
impl crate::Value for Reset {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Reset> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Reset::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// EN50530 Mode
///
/// EN50530 Mode - Enable or disable EN50530 profile mode
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum En50530 {
    /// EN50530 Mode
    ///
    /// EN50530 Mode
    En50530 = 1,
    /// Do Not Use EN50530 Mode
    ///
    /// Do Not Use EN50530 Mode
    DoNotEn50530 = 0,
}
impl crate::Value for En50530 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<En50530> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                En50530::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Enable Profile
///
/// Start/Stop the Profile
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum EnaProf {
    /// Start Profile
    ///
    /// Start the Profile
    Start = 1,
    /// Stop Profile
    ///
    /// Stop the Profile
    Stop = 0,
}
impl crate::Value for EnaProf {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<EnaProf> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                EnaProf::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Adopt Profile Result
///
/// Result of last adopt profile operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum AdptProfRslt {
    /// Update In Progress
    ///
    /// Profile update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Profile update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Profile update failed.
    Failed = 2,
}
impl crate::Value for AdptProfRslt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<AdptProfRslt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                AdptProfRslt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Stored Profiles
///
/// Stored profile sets.
///
/// Comments: Stored Pofile Sets - Number of profile sets = NProf - The first set is read-only and indicates the current settings.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Prof {
    /// Active Points
    ///
    /// Number of active points.
    pub act_pt: u16,
    /// Dependent References
    ///
    /// Profile references.
    pub dept_ref: ProfDeptRef,
    /// Stored Profile Points
    ///
    /// Stored profile points.
    ///
    /// Comments: Stored Profile Sets - Profile points for each stored profile - Number of profile points contained in NPt
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl Prof {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const DEPT_REF: crate::Point<Self, ProfDeptRef> = crate::Point::new(1, 1, true);
}
impl crate::Group for Prof {
    const LEN: u16 = 2;
}
impl Prof {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                dept_ref: Self::DEPT_REF.from_data(data)?,
                pt: Vec::new(),
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &DcSimInterface,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.pt) = Pt::parse_multiple(data, model)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &DcSimInterface,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..model.n_prof {
            let group;
            (data, group) = Prof::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " Dependent References"] #[doc = " "] #[doc = " Profile references."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ProfDeptRef : u32 {
    #[doc = " Voltage"] const Voltage = 1; #[doc = " Power"] const Power = 2; #[doc =
    " Current"] const Current = 4; #[doc = " Irradiance"] const Irradiance = 8; }
}
impl crate::Value for ProfDeptRef {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<ProfDeptRef> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(ProfDeptRef::from_bits_retain(value)))
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
/// Stored Profile Points
///
/// Stored profile points.
///
/// Comments: Stored Profile Sets - Profile points for each stored profile - Number of profile points contained in NPt
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pt {
    /// Profile Time
    ///
    /// Profile time.
    pub tms: Option<u16>,
    /// Voltage Point
    ///
    /// Profile voltage point in Volts.
    pub v: Option<u16>,
    /// Power Point
    ///
    /// Profile power point in Watts.
    pub p: Option<u16>,
    /// Current Point
    ///
    /// Profile current point in Amps.
    pub i: Option<u16>,
    /// Irradiance Point
    ///
    /// Profile irradiance point as percentage.
    pub g: Option<u16>,
}
#[allow(missing_docs)]
impl Pt {
    pub const TMS: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const P: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const I: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const G: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
}
impl crate::Group for Pt {
    const LEN: u16 = 5;
}
impl Pt {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                tms: Self::TMS.from_data(data)?,
                v: Self::V.from_data(data)?,
                p: Self::P.from_data(data)?,
                i: Self::I.from_data(data)?,
                g: Self::G.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &DcSimInterface,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &DcSimInterface,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..model.n_pt {
            let group;
            (data, group) = Pt::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
impl crate::Model for DcSimInterface {
    const ID: u16 = 64410;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64410
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
