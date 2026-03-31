//! Proxy Server
/// Proxy Server
///
/// Include this block to allow for a proxy server
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model14 {
    /// name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Capabilities
    ///
    /// Bitmask value.  Proxy configuration capabilities
    pub cap: Cap,
    /// Config
    ///
    /// Enumerated value.  Set proxy address type
    pub cfg: u16,
    /// Type
    ///
    /// Enumerate value.  Proxy server type
    pub typ: Typ,
    /// Address
    ///
    /// IPv4 or IPv6 proxy hostname or dotted address (40 chars)
    pub addr: String,
    /// Port
    ///
    /// Proxy port number
    pub port: u16,
    /// Username
    ///
    /// Proxy user name
    pub user: Option<String>,
    /// Password
    ///
    /// Proxy password
    pub pw: Option<String>,
}
#[allow(missing_docs)]
impl Model14 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const CAP: crate::Point<Self, Cap> = crate::Point::new(4, 1, true);
    pub const CFG: crate::Point<Self, u16> = crate::Point::new(5, 1, true);
    pub const TYP: crate::Point<Self, Typ> = crate::Point::new(6, 1, true);
    pub const ADDR: crate::Point<Self, String> = crate::Point::new(7, 20, true);
    pub const PORT: crate::Point<Self, u16> = crate::Point::new(27, 1, true);
    pub const USER: crate::Point<Self, Option<String>> = crate::Point::new(28, 12, true);
    pub const PW: crate::Point<Self, Option<String>> = crate::Point::new(40, 12, true);
}
static MODEL14_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "nam",
        label: "name",
        description: "Interface name (8 chars)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cap",
        label: "Capabilities",
        description: "Bitmask value.  Proxy configuration capabilities",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cfg",
        label: "Config",
        description: "Enumerated value.  Set proxy address type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "typ",
        label: "Type",
        description: "Enumerate value.  Proxy server type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "addr",
        label: "Address",
        description: "IPv4 or IPv6 proxy hostname or dotted address (40 chars)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "port",
        label: "Port",
        description: "Proxy port number",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "user",
        label: "Username",
        description: "Proxy user name",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pw",
        label: "Password",
        description: "Proxy password",
        kind: crate::FieldKind::Point,
    },
];
static MODEL14_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_14",
    label: "Proxy Server",
    description: "Include this block to allow for a proxy server",
    fields: MODEL14_FIELDS,
};
impl crate::GroupMeta for Model14 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL14_GROUP_INFO
    }
}
impl crate::Group for Model14 {
    const LEN: u16 = 52;
}
impl Model14 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                nam: Self::NAM.from_data(data)?,
                cap: Self::CAP.from_data(data)?,
                cfg: Self::CFG.from_data(data)?,
                typ: Self::TYP.from_data(data)?,
                addr: Self::ADDR.from_data(data)?,
                port: Self::PORT.from_data(data)?,
                user: Self::USER.from_data(data)?,
                pw: Self::PW.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Capabilities"] #[doc = " "] #[doc =
    " Bitmask value.  Proxy configuration capabilities"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Cap : u16 { #[allow(missing_docs)] const NoProxy =
    1; #[allow(missing_docs)] const Ipv4Proxy = 2; #[allow(missing_docs)] const Ipv6Proxy
    = 4; }
}
impl crate::Value for Cap {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Cap {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
bitflags::bitflags! {
    #[doc = " Type"] #[doc = " "] #[doc = " Enumerate value.  Proxy server type"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Typ : u16 {}
}
impl crate::Value for Typ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Typ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
impl crate::Model for Model14 {
    const ID: u16 = 14;
    const NAME: &'static str = "model_14";
    const LABEL: &'static str = "Proxy Server";
    const DESCRIPTION: &'static str = "Include this block to allow for a proxy server";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m14
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
