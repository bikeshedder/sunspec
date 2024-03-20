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
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_schd: Self::ACT_SCHD.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            n_schd: Self::N_SCHD.from_data(data)?,
            n_pts: Self::N_PTS.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "ActSchd\n\nBitfield of active schedules"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct ActSchd : u32 { # [doc = ""] const Sched1 = 1 ; # [doc = ""] const Sched2 = 2 ; # [doc = ""] const Sched3 = 4 ; # [doc = ""] const Sched4 = 8 ; # [doc = ""] const Sched5 = 16 ; # [doc = ""] const Sched6 = 32 ; # [doc = ""] const Sched7 = 64 ; # [doc = ""] const Sched8 = 128 ; # [doc = ""] const Sched9 = 256 ; # [doc = ""] const Sched10 = 512 ; # [doc = ""] const Sched12 = 1024 ; # [doc = ""] const Sched13 = 2048 ; # [doc = ""] const Sched14 = 4096 ; # [doc = ""] const Sched15 = 8192 ; # [doc = ""] const Sched16 = 16384 ; # [doc = ""] const Sched17 = 32768 ; # [doc = ""] const Sched18 = 65536 ; # [doc = ""] const Sched19 = 131072 ; # [doc = ""] const Sched20 = 262144 ; # [doc = ""] const Sched21 = 524288 ; # [doc = ""] const Sched22 = 2097152 ; # [doc = ""] const Sched23 = 4194304 ; # [doc = ""] const Sched24 = 8388608 ; # [doc = ""] const Sched25 = 16777216 ; # [doc = ""] const Sched26 = 33554432 ; # [doc = ""] const Sched27 = 67108864 ; # [doc = ""] const Sched28 = 134217728 ; # [doc = ""] const Sched29 = 268435456 ; # [doc = ""] const Sched30 = 536870912 ; # [doc = ""] const Sched31 = 1073741824 ; # [doc = ""] const Sched32 = 2147483648 ; } }
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

bitflags::bitflags! { # [doc = "ModEna\n\nIs basic scheduling active."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct ModEna : u16 { # [doc = ""] const Enabled = 1 ; } }
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
