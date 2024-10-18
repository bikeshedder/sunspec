//! Secure Write Sequential Request
/// Secure Write Sequential Request
///
/// Include a digital signature along with the control data
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model6 {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// Notes: A max of 50 (offset, value) pairs are allocated
    pub x: u16,
    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// Notes: X values to follow
    pub off: u16,
    /// Value1
    ///
    /// Value to write to control register at offset
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
    #[allow(missing_docs)]
    pub val51: u16,
    #[allow(missing_docs)]
    pub val52: u16,
    #[allow(missing_docs)]
    pub val53: u16,
    #[allow(missing_docs)]
    pub val54: u16,
    #[allow(missing_docs)]
    pub val55: u16,
    #[allow(missing_docs)]
    pub val56: u16,
    #[allow(missing_docs)]
    pub val57: u16,
    #[allow(missing_docs)]
    pub val58: u16,
    #[allow(missing_docs)]
    pub val59: u16,
    #[allow(missing_docs)]
    pub val60: u16,
    #[allow(missing_docs)]
    pub val61: u16,
    #[allow(missing_docs)]
    pub val62: u16,
    #[allow(missing_docs)]
    pub val63: u16,
    #[allow(missing_docs)]
    pub val64: u16,
    #[allow(missing_docs)]
    pub val65: u16,
    #[allow(missing_docs)]
    pub val66: u16,
    #[allow(missing_docs)]
    pub val67: u16,
    #[allow(missing_docs)]
    pub val68: u16,
    #[allow(missing_docs)]
    pub val69: u16,
    #[allow(missing_docs)]
    pub val70: u16,
    #[allow(missing_docs)]
    pub val71: u16,
    #[allow(missing_docs)]
    pub val72: u16,
    #[allow(missing_docs)]
    pub val73: u16,
    #[allow(missing_docs)]
    pub val74: u16,
    #[allow(missing_docs)]
    pub val75: u16,
    #[allow(missing_docs)]
    pub val76: u16,
    #[allow(missing_docs)]
    pub val77: u16,
    #[allow(missing_docs)]
    pub val78: u16,
    #[allow(missing_docs)]
    pub val79: u16,
    #[allow(missing_docs)]
    pub val80: u16,
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
    pub alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Notes: The value of N must be at least 4 (64 bits)
    pub n: u16,
}
#[allow(missing_docs)]
impl Model6 {
    pub const X: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const OFF: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const VAL1: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const VAL2: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const VAL3: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const VAL4: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const VAL5: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const VAL6: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const VAL7: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const VAL8: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const VAL9: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const VAL10: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const VAL11: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const VAL12: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const VAL13: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, true);
    pub const VAL14: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const VAL15: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const VAL16: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const VAL17: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const VAL18: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const VAL19: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const VAL20: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, true);
    pub const VAL21: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const VAL22: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, true);
    pub const VAL23: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, true);
    pub const VAL24: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, true);
    pub const VAL25: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, true);
    pub const VAL26: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const VAL27: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, true);
    pub const VAL28: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, true);
    pub const VAL29: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, true);
    pub const VAL30: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, true);
    pub const VAL31: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, true);
    pub const VAL32: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, true);
    pub const VAL33: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const VAL34: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const VAL35: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, true);
    pub const VAL36: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, true);
    pub const VAL37: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, true);
    pub const VAL38: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, true);
    pub const VAL39: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, true);
    pub const VAL40: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, true);
    pub const VAL41: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, true);
    pub const VAL42: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, true);
    pub const VAL43: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, true);
    pub const VAL44: crate::PointDef<Self, u16> = crate::PointDef::new(45, 1, true);
    pub const VAL45: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, true);
    pub const VAL46: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, true);
    pub const VAL47: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const VAL48: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const VAL49: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
    pub const VAL50: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, true);
    pub const VAL51: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, true);
    pub const VAL52: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, true);
    pub const VAL53: crate::PointDef<Self, u16> = crate::PointDef::new(54, 1, true);
    pub const VAL54: crate::PointDef<Self, u16> = crate::PointDef::new(55, 1, true);
    pub const VAL55: crate::PointDef<Self, u16> = crate::PointDef::new(56, 1, true);
    pub const VAL56: crate::PointDef<Self, u16> = crate::PointDef::new(57, 1, true);
    pub const VAL57: crate::PointDef<Self, u16> = crate::PointDef::new(58, 1, true);
    pub const VAL58: crate::PointDef<Self, u16> = crate::PointDef::new(59, 1, true);
    pub const VAL59: crate::PointDef<Self, u16> = crate::PointDef::new(60, 1, true);
    pub const VAL60: crate::PointDef<Self, u16> = crate::PointDef::new(61, 1, true);
    pub const VAL61: crate::PointDef<Self, u16> = crate::PointDef::new(62, 1, true);
    pub const VAL62: crate::PointDef<Self, u16> = crate::PointDef::new(63, 1, true);
    pub const VAL63: crate::PointDef<Self, u16> = crate::PointDef::new(64, 1, true);
    pub const VAL64: crate::PointDef<Self, u16> = crate::PointDef::new(65, 1, true);
    pub const VAL65: crate::PointDef<Self, u16> = crate::PointDef::new(66, 1, true);
    pub const VAL66: crate::PointDef<Self, u16> = crate::PointDef::new(67, 1, true);
    pub const VAL67: crate::PointDef<Self, u16> = crate::PointDef::new(68, 1, true);
    pub const VAL68: crate::PointDef<Self, u16> = crate::PointDef::new(69, 1, true);
    pub const VAL69: crate::PointDef<Self, u16> = crate::PointDef::new(70, 1, true);
    pub const VAL70: crate::PointDef<Self, u16> = crate::PointDef::new(71, 1, true);
    pub const VAL71: crate::PointDef<Self, u16> = crate::PointDef::new(72, 1, true);
    pub const VAL72: crate::PointDef<Self, u16> = crate::PointDef::new(73, 1, true);
    pub const VAL73: crate::PointDef<Self, u16> = crate::PointDef::new(74, 1, true);
    pub const VAL74: crate::PointDef<Self, u16> = crate::PointDef::new(75, 1, true);
    pub const VAL75: crate::PointDef<Self, u16> = crate::PointDef::new(76, 1, true);
    pub const VAL76: crate::PointDef<Self, u16> = crate::PointDef::new(77, 1, true);
    pub const VAL77: crate::PointDef<Self, u16> = crate::PointDef::new(78, 1, true);
    pub const VAL78: crate::PointDef<Self, u16> = crate::PointDef::new(79, 1, true);
    pub const VAL79: crate::PointDef<Self, u16> = crate::PointDef::new(80, 1, true);
    pub const VAL80: crate::PointDef<Self, u16> = crate::PointDef::new(81, 1, true);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(82, 2, true);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(84, 1, true);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(85, 1, true);
    pub const ROLE: crate::PointDef<Self, u16> = crate::PointDef::new(86, 1, true);
    pub const ALG: crate::PointDef<Self, Alg> = crate::PointDef::new(88, 1, true);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(89, 1, true);
}
impl crate::Model for Model6 {
    const ID: u16 = 6;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            x: Self::X.from_data(data)?,
            off: Self::OFF.from_data(data)?,
            val1: Self::VAL1.from_data(data)?,
            val2: Self::VAL2.from_data(data)?,
            val3: Self::VAL3.from_data(data)?,
            val4: Self::VAL4.from_data(data)?,
            val5: Self::VAL5.from_data(data)?,
            val6: Self::VAL6.from_data(data)?,
            val7: Self::VAL7.from_data(data)?,
            val8: Self::VAL8.from_data(data)?,
            val9: Self::VAL9.from_data(data)?,
            val10: Self::VAL10.from_data(data)?,
            val11: Self::VAL11.from_data(data)?,
            val12: Self::VAL12.from_data(data)?,
            val13: Self::VAL13.from_data(data)?,
            val14: Self::VAL14.from_data(data)?,
            val15: Self::VAL15.from_data(data)?,
            val16: Self::VAL16.from_data(data)?,
            val17: Self::VAL17.from_data(data)?,
            val18: Self::VAL18.from_data(data)?,
            val19: Self::VAL19.from_data(data)?,
            val20: Self::VAL20.from_data(data)?,
            val21: Self::VAL21.from_data(data)?,
            val22: Self::VAL22.from_data(data)?,
            val23: Self::VAL23.from_data(data)?,
            val24: Self::VAL24.from_data(data)?,
            val25: Self::VAL25.from_data(data)?,
            val26: Self::VAL26.from_data(data)?,
            val27: Self::VAL27.from_data(data)?,
            val28: Self::VAL28.from_data(data)?,
            val29: Self::VAL29.from_data(data)?,
            val30: Self::VAL30.from_data(data)?,
            val31: Self::VAL31.from_data(data)?,
            val32: Self::VAL32.from_data(data)?,
            val33: Self::VAL33.from_data(data)?,
            val34: Self::VAL34.from_data(data)?,
            val35: Self::VAL35.from_data(data)?,
            val36: Self::VAL36.from_data(data)?,
            val37: Self::VAL37.from_data(data)?,
            val38: Self::VAL38.from_data(data)?,
            val39: Self::VAL39.from_data(data)?,
            val40: Self::VAL40.from_data(data)?,
            val41: Self::VAL41.from_data(data)?,
            val42: Self::VAL42.from_data(data)?,
            val43: Self::VAL43.from_data(data)?,
            val44: Self::VAL44.from_data(data)?,
            val45: Self::VAL45.from_data(data)?,
            val46: Self::VAL46.from_data(data)?,
            val47: Self::VAL47.from_data(data)?,
            val48: Self::VAL48.from_data(data)?,
            val49: Self::VAL49.from_data(data)?,
            val50: Self::VAL50.from_data(data)?,
            val51: Self::VAL51.from_data(data)?,
            val52: Self::VAL52.from_data(data)?,
            val53: Self::VAL53.from_data(data)?,
            val54: Self::VAL54.from_data(data)?,
            val55: Self::VAL55.from_data(data)?,
            val56: Self::VAL56.from_data(data)?,
            val57: Self::VAL57.from_data(data)?,
            val58: Self::VAL58.from_data(data)?,
            val59: Self::VAL59.from_data(data)?,
            val60: Self::VAL60.from_data(data)?,
            val61: Self::VAL61.from_data(data)?,
            val62: Self::VAL62.from_data(data)?,
            val63: Self::VAL63.from_data(data)?,
            val64: Self::VAL64.from_data(data)?,
            val65: Self::VAL65.from_data(data)?,
            val66: Self::VAL66.from_data(data)?,
            val67: Self::VAL67.from_data(data)?,
            val68: Self::VAL68.from_data(data)?,
            val69: Self::VAL69.from_data(data)?,
            val70: Self::VAL70.from_data(data)?,
            val71: Self::VAL71.from_data(data)?,
            val72: Self::VAL72.from_data(data)?,
            val73: Self::VAL73.from_data(data)?,
            val74: Self::VAL74.from_data(data)?,
            val75: Self::VAL75.from_data(data)?,
            val76: Self::VAL76.from_data(data)?,
            val77: Self::VAL77.from_data(data)?,
            val78: Self::VAL78.from_data(data)?,
            val79: Self::VAL79.from_data(data)?,
            val80: Self::VAL80.from_data(data)?,
            ts: Self::TS.from_data(data)?,
            ms: Self::MS.from_data(data)?,
            seq: Self::SEQ.from_data(data)?,
            role: Self::ROLE.from_data(data)?,
            alg: Self::ALG.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m6
    }
}
/// Algorithm
///
/// Algorithm used to compute the digital signature
///
/// Notes: For future proof
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Alg {
    /// Notes: For test purposes only
    None = 0,
    #[allow(missing_docs)]
    AesGmac64 = 1,
    #[allow(missing_docs)]
    Ecc256 = 2,
}
impl crate::Value for Alg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Alg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Alg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
