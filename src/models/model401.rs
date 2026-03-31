//! String Combiner (Current)
/// Type alias for [`StringCombinerCurrent`].
pub type Model401 = StringCombinerCurrent;
/// String Combiner (Current)
///
/// A basic string combiner
///
/// Detail: This model is SUPERSEDED by model 403
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StringCombinerCurrent {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dc_ahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dca_max: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: Evt,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evt_vnd: Option<EvtVnd>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    #[allow(missing_docs)]
    pub string: Vec<String>,
}
#[allow(missing_docs)]
impl StringCombinerCurrent {
    pub const DCA_SF: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const DCV_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const DCA_MAX: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(5, 2, false);
    pub const EVT_VND: crate::Point<Self, Option<EvtVnd>> = crate::Point::new(7, 2, false);
    pub const DCA: crate::Point<Self, i16> = crate::Point::new(9, 1, false);
    pub const DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(10, 2, false);
    pub const DCV: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, false);
}
impl crate::Group for StringCombinerCurrent {
    const LEN: u16 = 14;
}
impl StringCombinerCurrent {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, string) = String::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                dca_sf: Self::DCA_SF.from_data(data)?,
                dc_ahr_sf: Self::DC_AHR_SF.from_data(data)?,
                dcv_sf: Self::DCV_SF.from_data(data)?,
                dca_max: Self::DCA_MAX.from_data(data)?,
                n: Self::N.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                evt_vnd: Self::EVT_VND.from_data(data)?,
                dca: Self::DCA.from_data(data)?,
                dc_ahr: Self::DC_AHR.from_data(data)?,
                dcv: Self::DCV.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                string,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Event"] #[doc = " "] #[doc = " Bitmask value.  Events"] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Evt : u32 {
    #[allow(missing_docs)] const LowVoltage = 1; #[allow(missing_docs)] const LowPower =
    2; #[allow(missing_docs)] const LowEfficiency = 4; #[allow(missing_docs)] const
    Current = 8; #[allow(missing_docs)] const Voltage = 16; #[allow(missing_docs)] const
    Power = 32; #[allow(missing_docs)] const Pr = 64; #[allow(missing_docs)] const
    Disconnected = 128; #[allow(missing_docs)] const FuseFault = 256;
    #[allow(missing_docs)] const CombinerFuseFault = 512; #[allow(missing_docs)] const
    CombinerCabinetOpen = 1024; #[allow(missing_docs)] const Temp = 2048;
    #[allow(missing_docs)] const Groundfault = 4096; #[allow(missing_docs)] const
    ReversedPolarity = 8192; #[allow(missing_docs)] const Incompatible = 16384;
    #[allow(missing_docs)] const CommError = 32768; #[allow(missing_docs)] const
    InternalError = 65536; #[allow(missing_docs)] const Theft = 131072;
    #[allow(missing_docs)] const ArcDetected = 262144; }
}
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
    #[doc = " Vendor Event"] #[doc = " "] #[doc =
    " Bitmask value.  Vendor defined events"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct EvtVnd : u32 {}
}
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
#[derive(Debug)]
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
    pub in_dca: i16,
    /// Amp-hours
    ///
    /// String Input Amp-Hours
    pub in_dc_ahr: Option<u32>,
}
#[allow(missing_docs)]
impl String {
    pub const IN_ID: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const IN_EVT: crate::Point<Self, StringInEvt> = crate::Point::new(1, 2, false);
    pub const IN_EVT_VND: crate::Point<Self, Option<StringInEvtVnd>> =
        crate::Point::new(3, 2, false);
    pub const IN_DCA: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const IN_DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(6, 2, false);
}
impl crate::Group for String {
    const LEN: u16 = 8;
}
impl String {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                in_id: Self::IN_ID.from_data(data)?,
                in_evt: Self::IN_EVT.from_data(data)?,
                in_evt_vnd: Self::IN_EVT_VND.from_data(data)?,
                in_dca: Self::IN_DCA.from_data(data)?,
                in_dc_ahr: Self::IN_DC_AHR.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<String as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = String::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " Input Event"] #[doc = " "] #[doc = " String Input Event Flags"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct StringInEvt : u32 {
    #[allow(missing_docs)] const LowVoltage = 1; #[allow(missing_docs)] const LowPower =
    2; #[allow(missing_docs)] const LowEfficiency = 4; #[allow(missing_docs)] const
    Current = 8; #[allow(missing_docs)] const Voltage = 16; #[allow(missing_docs)] const
    Power = 32; #[allow(missing_docs)] const Pr = 64; #[allow(missing_docs)] const
    Disconnected = 128; #[allow(missing_docs)] const FuseFault = 256;
    #[allow(missing_docs)] const CombinerFuseFault = 512; #[allow(missing_docs)] const
    CombinerCabinetOpen = 1024; #[allow(missing_docs)] const Temp = 2048;
    #[allow(missing_docs)] const Groundfault = 4096; #[allow(missing_docs)] const
    ReversedPolarity = 8192; #[allow(missing_docs)] const Incompatible = 16384;
    #[allow(missing_docs)] const CommError = 32768; #[allow(missing_docs)] const
    InternalError = 65536; #[allow(missing_docs)] const Theft = 131072;
    #[allow(missing_docs)] const ArcDetected = 262144; }
}
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
    #[doc = " Input Event Vendor"] #[doc = " "] #[doc =
    " String Input Vendor Event Flags"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct StringInEvtVnd : u32 {}
}
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
impl crate::Model for StringCombinerCurrent {
    const ID: u16 = 401;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m401
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
