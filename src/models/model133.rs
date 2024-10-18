//! Basic Scheduling
/// Basic Scheduling
///
/// Basic Scheduling
///
/// Notes: Ref 2: 2.2.8
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model133 {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    pub act_schd: ActSchd,
    /// ModEna
    ///
    /// Is basic scheduling active.
    pub mod_ena: ModEna,
    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    pub n_schd: u16,
    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    pub n_pts: u16,
}
#[allow(missing_docs)]
impl Model133 {
    pub const ACT_SCHD: crate::PointDef<Self, ActSchd> = crate::PointDef::new(0, 2, true);
    pub const MOD_ENA: crate::PointDef<Self, ModEna> = crate::PointDef::new(2, 1, true);
    pub const N_SCHD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N_PTS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
}
impl crate::Model for Model133 {
    const ID: u16 = 133;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            act_schd: Self::ACT_SCHD.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            n_schd: Self::N_SCHD.from_data(data)?,
            n_pts: Self::N_PTS.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m133
    }
}
bitflags::bitflags! {
    #[doc = " ActSchd"] #[doc = " "] #[doc = " Bitfield of active schedules"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ActSchd : u32 {
    #[allow(missing_docs)] const Sched1 = 1; #[allow(missing_docs)] const Sched2 = 2;
    #[allow(missing_docs)] const Sched3 = 4; #[allow(missing_docs)] const Sched4 = 8;
    #[allow(missing_docs)] const Sched5 = 16; #[allow(missing_docs)] const Sched6 = 32;
    #[allow(missing_docs)] const Sched7 = 64; #[allow(missing_docs)] const Sched8 = 128;
    #[allow(missing_docs)] const Sched9 = 256; #[allow(missing_docs)] const Sched10 =
    512; #[allow(missing_docs)] const Sched12 = 1024; #[allow(missing_docs)] const
    Sched13 = 2048; #[allow(missing_docs)] const Sched14 = 4096; #[allow(missing_docs)]
    const Sched15 = 8192; #[allow(missing_docs)] const Sched16 = 16384;
    #[allow(missing_docs)] const Sched17 = 32768; #[allow(missing_docs)] const Sched18 =
    65536; #[allow(missing_docs)] const Sched19 = 131072; #[allow(missing_docs)] const
    Sched20 = 262144; #[allow(missing_docs)] const Sched21 = 524288;
    #[allow(missing_docs)] const Sched22 = 2097152; #[allow(missing_docs)] const Sched23
    = 4194304; #[allow(missing_docs)] const Sched24 = 8388608; #[allow(missing_docs)]
    const Sched25 = 16777216; #[allow(missing_docs)] const Sched26 = 33554432;
    #[allow(missing_docs)] const Sched27 = 67108864; #[allow(missing_docs)] const Sched28
    = 134217728; #[allow(missing_docs)] const Sched29 = 268435456; #[allow(missing_docs)]
    const Sched30 = 536870912; #[allow(missing_docs)] const Sched31 = 1073741824;
    #[allow(missing_docs)] const Sched32 = 2147483648; }
}
impl crate::Value for ActSchd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<ActSchd> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(ActSchd::from_bits_retain(value)))
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
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc = " Is basic scheduling active."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ModEna : u16 {
    #[allow(missing_docs)] const Enabled = 1; }
}
impl crate::Value for ModEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<ModEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(ModEna::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
