/// Secure Dataset Read Response
///
/// Compute a digital signature over a specified set of data registers
#[derive(Debug)]
pub struct Model4 {
    /// Request Sequence
    ///
    /// Sequence number from the request
    pub rqseq: u16,
    /// Status
    ///
    /// Status of last read operation
    pub sts: u16,
    /// X
    ///
    /// Number of values from the request
    ///
    /// Notes: A max of 50 values are allocated
    pub x: u16,
    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Notes: Unused values shall return 0xFFFF (unimplemented)
    pub val1: u16,
    #[allow(missing_docs)]
    pub val2: u16,
    #[allow(missing_docs)]
    pub val3: u16,
    #[allow(missing_docs)]
    pub val4: u16,
    #[allow(missing_docs)]
    pub val5: u16,
    #[allow(missing_docs)]
    pub val6: u16,
    #[allow(missing_docs)]
    pub val7: u16,
    #[allow(missing_docs)]
    pub val8: u16,
    #[allow(missing_docs)]
    pub val9: u16,
    #[allow(missing_docs)]
    pub val10: u16,
    #[allow(missing_docs)]
    pub val11: u16,
    #[allow(missing_docs)]
    pub val12: u16,
    #[allow(missing_docs)]
    pub val13: u16,
    #[allow(missing_docs)]
    pub val14: u16,
    #[allow(missing_docs)]
    pub val15: u16,
    #[allow(missing_docs)]
    pub val16: u16,
    #[allow(missing_docs)]
    pub val17: u16,
    #[allow(missing_docs)]
    pub val18: u16,
    #[allow(missing_docs)]
    pub val19: u16,
    #[allow(missing_docs)]
    pub val20: u16,
    #[allow(missing_docs)]
    pub val21: u16,
    #[allow(missing_docs)]
    pub val22: u16,
    #[allow(missing_docs)]
    pub val23: u16,
    #[allow(missing_docs)]
    pub val24: u16,
    #[allow(missing_docs)]
    pub val25: u16,
    #[allow(missing_docs)]
    pub val26: u16,
    #[allow(missing_docs)]
    pub val27: u16,
    #[allow(missing_docs)]
    pub val28: u16,
    #[allow(missing_docs)]
    pub val29: u16,
    #[allow(missing_docs)]
    pub val30: u16,
    #[allow(missing_docs)]
    pub val31: u16,
    #[allow(missing_docs)]
    pub val32: u16,
    #[allow(missing_docs)]
    pub val33: u16,
    #[allow(missing_docs)]
    pub val34: u16,
    #[allow(missing_docs)]
    pub val35: u16,
    #[allow(missing_docs)]
    pub val36: u16,
    #[allow(missing_docs)]
    pub val37: u16,
    #[allow(missing_docs)]
    pub val38: u16,
    #[allow(missing_docs)]
    pub val39: u16,
    #[allow(missing_docs)]
    pub val40: u16,
    #[allow(missing_docs)]
    pub val41: u16,
    #[allow(missing_docs)]
    pub val42: u16,
    #[allow(missing_docs)]
    pub val43: u16,
    #[allow(missing_docs)]
    pub val44: u16,
    #[allow(missing_docs)]
    pub val45: u16,
    #[allow(missing_docs)]
    pub val46: u16,
    #[allow(missing_docs)]
    pub val47: u16,
    #[allow(missing_docs)]
    pub val48: u16,
    #[allow(missing_docs)]
    pub val49: u16,
    #[allow(missing_docs)]
    pub val50: u16,
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

impl Model4 {
    pub const RQSEQ: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STS: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const X: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const VAL1: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const VAL2: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const VAL3: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const VAL4: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const VAL5: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const VAL6: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const VAL7: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const VAL8: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const VAL9: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const VAL10: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const VAL11: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const VAL12: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const VAL13: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const VAL14: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const VAL15: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const VAL16: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, false);
    pub const VAL17: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const VAL18: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const VAL19: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const VAL20: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, false);
    pub const VAL21: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, false);
    pub const VAL22: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, false);
    pub const VAL23: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const VAL24: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, false);
    pub const VAL25: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const VAL26: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, false);
    pub const VAL27: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
    pub const VAL28: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, false);
    pub const VAL29: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, false);
    pub const VAL30: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, false);
    pub const VAL31: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, false);
    pub const VAL32: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, false);
    pub const VAL33: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, false);
    pub const VAL34: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, false);
    pub const VAL35: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, false);
    pub const VAL36: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, false);
    pub const VAL37: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, false);
    pub const VAL38: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, false);
    pub const VAL39: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
    pub const VAL40: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, false);
    pub const VAL41: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, false);
    pub const VAL42: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, false);
    pub const VAL43: crate::PointDef<Self, u16> = crate::PointDef::new(45, 1, false);
    pub const VAL44: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const VAL45: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, false);
    pub const VAL46: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, false);
    pub const VAL47: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, false);
    pub const VAL48: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, false);
    pub const VAL49: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, false);
    pub const VAL50: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, false);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(53, 2, false);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(55, 1, false);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(56, 1, false);
    pub const ALM: crate::PointDef<Self, u16> = crate::PointDef::new(57, 1, false);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(58, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(59, 1, false);
}

impl crate::Model for Model4 {
    const ID: u16 = 4;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            rqseq: Self::RQSEQ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sts: Self::STS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            x: Self::X
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val1: Self::VAL1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val2: Self::VAL2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val3: Self::VAL3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val4: Self::VAL4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val5: Self::VAL5
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val6: Self::VAL6
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val7: Self::VAL7
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val8: Self::VAL8
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val9: Self::VAL9
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val10: Self::VAL10
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val11: Self::VAL11
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val12: Self::VAL12
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val13: Self::VAL13
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val14: Self::VAL14
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val15: Self::VAL15
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val16: Self::VAL16
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val17: Self::VAL17
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val18: Self::VAL18
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val19: Self::VAL19
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val20: Self::VAL20
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val21: Self::VAL21
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val22: Self::VAL22
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val23: Self::VAL23
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val24: Self::VAL24
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val25: Self::VAL25
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val26: Self::VAL26
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val27: Self::VAL27
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val28: Self::VAL28
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val29: Self::VAL29
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val30: Self::VAL30
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val31: Self::VAL31
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val32: Self::VAL32
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val33: Self::VAL33
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val34: Self::VAL34
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val35: Self::VAL35
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val36: Self::VAL36
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val37: Self::VAL37
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val38: Self::VAL38
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val39: Self::VAL39
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val40: Self::VAL40
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val41: Self::VAL41
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val42: Self::VAL42
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val43: Self::VAL43
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val44: Self::VAL44
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val45: Self::VAL45
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val46: Self::VAL46
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val47: Self::VAL47
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val48: Self::VAL48
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val49: Self::VAL49
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val50: Self::VAL50
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ts: Self::TS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ms: Self::MS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            seq: Self::SEQ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alm: Self::ALM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alg: Self::ALG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
