/// Secure Write Request
///
/// Include a digital signature along with the control data
#[derive(Debug)]
pub struct Model5 {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// Notes: A max of 50 (offset, value) pairs are allocated
    pub x: u16,
    /// Offset1
    ///
    /// Offset of control register to write value to
    pub off1: u16,
    /// Value1
    ///
    /// Value to write to control register at offset
    pub val1: u16,
    #[allow(missing_docs)]
    pub off2: u16,
    #[allow(missing_docs)]
    pub val2: u16,
    #[allow(missing_docs)]
    pub off3: u16,
    #[allow(missing_docs)]
    pub val3: u16,
    #[allow(missing_docs)]
    pub off4: u16,
    #[allow(missing_docs)]
    pub val4: u16,
    #[allow(missing_docs)]
    pub off5: u16,
    #[allow(missing_docs)]
    pub val5: u16,
    #[allow(missing_docs)]
    pub off6: u16,
    #[allow(missing_docs)]
    pub val6: u16,
    #[allow(missing_docs)]
    pub off7: u16,
    #[allow(missing_docs)]
    pub val7: u16,
    #[allow(missing_docs)]
    pub off8: u16,
    #[allow(missing_docs)]
    pub val8: u16,
    #[allow(missing_docs)]
    pub off9: u16,
    #[allow(missing_docs)]
    pub val9: u16,
    #[allow(missing_docs)]
    pub off10: u16,
    #[allow(missing_docs)]
    pub val10: u16,
    #[allow(missing_docs)]
    pub off11: u16,
    #[allow(missing_docs)]
    pub val11: u16,
    #[allow(missing_docs)]
    pub off12: u16,
    #[allow(missing_docs)]
    pub val12: u16,
    #[allow(missing_docs)]
    pub off13: u16,
    #[allow(missing_docs)]
    pub val13: u16,
    #[allow(missing_docs)]
    pub off14: u16,
    #[allow(missing_docs)]
    pub val14: u16,
    #[allow(missing_docs)]
    pub off15: u16,
    #[allow(missing_docs)]
    pub val15: u16,
    #[allow(missing_docs)]
    pub off16: u16,
    #[allow(missing_docs)]
    pub val16: u16,
    #[allow(missing_docs)]
    pub off17: u16,
    #[allow(missing_docs)]
    pub val17: u16,
    #[allow(missing_docs)]
    pub off18: u16,
    #[allow(missing_docs)]
    pub val18: u16,
    #[allow(missing_docs)]
    pub off19: u16,
    #[allow(missing_docs)]
    pub val19: u16,
    #[allow(missing_docs)]
    pub off20: u16,
    #[allow(missing_docs)]
    pub val20: u16,
    #[allow(missing_docs)]
    pub off21: u16,
    #[allow(missing_docs)]
    pub val21: u16,
    #[allow(missing_docs)]
    pub off22: u16,
    #[allow(missing_docs)]
    pub val22: u16,
    #[allow(missing_docs)]
    pub off23: u16,
    #[allow(missing_docs)]
    pub val23: u16,
    #[allow(missing_docs)]
    pub off24: u16,
    #[allow(missing_docs)]
    pub val24: u16,
    #[allow(missing_docs)]
    pub off25: u16,
    #[allow(missing_docs)]
    pub val25: u16,
    #[allow(missing_docs)]
    pub off26: u16,
    #[allow(missing_docs)]
    pub val26: u16,
    #[allow(missing_docs)]
    pub off27: u16,
    #[allow(missing_docs)]
    pub val27: u16,
    #[allow(missing_docs)]
    pub off28: u16,
    #[allow(missing_docs)]
    pub val28: u16,
    #[allow(missing_docs)]
    pub off29: u16,
    #[allow(missing_docs)]
    pub val29: u16,
    #[allow(missing_docs)]
    pub off30: u16,
    #[allow(missing_docs)]
    pub val30: u16,
    #[allow(missing_docs)]
    pub off31: u16,
    #[allow(missing_docs)]
    pub val31: u16,
    #[allow(missing_docs)]
    pub off32: u16,
    #[allow(missing_docs)]
    pub val32: u16,
    #[allow(missing_docs)]
    pub off33: u16,
    #[allow(missing_docs)]
    pub val33: u16,
    #[allow(missing_docs)]
    pub off34: u16,
    #[allow(missing_docs)]
    pub val34: u16,
    #[allow(missing_docs)]
    pub off35: u16,
    #[allow(missing_docs)]
    pub val35: u16,
    #[allow(missing_docs)]
    pub off36: u16,
    #[allow(missing_docs)]
    pub val36: u16,
    #[allow(missing_docs)]
    pub off37: u16,
    #[allow(missing_docs)]
    pub val37: u16,
    #[allow(missing_docs)]
    pub off38: u16,
    #[allow(missing_docs)]
    pub val38: u16,
    #[allow(missing_docs)]
    pub off39: u16,
    #[allow(missing_docs)]
    pub val39: u16,
    #[allow(missing_docs)]
    pub off40: u16,
    #[allow(missing_docs)]
    pub val40: u16,
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
    /// Sequence number of request
    ///
    /// Notes: Shall be advanced for each request
    pub seq: u16,
    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Notes: Each controller is assigned a key index that maps to their access control role
    pub role: u16,
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

impl Model5 {
    pub const X: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const OFF1: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const VAL1: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const OFF2: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const VAL2: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const OFF3: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const VAL3: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const OFF4: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const VAL4: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const OFF5: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const VAL5: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const OFF6: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const VAL6: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const OFF7: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const VAL7: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, true);
    pub const OFF8: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const VAL8: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const OFF9: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const VAL9: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const OFF10: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const VAL10: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const OFF11: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, true);
    pub const VAL11: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const OFF12: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, true);
    pub const VAL12: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, true);
    pub const OFF13: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, true);
    pub const VAL13: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, true);
    pub const OFF14: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const VAL14: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, true);
    pub const OFF15: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, true);
    pub const VAL15: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, true);
    pub const OFF16: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, true);
    pub const VAL16: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, true);
    pub const OFF17: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, true);
    pub const VAL17: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const OFF18: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const VAL18: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, true);
    pub const OFF19: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, true);
    pub const VAL19: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, true);
    pub const OFF20: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, true);
    pub const VAL20: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, true);
    pub const OFF21: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, true);
    pub const VAL21: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, true);
    pub const OFF22: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, true);
    pub const VAL22: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, true);
    pub const OFF23: crate::PointDef<Self, u16> = crate::PointDef::new(45, 1, true);
    pub const VAL23: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, true);
    pub const OFF24: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, true);
    pub const VAL24: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const OFF25: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const VAL25: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
    pub const OFF26: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, true);
    pub const VAL26: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, true);
    pub const OFF27: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, true);
    pub const VAL27: crate::PointDef<Self, u16> = crate::PointDef::new(54, 1, true);
    pub const OFF28: crate::PointDef<Self, u16> = crate::PointDef::new(55, 1, true);
    pub const VAL28: crate::PointDef<Self, u16> = crate::PointDef::new(56, 1, true);
    pub const OFF29: crate::PointDef<Self, u16> = crate::PointDef::new(57, 1, true);
    pub const VAL29: crate::PointDef<Self, u16> = crate::PointDef::new(58, 1, true);
    pub const OFF30: crate::PointDef<Self, u16> = crate::PointDef::new(59, 1, true);
    pub const VAL30: crate::PointDef<Self, u16> = crate::PointDef::new(60, 1, true);
    pub const OFF31: crate::PointDef<Self, u16> = crate::PointDef::new(61, 1, true);
    pub const VAL31: crate::PointDef<Self, u16> = crate::PointDef::new(62, 1, true);
    pub const OFF32: crate::PointDef<Self, u16> = crate::PointDef::new(63, 1, true);
    pub const VAL32: crate::PointDef<Self, u16> = crate::PointDef::new(64, 1, true);
    pub const OFF33: crate::PointDef<Self, u16> = crate::PointDef::new(65, 1, true);
    pub const VAL33: crate::PointDef<Self, u16> = crate::PointDef::new(66, 1, true);
    pub const OFF34: crate::PointDef<Self, u16> = crate::PointDef::new(67, 1, true);
    pub const VAL34: crate::PointDef<Self, u16> = crate::PointDef::new(68, 1, true);
    pub const OFF35: crate::PointDef<Self, u16> = crate::PointDef::new(69, 1, true);
    pub const VAL35: crate::PointDef<Self, u16> = crate::PointDef::new(70, 1, true);
    pub const OFF36: crate::PointDef<Self, u16> = crate::PointDef::new(71, 1, true);
    pub const VAL36: crate::PointDef<Self, u16> = crate::PointDef::new(72, 1, true);
    pub const OFF37: crate::PointDef<Self, u16> = crate::PointDef::new(73, 1, true);
    pub const VAL37: crate::PointDef<Self, u16> = crate::PointDef::new(74, 1, true);
    pub const OFF38: crate::PointDef<Self, u16> = crate::PointDef::new(75, 1, true);
    pub const VAL38: crate::PointDef<Self, u16> = crate::PointDef::new(76, 1, true);
    pub const OFF39: crate::PointDef<Self, u16> = crate::PointDef::new(77, 1, true);
    pub const VAL39: crate::PointDef<Self, u16> = crate::PointDef::new(78, 1, true);
    pub const OFF40: crate::PointDef<Self, u16> = crate::PointDef::new(79, 1, true);
    pub const VAL40: crate::PointDef<Self, u16> = crate::PointDef::new(80, 1, true);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(81, 2, true);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(83, 1, true);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(84, 1, true);
    pub const ROLE: crate::PointDef<Self, u16> = crate::PointDef::new(85, 1, true);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(86, 1, true);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(87, 1, true);
}

