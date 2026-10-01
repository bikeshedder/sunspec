//! Inverter (Split Phase) FLOAT
/// Type alias for [`InverterSplitPhaseFloat`].
pub type Model112 = InverterSplitPhaseFloat;
/// Inverter (Split Phase) FLOAT
///
/// Include this model for split phase inverter monitoring using float values
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct InverterSplitPhaseFloat {
    /// Amps
    ///
    /// AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Detail: Connected Phase
    pub a_ph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Detail: Connected Phase
    pub a_ph_b: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub a_ph_c: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub pp_v_ph_ab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub pp_v_ph_bc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub pp_v_ph_ca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_v_ph_a: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_v_ph_b: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_v_ph_c: Option<f32>,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dc_a: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dc_v: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dc_w: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmp_cab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmp_snk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmp_trns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmp_ot: Option<f32>,
    /// Operating State
    ///
    /// Operating state
    pub st: St,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub st_vnd: Option<u16>,
    /// Event1
    ///
    /// Event fields
    pub evt1: Evt1,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: Evt2,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evt_vnd1: Option<EvtVnd1>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evt_vnd2: Option<EvtVnd2>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evt_vnd3: Option<EvtVnd3>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evt_vnd4: Option<EvtVnd4>,
}
#[allow(missing_docs)]
impl InverterSplitPhaseFloat {
    pub const A: crate::Point<Self, f32> = crate::Point::new(0, 2, false);
    pub const A_PH_A: crate::Point<Self, f32> = crate::Point::new(2, 2, false);
    pub const A_PH_B: crate::Point<Self, f32> = crate::Point::new(4, 2, false);
    pub const A_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(6, 2, false);
    pub const PP_V_PH_AB: crate::Point<Self, Option<f32>> = crate::Point::new(8, 2, false);
    pub const PP_V_PH_BC: crate::Point<Self, Option<f32>> = crate::Point::new(10, 2, false);
    pub const PP_V_PH_CA: crate::Point<Self, Option<f32>> = crate::Point::new(12, 2, false);
    pub const PH_V_PH_A: crate::Point<Self, f32> = crate::Point::new(14, 2, false);
    pub const PH_V_PH_B: crate::Point<Self, f32> = crate::Point::new(16, 2, false);
    pub const PH_V_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(18, 2, false);
    pub const W: crate::Point<Self, f32> = crate::Point::new(20, 2, false);
    pub const HZ: crate::Point<Self, f32> = crate::Point::new(22, 2, false);
    pub const VA: crate::Point<Self, Option<f32>> = crate::Point::new(24, 2, false);
    pub const VAR: crate::Point<Self, Option<f32>> = crate::Point::new(26, 2, false);
    pub const PF: crate::Point<Self, Option<f32>> = crate::Point::new(28, 2, false);
    pub const WH: crate::Point<Self, f32> = crate::Point::new(30, 2, false);
    pub const DC_A: crate::Point<Self, Option<f32>> = crate::Point::new(32, 2, false);
    pub const DC_V: crate::Point<Self, Option<f32>> = crate::Point::new(34, 2, false);
    pub const DC_W: crate::Point<Self, Option<f32>> = crate::Point::new(36, 2, false);
    pub const TMP_CAB: crate::Point<Self, f32> = crate::Point::new(38, 2, false);
    pub const TMP_SNK: crate::Point<Self, Option<f32>> = crate::Point::new(40, 2, false);
    pub const TMP_TRNS: crate::Point<Self, Option<f32>> = crate::Point::new(42, 2, false);
    pub const TMP_OT: crate::Point<Self, Option<f32>> = crate::Point::new(44, 2, false);
    pub const ST: crate::Point<Self, St> = crate::Point::new(46, 1, false);
    pub const ST_VND: crate::Point<Self, Option<u16>> = crate::Point::new(47, 1, false);
    pub const EVT1: crate::Point<Self, Evt1> = crate::Point::new(48, 2, false);
    pub const EVT2: crate::Point<Self, Evt2> = crate::Point::new(50, 2, false);
    pub const EVT_VND1: crate::Point<Self, Option<EvtVnd1>> = crate::Point::new(52, 2, false);
    pub const EVT_VND2: crate::Point<Self, Option<EvtVnd2>> = crate::Point::new(54, 2, false);
    pub const EVT_VND3: crate::Point<Self, Option<EvtVnd3>> = crate::Point::new(56, 2, false);
    pub const EVT_VND4: crate::Point<Self, Option<EvtVnd4>> = crate::Point::new(58, 2, false);
}
impl crate::sealed::Sealed for InverterSplitPhaseFloat {}
impl crate::Group for InverterSplitPhaseFloat {
    const LEN: u16 = 60;
}
impl InverterSplitPhaseFloat {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(60..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                a: Self::A.from_data(data)?,
                a_ph_a: Self::A_PH_A.from_data(data)?,
                a_ph_b: Self::A_PH_B.from_data(data)?,
                a_ph_c: Self::A_PH_C.from_data(data)?,
                pp_v_ph_ab: Self::PP_V_PH_AB.from_data(data)?,
                pp_v_ph_bc: Self::PP_V_PH_BC.from_data(data)?,
                pp_v_ph_ca: Self::PP_V_PH_CA.from_data(data)?,
                ph_v_ph_a: Self::PH_V_PH_A.from_data(data)?,
                ph_v_ph_b: Self::PH_V_PH_B.from_data(data)?,
                ph_v_ph_c: Self::PH_V_PH_C.from_data(data)?,
                w: Self::W.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                va: Self::VA.from_data(data)?,
                var: Self::VAR.from_data(data)?,
                pf: Self::PF.from_data(data)?,
                wh: Self::WH.from_data(data)?,
                dc_a: Self::DC_A.from_data(data)?,
                dc_v: Self::DC_V.from_data(data)?,
                dc_w: Self::DC_W.from_data(data)?,
                tmp_cab: Self::TMP_CAB.from_data(data)?,
                tmp_snk: Self::TMP_SNK.from_data(data)?,
                tmp_trns: Self::TMP_TRNS.from_data(data)?,
                tmp_ot: Self::TMP_OT.from_data(data)?,
                st: Self::ST.from_data(data)?,
                st_vnd: Self::ST_VND.from_data(data)?,
                evt1: Self::EVT1.from_data(data)?,
                evt2: Self::EVT2.from_data(data)?,
                evt_vnd1: Self::EVT_VND1.from_data(data)?,
                evt_vnd2: Self::EVT_VND2.from_data(data)?,
                evt_vnd3: Self::EVT_VND3.from_data(data)?,
                evt_vnd4: Self::EVT_VND4.from_data(data)?,
            },
        ))
    }
}
/// Operating State
///
/// Operating state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum St {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Sleeping,
    #[allow(missing_docs)]
    Starting,
    #[allow(missing_docs)]
    Mppt,
    #[allow(missing_docs)]
    Throttled,
    #[allow(missing_docs)]
    ShuttingDown,
    #[allow(missing_docs)]
    Fault,
    #[allow(missing_docs)]
    Standby,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for St {}
