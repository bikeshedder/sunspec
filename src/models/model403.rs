//! String Combiner (Current)
/// Type alias for [`StringCombinerCurrentInput`].
pub type Model403 = StringCombinerCurrentInput;
/// String Combiner (Current)
///
/// A basic string combiner model
///
/// Detail: This model supersedes model 401
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StringCombinerCurrentInput {
    /// Current scale factor
    pub dc_a_sf: i16,
    /// Amp-hour scale factor
    pub dc_ahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dc_v_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dc_a_max: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Events
    pub evt: Evt,
    /// Vendor Event
    ///
    /// Vendor defined events
    pub evt_vnd: Option<EvtVnd>,
    /// Amps
    ///
    /// Total measured current
    pub dc_a: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dc_v: Option<i16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Current scale factor for inputs
    pub in_dc_a_sf: Option<i16>,
    /// Amp-hour scale factor for inputs
    pub in_dc_ahr_sf: Option<i16>,
    #[allow(missing_docs)]
    pub string: Vec<String>,
}
#[allow(missing_docs)]
impl StringCombinerCurrentInput {
    pub const DC_A_SF: crate::Point<Self, i16> = crate::Point::new(0, 1);
    pub const DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1);
    pub const DC_V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1);
    pub const DC_A_MAX: crate::Point<Self, u16> = crate::Point::new(3, 1);
    pub const N: crate::Point<Self, u16> = crate::Point::new(4, 1);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(5, 2);
    pub const EVT_VND: crate::Point<Self, Option<EvtVnd>> = crate::Point::new(7, 2);
    pub const DC_A: crate::Point<Self, i16> = crate::Point::new(9, 1);
    pub const DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(10, 2);
    pub const DC_V: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1);
    pub const IN_DC_A_SF: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1);
    pub const IN_DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1);
}
impl crate::sealed::Sealed for StringCombinerCurrentInput {}
impl crate::Group for StringCombinerCurrentInput {
    const LEN: u16 = 16;
}
impl StringCombinerCurrentInput {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(16..).unwrap_or(&[]);
        let (nested_data, string) = String::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                dc_a_sf: Self::DC_A_SF.from_data(data)?,
                dc_ahr_sf: Self::DC_AHR_SF.from_data(data)?,
                dc_v_sf: Self::DC_V_SF.from_data(data)?,
                dc_a_max: Self::DC_A_MAX.from_data(data)?,
                n: Self::N.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                evt_vnd: Self::EVT_VND.from_data(data)?,
                dc_a: Self::DC_A.from_data(data)?,
                dc_ahr: Self::DC_AHR.from_data(data)?,
                dc_v: Self::DC_V.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                in_dc_a_sf: Self::IN_DC_A_SF.from_data(data)?,
                in_dc_ahr_sf: Self::IN_DC_AHR_SF.from_data(data)?,
                string,
            },
        ))
    }
}
bitflags::bitflags! {
    /// Event
    ///
    /// Events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Evt: u32 {
        #[allow(missing_docs)]
        const LowVoltage = 1;
        #[allow(missing_docs)]
        const LowPower = 2;
        #[allow(missing_docs)]
        const LowEfficiency = 4;
        #[allow(missing_docs)]
        const Current = 8;
        #[allow(missing_docs)]
        const Voltage = 16;
        #[allow(missing_docs)]
        const Power = 32;
        #[allow(missing_docs)]
        const Pr = 64;
        #[allow(missing_docs)]
        const Disconnected = 128;
        #[allow(missing_docs)]
        const FuseFault = 256;
        #[allow(missing_docs)]
        const CombinerFuseFault = 512;
        #[allow(missing_docs)]
        const CombinerCabinetOpen = 1024;
        #[allow(missing_docs)]
        const Temp = 2048;
        #[allow(missing_docs)]
        const Groundfault = 4096;
        #[allow(missing_docs)]
        const ReversedPolarity = 8192;
        #[allow(missing_docs)]
        const Incompatible = 16384;
        #[allow(missing_docs)]
        const CommError = 32768;
        #[allow(missing_docs)]
        const InternalError = 65536;
        #[allow(missing_docs)]
        const Theft = 131072;
        #[allow(missing_docs)]
        const ArcDetected = 262144;
    }
}
impl crate::sealed::Sealed for Evt {}
impl crate::Value for Evt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Vendor Event
    ///
    /// Vendor defined events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct EvtVnd: u32 {
    }
}
impl crate::sealed::Sealed for EvtVnd {}
impl crate::Value for EvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct String {
    /// ID
    ///
    /// Uniquely identifies this input set
    pub in_id: u16,
    /// Input Event
    ///
    /// String Input Event Flags
    pub in_evt: StringInEvt,
    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    pub in_evt_vnd: Option<StringInEvtVnd>,
    /// Amps
    ///
    /// String Input Current
    pub in_dc_a: i16,
    /// Amp-hours
    ///
    /// String Input Amp-Hours
    pub in_dc_ahr: Option<u32>,
}
#[allow(missing_docs)]
impl String {
    pub const IN_ID: crate::Point<Self, u16> = crate::Point::new(0, 1);
    pub const IN_EVT: crate::Point<Self, StringInEvt> = crate::Point::new(1, 2);
    pub const IN_EVT_VND: crate::Point<Self, Option<StringInEvtVnd>> = crate::Point::new(3, 2);
    pub const IN_DC_A: crate::Point<Self, i16> = crate::Point::new(5, 1);
    pub const IN_DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(6, 2);
}
impl crate::sealed::Sealed for String {}
impl crate::Group for String {
    const LEN: u16 = 8;
}
impl String {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(8..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                in_id: Self::IN_ID.from_data(data)?,
                in_evt: Self::IN_EVT.from_data(data)?,
                in_evt_vnd: Self::IN_EVT_VND.from_data(data)?,
                in_dc_a: Self::IN_DC_A.from_data(data)?,
                in_dc_ahr: Self::IN_DC_AHR.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::ParseError> {
        let group_len = usize::from(<String as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::ParseError::InvalidGroupLength);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = String::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::ParseError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    /// Input Event
    ///
    /// String Input Event Flags
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct StringInEvt: u32 {
        #[allow(missing_docs)]
        const LowVoltage = 1;
        #[allow(missing_docs)]
        const LowPower = 2;
        #[allow(missing_docs)]
        const LowEfficiency = 4;
        #[allow(missing_docs)]
        const Current = 8;
        #[allow(missing_docs)]
        const Voltage = 16;
        #[allow(missing_docs)]
        const Power = 32;
        #[allow(missing_docs)]
        const Pr = 64;
        #[allow(missing_docs)]
        const Disconnected = 128;
        #[allow(missing_docs)]
        const FuseFault = 256;
        #[allow(missing_docs)]
        const CombinerFuseFault = 512;
        #[allow(missing_docs)]
        const CombinerCabinetOpen = 1024;
        #[allow(missing_docs)]
        const Temp = 2048;
        #[allow(missing_docs)]
        const Groundfault = 4096;
        #[allow(missing_docs)]
        const ReversedPolarity = 8192;
        #[allow(missing_docs)]
        const Incompatible = 16384;
        #[allow(missing_docs)]
        const CommError = 32768;
        #[allow(missing_docs)]
        const InternalError = 65536;
        #[allow(missing_docs)]
        const Theft = 131072;
        #[allow(missing_docs)]
        const ArcDetected = 262144;
    }
}
impl crate::sealed::Sealed for StringInEvt {}
impl crate::Value for StringInEvt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringInEvt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct StringInEvtVnd: u32 {
    }
}
impl crate::sealed::Sealed for StringInEvtVnd {}
impl crate::Value for StringInEvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringInEvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl From<StringCombinerCurrentInput> for crate::AnyModel {
    fn from(model: StringCombinerCurrentInput) -> Self {
        Self::M403(model)
    }
}
impl crate::Model for StringCombinerCurrentInput {
    const ID: u16 = 403;
    const NAME: &'static str = "string_combiner_current_input";
    const LABEL: &'static str = "String Combiner (Current)";
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
