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
    /// Detail: A max of 50 (offset, value) pairs are allocated
    pub x: u16,
    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// Detail: X values to follow
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
impl Model6 {
    pub const X: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const OFF: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const VAL1: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const VAL2: crate::Point<Self, u16> = crate::Point::new(3, 1, true);
    pub const VAL3: crate::Point<Self, u16> = crate::Point::new(4, 1, true);
    pub const VAL4: crate::Point<Self, u16> = crate::Point::new(5, 1, true);
    pub const VAL5: crate::Point<Self, u16> = crate::Point::new(6, 1, true);
    pub const VAL6: crate::Point<Self, u16> = crate::Point::new(7, 1, true);
    pub const VAL7: crate::Point<Self, u16> = crate::Point::new(8, 1, true);
    pub const VAL8: crate::Point<Self, u16> = crate::Point::new(9, 1, true);
    pub const VAL9: crate::Point<Self, u16> = crate::Point::new(10, 1, true);
    pub const VAL10: crate::Point<Self, u16> = crate::Point::new(11, 1, true);
    pub const VAL11: crate::Point<Self, u16> = crate::Point::new(12, 1, true);
    pub const VAL12: crate::Point<Self, u16> = crate::Point::new(13, 1, true);
    pub const VAL13: crate::Point<Self, u16> = crate::Point::new(14, 1, true);
    pub const VAL14: crate::Point<Self, u16> = crate::Point::new(15, 1, true);
    pub const VAL15: crate::Point<Self, u16> = crate::Point::new(16, 1, true);
    pub const VAL16: crate::Point<Self, u16> = crate::Point::new(17, 1, true);
    pub const VAL17: crate::Point<Self, u16> = crate::Point::new(18, 1, true);
    pub const VAL18: crate::Point<Self, u16> = crate::Point::new(19, 1, true);
    pub const VAL19: crate::Point<Self, u16> = crate::Point::new(20, 1, true);
    pub const VAL20: crate::Point<Self, u16> = crate::Point::new(21, 1, true);
    pub const VAL21: crate::Point<Self, u16> = crate::Point::new(22, 1, true);
    pub const VAL22: crate::Point<Self, u16> = crate::Point::new(23, 1, true);
    pub const VAL23: crate::Point<Self, u16> = crate::Point::new(24, 1, true);
    pub const VAL24: crate::Point<Self, u16> = crate::Point::new(25, 1, true);
    pub const VAL25: crate::Point<Self, u16> = crate::Point::new(26, 1, true);
    pub const VAL26: crate::Point<Self, u16> = crate::Point::new(27, 1, true);
    pub const VAL27: crate::Point<Self, u16> = crate::Point::new(28, 1, true);
    pub const VAL28: crate::Point<Self, u16> = crate::Point::new(29, 1, true);
    pub const VAL29: crate::Point<Self, u16> = crate::Point::new(30, 1, true);
    pub const VAL30: crate::Point<Self, u16> = crate::Point::new(31, 1, true);
    pub const VAL31: crate::Point<Self, u16> = crate::Point::new(32, 1, true);
    pub const VAL32: crate::Point<Self, u16> = crate::Point::new(33, 1, true);
    pub const VAL33: crate::Point<Self, u16> = crate::Point::new(34, 1, true);
    pub const VAL34: crate::Point<Self, u16> = crate::Point::new(35, 1, true);
    pub const VAL35: crate::Point<Self, u16> = crate::Point::new(36, 1, true);
    pub const VAL36: crate::Point<Self, u16> = crate::Point::new(37, 1, true);
    pub const VAL37: crate::Point<Self, u16> = crate::Point::new(38, 1, true);
    pub const VAL38: crate::Point<Self, u16> = crate::Point::new(39, 1, true);
    pub const VAL39: crate::Point<Self, u16> = crate::Point::new(40, 1, true);
    pub const VAL40: crate::Point<Self, u16> = crate::Point::new(41, 1, true);
    pub const VAL41: crate::Point<Self, u16> = crate::Point::new(42, 1, true);
    pub const VAL42: crate::Point<Self, u16> = crate::Point::new(43, 1, true);
    pub const VAL43: crate::Point<Self, u16> = crate::Point::new(44, 1, true);
    pub const VAL44: crate::Point<Self, u16> = crate::Point::new(45, 1, true);
    pub const VAL45: crate::Point<Self, u16> = crate::Point::new(46, 1, true);
    pub const VAL46: crate::Point<Self, u16> = crate::Point::new(47, 1, true);
    pub const VAL47: crate::Point<Self, u16> = crate::Point::new(48, 1, true);
    pub const VAL48: crate::Point<Self, u16> = crate::Point::new(49, 1, true);
    pub const VAL49: crate::Point<Self, u16> = crate::Point::new(50, 1, true);
    pub const VAL50: crate::Point<Self, u16> = crate::Point::new(51, 1, true);
    pub const VAL51: crate::Point<Self, u16> = crate::Point::new(52, 1, true);
    pub const VAL52: crate::Point<Self, u16> = crate::Point::new(53, 1, true);
    pub const VAL53: crate::Point<Self, u16> = crate::Point::new(54, 1, true);
    pub const VAL54: crate::Point<Self, u16> = crate::Point::new(55, 1, true);
    pub const VAL55: crate::Point<Self, u16> = crate::Point::new(56, 1, true);
    pub const VAL56: crate::Point<Self, u16> = crate::Point::new(57, 1, true);
    pub const VAL57: crate::Point<Self, u16> = crate::Point::new(58, 1, true);
    pub const VAL58: crate::Point<Self, u16> = crate::Point::new(59, 1, true);
    pub const VAL59: crate::Point<Self, u16> = crate::Point::new(60, 1, true);
    pub const VAL60: crate::Point<Self, u16> = crate::Point::new(61, 1, true);
    pub const VAL61: crate::Point<Self, u16> = crate::Point::new(62, 1, true);
    pub const VAL62: crate::Point<Self, u16> = crate::Point::new(63, 1, true);
    pub const VAL63: crate::Point<Self, u16> = crate::Point::new(64, 1, true);
    pub const VAL64: crate::Point<Self, u16> = crate::Point::new(65, 1, true);
    pub const VAL65: crate::Point<Self, u16> = crate::Point::new(66, 1, true);
    pub const VAL66: crate::Point<Self, u16> = crate::Point::new(67, 1, true);
    pub const VAL67: crate::Point<Self, u16> = crate::Point::new(68, 1, true);
    pub const VAL68: crate::Point<Self, u16> = crate::Point::new(69, 1, true);
    pub const VAL69: crate::Point<Self, u16> = crate::Point::new(70, 1, true);
    pub const VAL70: crate::Point<Self, u16> = crate::Point::new(71, 1, true);
    pub const VAL71: crate::Point<Self, u16> = crate::Point::new(72, 1, true);
    pub const VAL72: crate::Point<Self, u16> = crate::Point::new(73, 1, true);
    pub const VAL73: crate::Point<Self, u16> = crate::Point::new(74, 1, true);
    pub const VAL74: crate::Point<Self, u16> = crate::Point::new(75, 1, true);
    pub const VAL75: crate::Point<Self, u16> = crate::Point::new(76, 1, true);
    pub const VAL76: crate::Point<Self, u16> = crate::Point::new(77, 1, true);
    pub const VAL77: crate::Point<Self, u16> = crate::Point::new(78, 1, true);
    pub const VAL78: crate::Point<Self, u16> = crate::Point::new(79, 1, true);
    pub const VAL79: crate::Point<Self, u16> = crate::Point::new(80, 1, true);
    pub const VAL80: crate::Point<Self, u16> = crate::Point::new(81, 1, true);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(82, 2, true);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(84, 1, true);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(85, 1, true);
    pub const ROLE: crate::Point<Self, u16> = crate::Point::new(86, 1, true);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(88, 1, true);
    pub const N: crate::Point<Self, u16> = crate::Point::new(89, 1, true);
}
static MODEL6_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "x",
        label: "X",
        description: "Number of (offset, value) pairs being written",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "off",
        label: "Offset",
        description: "Starting offset for write operation",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val1",
        label: "Value1",
        description: "Value to write to control register at offset",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val2",
        label: "Val2",
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
        name: "val4",
        label: "Val4",
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
        name: "val6",
        label: "Val6",
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
        name: "val8",
        label: "Val8",
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
        name: "val10",
        label: "Val10",
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
        name: "val12",
        label: "Val12",
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
        name: "val14",
        label: "Val14",
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
        name: "val16",
        label: "Val16",
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
        name: "val18",
        label: "Val18",
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
        name: "val20",
        label: "Val20",
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
        name: "val22",
        label: "Val22",
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
        name: "val24",
        label: "Val24",
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
        name: "val26",
        label: "Val26",
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
        name: "val28",
        label: "Val28",
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
        name: "val30",
        label: "Val30",
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
        name: "val32",
        label: "Val32",
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
        name: "val34",
        label: "Val34",
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
        name: "val36",
        label: "Val36",
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
        name: "val38",
        label: "Val38",
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
        name: "val40",
        label: "Val40",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val41",
        label: "Val41",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val42",
        label: "Val42",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val43",
        label: "Val43",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val44",
        label: "Val44",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val45",
        label: "Val45",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val46",
        label: "Val46",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val47",
        label: "Val47",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val48",
        label: "Val48",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val49",
        label: "Val49",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val50",
        label: "Val50",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val51",
        label: "Val51",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val52",
        label: "Val52",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val53",
        label: "Val53",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val54",
        label: "Val54",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val55",
        label: "Val55",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val56",
        label: "Val56",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val57",
        label: "Val57",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val58",
        label: "Val58",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val59",
        label: "Val59",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val60",
        label: "Val60",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val61",
        label: "Val61",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val62",
        label: "Val62",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val63",
        label: "Val63",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val64",
        label: "Val64",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val65",
        label: "Val65",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val66",
        label: "Val66",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val67",
        label: "Val67",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val68",
        label: "Val68",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val69",
        label: "Val69",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val70",
        label: "Val70",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val71",
        label: "Val71",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val72",
        label: "Val72",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val73",
        label: "Val73",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val74",
        label: "Val74",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val75",
        label: "Val75",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val76",
        label: "Val76",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val77",
        label: "Val77",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val78",
        label: "Val78",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val79",
        label: "Val79",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val80",
        label: "Val80",
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
static MODEL6_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "model_6",
    label: "Secure Write Sequential Request",
    description: "Include a digital signature along with the control data",
    fields: MODEL6_FIELDS,
};
impl crate::GroupMeta for Model6 {
    fn group_info() -> &'static crate::GroupInfo {
        &MODEL6_GROUP_INFO
    }
}
impl crate::Group for Model6 {
    const LEN: u16 = 90;
}
impl Model6 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
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
impl crate::Model for Model6 {
    const ID: u16 = 6;
    const NAME: &'static str = "model_6";
    const LABEL: &'static str = "Secure Write Sequential Request";
    const DESCRIPTION: &'static str = "Include a digital signature along with the control data";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m6
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