impl crate::EnumValue for St {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Off,
            2 => Self::Sleeping,
            3 => Self::Starting,
            4 => Self::Mppt,
            5 => Self::Throttled,
            6 => Self::ShuttingDown,
            7 => Self::Fault,
            8 => Self::Standby,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 1,
            Self::Sleeping => 2,
            Self::Starting => 3,
            Self::Mppt => 4,
            Self::Throttled => 5,
            Self::ShuttingDown => 6,
            Self::Fault => 7,
            Self::Standby => 8,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for St {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    /// Event1
    ///
    /// Event fields
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Evt1: u32 {
        #[allow(missing_docs)]
        const GroundFault = 1;
        #[allow(missing_docs)]
        const DcOverVolt = 2;
        #[allow(missing_docs)]
        const AcDisconnect = 4;
        #[allow(missing_docs)]
        const DcDisconnect = 8;
        #[allow(missing_docs)]
        const GridDisconnect = 16;
        #[allow(missing_docs)]
        const CabinetOpen = 32;
        #[allow(missing_docs)]
        const ManualShutdown = 64;
        #[allow(missing_docs)]
        const OverTemp = 128;
        #[allow(missing_docs)]
        const OverFrequency = 256;
        #[allow(missing_docs)]
        const UnderFrequency = 512;
        #[allow(missing_docs)]
        const AcOverVolt = 1024;
        #[allow(missing_docs)]
        const AcUnderVolt = 2048;
        #[allow(missing_docs)]
        const BlownStringFuse = 4096;
        #[allow(missing_docs)]
        const UnderTemp = 8192;
        #[allow(missing_docs)]
        const MemoryLoss = 16384;
        #[allow(missing_docs)]
        const HwTestFailure = 32768;
    }
}
impl crate::sealed::Sealed for Evt1 {}
impl crate::Value for Evt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Evt2: u32 {
    }
}
impl crate::sealed::Sealed for Evt2 {}
impl crate::Value for Evt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct EvtVnd1: u32 {
    }
}
impl crate::sealed::Sealed for EvtVnd1 {}
impl crate::Value for EvtVnd1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct EvtVnd2: u32 {
    }
}
impl crate::sealed::Sealed for EvtVnd2 {}
impl crate::Value for EvtVnd2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct EvtVnd3: u32 {
    }
}
impl crate::sealed::Sealed for EvtVnd3 {}
impl crate::Value for EvtVnd3 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd3 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct EvtVnd4: u32 {
    }
}
impl crate::sealed::Sealed for EvtVnd4 {}
impl crate::Value for EvtVnd4 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd4 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl From<InverterSplitPhaseFloat> for crate::AnyModel {
    fn from(model: InverterSplitPhaseFloat) -> Self {
        Self::M112(model)
    }
}
impl crate::Model for InverterSplitPhaseFloat {
    const ID: u16 = 112;
    const NAME: &'static str = "inverter_split_phase_float";
    const LABEL: &'static str = "Inverter (Split Phase) FLOAT";
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
