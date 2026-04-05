//! Secure Write Request
/// Secure Write Request
///
/// Include a digital signature along with the control data
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model5 {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// Detail: A max of 50 (offset, value) pairs are allocated
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
    /// Detail: Shall be advanced for each request
    pub seq: u16,
    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Detail: Each controller is assigned a key index that maps to their access control role
    pub role: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Detail: For future proof
    pub alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Detail: The value of N must be at least 4 (64 bits)
    pub n: u16,
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl Model5 {
    pub const X: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const OFF1: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const VAL1: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const OFF2: crate::Point<Self, u16> = crate::Point::new(3, 1, true);
    pub const VAL2: crate::Point<Self, u16> = crate::Point::new(4, 1, true);
    pub const OFF3: crate::Point<Self, u16> = crate::Point::new(5, 1, true);
    pub const VAL3: crate::Point<Self, u16> = crate::Point::new(6, 1, true);
    pub const OFF4: crate::Point<Self, u16> = crate::Point::new(7, 1, true);
    pub const VAL4: crate::Point<Self, u16> = crate::Point::new(8, 1, true);
    pub const OFF5: crate::Point<Self, u16> = crate::Point::new(9, 1, true);
    pub const VAL5: crate::Point<Self, u16> = crate::Point::new(10, 1, true);
    pub const OFF6: crate::Point<Self, u16> = crate::Point::new(11, 1, true);
    pub const VAL6: crate::Point<Self, u16> = crate::Point::new(12, 1, true);
    pub const OFF7: crate::Point<Self, u16> = crate::Point::new(13, 1, true);
    pub const VAL7: crate::Point<Self, u16> = crate::Point::new(14, 1, true);
    pub const OFF8: crate::Point<Self, u16> = crate::Point::new(15, 1, true);
    pub const VAL8: crate::Point<Self, u16> = crate::Point::new(16, 1, true);
    pub const OFF9: crate::Point<Self, u16> = crate::Point::new(17, 1, true);
    pub const VAL9: crate::Point<Self, u16> = crate::Point::new(18, 1, true);
    pub const OFF10: crate::Point<Self, u16> = crate::Point::new(19, 1, true);
    pub const VAL10: crate::Point<Self, u16> = crate::Point::new(20, 1, true);
    pub const OFF11: crate::Point<Self, u16> = crate::Point::new(21, 1, true);
    pub const VAL11: crate::Point<Self, u16> = crate::Point::new(22, 1, true);
    pub const OFF12: crate::Point<Self, u16> = crate::Point::new(23, 1, true);
    pub const VAL12: crate::Point<Self, u16> = crate::Point::new(24, 1, true);
    pub const OFF13: crate::Point<Self, u16> = crate::Point::new(25, 1, true);
    pub const VAL13: crate::Point<Self, u16> = crate::Point::new(26, 1, true);
    pub const OFF14: crate::Point<Self, u16> = crate::Point::new(27, 1, true);
    pub const VAL14: crate::Point<Self, u16> = crate::Point::new(28, 1, true);
    pub const OFF15: crate::Point<Self, u16> = crate::Point::new(29, 1, true);
    pub const VAL15: crate::Point<Self, u16> = crate::Point::new(30, 1, true);
    pub const OFF16: crate::Point<Self, u16> = crate::Point::new(31, 1, true);
    pub const VAL16: crate::Point<Self, u16> = crate::Point::new(32, 1, true);
    pub const OFF17: crate::Point<Self, u16> = crate::Point::new(33, 1, true);
    pub const VAL17: crate::Point<Self, u16> = crate::Point::new(34, 1, true);
    pub const OFF18: crate::Point<Self, u16> = crate::Point::new(35, 1, true);
    pub const VAL18: crate::Point<Self, u16> = crate::Point::new(36, 1, true);
    pub const OFF19: crate::Point<Self, u16> = crate::Point::new(37, 1, true);
    pub const VAL19: crate::Point<Self, u16> = crate::Point::new(38, 1, true);
    pub const OFF20: crate::Point<Self, u16> = crate::Point::new(39, 1, true);
    pub const VAL20: crate::Point<Self, u16> = crate::Point::new(40, 1, true);
    pub const OFF21: crate::Point<Self, u16> = crate::Point::new(41, 1, true);
    pub const VAL21: crate::Point<Self, u16> = crate::Point::new(42, 1, true);
    pub const OFF22: crate::Point<Self, u16> = crate::Point::new(43, 1, true);
    pub const VAL22: crate::Point<Self, u16> = crate::Point::new(44, 1, true);
    pub const OFF23: crate::Point<Self, u16> = crate::Point::new(45, 1, true);
    pub const VAL23: crate::Point<Self, u16> = crate::Point::new(46, 1, true);
    pub const OFF24: crate::Point<Self, u16> = crate::Point::new(47, 1, true);
    pub const VAL24: crate::Point<Self, u16> = crate::Point::new(48, 1, true);
    pub const OFF25: crate::Point<Self, u16> = crate::Point::new(49, 1, true);
    pub const VAL25: crate::Point<Self, u16> = crate::Point::new(50, 1, true);
    pub const OFF26: crate::Point<Self, u16> = crate::Point::new(51, 1, true);
    pub const VAL26: crate::Point<Self, u16> = crate::Point::new(52, 1, true);
    pub const OFF27: crate::Point<Self, u16> = crate::Point::new(53, 1, true);
    pub const VAL27: crate::Point<Self, u16> = crate::Point::new(54, 1, true);
    pub const OFF28: crate::Point<Self, u16> = crate::Point::new(55, 1, true);
    pub const VAL28: crate::Point<Self, u16> = crate::Point::new(56, 1, true);
    pub const OFF29: crate::Point<Self, u16> = crate::Point::new(57, 1, true);
    pub const VAL29: crate::Point<Self, u16> = crate::Point::new(58, 1, true);
    pub const OFF30: crate::Point<Self, u16> = crate::Point::new(59, 1, true);
    pub const VAL30: crate::Point<Self, u16> = crate::Point::new(60, 1, true);
    pub const OFF31: crate::Point<Self, u16> = crate::Point::new(61, 1, true);
    pub const VAL31: crate::Point<Self, u16> = crate::Point::new(62, 1, true);
    pub const OFF32: crate::Point<Self, u16> = crate::Point::new(63, 1, true);
    pub const VAL32: crate::Point<Self, u16> = crate::Point::new(64, 1, true);
    pub const OFF33: crate::Point<Self, u16> = crate::Point::new(65, 1, true);
    pub const VAL33: crate::Point<Self, u16> = crate::Point::new(66, 1, true);
    pub const OFF34: crate::Point<Self, u16> = crate::Point::new(67, 1, true);
    pub const VAL34: crate::Point<Self, u16> = crate::Point::new(68, 1, true);
    pub const OFF35: crate::Point<Self, u16> = crate::Point::new(69, 1, true);
    pub const VAL35: crate::Point<Self, u16> = crate::Point::new(70, 1, true);
    pub const OFF36: crate::Point<Self, u16> = crate::Point::new(71, 1, true);
    pub const VAL36: crate::Point<Self, u16> = crate::Point::new(72, 1, true);
    pub const OFF37: crate::Point<Self, u16> = crate::Point::new(73, 1, true);
    pub const VAL37: crate::Point<Self, u16> = crate::Point::new(74, 1, true);
    pub const OFF38: crate::Point<Self, u16> = crate::Point::new(75, 1, true);
    pub const VAL38: crate::Point<Self, u16> = crate::Point::new(76, 1, true);
    pub const OFF39: crate::Point<Self, u16> = crate::Point::new(77, 1, true);
    pub const VAL39: crate::Point<Self, u16> = crate::Point::new(78, 1, true);
    pub const OFF40: crate::Point<Self, u16> = crate::Point::new(79, 1, true);
    pub const VAL40: crate::Point<Self, u16> = crate::Point::new(80, 1, true);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(81, 2, true);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(83, 1, true);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(84, 1, true);
    pub const ROLE: crate::Point<Self, u16> = crate::Point::new(85, 1, true);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(86, 1, true);
    pub const N: crate::Point<Self, u16> = crate::Point::new(87, 1, true);
}
static MODEL5_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "x",
        label: "X",
        description: "Number of (offset, value) pairs being written",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off1",
        label: "Offset1",
        description: "Offset of control register to write value to",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val1",
        label: "Value1",
        description: "Value to write to control register at offset",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off2",
        label: "Off2",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val2",
        label: "Val2",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off3",
        label: "Off3",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val3",
        label: "Val3",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off4",
        label: "Off4",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val4",
        label: "Val4",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off5",
        label: "Off5",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val5",
        label: "Val5",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off6",
        label: "Off6",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val6",
        label: "Val6",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off7",
        label: "Off7",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val7",
        label: "Val7",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off8",
        label: "Off8",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val8",
        label: "Val8",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off9",
        label: "Off9",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val9",
        label: "Val9",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off10",
        label: "Off10",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val10",
        label: "Val10",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off11",
        label: "Off11",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val11",
        label: "Val11",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off12",
        label: "Off12",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val12",
        label: "Val12",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off13",
        label: "Off13",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val13",
        label: "Val13",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off14",
        label: "Off14",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val14",
        label: "Val14",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off15",
        label: "Off15",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val15",
        label: "Val15",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off16",
        label: "Off16",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val16",
        label: "Val16",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off17",
        label: "Off17",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val17",
        label: "Val17",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off18",
        label: "Off18",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val18",
        label: "Val18",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off19",
        label: "Off19",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val19",
        label: "Val19",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off20",
        label: "Off20",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val20",
        label: "Val20",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off21",
        label: "Off21",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val21",
        label: "Val21",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off22",
        label: "Off22",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val22",
        label: "Val22",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off23",
        label: "Off23",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val23",
        label: "Val23",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off24",
        label: "Off24",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val24",
        label: "Val24",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off25",
        label: "Off25",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val25",
        label: "Val25",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off26",
        label: "Off26",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val26",
        label: "Val26",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off27",
        label: "Off27",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val27",
        label: "Val27",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off28",
        label: "Off28",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val28",
        label: "Val28",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off29",
        label: "Off29",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val29",
        label: "Val29",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off30",
        label: "Off30",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val30",
        label: "Val30",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off31",
        label: "Off31",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val31",
        label: "Val31",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off32",
        label: "Off32",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val32",
        label: "Val32",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off33",
        label: "Off33",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val33",
        label: "Val33",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off34",
        label: "Off34",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val34",
        label: "Val34",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off35",
        label: "Off35",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val35",
        label: "Val35",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off36",
        label: "Off36",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val36",
        label: "Val36",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off37",
        label: "Off37",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val37",
        label: "Val37",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off38",
        label: "Off38",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val38",
        label: "Val38",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off39",
        label: "Off39",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val39",
        label: "Val39",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off40",
        label: "Off40",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val40",
        label: "Val40",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ts",
        label: "Timestamp",
        description: "Timestamp value is the number of seconds since January 1, 2000",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ms",
        label: "Milliseconds",
        description: "Millisecond counter 0-999",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "seq",
        label: "Sequence",
        description: "Sequence number of request",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "role",
        label: "Role",
        description: "Signing key used 0-5",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "alg",
        label: "Algorithm",
        description: "Algorithm used to compute the digital signature",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n",
        label: "N",
        description: "Number of registers comprising the digital signature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "repeating",
        label: "repeating",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<Repeating as crate::GroupMeta>::group_info),
    },
];
static MODEL5_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_5",
    label: "Secure Write Request",
    description: "Include a digital signature along with the control data",
    fields: MODEL5_FIELDS,
};
impl crate::GroupMeta for Model5 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL5_GROUP_INFO
    }
}
impl crate::Group for Model5 {
    const LEN: u16 = 88;
}
impl Model5 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                x: Self::X.from_data(data)?,
                off1: Self::OFF1.from_data(data)?,
                val1: Self::VAL1.from_data(data)?,
                off2: Self::OFF2.from_data(data)?,
                val2: Self::VAL2.from_data(data)?,
                off3: Self::OFF3.from_data(data)?,
                val3: Self::VAL3.from_data(data)?,
                off4: Self::OFF4.from_data(data)?,
                val4: Self::VAL4.from_data(data)?,
                off5: Self::OFF5.from_data(data)?,
                val5: Self::VAL5.from_data(data)?,
                off6: Self::OFF6.from_data(data)?,
                val6: Self::VAL6.from_data(data)?,
                off7: Self::OFF7.from_data(data)?,
                val7: Self::VAL7.from_data(data)?,
                off8: Self::OFF8.from_data(data)?,
                val8: Self::VAL8.from_data(data)?,
                off9: Self::OFF9.from_data(data)?,
                val9: Self::VAL9.from_data(data)?,
                off10: Self::OFF10.from_data(data)?,
                val10: Self::VAL10.from_data(data)?,
                off11: Self::OFF11.from_data(data)?,
                val11: Self::VAL11.from_data(data)?,
                off12: Self::OFF12.from_data(data)?,
                val12: Self::VAL12.from_data(data)?,
                off13: Self::OFF13.from_data(data)?,
                val13: Self::VAL13.from_data(data)?,
                off14: Self::OFF14.from_data(data)?,
                val14: Self::VAL14.from_data(data)?,
                off15: Self::OFF15.from_data(data)?,
                val15: Self::VAL15.from_data(data)?,
                off16: Self::OFF16.from_data(data)?,
                val16: Self::VAL16.from_data(data)?,
                off17: Self::OFF17.from_data(data)?,
                val17: Self::VAL17.from_data(data)?,
                off18: Self::OFF18.from_data(data)?,
                val18: Self::VAL18.from_data(data)?,
                off19: Self::OFF19.from_data(data)?,
                val19: Self::VAL19.from_data(data)?,
                off20: Self::OFF20.from_data(data)?,
                val20: Self::VAL20.from_data(data)?,
                off21: Self::OFF21.from_data(data)?,
                val21: Self::VAL21.from_data(data)?,
                off22: Self::OFF22.from_data(data)?,
                val22: Self::VAL22.from_data(data)?,
                off23: Self::OFF23.from_data(data)?,
                val23: Self::VAL23.from_data(data)?,
                off24: Self::OFF24.from_data(data)?,
                val24: Self::VAL24.from_data(data)?,
                off25: Self::OFF25.from_data(data)?,
                val25: Self::VAL25.from_data(data)?,
                off26: Self::OFF26.from_data(data)?,
                val26: Self::VAL26.from_data(data)?,
                off27: Self::OFF27.from_data(data)?,
                val27: Self::VAL27.from_data(data)?,
                off28: Self::OFF28.from_data(data)?,
                val28: Self::VAL28.from_data(data)?,
                off29: Self::OFF29.from_data(data)?,
                val29: Self::VAL29.from_data(data)?,
                off30: Self::OFF30.from_data(data)?,
                val30: Self::VAL30.from_data(data)?,
                off31: Self::OFF31.from_data(data)?,
                val31: Self::VAL31.from_data(data)?,
                off32: Self::OFF32.from_data(data)?,
                val32: Self::VAL32.from_data(data)?,
                off33: Self::OFF33.from_data(data)?,
                val33: Self::VAL33.from_data(data)?,
                off34: Self::OFF34.from_data(data)?,
                val34: Self::VAL34.from_data(data)?,
                off35: Self::OFF35.from_data(data)?,
                val35: Self::VAL35.from_data(data)?,
                off36: Self::OFF36.from_data(data)?,
                val36: Self::VAL36.from_data(data)?,
                off37: Self::OFF37.from_data(data)?,
                val37: Self::VAL37.from_data(data)?,
                off38: Self::OFF38.from_data(data)?,
                val38: Self::VAL38.from_data(data)?,
                off39: Self::OFF39.from_data(data)?,
                val39: Self::VAL39.from_data(data)?,
                off40: Self::OFF40.from_data(data)?,
                val40: Self::VAL40.from_data(data)?,
                ts: Self::TS.from_data(data)?,
                ms: Self::MS.from_data(data)?,
                seq: Self::SEQ.from_data(data)?,
                role: Self::ROLE.from_data(data)?,
                alg: Self::ALG.from_data(data)?,
                n: Self::N.from_data(data)?,
                repeating,
            },
        ))
    }
}
/// Algorithm
///
/// Algorithm used to compute the digital signature
///
/// Detail: For future proof
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Alg {
    /// Detail: For test purposes only
    None,
    #[allow(missing_docs)]
    AesGmac64,
    #[allow(missing_docs)]
    Ecc256,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Alg {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::AesGmac64,
            2 => Self::Ecc256,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::AesGmac64 => 1,
            Self::Ecc256 => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Alg {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    /// DS
    ///
    /// Digital Signature
    pub ds: u16,
}
#[allow(missing_docs)]
impl Repeating {
    pub const DS: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
}
static REPEATING_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "ds",
    label: "DS",
    description: "Digital Signature",
    kind: crate::FieldKind::Point,
}];
static REPEATING_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "repeating",
    label: "repeating",
    description: "",
    fields: REPEATING_FIELDS,
};
impl crate::GroupMeta for Repeating {
    fn group_info() -> &'static crate::GroupInfo {
        &REPEATING_GROUP_INFO
    }
}
impl crate::Group for Repeating {
    const LEN: u16 = 1;
}
impl Repeating {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ds: Self::DS.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Repeating as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Repeating::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for Model5 {
    const ID: u16 = 5;
    const NAME: &'static str = "model_5";
    const LABEL: &'static str = "Secure Write Request";
    const DESCRIPTION: &'static str = "Include a digital signature along with the control data";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m5
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
