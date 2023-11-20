/// Serial Interface
///
/// Include this model for serial interface configuration support
#[derive(Debug)]
pub struct Model17 {
    /// Name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Rate
    ///
    /// Interface baud rate in bits per second
    pub rte: u32,
    /// Bits
    ///
    /// Number of data bits per character
    pub bits: u16,
    /// Parity
    ///
    /// Bitmask value.  Parity setting
    pub pty: u16,
    /// Duplex
    ///
    /// Enumerated value.  Duplex mode
    pub dup: Option<u16>,
    /// Flow Control
    ///
    /// Flow Control Method
    pub flw: Option<u16>,
    /// Interface Type
    ///
    /// Enumerated value.  Interface type
    pub typ: Option<u16>,
    /// Protocol
    ///
    /// Enumerated value. Serial protocol selection
    pub pcol: Option<u16>,
}

#[allow(missing_docs)]

impl Model17 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const RTE: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, true);
    pub const BITS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const PTY: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const DUP: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const FLW: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const PCOL: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
}

impl crate::Model for Model17 {
    const ID: u16 = 17;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            rte: Self::RTE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            bits: Self::BITS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pty: Self::PTY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dup: Self::DUP.from_data(data)?,
            flw: Self::FLW.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
            pcol: Self::PCOL.from_data(data)?,
        })
    }
}
