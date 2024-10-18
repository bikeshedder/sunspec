//! SunSpec Test Model 1
/// SunSpec Test Model 1
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model63001 {
    #[allow(missing_docs)]
    pub sunssf_1: Option<i16>,
    #[allow(missing_docs)]
    pub sunssf_2: Option<i16>,
    #[allow(missing_docs)]
    pub sunssf_3: Option<i16>,
    #[allow(missing_docs)]
    pub sunssf_4: Option<i16>,
    #[allow(missing_docs)]
    pub int16_1: Option<i16>,
    #[allow(missing_docs)]
    pub int16_2: Option<i16>,
    #[allow(missing_docs)]
    pub int16_3: Option<i16>,
    #[allow(missing_docs)]
    pub int16_4: Option<i16>,
    #[allow(missing_docs)]
    pub int16_5: Option<i16>,
    #[allow(missing_docs)]
    pub int16_u: Option<i16>,
    #[allow(missing_docs)]
    pub uint16_1: Option<u16>,
    #[allow(missing_docs)]
    pub uint16_2: Option<u16>,
    #[allow(missing_docs)]
    pub uint16_3: Option<u16>,
    #[allow(missing_docs)]
    pub uint16_4: Option<u16>,
    #[allow(missing_docs)]
    pub uint16_5: Option<u16>,
    #[allow(missing_docs)]
    pub uint16_u: Option<u16>,
    #[allow(missing_docs)]
    pub acc16: Option<u16>,
    #[allow(missing_docs)]
    pub acc16_u: Option<u16>,
    #[allow(missing_docs)]
    pub enum16: Option<u16>,
    #[allow(missing_docs)]
    pub enum16_u: Option<u16>,
    #[allow(missing_docs)]
    pub bitfield16: Option<Bitfield16>,
    #[allow(missing_docs)]
    pub bitfield16_u: Option<Bitfield16U>,
    #[allow(missing_docs)]
    pub int32_1: Option<i32>,
    #[allow(missing_docs)]
    pub int32_2: Option<i32>,
    #[allow(missing_docs)]
    pub int32_3: Option<i32>,
    #[allow(missing_docs)]
    pub int32_4: Option<i32>,
    #[allow(missing_docs)]
    pub int32_5: Option<i32>,
    #[allow(missing_docs)]
    pub int32_u: Option<i32>,
    #[allow(missing_docs)]
    pub uint32_1: Option<u32>,
    #[allow(missing_docs)]
    pub uint32_2: Option<u32>,
    #[allow(missing_docs)]
    pub uint32_3: Option<u32>,
    #[allow(missing_docs)]
    pub uint32_4: Option<u32>,
    #[allow(missing_docs)]
    pub uint32_5: Option<u32>,
    #[allow(missing_docs)]
    pub uint32_u: Option<u32>,
    #[allow(missing_docs)]
    pub acc32: Option<u32>,
    #[allow(missing_docs)]
    pub acc32_u: Option<u32>,
    #[allow(missing_docs)]
    pub enum32: Option<u32>,
    #[allow(missing_docs)]
    pub enum32_u: Option<u32>,
    #[allow(missing_docs)]
    pub bitfield32: Option<Bitfield32>,
    #[allow(missing_docs)]
    pub bitfield32_u: Option<Bitfield32U>,
    #[allow(missing_docs)]
    pub ipaddr: Option<std::net::Ipv4Addr>,
    #[allow(missing_docs)]
    pub ipaddr_u: Option<std::net::Ipv4Addr>,
    #[allow(missing_docs)]
    pub int64: Option<i64>,
    #[allow(missing_docs)]
    pub int64_u: Option<i64>,
    #[allow(missing_docs)]
    pub acc64: Option<u64>,
    #[allow(missing_docs)]
    pub acc64_u: Option<u64>,
    #[allow(missing_docs)]
    pub ipv6addr: Option<std::net::Ipv6Addr>,
    #[allow(missing_docs)]
    pub ipv6addr_u: Option<std::net::Ipv6Addr>,
    #[allow(missing_docs)]
    pub float32: Option<f32>,
    #[allow(missing_docs)]
    pub float32_u: Option<f32>,
    #[allow(missing_docs)]
    pub string: Option<String>,
    #[allow(missing_docs)]
    pub string_u: Option<String>,
    #[allow(missing_docs)]
    pub sunssf_5: Option<i16>,
    #[allow(missing_docs)]
    pub sunssf_6: Option<i16>,
    #[allow(missing_docs)]
    pub sunssf_7: Option<i16>,
}
#[allow(missing_docs)]
impl Model63001 {
    pub const SUNSSF_1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const SUNSSF_2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const SUNSSF_3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const SUNSSF_4: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const INT16_1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(4, 1, false);
    pub const INT16_2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(5, 1, false);
    pub const INT16_3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(6, 1, false);
    pub const INT16_4: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(7, 1, true);
    pub const INT16_5: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(8, 1, false);
    pub const INT16_U: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(9, 1, false);
    pub const UINT16_1: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const UINT16_2: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(11, 1, false);
    pub const UINT16_3: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, false);
    pub const UINT16_4: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(13, 1, true);
    pub const UINT16_5: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const UINT16_U: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(15, 1, false);
    pub const ACC16: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, false);
    pub const ACC16_U: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, false);
    pub const ENUM16: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(18, 1, false);
    pub const ENUM16_U: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(19, 1, false);
    pub const BITFIELD16: crate::PointDef<Self, Option<Bitfield16>> =
        crate::PointDef::new(20, 1, false);
    pub const BITFIELD16_U: crate::PointDef<Self, Option<Bitfield16U>> =
        crate::PointDef::new(21, 1, false);
    pub const INT32_1: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(22, 2, false);
    pub const INT32_2: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(24, 2, false);
    pub const INT32_3: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(26, 2, true);
    pub const INT32_4: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(28, 2, false);
    pub const INT32_5: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(30, 2, false);
    pub const INT32_U: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(32, 2, false);
    pub const UINT32_1: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(34, 2, false);
    pub const UINT32_2: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(36, 2, false);
    pub const UINT32_3: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(38, 2, true);
    pub const UINT32_4: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(40, 2, false);
    pub const UINT32_5: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(42, 2, false);
    pub const UINT32_U: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(44, 2, false);
    pub const ACC32: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(46, 2, false);
    pub const ACC32_U: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(48, 2, false);
    pub const ENUM32: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(50, 2, false);
    pub const ENUM32_U: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(52, 2, false);
    pub const BITFIELD32: crate::PointDef<Self, Option<Bitfield32>> =
        crate::PointDef::new(54, 2, false);
    pub const BITFIELD32_U: crate::PointDef<Self, Option<Bitfield32U>> =
        crate::PointDef::new(56, 2, false);
    pub const IPADDR: crate::PointDef<Self, Option<std::net::Ipv4Addr>> =
        crate::PointDef::new(58, 2, true);
    pub const IPADDR_U: crate::PointDef<Self, Option<std::net::Ipv4Addr>> =
        crate::PointDef::new(60, 2, false);
    pub const INT64: crate::PointDef<Self, Option<i64>> = crate::PointDef::new(62, 4, true);
    pub const INT64_U: crate::PointDef<Self, Option<i64>> = crate::PointDef::new(66, 4, false);
    pub const ACC64: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(70, 4, false);
    pub const ACC64_U: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(74, 4, false);
    pub const IPV6ADDR: crate::PointDef<Self, Option<std::net::Ipv6Addr>> =
        crate::PointDef::new(78, 8, false);
    pub const IPV6ADDR_U: crate::PointDef<Self, Option<std::net::Ipv6Addr>> =
        crate::PointDef::new(86, 8, false);
    pub const FLOAT32: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(94, 2, true);
    pub const FLOAT32_U: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(96, 2, false);
    pub const STRING: crate::PointDef<Self, Option<String>> = crate::PointDef::new(98, 16, true);
    pub const STRING_U: crate::PointDef<Self, Option<String>> =
        crate::PointDef::new(114, 16, false);
    pub const SUNSSF_5: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(130, 1, false);
    pub const SUNSSF_6: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(131, 1, false);
    pub const SUNSSF_7: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(132, 1, false);
}
impl crate::Model for Model63001 {
    const ID: u16 = 63001;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            sunssf_1: Self::SUNSSF_1.from_data(data)?,
            sunssf_2: Self::SUNSSF_2.from_data(data)?,
            sunssf_3: Self::SUNSSF_3.from_data(data)?,
            sunssf_4: Self::SUNSSF_4.from_data(data)?,
            int16_1: Self::INT16_1.from_data(data)?,
            int16_2: Self::INT16_2.from_data(data)?,
            int16_3: Self::INT16_3.from_data(data)?,
            int16_4: Self::INT16_4.from_data(data)?,
            int16_5: Self::INT16_5.from_data(data)?,
            int16_u: Self::INT16_U.from_data(data)?,
            uint16_1: Self::UINT16_1.from_data(data)?,
            uint16_2: Self::UINT16_2.from_data(data)?,
            uint16_3: Self::UINT16_3.from_data(data)?,
            uint16_4: Self::UINT16_4.from_data(data)?,
            uint16_5: Self::UINT16_5.from_data(data)?,
            uint16_u: Self::UINT16_U.from_data(data)?,
            acc16: Self::ACC16.from_data(data)?,
            acc16_u: Self::ACC16_U.from_data(data)?,
            enum16: Self::ENUM16.from_data(data)?,
            enum16_u: Self::ENUM16_U.from_data(data)?,
            bitfield16: Self::BITFIELD16.from_data(data)?,
            bitfield16_u: Self::BITFIELD16_U.from_data(data)?,
            int32_1: Self::INT32_1.from_data(data)?,
            int32_2: Self::INT32_2.from_data(data)?,
            int32_3: Self::INT32_3.from_data(data)?,
            int32_4: Self::INT32_4.from_data(data)?,
            int32_5: Self::INT32_5.from_data(data)?,
            int32_u: Self::INT32_U.from_data(data)?,
            uint32_1: Self::UINT32_1.from_data(data)?,
            uint32_2: Self::UINT32_2.from_data(data)?,
            uint32_3: Self::UINT32_3.from_data(data)?,
            uint32_4: Self::UINT32_4.from_data(data)?,
            uint32_5: Self::UINT32_5.from_data(data)?,
            uint32_u: Self::UINT32_U.from_data(data)?,
            acc32: Self::ACC32.from_data(data)?,
            acc32_u: Self::ACC32_U.from_data(data)?,
            enum32: Self::ENUM32.from_data(data)?,
            enum32_u: Self::ENUM32_U.from_data(data)?,
            bitfield32: Self::BITFIELD32.from_data(data)?,
            bitfield32_u: Self::BITFIELD32_U.from_data(data)?,
            ipaddr: Self::IPADDR.from_data(data)?,
            ipaddr_u: Self::IPADDR_U.from_data(data)?,
            int64: Self::INT64.from_data(data)?,
            int64_u: Self::INT64_U.from_data(data)?,
            acc64: Self::ACC64.from_data(data)?,
            acc64_u: Self::ACC64_U.from_data(data)?,
            ipv6addr: Self::IPV6ADDR.from_data(data)?,
            ipv6addr_u: Self::IPV6ADDR_U.from_data(data)?,
            float32: Self::FLOAT32.from_data(data)?,
            float32_u: Self::FLOAT32_U.from_data(data)?,
            string: Self::STRING.from_data(data)?,
            string_u: Self::STRING_U.from_data(data)?,
            sunssf_5: Self::SUNSSF_5.from_data(data)?,
            sunssf_6: Self::SUNSSF_6.from_data(data)?,
            sunssf_7: Self::SUNSSF_7.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m63001
    }
}
bitflags::bitflags! {
    #[allow(missing_docs)] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Bitfield16 : u16 {}
}
impl crate::Value for Bitfield16 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Bitfield16> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Bitfield16::from_bits_retain(value)))
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
bitflags::bitflags! {
    #[allow(missing_docs)] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Bitfield16U : u16 {}
}
impl crate::Value for Bitfield16U {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Bitfield16U> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Bitfield16U::from_bits_retain(value)))
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
bitflags::bitflags! {
    #[allow(missing_docs)] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Bitfield32 : u32 {}
}
impl crate::Value for Bitfield32 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Bitfield32> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Bitfield32::from_bits_retain(value)))
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
    #[allow(missing_docs)] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Bitfield32U : u32 {}
}
impl crate::Value for Bitfield32U {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Bitfield32U> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Bitfield32U::from_bits_retain(value)))
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