impl crate::Model for Model5 {
    const ID: u16 = 5;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            x: Self::X
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off1: Self::OFF1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val1: Self::VAL1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off2: Self::OFF2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val2: Self::VAL2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off3: Self::OFF3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val3: Self::VAL3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off4: Self::OFF4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val4: Self::VAL4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off5: Self::OFF5
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val5: Self::VAL5
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off6: Self::OFF6
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val6: Self::VAL6
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off7: Self::OFF7
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val7: Self::VAL7
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off8: Self::OFF8
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val8: Self::VAL8
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off9: Self::OFF9
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val9: Self::VAL9
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off10: Self::OFF10
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val10: Self::VAL10
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off11: Self::OFF11
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val11: Self::VAL11
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off12: Self::OFF12
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val12: Self::VAL12
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off13: Self::OFF13
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val13: Self::VAL13
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off14: Self::OFF14
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val14: Self::VAL14
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off15: Self::OFF15
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val15: Self::VAL15
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off16: Self::OFF16
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val16: Self::VAL16
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off17: Self::OFF17
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val17: Self::VAL17
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off18: Self::OFF18
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val18: Self::VAL18
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off19: Self::OFF19
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val19: Self::VAL19
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off20: Self::OFF20
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val20: Self::VAL20
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off21: Self::OFF21
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val21: Self::VAL21
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off22: Self::OFF22
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val22: Self::VAL22
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off23: Self::OFF23
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val23: Self::VAL23
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off24: Self::OFF24
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val24: Self::VAL24
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off25: Self::OFF25
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val25: Self::VAL25
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off26: Self::OFF26
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val26: Self::VAL26
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off27: Self::OFF27
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val27: Self::VAL27
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off28: Self::OFF28
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val28: Self::VAL28
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off29: Self::OFF29
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val29: Self::VAL29
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off30: Self::OFF30
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val30: Self::VAL30
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off31: Self::OFF31
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val31: Self::VAL31
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off32: Self::OFF32
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val32: Self::VAL32
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off33: Self::OFF33
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val33: Self::VAL33
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off34: Self::OFF34
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val34: Self::VAL34
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off35: Self::OFF35
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val35: Self::VAL35
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off36: Self::OFF36
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val36: Self::VAL36
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off37: Self::OFF37
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val37: Self::VAL37
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off38: Self::OFF38
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val38: Self::VAL38
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off39: Self::OFF39
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val39: Self::VAL39
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off40: Self::OFF40
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val40: Self::VAL40
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
            role: Self::ROLE
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
