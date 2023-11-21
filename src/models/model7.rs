/// Secure Write Response Model (DRAFT 1)
///
/// Include a digital signature over the response
///
/// Notes: Used in conjunction with a Secure Write Request
#[derive(Debug)]
pub struct Model7 {
    /// Request Sequence
    ///
    /// Sequence number from the request
    pub rqseq: u16,
    /// Status
    ///
    /// Status of last write operation
    pub sts: u16,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    pub ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    pub ms: u16,
    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Notes: Shall be advanced for each response
    pub seq: u16,
    /// Alarm
    ///
    /// Bitmask alarm code
    pub alm: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Notes: For future proof
    pub alg: u16,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Notes: The value of N must be at least 4 (64 bits)
    pub n: u16,
}

#[allow(missing_docs)]

impl Model7 {
    pub const RQSEQ: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STS: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const ALM: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
}

impl crate::Model for Model7 {
    const ID: u16 = 7;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            rqseq: Self::RQSEQ.from_data(data)?,
            sts: Self::STS.from_data(data)?,
            ts: Self::TS.from_data(data)?,
            ms: Self::MS.from_data(data)?,
            seq: Self::SEQ.from_data(data)?,
            alm: Self::ALM.from_data(data)?,
            alg: Self::ALG.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
}
