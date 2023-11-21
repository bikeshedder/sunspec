/// Get Device Security Certificate
///
/// Security model for PKI
#[derive(Debug)]
pub struct Model8 {
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    pub fmt: u16,
    /// N
    ///
    /// Number of registers to follow for the certificate
    pub n: u16,
}

#[allow(missing_docs)]

impl Model8 {
    pub const FMT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
}

impl crate::Model for Model8 {
    const ID: u16 = 8;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            fmt: Self::FMT.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
}
