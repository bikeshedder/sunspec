/// This struct contains the addresses of all discovered models.
#[derive(Debug, Default)]
pub struct Models {
    /// Common
    pub m1: crate::ModelAddr<Model1>,
    /// Basic Aggregator
    pub m2: crate::ModelAddr<Model2>,
    /// Secure Dataset Read Request
    pub m3: crate::ModelAddr<Model3>,
    /// Secure Dataset Read Response
    pub m4: crate::ModelAddr<Model4>,
    /// Secure Write Request
    pub m5: crate::ModelAddr<Model5>,
    /// Secure Write Sequential Request
    pub m6: crate::ModelAddr<Model6>,
    /// Secure Write Response Model (DRAFT 1)
    pub m7: crate::ModelAddr<Model7>,
    /// Get Device Security Certificate
    pub m8: crate::ModelAddr<Model8>,
    /// Set Operator Security Certificate
    pub m9: crate::ModelAddr<Model9>,
    /// Communication Interface Header
    pub m10: crate::ModelAddr<Model10>,
    /// Ethernet Link Layer
    pub m11: crate::ModelAddr<Model11>,
    /// IPv4
    pub m12: crate::ModelAddr<Model12>,
    /// IPv6
    pub m13: crate::ModelAddr<Model13>,
    /// Proxy Server
    pub m14: crate::ModelAddr<Model14>,
    /// Interface Counters Model
    pub m15: crate::ModelAddr<Model15>,
    /// Simple IP Network
    pub m16: crate::ModelAddr<Model16>,
    /// Serial Interface
    pub m17: crate::ModelAddr<Model17>,
    /// Cellular Link
    pub m18: crate::ModelAddr<Model18>,
    /// PPP Link
    pub m19: crate::ModelAddr<Model19>,
    /// Inverter (Single Phase)
    pub m101: crate::ModelAddr<Model101>,
    /// Inverter (Split-Phase)
    pub m102: crate::ModelAddr<Model102>,
    /// Inverter (Three Phase)
    pub m103: crate::ModelAddr<Model103>,
    /// Inverter (Single Phase) FLOAT
    pub m111: crate::ModelAddr<Model111>,
    /// Inverter (Split Phase) FLOAT
    pub m112: crate::ModelAddr<Model112>,
    /// Inverter (Three Phase) FLOAT
    pub m113: crate::ModelAddr<Model113>,
    /// Nameplate
    pub m120: crate::ModelAddr<Model120>,
    /// Basic Settings
    pub m121: crate::ModelAddr<Model121>,
    /// Measurements_Status
    pub m122: crate::ModelAddr<Model122>,
    /// Immediate Controls
    pub m123: crate::ModelAddr<Model123>,
    /// Storage
    pub m124: crate::ModelAddr<Model124>,
    /// Pricing
    pub m125: crate::ModelAddr<Model125>,
    /// Static Volt-VAR
    pub m126: crate::ModelAddr<Model126>,
    /// Freq-Watt Param
    pub m127: crate::ModelAddr<Model127>,
    /// Dynamic Reactive Current
    pub m128: crate::ModelAddr<Model128>,
    /// LVRTD
    pub m129: crate::ModelAddr<Model129>,
    /// HVRTD
    pub m130: crate::ModelAddr<Model130>,
    /// Watt-PF
    pub m131: crate::ModelAddr<Model131>,
    /// Volt-Watt
    pub m132: crate::ModelAddr<Model132>,
    /// Basic Scheduling
    pub m133: crate::ModelAddr<Model133>,
    /// Freq-Watt Crv
    pub m134: crate::ModelAddr<Model134>,
    /// LFRT
    pub m135: crate::ModelAddr<Model135>,
    /// HFRT
    pub m136: crate::ModelAddr<Model136>,
    /// LVRTC
    pub m137: crate::ModelAddr<Model137>,
    /// HVRTC
    pub m138: crate::ModelAddr<Model138>,
    /// LVRTX
    pub m139: crate::ModelAddr<Model139>,
    /// HVRTX
    pub m140: crate::ModelAddr<Model140>,
    /// LFRTC
    pub m141: crate::ModelAddr<Model141>,
    /// HFRTC
    pub m142: crate::ModelAddr<Model142>,
    /// LFRTX
    pub m143: crate::ModelAddr<Model143>,
    /// HFRTX
    pub m144: crate::ModelAddr<Model144>,
    /// Extended Settings
    pub m145: crate::ModelAddr<Model145>,
    /// Multiple MPPT Inverter Extension Model
    pub m160: crate::ModelAddr<Model160>,
    /// Meter (Single Phase)single phase (AN or AB) meter
    pub m201: crate::ModelAddr<Model201>,
    /// split single phase (ABN) meter
    pub m202: crate::ModelAddr<Model202>,
    /// wye-connect three phase (abcn) meter
    pub m203: crate::ModelAddr<Model203>,
    /// delta-connect three phase (abc) meter
    pub m204: crate::ModelAddr<Model204>,
    /// single phase (AN or AB) meter
    pub m211: crate::ModelAddr<Model211>,
    /// split single phase (ABN) meter
    pub m212: crate::ModelAddr<Model212>,
    /// wye-connect three phase (abcn) meter
    pub m213: crate::ModelAddr<Model213>,
    /// delta-connect three phase (abc) meter
    pub m214: crate::ModelAddr<Model214>,
    /// Secure AC Meter Selected Readings
    pub m220: crate::ModelAddr<Model220>,
    /// Irradiance Model
    pub m302: crate::ModelAddr<Model302>,
    /// Back of Module Temperature Model
    pub m303: crate::ModelAddr<Model303>,
    /// Inclinometer Model
    pub m304: crate::ModelAddr<Model304>,
    /// GPS
    pub m305: crate::ModelAddr<Model305>,
    /// Reference Point Model
    pub m306: crate::ModelAddr<Model306>,
    /// Base Met
    pub m307: crate::ModelAddr<Model307>,
    /// Mini Met Model
    pub m308: crate::ModelAddr<Model308>,
    /// String Combiner (Current)
    pub m401: crate::ModelAddr<Model401>,
    /// String Combiner (Advanced)
    pub m402: crate::ModelAddr<Model402>,
    /// String Combiner (Current)
    pub m403: crate::ModelAddr<Model403>,
    /// String Combiner (Advanced)
    pub m404: crate::ModelAddr<Model404>,
    /// Solar Module
    pub m501: crate::ModelAddr<Model501>,
    /// Solar Module
    pub m502: crate::ModelAddr<Model502>,
    /// Tracker Controller DRAFT 2
    pub m601: crate::ModelAddr<Model601>,
    /// Energy Storage Base Model (DEPRECATED)
    pub m801: crate::ModelAddr<Model801>,
    /// Battery Base Model
    pub m802: crate::ModelAddr<Model802>,
    /// Lithium-Ion Battery Bank Model
    pub m803: crate::ModelAddr<Model803>,
    /// Lithium-Ion String Model
    pub m804: crate::ModelAddr<Model804>,
    /// Lithium-Ion Module Model
    pub m805: crate::ModelAddr<Model805>,
    /// Flow Battery Model
    pub m806: crate::ModelAddr<Model806>,
    /// Flow Battery String Model
    pub m807: crate::ModelAddr<Model807>,
    /// Flow Battery Module Model
    pub m808: crate::ModelAddr<Model808>,
    /// Flow Battery Stack Model
    pub m809: crate::ModelAddr<Model809>,
    /// SunSpec Test Model 1
    pub m63001: crate::ModelAddr<Model63001>,
    /// SunSpec Test Model 2
    pub m63002: crate::ModelAddr<Model63002>,
    /// Veris Status and Configuration
    pub m64001: crate::ModelAddr<Model64001>,
    /// Mersen GreenString
    pub m64020: crate::ModelAddr<Model64020>,
    /// Eltek Inverter Extension
    pub m64101: crate::ModelAddr<Model64101>,
    /// OutBack AXS device
    pub m64110: crate::ModelAddr<Model64110>,
    /// Basic Charge Controller
    pub m64111: crate::ModelAddr<Model64111>,
    /// OutBack FM Charge Controller
    pub m64112: crate::ModelAddr<Model64112>,
}

impl Models {
    /// Returns a list of all supported model ids
    pub fn supported_model_ids(&self) -> Vec<u16> {
        let mut v = Vec::with_capacity(92);
        if self.m1.addr != 0 {
            v.push(1);
        }
        if self.m2.addr != 0 {
            v.push(2);
        }
        if self.m3.addr != 0 {
            v.push(3);
        }
        if self.m4.addr != 0 {
            v.push(4);
        }
        if self.m5.addr != 0 {
            v.push(5);
        }
        if self.m6.addr != 0 {
            v.push(6);
        }
        if self.m7.addr != 0 {
            v.push(7);
        }
        if self.m8.addr != 0 {
            v.push(8);
        }
        if self.m9.addr != 0 {
            v.push(9);
        }
        if self.m10.addr != 0 {
            v.push(10);
        }
        if self.m11.addr != 0 {
            v.push(11);
        }
        if self.m12.addr != 0 {
            v.push(12);
        }
        if self.m13.addr != 0 {
            v.push(13);
        }
        if self.m14.addr != 0 {
            v.push(14);
        }
        if self.m15.addr != 0 {
            v.push(15);
        }
        if self.m16.addr != 0 {
            v.push(16);
        }
        if self.m17.addr != 0 {
            v.push(17);
        }
        if self.m18.addr != 0 {
            v.push(18);
        }
        if self.m19.addr != 0 {
            v.push(19);
        }
        if self.m101.addr != 0 {
            v.push(101);
        }
        if self.m102.addr != 0 {
            v.push(102);
        }
        if self.m103.addr != 0 {
            v.push(103);
        }
        if self.m111.addr != 0 {
            v.push(111);
        }
        if self.m112.addr != 0 {
            v.push(112);
        }
        if self.m113.addr != 0 {
            v.push(113);
        }
        if self.m120.addr != 0 {
            v.push(120);
        }
        if self.m121.addr != 0 {
            v.push(121);
        }
        if self.m122.addr != 0 {
            v.push(122);
        }
        if self.m123.addr != 0 {
            v.push(123);
        }
        if self.m124.addr != 0 {
            v.push(124);
        }
        if self.m125.addr != 0 {
            v.push(125);
        }
        if self.m126.addr != 0 {
            v.push(126);
        }
        if self.m127.addr != 0 {
            v.push(127);
        }
        if self.m128.addr != 0 {
            v.push(128);
        }
        if self.m129.addr != 0 {
            v.push(129);
        }
        if self.m130.addr != 0 {
            v.push(130);
        }
        if self.m131.addr != 0 {
            v.push(131);
        }
        if self.m132.addr != 0 {
            v.push(132);
        }
        if self.m133.addr != 0 {
            v.push(133);
        }
        if self.m134.addr != 0 {
            v.push(134);
        }
        if self.m135.addr != 0 {
            v.push(135);
        }
        if self.m136.addr != 0 {
            v.push(136);
        }
        if self.m137.addr != 0 {
            v.push(137);
        }
        if self.m138.addr != 0 {
            v.push(138);
        }
        if self.m139.addr != 0 {
            v.push(139);
        }
        if self.m140.addr != 0 {
            v.push(140);
        }
        if self.m141.addr != 0 {
            v.push(141);
        }
        if self.m142.addr != 0 {
            v.push(142);
        }
        if self.m143.addr != 0 {
            v.push(143);
        }
        if self.m144.addr != 0 {
            v.push(144);
        }
        if self.m145.addr != 0 {
            v.push(145);
        }
        if self.m160.addr != 0 {
            v.push(160);
        }
        if self.m201.addr != 0 {
            v.push(201);
        }
        if self.m202.addr != 0 {
            v.push(202);
        }
        if self.m203.addr != 0 {
            v.push(203);
        }
        if self.m204.addr != 0 {
            v.push(204);
        }
        if self.m211.addr != 0 {
            v.push(211);
        }
        if self.m212.addr != 0 {
            v.push(212);
        }
        if self.m213.addr != 0 {
            v.push(213);
        }
        if self.m214.addr != 0 {
            v.push(214);
        }
        if self.m220.addr != 0 {
            v.push(220);
        }
        if self.m302.addr != 0 {
            v.push(302);
        }
        if self.m303.addr != 0 {
            v.push(303);
        }
        if self.m304.addr != 0 {
            v.push(304);
        }
        if self.m305.addr != 0 {
            v.push(305);
        }
        if self.m306.addr != 0 {
            v.push(306);
        }
        if self.m307.addr != 0 {
            v.push(307);
        }
        if self.m308.addr != 0 {
            v.push(308);
        }
        if self.m401.addr != 0 {
            v.push(401);
        }
        if self.m402.addr != 0 {
            v.push(402);
        }
        if self.m403.addr != 0 {
            v.push(403);
        }
        if self.m404.addr != 0 {
            v.push(404);
        }
        if self.m501.addr != 0 {
            v.push(501);
        }
        if self.m502.addr != 0 {
            v.push(502);
        }
        if self.m601.addr != 0 {
            v.push(601);
        }
        if self.m801.addr != 0 {
            v.push(801);
        }
        if self.m802.addr != 0 {
            v.push(802);
        }
        if self.m803.addr != 0 {
            v.push(803);
        }
        if self.m804.addr != 0 {
            v.push(804);
        }
        if self.m805.addr != 0 {
            v.push(805);
        }
        if self.m806.addr != 0 {
            v.push(806);
        }
        if self.m807.addr != 0 {
            v.push(807);
        }
        if self.m808.addr != 0 {
            v.push(808);
        }
        if self.m809.addr != 0 {
            v.push(809);
        }
        if self.m63001.addr != 0 {
            v.push(63001);
        }
        if self.m63002.addr != 0 {
            v.push(63002);
        }
        if self.m64001.addr != 0 {
            v.push(64001);
        }
        if self.m64020.addr != 0 {
            v.push(64020);
        }
        if self.m64101.addr != 0 {
            v.push(64101);
        }
        if self.m64110.addr != 0 {
            v.push(64110);
        }
        if self.m64111.addr != 0 {
            v.push(64111);
        }
        if self.m64112.addr != 0 {
            v.push(64112);
        }
        v
    }

    /// Set address and length of the given model.
    ///
    /// This method is used by the model discovery.
    pub fn set_addr(&mut self, model_id: u16, addr: u16, len: u16) -> bool {
        match model_id {
            1 => self.m1.set_addr(addr, len),
            2 => self.m2.set_addr(addr, len),
            3 => self.m3.set_addr(addr, len),
            4 => self.m4.set_addr(addr, len),
            5 => self.m5.set_addr(addr, len),
            6 => self.m6.set_addr(addr, len),
            7 => self.m7.set_addr(addr, len),
            8 => self.m8.set_addr(addr, len),
            9 => self.m9.set_addr(addr, len),
            10 => self.m10.set_addr(addr, len),
            11 => self.m11.set_addr(addr, len),
            12 => self.m12.set_addr(addr, len),
            13 => self.m13.set_addr(addr, len),
            14 => self.m14.set_addr(addr, len),
            15 => self.m15.set_addr(addr, len),
            16 => self.m16.set_addr(addr, len),
            17 => self.m17.set_addr(addr, len),
            18 => self.m18.set_addr(addr, len),
            19 => self.m19.set_addr(addr, len),
            101 => self.m101.set_addr(addr, len),
            102 => self.m102.set_addr(addr, len),
            103 => self.m103.set_addr(addr, len),
            111 => self.m111.set_addr(addr, len),
            112 => self.m112.set_addr(addr, len),
            113 => self.m113.set_addr(addr, len),
            120 => self.m120.set_addr(addr, len),
            121 => self.m121.set_addr(addr, len),
            122 => self.m122.set_addr(addr, len),
            123 => self.m123.set_addr(addr, len),
            124 => self.m124.set_addr(addr, len),
            125 => self.m125.set_addr(addr, len),
            126 => self.m126.set_addr(addr, len),
            127 => self.m127.set_addr(addr, len),
            128 => self.m128.set_addr(addr, len),
            129 => self.m129.set_addr(addr, len),
            130 => self.m130.set_addr(addr, len),
            131 => self.m131.set_addr(addr, len),
            132 => self.m132.set_addr(addr, len),
            133 => self.m133.set_addr(addr, len),
            134 => self.m134.set_addr(addr, len),
            135 => self.m135.set_addr(addr, len),
            136 => self.m136.set_addr(addr, len),
            137 => self.m137.set_addr(addr, len),
            138 => self.m138.set_addr(addr, len),
            139 => self.m139.set_addr(addr, len),
            140 => self.m140.set_addr(addr, len),
            141 => self.m141.set_addr(addr, len),
            142 => self.m142.set_addr(addr, len),
            143 => self.m143.set_addr(addr, len),
            144 => self.m144.set_addr(addr, len),
            145 => self.m145.set_addr(addr, len),
            160 => self.m160.set_addr(addr, len),
            201 => self.m201.set_addr(addr, len),
            202 => self.m202.set_addr(addr, len),
            203 => self.m203.set_addr(addr, len),
            204 => self.m204.set_addr(addr, len),
            211 => self.m211.set_addr(addr, len),
            212 => self.m212.set_addr(addr, len),
            213 => self.m213.set_addr(addr, len),
            214 => self.m214.set_addr(addr, len),
            220 => self.m220.set_addr(addr, len),
            302 => self.m302.set_addr(addr, len),
            303 => self.m303.set_addr(addr, len),
            304 => self.m304.set_addr(addr, len),
            305 => self.m305.set_addr(addr, len),
            306 => self.m306.set_addr(addr, len),
            307 => self.m307.set_addr(addr, len),
            308 => self.m308.set_addr(addr, len),
            401 => self.m401.set_addr(addr, len),
            402 => self.m402.set_addr(addr, len),
            403 => self.m403.set_addr(addr, len),
            404 => self.m404.set_addr(addr, len),
            501 => self.m501.set_addr(addr, len),
            502 => self.m502.set_addr(addr, len),
            601 => self.m601.set_addr(addr, len),
            801 => self.m801.set_addr(addr, len),
            802 => self.m802.set_addr(addr, len),
            803 => self.m803.set_addr(addr, len),
            804 => self.m804.set_addr(addr, len),
            805 => self.m805.set_addr(addr, len),
            806 => self.m806.set_addr(addr, len),
            807 => self.m807.set_addr(addr, len),
            808 => self.m808.set_addr(addr, len),
            809 => self.m809.set_addr(addr, len),
            63001 => self.m63001.set_addr(addr, len),
            63002 => self.m63002.set_addr(addr, len),
            64001 => self.m64001.set_addr(addr, len),
            64020 => self.m64020.set_addr(addr, len),
            64101 => self.m64101.set_addr(addr, len),
            64110 => self.m64110.set_addr(addr, len),
            64111 => self.m64111.set_addr(addr, len),
            64112 => self.m64112.set_addr(addr, len),
            _ => {
                return false;
            }
        }
        true
    }
}

/// Common
///
/// All SunSpec compliant devices must include this as the first model
#[derive(Debug)]
pub struct Model1 {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    pub mn: String,
    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    pub md: String,
    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    pub opt: Option<String>,
    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    pub vr: Option<String>,
    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    pub sn: String,
    /// Device Address
    ///
    /// Modbus device address
    pub da: Option<u16>,
}

#[allow(missing_docs)]

impl Model1 {
    pub const MN: crate::PointDef<Self, String> = crate::PointDef::new(0, 16, false);
    pub const MD: crate::PointDef<Self, String> = crate::PointDef::new(16, 16, false);
    pub const OPT: crate::PointDef<Self, String> = crate::PointDef::new(32, 8, false);
    pub const VR: crate::PointDef<Self, String> = crate::PointDef::new(40, 8, false);
    pub const SN: crate::PointDef<Self, String> = crate::PointDef::new(48, 16, false);
    pub const DA: crate::PointDef<Self, u16> = crate::PointDef::new(64, 1, true);
}

impl crate::Model for Model1 {
    const ID: u16 = 1;
    const LENGTH: u16 = 66;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            mn: Self::MN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            md: Self::MD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            opt: Self::OPT.from_data(data)?,
            vr: Self::VR.from_data(data)?,
            sn: Self::SN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            da: Self::DA.from_data(data)?,
        })
    }
}

/// Basic Aggregator
///
/// Aggregates a collection of models for a given model id
#[derive(Debug)]
pub struct Model2 {
    /// AID
    ///
    /// Aggregated model id
    pub aid: u16,
    /// N
    ///
    /// Number of aggregated models
    pub n: u16,
    /// UN
    ///
    /// Update Number.  Incrementing number each time the mapping is changed.  If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before.  Otherwise the entire model must be read to refresh the changes
    pub un: u16,
    /// Status
    ///
    /// Enumerated status code
    pub st: u16,
    /// Vendor Status
    ///
    /// Vendor specific status code
    pub stvnd: Option<u16>,
    /// Event Code
    ///
    /// Bitmask event code
    pub evt: u32,
    /// Vendor Event Code
    ///
    /// Vendor specific event code
    pub evtvnd: Option<u32>,
    /// Control
    ///
    /// Control register for all aggregated devices
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    pub ctlvnd: Option<u32>,
    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    pub ctlvl: Option<u32>,
}

#[allow(missing_docs)]

impl Model2 {
    pub const AID: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const UN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CTLVND: crate::PointDef<Self, u32> = crate::PointDef::new(10, 2, false);
    pub const CTLVL: crate::PointDef<Self, u32> = crate::PointDef::new(12, 2, false);
}

impl crate::Model for Model2 {
    const ID: u16 = 2;
    const LENGTH: u16 = 14;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            aid: Self::AID
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            un: Self::UN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctlvnd: Self::CTLVND.from_data(data)?,
            ctlvl: Self::CTLVL.from_data(data)?,
        })
    }
}

/// Secure Dataset Read Request
///
/// Request a digital signature over a specified set of data registers
///
/// Notes: Used in conjunction with Secure Dataset Read Response Model
#[derive(Debug)]
pub struct Model3 {
    /// X
    ///
    /// Number of registers being requested
    ///
    /// Notes: A max of 50 registers are allowed
    pub x: u16,
    /// Offset1
    ///
    /// Offset of value to read
    pub off1: u16,
    #[allow(missing_docs)]
    pub off2: u16,
    #[allow(missing_docs)]
    pub off3: u16,
    #[allow(missing_docs)]
    pub off4: u16,
    #[allow(missing_docs)]
    pub off5: u16,
    #[allow(missing_docs)]
    pub off6: u16,
    #[allow(missing_docs)]
    pub off7: u16,
    #[allow(missing_docs)]
    pub off8: u16,
    #[allow(missing_docs)]
    pub off9: u16,
    #[allow(missing_docs)]
    pub off10: u16,
    #[allow(missing_docs)]
    pub off11: u16,
    #[allow(missing_docs)]
    pub off12: u16,
    #[allow(missing_docs)]
    pub off13: u16,
    #[allow(missing_docs)]
    pub off14: u16,
    #[allow(missing_docs)]
    pub off15: u16,
    #[allow(missing_docs)]
    pub off16: u16,
    #[allow(missing_docs)]
    pub off17: u16,
    #[allow(missing_docs)]
    pub off18: u16,
    #[allow(missing_docs)]
    pub off19: u16,
    #[allow(missing_docs)]
    pub off20: u16,
    #[allow(missing_docs)]
    pub off21: u16,
    #[allow(missing_docs)]
    pub off22: u16,
    #[allow(missing_docs)]
    pub off23: u16,
    #[allow(missing_docs)]
    pub off24: u16,
    #[allow(missing_docs)]
    pub off25: u16,
    #[allow(missing_docs)]
    pub off26: u16,
    #[allow(missing_docs)]
    pub off27: u16,
    #[allow(missing_docs)]
    pub off28: u16,
    #[allow(missing_docs)]
    pub off29: u16,
    #[allow(missing_docs)]
    pub off30: u16,
    #[allow(missing_docs)]
    pub off31: u16,
    #[allow(missing_docs)]
    pub off32: u16,
    #[allow(missing_docs)]
    pub off33: u16,
    #[allow(missing_docs)]
    pub off34: u16,
    #[allow(missing_docs)]
    pub off35: u16,
    #[allow(missing_docs)]
    pub off36: u16,
    #[allow(missing_docs)]
    pub off37: u16,
    #[allow(missing_docs)]
    pub off38: u16,
    #[allow(missing_docs)]
    pub off39: u16,
    #[allow(missing_docs)]
    pub off40: u16,
    #[allow(missing_docs)]
    pub off41: u16,
    #[allow(missing_docs)]
    pub off42: u16,
    #[allow(missing_docs)]
    pub off43: u16,
    #[allow(missing_docs)]
    pub off44: u16,
    #[allow(missing_docs)]
    pub off45: u16,
    #[allow(missing_docs)]
    pub off46: u16,
    #[allow(missing_docs)]
    pub off47: u16,
    #[allow(missing_docs)]
    pub off48: u16,
    #[allow(missing_docs)]
    pub off49: u16,
    #[allow(missing_docs)]
    pub off50: u16,
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
    /// Digital Signature ID
    ///
    /// Notes: User's role id 0-5
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

impl Model3 {
    pub const X: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const OFF1: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const OFF2: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const OFF3: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const OFF4: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const OFF5: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const OFF6: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const OFF7: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const OFF8: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const OFF9: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const OFF10: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const OFF11: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const OFF12: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const OFF13: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const OFF14: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, true);
    pub const OFF15: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const OFF16: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const OFF17: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const OFF18: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const OFF19: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const OFF20: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const OFF21: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, true);
    pub const OFF22: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const OFF23: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, true);
    pub const OFF24: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, true);
    pub const OFF25: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, true);
    pub const OFF26: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, true);
    pub const OFF27: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const OFF28: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, true);
    pub const OFF29: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, true);
    pub const OFF30: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, true);
    pub const OFF31: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, true);
    pub const OFF32: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, true);
    pub const OFF33: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, true);
    pub const OFF34: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const OFF35: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const OFF36: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, true);
    pub const OFF37: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, true);
    pub const OFF38: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, true);
    pub const OFF39: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, true);
    pub const OFF40: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, true);
    pub const OFF41: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, true);
    pub const OFF42: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, true);
    pub const OFF43: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, true);
    pub const OFF44: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, true);
    pub const OFF45: crate::PointDef<Self, u16> = crate::PointDef::new(45, 1, true);
    pub const OFF46: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, true);
    pub const OFF47: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, true);
    pub const OFF48: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const OFF49: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const OFF50: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(51, 2, true);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, true);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(54, 1, true);
    pub const ROLE: crate::PointDef<Self, u16> = crate::PointDef::new(55, 1, true);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(56, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(57, 1, false);
}

impl crate::Model for Model3 {
    const ID: u16 = 3;
    const LENGTH: u16 = 59;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            x: Self::X
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off1: Self::OFF1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off2: Self::OFF2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off3: Self::OFF3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off4: Self::OFF4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off5: Self::OFF5
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off6: Self::OFF6
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off7: Self::OFF7
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off8: Self::OFF8
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off9: Self::OFF9
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off10: Self::OFF10
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off11: Self::OFF11
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off12: Self::OFF12
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off13: Self::OFF13
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off14: Self::OFF14
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off15: Self::OFF15
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off16: Self::OFF16
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off17: Self::OFF17
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off18: Self::OFF18
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off19: Self::OFF19
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off20: Self::OFF20
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off21: Self::OFF21
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off22: Self::OFF22
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off23: Self::OFF23
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off24: Self::OFF24
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off25: Self::OFF25
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off26: Self::OFF26
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off27: Self::OFF27
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off28: Self::OFF28
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off29: Self::OFF29
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off30: Self::OFF30
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off31: Self::OFF31
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off32: Self::OFF32
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off33: Self::OFF33
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off34: Self::OFF34
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off35: Self::OFF35
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off36: Self::OFF36
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off37: Self::OFF37
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off38: Self::OFF38
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off39: Self::OFF39
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off40: Self::OFF40
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off41: Self::OFF41
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off42: Self::OFF42
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off43: Self::OFF43
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off44: Self::OFF44
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off45: Self::OFF45
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off46: Self::OFF46
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off47: Self::OFF47
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off48: Self::OFF48
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off49: Self::OFF49
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off50: Self::OFF50
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
    const LENGTH: u16 = 61;
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
    const LENGTH: u16 = 89;
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

/// Secure Write Sequential Request
///
/// Include a digital signature along with the control data
#[derive(Debug)]
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
    pub alg: u16,
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
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(88, 1, true);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(89, 1, true);
}

impl crate::Model for Model6 {
    const ID: u16 = 6;
    const LENGTH: u16 = 91;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            x: Self::X
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            off: Self::OFF
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
            val51: Self::VAL51
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val52: Self::VAL52
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val53: Self::VAL53
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val54: Self::VAL54
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val55: Self::VAL55
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val56: Self::VAL56
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val57: Self::VAL57
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val58: Self::VAL58
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val59: Self::VAL59
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val60: Self::VAL60
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val61: Self::VAL61
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val62: Self::VAL62
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val63: Self::VAL63
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val64: Self::VAL64
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val65: Self::VAL65
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val66: Self::VAL66
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val67: Self::VAL67
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val68: Self::VAL68
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val69: Self::VAL69
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val70: Self::VAL70
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val71: Self::VAL71
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val72: Self::VAL72
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val73: Self::VAL73
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val74: Self::VAL74
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val75: Self::VAL75
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val76: Self::VAL76
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val77: Self::VAL77
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val78: Self::VAL78
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val79: Self::VAL79
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            val80: Self::VAL80
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
    const LENGTH: u16 = 11;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            rqseq: Self::RQSEQ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sts: Self::STS
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
    const LENGTH: u16 = 3;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            fmt: Self::FMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Set Operator Security Certificate
///
/// Security model for PKI
#[derive(Debug)]
pub struct Model9 {
    /// Cert_UID
    ///
    /// User ID for this certificate
    pub certuid: u16,
    /// Cert_Role
    ///
    /// Role for this certificate
    pub certrole: u16,
    /// Format
    ///
    /// Format of this certificate
    pub fmt: u16,
    /// Type
    ///
    /// Type of this certificate
    pub typ: u16,
    /// Total Length
    ///
    /// Total Length of the Certificate
    ///
    /// Notes: In registers, zero padded.
    pub totln: u16,
    /// Fragment length
    ///
    /// Length of this fragment
    ///
    /// Notes: Maximum fragment length is 80 registers
    pub frgln: u16,
    /// Frag1
    ///
    /// First word of this fragment
    pub frg1: u16,
    #[allow(missing_docs)]
    pub frg2: u16,
    #[allow(missing_docs)]
    pub frg3: u16,
    #[allow(missing_docs)]
    pub frg4: u16,
    #[allow(missing_docs)]
    pub frg5: u16,
    #[allow(missing_docs)]
    pub frg6: u16,
    #[allow(missing_docs)]
    pub frg7: u16,
    #[allow(missing_docs)]
    pub frg8: u16,
    #[allow(missing_docs)]
    pub frg9: u16,
    #[allow(missing_docs)]
    pub frg10: u16,
    #[allow(missing_docs)]
    pub frg11: u16,
    #[allow(missing_docs)]
    pub frg12: u16,
    #[allow(missing_docs)]
    pub frg13: u16,
    #[allow(missing_docs)]
    pub frg14: u16,
    #[allow(missing_docs)]
    pub frg15: u16,
    #[allow(missing_docs)]
    pub frg16: u16,
    #[allow(missing_docs)]
    pub frg17: u16,
    #[allow(missing_docs)]
    pub frg18: u16,
    #[allow(missing_docs)]
    pub frg19: u16,
    #[allow(missing_docs)]
    pub frg20: u16,
    #[allow(missing_docs)]
    pub frg21: u16,
    #[allow(missing_docs)]
    pub frg22: u16,
    #[allow(missing_docs)]
    pub frg23: u16,
    #[allow(missing_docs)]
    pub frg24: u16,
    #[allow(missing_docs)]
    pub frg25: u16,
    #[allow(missing_docs)]
    pub frg26: u16,
    #[allow(missing_docs)]
    pub frg27: u16,
    #[allow(missing_docs)]
    pub frg28: u16,
    #[allow(missing_docs)]
    pub frg29: u16,
    #[allow(missing_docs)]
    pub frg30: u16,
    #[allow(missing_docs)]
    pub frg31: u16,
    #[allow(missing_docs)]
    pub frg32: u16,
    #[allow(missing_docs)]
    pub frg33: u16,
    #[allow(missing_docs)]
    pub frg34: u16,
    #[allow(missing_docs)]
    pub frg35: u16,
    #[allow(missing_docs)]
    pub frg36: u16,
    #[allow(missing_docs)]
    pub frg37: u16,
    #[allow(missing_docs)]
    pub frg38: u16,
    #[allow(missing_docs)]
    pub frg39: u16,
    #[allow(missing_docs)]
    pub frg40: u16,
    #[allow(missing_docs)]
    pub frg41: u16,
    #[allow(missing_docs)]
    pub frg42: u16,
    #[allow(missing_docs)]
    pub frg43: u16,
    #[allow(missing_docs)]
    pub frg44: u16,
    #[allow(missing_docs)]
    pub frg45: u16,
    #[allow(missing_docs)]
    pub frg46: u16,
    #[allow(missing_docs)]
    pub frg47: u16,
    #[allow(missing_docs)]
    pub frg48: u16,
    #[allow(missing_docs)]
    pub frg49: u16,
    #[allow(missing_docs)]
    pub frg50: u16,
    #[allow(missing_docs)]
    pub frg51: u16,
    #[allow(missing_docs)]
    pub frg52: u16,
    #[allow(missing_docs)]
    pub frg53: u16,
    #[allow(missing_docs)]
    pub frg54: u16,
    #[allow(missing_docs)]
    pub frg55: u16,
    #[allow(missing_docs)]
    pub frg56: u16,
    #[allow(missing_docs)]
    pub frg57: u16,
    #[allow(missing_docs)]
    pub frg58: u16,
    #[allow(missing_docs)]
    pub frg59: u16,
    #[allow(missing_docs)]
    pub frg60: u16,
    #[allow(missing_docs)]
    pub frg61: u16,
    #[allow(missing_docs)]
    pub frg62: u16,
    #[allow(missing_docs)]
    pub frg63: u16,
    #[allow(missing_docs)]
    pub frg64: u16,
    #[allow(missing_docs)]
    pub frg65: u16,
    #[allow(missing_docs)]
    pub frg66: u16,
    #[allow(missing_docs)]
    pub frg67: u16,
    #[allow(missing_docs)]
    pub frg68: u16,
    #[allow(missing_docs)]
    pub frg69: u16,
    #[allow(missing_docs)]
    pub frg70: u16,
    #[allow(missing_docs)]
    pub frg71: u16,
    #[allow(missing_docs)]
    pub frg72: u16,
    #[allow(missing_docs)]
    pub frg73: u16,
    #[allow(missing_docs)]
    pub frg74: u16,
    #[allow(missing_docs)]
    pub frg75: u16,
    #[allow(missing_docs)]
    pub frg78: u16,
    #[allow(missing_docs)]
    pub frg79: u16,
    /// Frag80
    ///
    /// Last word of this fragment
    pub frg80: u16,
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
    /// UID
    ///
    /// User ID for the request signature
    pub uid: u16,
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
    /// Number of registers to follow for the certificate
    pub n: u16,
}

#[allow(missing_docs)]

impl Model9 {
    pub const CERTUID: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const CERTROLE: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const FMT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const TOTLN: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const FRGLN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const FRG1: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const FRG2: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const FRG3: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const FRG4: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const FRG5: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const FRG6: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const FRG7: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const FRG8: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const FRG9: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, true);
    pub const FRG10: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const FRG11: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const FRG12: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const FRG13: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const FRG14: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const FRG15: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const FRG16: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, true);
    pub const FRG17: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const FRG18: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, true);
    pub const FRG19: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, true);
    pub const FRG20: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, true);
    pub const FRG21: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, true);
    pub const FRG22: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const FRG23: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, true);
    pub const FRG24: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, true);
    pub const FRG25: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, true);
    pub const FRG26: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, true);
    pub const FRG27: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, true);
    pub const FRG28: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, true);
    pub const FRG29: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const FRG30: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const FRG31: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, true);
    pub const FRG32: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, true);
    pub const FRG33: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, true);
    pub const FRG34: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, true);
    pub const FRG35: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, true);
    pub const FRG36: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, true);
    pub const FRG37: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, true);
    pub const FRG38: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, true);
    pub const FRG39: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, true);
    pub const FRG40: crate::PointDef<Self, u16> = crate::PointDef::new(45, 1, true);
    pub const FRG41: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, true);
    pub const FRG42: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, true);
    pub const FRG43: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const FRG44: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const FRG45: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
    pub const FRG46: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, true);
    pub const FRG47: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, true);
    pub const FRG48: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, true);
    pub const FRG49: crate::PointDef<Self, u16> = crate::PointDef::new(54, 1, true);
    pub const FRG50: crate::PointDef<Self, u16> = crate::PointDef::new(55, 1, true);
    pub const FRG51: crate::PointDef<Self, u16> = crate::PointDef::new(56, 1, true);
    pub const FRG52: crate::PointDef<Self, u16> = crate::PointDef::new(57, 1, true);
    pub const FRG53: crate::PointDef<Self, u16> = crate::PointDef::new(58, 1, true);
    pub const FRG54: crate::PointDef<Self, u16> = crate::PointDef::new(59, 1, true);
    pub const FRG55: crate::PointDef<Self, u16> = crate::PointDef::new(60, 1, true);
    pub const FRG56: crate::PointDef<Self, u16> = crate::PointDef::new(61, 1, true);
    pub const FRG57: crate::PointDef<Self, u16> = crate::PointDef::new(62, 1, true);
    pub const FRG58: crate::PointDef<Self, u16> = crate::PointDef::new(63, 1, true);
    pub const FRG59: crate::PointDef<Self, u16> = crate::PointDef::new(64, 1, true);
    pub const FRG60: crate::PointDef<Self, u16> = crate::PointDef::new(65, 1, true);
    pub const FRG61: crate::PointDef<Self, u16> = crate::PointDef::new(66, 1, true);
    pub const FRG62: crate::PointDef<Self, u16> = crate::PointDef::new(67, 1, true);
    pub const FRG63: crate::PointDef<Self, u16> = crate::PointDef::new(68, 1, true);
    pub const FRG64: crate::PointDef<Self, u16> = crate::PointDef::new(69, 1, true);
    pub const FRG65: crate::PointDef<Self, u16> = crate::PointDef::new(70, 1, true);
    pub const FRG66: crate::PointDef<Self, u16> = crate::PointDef::new(71, 1, true);
    pub const FRG67: crate::PointDef<Self, u16> = crate::PointDef::new(72, 1, true);
    pub const FRG68: crate::PointDef<Self, u16> = crate::PointDef::new(73, 1, true);
    pub const FRG69: crate::PointDef<Self, u16> = crate::PointDef::new(74, 1, true);
    pub const FRG70: crate::PointDef<Self, u16> = crate::PointDef::new(75, 1, true);
    pub const FRG71: crate::PointDef<Self, u16> = crate::PointDef::new(76, 1, true);
    pub const FRG72: crate::PointDef<Self, u16> = crate::PointDef::new(77, 1, true);
    pub const FRG73: crate::PointDef<Self, u16> = crate::PointDef::new(78, 1, true);
    pub const FRG74: crate::PointDef<Self, u16> = crate::PointDef::new(79, 1, true);
    pub const FRG75: crate::PointDef<Self, u16> = crate::PointDef::new(80, 1, true);
    pub const FRG78: crate::PointDef<Self, u16> = crate::PointDef::new(81, 1, true);
    pub const FRG79: crate::PointDef<Self, u16> = crate::PointDef::new(82, 1, true);
    pub const FRG80: crate::PointDef<Self, u16> = crate::PointDef::new(83, 1, true);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, true);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(86, 1, true);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(87, 1, true);
    pub const UID: crate::PointDef<Self, u16> = crate::PointDef::new(88, 1, true);
    pub const ROLE: crate::PointDef<Self, u16> = crate::PointDef::new(89, 1, true);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(90, 1, true);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(91, 1, true);
}

impl crate::Model for Model9 {
    const ID: u16 = 9;
    const LENGTH: u16 = 93;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            certuid: Self::CERTUID
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            certrole: Self::CERTROLE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            fmt: Self::FMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totln: Self::TOTLN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frgln: Self::FRGLN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg1: Self::FRG1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg2: Self::FRG2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg3: Self::FRG3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg4: Self::FRG4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg5: Self::FRG5
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg6: Self::FRG6
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg7: Self::FRG7
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg8: Self::FRG8
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg9: Self::FRG9
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg10: Self::FRG10
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg11: Self::FRG11
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg12: Self::FRG12
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg13: Self::FRG13
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg14: Self::FRG14
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg15: Self::FRG15
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg16: Self::FRG16
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg17: Self::FRG17
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg18: Self::FRG18
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg19: Self::FRG19
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg20: Self::FRG20
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg21: Self::FRG21
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg22: Self::FRG22
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg23: Self::FRG23
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg24: Self::FRG24
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg25: Self::FRG25
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg26: Self::FRG26
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg27: Self::FRG27
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg28: Self::FRG28
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg29: Self::FRG29
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg30: Self::FRG30
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg31: Self::FRG31
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg32: Self::FRG32
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg33: Self::FRG33
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg34: Self::FRG34
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg35: Self::FRG35
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg36: Self::FRG36
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg37: Self::FRG37
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg38: Self::FRG38
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg39: Self::FRG39
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg40: Self::FRG40
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg41: Self::FRG41
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg42: Self::FRG42
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg43: Self::FRG43
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg44: Self::FRG44
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg45: Self::FRG45
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg46: Self::FRG46
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg47: Self::FRG47
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg48: Self::FRG48
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg49: Self::FRG49
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg50: Self::FRG50
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg51: Self::FRG51
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg52: Self::FRG52
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg53: Self::FRG53
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg54: Self::FRG54
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg55: Self::FRG55
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg56: Self::FRG56
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg57: Self::FRG57
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg58: Self::FRG58
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg59: Self::FRG59
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg60: Self::FRG60
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg61: Self::FRG61
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg62: Self::FRG62
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg63: Self::FRG63
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg64: Self::FRG64
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg65: Self::FRG65
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg66: Self::FRG66
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg67: Self::FRG67
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg68: Self::FRG68
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg69: Self::FRG69
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg70: Self::FRG70
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg71: Self::FRG71
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg72: Self::FRG72
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg73: Self::FRG73
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg74: Self::FRG74
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg75: Self::FRG75
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg78: Self::FRG78
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg79: Self::FRG79
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            frg80: Self::FRG80
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
            uid: Self::UID
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

/// Communication Interface Header
///
/// To be included first for a complete interface description
#[derive(Debug)]
pub struct Model10 {
    /// Interface Status
    ///
    /// Overall interface status
    pub st: u16,
    /// Interface Control
    ///
    /// Overall interface control (TBD)
    pub ctl: Option<u16>,
    /// Physical Access Type
    ///
    /// Enumerated value.  Type of physical media
    pub typ: Option<u16>,
}

#[allow(missing_docs)]

impl Model10 {
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
}

impl crate::Model for Model10 {
    const ID: u16 = 10;
    const LENGTH: u16 = 4;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ctl: Self::CTL.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
        })
    }
}

/// Ethernet Link Layer
///
/// Include to support a wired ethernet port
#[derive(Debug)]
pub struct Model11 {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    pub spd: u16,
    /// Interface Status Flags
    ///
    /// Bitmask values Interface flags.
    pub cfgst: u16,
    /// Link State
    ///
    /// Enumerated value. State information for this interface
    pub st: u16,
    /// MAC
    ///
    /// IEEE MAC address of this interface
    pub mac: Option<String>,
    /// Name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Control
    ///
    /// Control flags
    pub ctl: Option<u16>,
    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    pub frcspd: Option<u16>,
}

#[allow(missing_docs)]

impl Model11 {
    pub const SPD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const CFGST: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const MAC: crate::PointDef<Self, String> = crate::PointDef::new(3, 3, false);
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(7, 4, true);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const FRCSPD: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
}

impl crate::Model for Model11 {
    const ID: u16 = 11;
    const LENGTH: u16 = 13;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            spd: Self::SPD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfgst: Self::CFGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            mac: Self::MAC.from_data(data)?,
            nam: Self::NAM.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            frcspd: Self::FRCSPD.from_data(data)?,
        })
    }
}

/// IPv4
///
/// Include to support an IPv4 protocol stack on this interface
#[derive(Debug)]
pub struct Model12 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// Config Status
    ///
    /// Enumerated value.  Configuration status
    pub cfgst: u16,
    /// Change Status
    ///
    /// Bitmask value.  A configuration change is pending
    pub chgst: u16,
    /// Config Capability
    ///
    /// Bitmask value. Identify capable sources of configuration
    pub cap: u16,
    /// IPv4 Config
    ///
    /// Enumerated value.  Configuration method used.
    pub cfg: u16,
    /// Control
    ///
    /// Configure use of services
    pub ctl: u16,
    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    pub addr: String,
    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    pub msk: String,
    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    pub gw: Option<String>,
    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    pub dns1: Option<String>,
    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    pub dns2: Option<String>,
    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    pub ntp1: Option<String>,
    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    pub ntp2: Option<String>,
    /// Domain
    ///
    /// Domain name (24 chars max)
    pub domnam: Option<String>,
    /// Host Name
    ///
    /// Host name (24 chars max)
    pub hostnam: Option<String>,
}

#[allow(missing_docs)]

impl Model12 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CFGST: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const CHGST: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const CAP: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(9, 8, true);
    pub const MSK: crate::PointDef<Self, String> = crate::PointDef::new(17, 8, true);
    pub const GW: crate::PointDef<Self, String> = crate::PointDef::new(25, 8, true);
    pub const DNS1: crate::PointDef<Self, String> = crate::PointDef::new(33, 8, true);
    pub const DNS2: crate::PointDef<Self, String> = crate::PointDef::new(41, 8, true);
    pub const NTP1: crate::PointDef<Self, String> = crate::PointDef::new(49, 12, true);
    pub const NTP2: crate::PointDef<Self, String> = crate::PointDef::new(61, 12, true);
    pub const DOMNAM: crate::PointDef<Self, String> = crate::PointDef::new(73, 12, true);
    pub const HOSTNAM: crate::PointDef<Self, String> = crate::PointDef::new(85, 12, true);
}

impl crate::Model for Model12 {
    const ID: u16 = 12;
    const LENGTH: u16 = 98;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfgst: Self::CFGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            chgst: Self::CHGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cap: Self::CAP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ctl: Self::CTL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            msk: Self::MSK
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            ntp1: Self::NTP1.from_data(data)?,
            ntp2: Self::NTP2.from_data(data)?,
            domnam: Self::DOMNAM.from_data(data)?,
            hostnam: Self::HOSTNAM.from_data(data)?,
        })
    }
}

/// IPv6
///
/// Include to support an IPv6 protocol stack on this interface
#[derive(Debug)]
pub struct Model13 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// Config Status
    ///
    /// Enumerated value.  Configuration status
    pub cfgst: u16,
    /// Change Status
    ///
    /// Bitmask value.  A configuration change is pending
    pub chgst: u16,
    /// Config Capability
    ///
    /// Bitmask value. Identify capable sources of configuration
    pub cap: u16,
    /// IPv6 Config
    ///
    /// Enumerated value.  Configuration method used.
    pub cfg: u16,
    /// Control
    ///
    /// Bitmask value.  Configure use of services
    pub ctl: u16,
    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub addr: String,
    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    pub cidr: Option<String>,
    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub gw: Option<String>,
    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub dns1: Option<String>,
    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    pub dns2: Option<String>,
    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    pub ntp1: Option<String>,
    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    pub ntp2: Option<String>,
    /// Domain
    ///
    /// Domain name (24 chars max)
    pub domnam: Option<String>,
    /// Host Name
    ///
    /// Host name (24 chars max)
    pub hostnam: Option<String>,
}

#[allow(missing_docs)]

impl Model13 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CFGST: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const CHGST: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const CAP: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(9, 20, true);
    pub const CIDR: crate::PointDef<Self, String> = crate::PointDef::new(29, 20, true);
    pub const GW: crate::PointDef<Self, String> = crate::PointDef::new(49, 20, true);
    pub const DNS1: crate::PointDef<Self, String> = crate::PointDef::new(69, 20, true);
    pub const DNS2: crate::PointDef<Self, String> = crate::PointDef::new(89, 20, true);
    pub const NTP1: crate::PointDef<Self, String> = crate::PointDef::new(109, 20, true);
    pub const NTP2: crate::PointDef<Self, String> = crate::PointDef::new(129, 20, true);
    pub const DOMNAM: crate::PointDef<Self, String> = crate::PointDef::new(149, 12, true);
    pub const HOSTNAM: crate::PointDef<Self, String> = crate::PointDef::new(161, 12, true);
}

impl crate::Model for Model13 {
    const ID: u16 = 13;
    const LENGTH: u16 = 174;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfgst: Self::CFGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            chgst: Self::CHGST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cap: Self::CAP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ctl: Self::CTL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cidr: Self::CIDR.from_data(data)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            ntp1: Self::NTP1.from_data(data)?,
            ntp2: Self::NTP2.from_data(data)?,
            domnam: Self::DOMNAM.from_data(data)?,
            hostnam: Self::HOSTNAM.from_data(data)?,
        })
    }
}

/// Proxy Server
///
/// Include this block to allow for a proxy server
#[derive(Debug)]
pub struct Model14 {
    /// name
    ///
    /// Interface name (8 chars)
    pub nam: Option<String>,
    /// Capabilities
    ///
    /// Bitmask value.  Proxy configuration capabilities
    pub cap: u16,
    /// Config
    ///
    /// Enumerated value.  Set proxy address type
    pub cfg: u16,
    /// Type
    ///
    /// Enumerate value.  Proxy server type
    pub typ: u16,
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
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CAP: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(7, 20, true);
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const USER: crate::PointDef<Self, String> = crate::PointDef::new(28, 12, true);
    pub const PW: crate::PointDef<Self, String> = crate::PointDef::new(40, 12, true);
}

impl crate::Model for Model14 {
    const ID: u16 = 14;
    const LENGTH: u16 = 52;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cap: Self::CAP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            port: Self::PORT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            user: Self::USER.from_data(data)?,
            pw: Self::PW.from_data(data)?,
        })
    }
}

/// Interface Counters Model
///
/// Interface counters
#[derive(Debug)]
pub struct Model15 {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    pub clr: Option<u16>,
    /// Input Count
    ///
    /// Number of bytes received
    pub incnt: Option<u32>,
    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    pub inuccnt: Option<u32>,
    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    pub innuccnt: Option<u32>,
    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    pub indsccnt: Option<u32>,
    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    pub inerrcnt: Option<u32>,
    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    pub inunkcnt: Option<u32>,
    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    pub outcnt: Option<u32>,
    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    pub outuccnt: Option<u32>,
    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    pub outnuccnt: Option<u32>,
    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    pub outdsccnt: Option<u32>,
    /// Output Error Count
    ///
    /// Number of outbound error packets
    pub outerrcnt: Option<u32>,
}

#[allow(missing_docs)]

impl Model15 {
    pub const CLR: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const INCNT: crate::PointDef<Self, u32> = crate::PointDef::new(1, 2, false);
    pub const INUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(3, 2, false);
    pub const INNUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const INDSCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const INERRCNT: crate::PointDef<Self, u32> = crate::PointDef::new(9, 2, false);
    pub const INUNKCNT: crate::PointDef<Self, u32> = crate::PointDef::new(11, 2, false);
    pub const OUTCNT: crate::PointDef<Self, u32> = crate::PointDef::new(13, 2, false);
    pub const OUTUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(15, 2, false);
    pub const OUTNUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(17, 2, false);
    pub const OUTDSCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(19, 2, false);
    pub const OUTERRCNT: crate::PointDef<Self, u32> = crate::PointDef::new(21, 2, false);
}

impl crate::Model for Model15 {
    const ID: u16 = 15;
    const LENGTH: u16 = 24;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            clr: Self::CLR.from_data(data)?,
            incnt: Self::INCNT.from_data(data)?,
            inuccnt: Self::INUCCNT.from_data(data)?,
            innuccnt: Self::INNUCCNT.from_data(data)?,
            indsccnt: Self::INDSCCNT.from_data(data)?,
            inerrcnt: Self::INERRCNT.from_data(data)?,
            inunkcnt: Self::INUNKCNT.from_data(data)?,
            outcnt: Self::OUTCNT.from_data(data)?,
            outuccnt: Self::OUTUCCNT.from_data(data)?,
            outnuccnt: Self::OUTNUCCNT.from_data(data)?,
            outdsccnt: Self::OUTDSCCNT.from_data(data)?,
            outerrcnt: Self::OUTERRCNT.from_data(data)?,
        })
    }
}

/// Simple IP Network
///
/// Include this model for a simple IPv4 network stack
#[derive(Debug)]
pub struct Model16 {
    /// Name
    ///
    /// Interface name.  (8 chars)
    pub nam: Option<String>,
    /// Config
    ///
    /// Enumerated value.  Force IPv4 configuration method
    pub cfg: u16,
    /// Control
    ///
    /// Bitmask value Configure use of services
    pub ctl: u16,
    /// Address
    ///
    /// IP address
    pub addr: String,
    /// Netmask
    ///
    /// Netmask
    pub msk: String,
    /// Gateway
    ///
    /// Gateway IP address
    pub gw: Option<String>,
    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    pub dns1: Option<String>,
    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    pub dns2: Option<String>,
    /// MAC
    ///
    /// IEEE MAC address of this interface
    pub mac: Option<String>,
    /// Link Control
    ///
    /// Bitmask value.  Link control flags
    pub lnkctl: Option<u16>,
}

#[allow(missing_docs)]

impl Model16 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const CFG: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const ADDR: crate::PointDef<Self, String> = crate::PointDef::new(6, 8, true);
    pub const MSK: crate::PointDef<Self, String> = crate::PointDef::new(14, 8, true);
    pub const GW: crate::PointDef<Self, String> = crate::PointDef::new(22, 8, true);
    pub const DNS1: crate::PointDef<Self, String> = crate::PointDef::new(30, 8, true);
    pub const DNS2: crate::PointDef<Self, String> = crate::PointDef::new(38, 8, true);
    pub const MAC: crate::PointDef<Self, String> = crate::PointDef::new(46, 3, false);
    pub const LNKCTL: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, true);
}

impl crate::Model for Model16 {
    const ID: u16 = 16;
    const LENGTH: u16 = 52;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            cfg: Self::CFG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ctl: Self::CTL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            addr: Self::ADDR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            msk: Self::MSK
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            gw: Self::GW.from_data(data)?,
            dns1: Self::DNS1.from_data(data)?,
            dns2: Self::DNS2.from_data(data)?,
            mac: Self::MAC.from_data(data)?,
            lnkctl: Self::LNKCTL.from_data(data)?,
        })
    }
}

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
    const LENGTH: u16 = 12;
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

/// Cellular Link
///
/// Include this model to support a cellular interface link
#[derive(Debug)]
pub struct Model18 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    pub imei: Option<u32>,
    /// APN
    ///
    /// Access Point Name for the interface
    pub apn: Option<String>,
    /// Number
    ///
    /// Phone number for the interface
    pub num: Option<String>,
    /// PIN
    ///
    /// Personal Identification Number for the interface
    pub pin: Option<String>,
}

#[allow(missing_docs)]

impl Model18 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const IMEI: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, true);
    pub const APN: crate::PointDef<Self, String> = crate::PointDef::new(6, 4, true);
    pub const NUM: crate::PointDef<Self, String> = crate::PointDef::new(10, 6, true);
    pub const PIN: crate::PointDef<Self, String> = crate::PointDef::new(16, 6, true);
}

impl crate::Model for Model18 {
    const ID: u16 = 18;
    const LENGTH: u16 = 22;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            imei: Self::IMEI.from_data(data)?,
            apn: Self::APN.from_data(data)?,
            num: Self::NUM.from_data(data)?,
            pin: Self::PIN.from_data(data)?,
        })
    }
}

/// PPP Link
///
/// Include this model to configure a Point-to-Point Protocol link
#[derive(Debug)]
pub struct Model19 {
    /// Name
    ///
    /// Interface name
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
    /// Authentication
    ///
    /// Enumerated value.  Authentication method
    pub auth: Option<u16>,
    /// Username
    ///
    /// Username for authentication
    pub usrnam: Option<String>,
    /// Password
    ///
    /// Password for authentication
    pub pw: Option<String>,
}

#[allow(missing_docs)]

impl Model19 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 4, true);
    pub const RTE: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, true);
    pub const BITS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const PTY: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const DUP: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const FLW: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const AUTH: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const USRNAM: crate::PointDef<Self, String> = crate::PointDef::new(11, 12, false);
    pub const PW: crate::PointDef<Self, String> = crate::PointDef::new(23, 6, false);
}

impl crate::Model for Model19 {
    const ID: u16 = 19;
    const LENGTH: u16 = 30;
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
            auth: Self::AUTH.from_data(data)?,
            usrnam: Self::USRNAM.from_data(data)?,
            pw: Self::PW.from_data(data)?,
        })
    }
}

/// Inverter (Single Phase)
///
/// Include this model for single phase inverter monitoring
#[derive(Debug)]
pub struct Model101 {
    /// Amps
    ///
    /// AC Current
    pub a: u16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: u16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: Option<u16>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<u16>,
    #[allow(missing_docs)]
    pub a_sf: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<u16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<u16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<u16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: u16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<u16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<u16>,
    #[allow(missing_docs)]
    pub v_sf: i16,
    /// Watts
    ///
    /// AC Power
    pub w: i16,
    #[allow(missing_docs)]
    pub w_sf: i16,
    /// Hz
    ///
    /// Line Frequency
    pub hz: u16,
    #[allow(missing_docs)]
    pub hz_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    #[allow(missing_docs)]
    pub va_sf: Option<i16>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<i16>,
    #[allow(missing_docs)]
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<i16>,
    #[allow(missing_docs)]
    pub pf_sf: Option<i16>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: u32,
    #[allow(missing_docs)]
    pub wh_sf: i16,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<u16>,
    #[allow(missing_docs)]
    pub dca_sf: Option<i16>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<u16>,
    #[allow(missing_docs)]
    pub dcv_sf: Option<i16>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<i16>,
    #[allow(missing_docs)]
    pub dcw_sf: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: i16,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<i16>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<i16>,
    #[allow(missing_docs)]
    pub tmp_sf: i16,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model101 {
    pub const A: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PPVPHAB: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const PPVPHBC: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const PPVPHCA: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHA: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const PHVPHB: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHC: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const WH: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const DCA: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const DCW: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const TMPCAB: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const TMPSNK: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const TMPTRNS: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const TMPOT: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
}

impl crate::Model for Model101 {
    const ID: u16 = 101;
    const LENGTH: u16 = 50;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB.from_data(data)?,
            aphc: Self::APHC.from_data(data)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wh_sf: Self::WH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}

/// Inverter (Split-Phase)
///
/// Include this model for split phase inverter monitoring
#[derive(Debug)]
pub struct Model102 {
    /// Amps
    ///
    /// AC Current
    ///
    /// Notes: Sum of active phases
    pub a: u16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: u16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Notes: Connected Phase
    pub aphb: u16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<u16>,
    #[allow(missing_docs)]
    pub a_sf: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<u16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<u16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<u16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: u16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: u16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<u16>,
    #[allow(missing_docs)]
    pub v_sf: i16,
    /// Watts
    ///
    /// AC Power
    pub w: i16,
    #[allow(missing_docs)]
    pub w_sf: i16,
    /// Hz
    ///
    /// Line Frequency
    pub hz: u16,
    #[allow(missing_docs)]
    pub hz_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    #[allow(missing_docs)]
    pub va_sf: Option<i16>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<i16>,
    #[allow(missing_docs)]
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<i16>,
    #[allow(missing_docs)]
    pub pf_sf: Option<i16>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: u32,
    #[allow(missing_docs)]
    pub wh_sf: i16,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<u16>,
    #[allow(missing_docs)]
    pub dca_sf: Option<i16>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<u16>,
    #[allow(missing_docs)]
    pub dcv_sf: Option<i16>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<i16>,
    #[allow(missing_docs)]
    pub dcw_sf: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: i16,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<i16>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<i16>,
    #[allow(missing_docs)]
    pub tmp_sf: i16,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model102 {
    pub const A: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PPVPHAB: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const PPVPHBC: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const PPVPHCA: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHA: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const PHVPHB: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHC: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const WH: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const DCA: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const DCW: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const TMPCAB: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const TMPSNK: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const TMPTRNS: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const TMPOT: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
}

impl crate::Model for Model102 {
    const ID: u16 = 102;
    const LENGTH: u16 = 50;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC.from_data(data)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wh_sf: Self::WH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}

/// Inverter (Three Phase)
///
/// Include this model for three phase inverter monitoring
#[derive(Debug)]
pub struct Model103 {
    /// Amps
    ///
    /// AC Current
    ///
    /// Notes: Sum of active phases
    pub a: u16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: u16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Notes: Connected Phase
    pub aphb: u16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    ///
    /// Notes: Connected Phase
    pub aphc: u16,
    #[allow(missing_docs)]
    pub a_sf: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<u16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<u16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<u16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: u16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: u16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: u16,
    #[allow(missing_docs)]
    pub v_sf: i16,
    /// Watts
    ///
    /// AC Power
    pub w: i16,
    #[allow(missing_docs)]
    pub w_sf: i16,
    /// Hz
    ///
    /// Line Frequency
    pub hz: u16,
    #[allow(missing_docs)]
    pub hz_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    #[allow(missing_docs)]
    pub va_sf: Option<i16>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<i16>,
    #[allow(missing_docs)]
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<i16>,
    #[allow(missing_docs)]
    pub pf_sf: Option<i16>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: u32,
    #[allow(missing_docs)]
    pub wh_sf: i16,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<u16>,
    #[allow(missing_docs)]
    pub dca_sf: Option<i16>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<u16>,
    #[allow(missing_docs)]
    pub dcv_sf: Option<i16>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<i16>,
    #[allow(missing_docs)]
    pub dcw_sf: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: i16,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<i16>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<i16>,
    #[allow(missing_docs)]
    pub tmp_sf: i16,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model103 {
    pub const A: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PPVPHAB: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const PPVPHBC: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const PPVPHCA: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHA: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const PHVPHB: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHC: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const WH: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const DCA: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const DCW: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const TMPCAB: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const TMPSNK: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const TMPTRNS: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const TMPOT: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
}

impl crate::Model for Model103 {
    const ID: u16 = 103;
    const LENGTH: u16 = 50;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wh_sf: Self::WH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}

/// Inverter (Single Phase) FLOAT
///
/// Include this model for single phase inverter monitoring using float values
#[derive(Debug)]
pub struct Model111 {
    /// Amps
    ///
    /// AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: Option<f32>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<f32>,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model111 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WH: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const DCA: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const DCV: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const DCW: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const TMPCAB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const TMPSNK: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const TMPTRNS: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const TMPOT: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(52, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(54, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(56, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(58, 2, false);
}

impl crate::Model for Model111 {
    const ID: u16 = 111;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB.from_data(data)?,
            aphc: Self::APHC.from_data(data)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}

/// Inverter (Split Phase) FLOAT
///
/// Include this model for split phase inverter monitoring using float values
#[derive(Debug)]
pub struct Model112 {
    /// Amps
    ///
    /// AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Notes: Connected Phase
    pub aphb: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<f32>,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model112 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WH: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const DCA: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const DCV: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const DCW: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const TMPCAB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const TMPSNK: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const TMPTRNS: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const TMPOT: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(52, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(54, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(56, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(58, 2, false);
}

impl crate::Model for Model112 {
    const ID: u16 = 112;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC.from_data(data)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}

/// Inverter (Three Phase) FLOAT
///
/// Include this model for three phase inverter monitoring using float values
#[derive(Debug)]
pub struct Model113 {
    /// Amps
    ///
    /// AC Current
    ///
    /// Notes: Sum of active phases
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Notes: Connected Phase
    pub aphb: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    ///
    /// Notes: Connected Phase
    pub aphc: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: f32,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<f32>,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model113 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WH: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const DCA: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const DCV: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const DCW: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const TMPCAB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const TMPSNK: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const TMPTRNS: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const TMPOT: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(47, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(52, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(54, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(56, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(58, 2, false);
}

impl crate::Model for Model113 {
    const ID: u16 = 113;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}

/// Nameplate
///
/// Inverter Controls Nameplate Ratings
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
pub struct Model120 {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    pub dertyp: u16,
    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    pub wrtg: u16,
    /// WRtg_SF
    ///
    /// Scale factor
    pub wrtg_sf: i16,
    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    pub vartg: u16,
    /// VARtg_SF
    ///
    /// Scale factor
    pub vartg_sf: i16,
    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    pub varrtgq1: i16,
    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    pub varrtgq2: i16,
    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    pub varrtgq3: i16,
    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    pub varrtgq4: i16,
    /// VArRtg_SF
    ///
    /// Scale factor
    pub varrtg_sf: i16,
    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Notes: Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    pub artg: u16,
    /// ARtg_SF
    ///
    /// Scale factor
    pub artg_sf: i16,
    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq1: i16,
    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq2: i16,
    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq3: i16,
    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq4: i16,
    /// PFRtg_SF
    ///
    /// Scale factor
    pub pfrtg_sf: i16,
    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    pub whrtg: Option<u16>,
    /// WHRtg_SF
    ///
    /// Scale factor
    pub whrtg_sf: Option<i16>,
    /// AhrRtg
    ///
    /// The usable capacity of the battery.  Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    pub ahrrtg: Option<u16>,
    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    pub ahrrtg_sf: Option<i16>,
    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    pub maxcharte: Option<u16>,
    /// MaxChaRte_SF
    ///
    /// Scale factor
    pub maxcharte_sf: Option<i16>,
    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    pub maxdischarte: Option<u16>,
    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    pub maxdischarte_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model120 {
    pub const DERTYP: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const WRTG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const WRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const VARTG: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const VARTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const VARRTGQ1: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const VARRTGQ2: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const VARRTGQ3: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const VARRTGQ4: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const VARRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const ARTG: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const ARTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PFRTGQ1: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const PFRTGQ2: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const PFRTGQ3: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const PFRTGQ4: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const PFRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WHRTG: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const WHRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const AHRRTG: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const AHRRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const MAXCHARTE: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const MAXCHARTE_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const MAXDISCHARTE: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, false);
    pub const MAXDISCHARTE_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
}

impl crate::Model for Model120 {
    const ID: u16 = 120;
    const LENGTH: u16 = 26;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dertyp: Self::DERTYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wrtg: Self::WRTG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wrtg_sf: Self::WRTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vartg: Self::VARTG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vartg_sf: Self::VARTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varrtgq1: Self::VARRTGQ1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varrtgq2: Self::VARRTGQ2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varrtgq3: Self::VARRTGQ3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varrtgq4: Self::VARRTGQ4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varrtg_sf: Self::VARRTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            artg: Self::ARTG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            artg_sf: Self::ARTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pfrtgq1: Self::PFRTGQ1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pfrtgq2: Self::PFRTGQ2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pfrtgq3: Self::PFRTGQ3
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pfrtgq4: Self::PFRTGQ4
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pfrtg_sf: Self::PFRTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            whrtg: Self::WHRTG.from_data(data)?,
            whrtg_sf: Self::WHRTG_SF.from_data(data)?,
            ahrrtg: Self::AHRRTG.from_data(data)?,
            ahrrtg_sf: Self::AHRRTG_SF.from_data(data)?,
            maxcharte: Self::MAXCHARTE.from_data(data)?,
            maxcharte_sf: Self::MAXCHARTE_SF.from_data(data)?,
            maxdischarte: Self::MAXDISCHARTE.from_data(data)?,
            maxdischarte_sf: Self::MAXDISCHARTE_SF.from_data(data)?,
        })
    }
}

/// Basic Settings
///
/// Inverter Controls Basic Settings
///
/// Notes: Ref 3: 8.4.2.1, Ref 4: 17
#[derive(Debug)]
pub struct Model121 {
    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    pub wmax: u16,
    /// VRef
    ///
    /// Voltage at the PCC.
    pub vref: u16,
    /// VRefOfs
    ///
    /// Offset  from PCC to inverter.
    pub vrefofs: i16,
    /// VMax
    ///
    /// Setpoint for maximum voltage.
    pub vmax: Option<u16>,
    /// VMin
    ///
    /// Setpoint for minimum voltage.
    pub vmin: Option<u16>,
    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    pub vamax: Option<u16>,
    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    pub varmaxq1: Option<i16>,
    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    pub varmaxq2: Option<i16>,
    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    pub varmaxq3: Option<i16>,
    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    pub varmaxq4: Option<i16>,
    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    pub wgra: Option<u16>,
    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// Notes: EEI sign convention.
    pub pfminq1: Option<i16>,
    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// Notes: EEI sign convention.
    pub pfminq2: Option<i16>,
    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// Notes: EEI sign convention.
    pub pfminq3: Option<i16>,
    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// Notes: EEI sign convention.
    pub pfminq4: Option<i16>,
    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    pub varact: Option<u16>,
    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    pub clctotva: Option<u16>,
    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    pub maxrmprte: Option<u16>,
    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    pub ecpnomhz: Option<u16>,
    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    pub connph: Option<u16>,
    /// WMax_SF
    ///
    /// Scale factor for real power.
    pub wmax_sf: i16,
    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    pub vref_sf: i16,
    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    pub vrefofs_sf: i16,
    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    pub vminmax_sf: Option<i16>,
    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    pub vamax_sf: Option<i16>,
    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    pub varmax_sf: Option<i16>,
    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    pub wgra_sf: Option<i16>,
    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    pub pfmin_sf: Option<i16>,
    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    pub maxrmprte_sf: Option<i16>,
    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    pub ecpnomhz_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model121 {
    pub const WMAX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const VREF: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const VREFOFS: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, true);
    pub const VMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const VMIN: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const VAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const VARMAXQ1: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, true);
    pub const VARMAXQ2: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, true);
    pub const VARMAXQ3: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, true);
    pub const VARMAXQ4: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, true);
    pub const WGRA: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const PFMINQ1: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, true);
    pub const PFMINQ2: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, true);
    pub const PFMINQ3: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, true);
    pub const PFMINQ4: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, true);
    pub const VARACT: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const CLCTOTVA: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const MAXRMPRTE: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const ECPNOMHZ: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const CONNPH: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const WMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VREFOFS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VMINMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const VAMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const VARMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
    pub const WGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const PFMIN_SF: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const MAXRMPRTE_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const ECPNOMHZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
}

impl crate::Model for Model121 {
    const ID: u16 = 121;
    const LENGTH: u16 = 30;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wmax: Self::WMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vref: Self::VREF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vrefofs: Self::VREFOFS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vmax: Self::VMAX.from_data(data)?,
            vmin: Self::VMIN.from_data(data)?,
            vamax: Self::VAMAX.from_data(data)?,
            varmaxq1: Self::VARMAXQ1.from_data(data)?,
            varmaxq2: Self::VARMAXQ2.from_data(data)?,
            varmaxq3: Self::VARMAXQ3.from_data(data)?,
            varmaxq4: Self::VARMAXQ4.from_data(data)?,
            wgra: Self::WGRA.from_data(data)?,
            pfminq1: Self::PFMINQ1.from_data(data)?,
            pfminq2: Self::PFMINQ2.from_data(data)?,
            pfminq3: Self::PFMINQ3.from_data(data)?,
            pfminq4: Self::PFMINQ4.from_data(data)?,
            varact: Self::VARACT.from_data(data)?,
            clctotva: Self::CLCTOTVA.from_data(data)?,
            maxrmprte: Self::MAXRMPRTE.from_data(data)?,
            ecpnomhz: Self::ECPNOMHZ.from_data(data)?,
            connph: Self::CONNPH.from_data(data)?,
            wmax_sf: Self::WMAX_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vref_sf: Self::VREF_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vrefofs_sf: Self::VREFOFS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vminmax_sf: Self::VMINMAX_SF.from_data(data)?,
            vamax_sf: Self::VAMAX_SF.from_data(data)?,
            varmax_sf: Self::VARMAX_SF.from_data(data)?,
            wgra_sf: Self::WGRA_SF.from_data(data)?,
            pfmin_sf: Self::PFMIN_SF.from_data(data)?,
            maxrmprte_sf: Self::MAXRMPRTE_SF.from_data(data)?,
            ecpnomhz_sf: Self::ECPNOMHZ_SF.from_data(data)?,
        })
    }
}

/// Measurements_Status
///
/// Inverter Controls Extended Measurements and Status
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
pub struct Model122 {
    /// PVConn
    ///
    /// PV inverter present/available status. Enumerated value.
    pub pvconn: u16,
    /// StorConn
    ///
    /// Storage inverter present/available status. Enumerated value.
    pub storconn: u16,
    /// ECPConn
    ///
    /// ECP connection status: disconnected=0  connected=1.
    pub ecpconn: u16,
    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    pub actwh: Option<u64>,
    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    pub actvah: Option<u64>,
    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    pub actvarhq1: Option<u64>,
    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    pub actvarhq2: Option<u64>,
    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output  in quadrant 3.
    pub actvarhq3: Option<u64>,
    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output  in quadrant 4.
    pub actvarhq4: Option<u64>,
    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    pub varaval: Option<i16>,
    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    pub varaval_sf: Option<i16>,
    /// WAval
    ///
    /// Amount of Watts available.
    pub waval: Option<u16>,
    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    pub waval_sf: Option<i16>,
    /// StSetLimMsk
    ///
    /// Bit Mask indicating setpoint limit(s) reached.
    ///
    /// Notes: Bits shall be automatically cleared on read.
    pub stsetlimmsk: Option<u32>,
    /// StActCtl
    ///
    /// Bit Mask indicating which inverter controls are currently active.
    pub stactctl: Option<u32>,
    /// TmSrc
    ///
    /// Source of time synchronization.
    pub tmsrc: Option<String>,
    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    pub tms: Option<u32>,
    /// RtSt
    ///
    /// Bit Mask indicating active ride-through status.
    pub rtst: Option<u16>,
    /// Ris
    ///
    /// Isolation resistance.
    pub ris: Option<u16>,
    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    pub ris_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model122 {
    pub const PVCONN: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STORCONN: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ECPCONN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ACTWH: crate::PointDef<Self, u64> = crate::PointDef::new(3, 4, false);
    pub const ACTVAH: crate::PointDef<Self, u64> = crate::PointDef::new(7, 4, false);
    pub const ACTVARHQ1: crate::PointDef<Self, u64> = crate::PointDef::new(11, 4, false);
    pub const ACTVARHQ2: crate::PointDef<Self, u64> = crate::PointDef::new(15, 4, false);
    pub const ACTVARHQ3: crate::PointDef<Self, u64> = crate::PointDef::new(19, 4, false);
    pub const ACTVARHQ4: crate::PointDef<Self, u64> = crate::PointDef::new(23, 4, false);
    pub const VARAVAL: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARAVAL_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const WAVAL: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
    pub const WAVAL_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const STSETLIMMSK: crate::PointDef<Self, u32> = crate::PointDef::new(31, 2, false);
    pub const STACTCTL: crate::PointDef<Self, u32> = crate::PointDef::new(33, 2, false);
    pub const TMSRC: crate::PointDef<Self, String> = crate::PointDef::new(35, 4, false);
    pub const TMS: crate::PointDef<Self, u32> = crate::PointDef::new(39, 2, false);
    pub const RTST: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
    pub const RIS: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, false);
    pub const RIS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(43, 1, false);
}

impl crate::Model for Model122 {
    const ID: u16 = 122;
    const LENGTH: u16 = 44;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            pvconn: Self::PVCONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            storconn: Self::STORCONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ecpconn: Self::ECPCONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            actwh: Self::ACTWH.from_data(data)?,
            actvah: Self::ACTVAH.from_data(data)?,
            actvarhq1: Self::ACTVARHQ1.from_data(data)?,
            actvarhq2: Self::ACTVARHQ2.from_data(data)?,
            actvarhq3: Self::ACTVARHQ3.from_data(data)?,
            actvarhq4: Self::ACTVARHQ4.from_data(data)?,
            varaval: Self::VARAVAL.from_data(data)?,
            varaval_sf: Self::VARAVAL_SF.from_data(data)?,
            waval: Self::WAVAL.from_data(data)?,
            waval_sf: Self::WAVAL_SF.from_data(data)?,
            stsetlimmsk: Self::STSETLIMMSK.from_data(data)?,
            stactctl: Self::STACTCTL.from_data(data)?,
            tmsrc: Self::TMSRC.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            rtst: Self::RTST.from_data(data)?,
            ris: Self::RIS.from_data(data)?,
            ris_sf: Self::RIS_SF.from_data(data)?,
        })
    }
}

/// Immediate Controls
///
/// Immediate Inverter Controls
///
/// Notes: Ref 3: 8.7.1.2, 8.7.2.2, 8.7.3.2
#[derive(Debug)]
pub struct Model123 {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    pub conn_wintms: Option<u16>,
    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    pub conn_rvrttms: Option<u16>,
    /// Conn
    ///
    /// Enumerated valued.  Connection control.
    pub conn: u16,
    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    pub wmaxlimpct: u16,
    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    pub wmaxlimpct_wintms: Option<u16>,
    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    pub wmaxlimpct_rvrttms: Option<u16>,
    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub wmaxlimpct_rmptms: Option<u16>,
    /// WMaxLim_Ena
    ///
    /// Enumerated valued.  Throttle enable/disable control.
    pub wmaxlim_ena: u16,
    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    pub outpfset: i16,
    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    pub outpfset_wintms: Option<u16>,
    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    pub outpfset_rvrttms: Option<u16>,
    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub outpfset_rmptms: Option<u16>,
    /// OutPFSet_Ena
    ///
    /// Enumerated valued.  Fixed power factor enable/disable control.
    pub outpfset_ena: u16,
    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    pub varwmaxpct: Option<i16>,
    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    pub varmaxpct: Option<i16>,
    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    pub varavalpct: Option<i16>,
    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    pub varpct_wintms: Option<u16>,
    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    pub varpct_rvrttms: Option<u16>,
    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub varpct_rmptms: Option<u16>,
    /// VArPct_Mod
    ///
    /// Enumerated value. VAR percent limit mode.
    pub varpct_mod: Option<u16>,
    /// VArPct_Ena
    ///
    /// Enumerated valued.  Percent limit VAr enable/disable control.
    pub varpct_ena: u16,
    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    pub wmaxlimpct_sf: i16,
    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    pub outpfset_sf: i16,
    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    pub varpct_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model123 {
    pub const CONN_WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const CONN_RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const CONN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const WMAXLIMPCT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const WMAXLIMPCT_WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const WMAXLIMPCT_RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const WMAXLIMPCT_RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const WMAXLIM_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const OUTPFSET: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, true);
    pub const OUTPFSET_WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const OUTPFSET_RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const OUTPFSET_RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, true);
    pub const OUTPFSET_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const VARWMAXPCT: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, true);
    pub const VARMAXPCT: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, true);
    pub const VARAVALPCT: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, true);
    pub const VARPCT_WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, true);
    pub const VARPCT_RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const VARPCT_RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const VARPCT_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, true);
    pub const VARPCT_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const WMAXLIMPCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const OUTPFSET_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VARPCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
}

impl crate::Model for Model123 {
    const ID: u16 = 123;
    const LENGTH: u16 = 24;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            conn_wintms: Self::CONN_WINTMS.from_data(data)?,
            conn_rvrttms: Self::CONN_RVRTTMS.from_data(data)?,
            conn: Self::CONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wmaxlimpct: Self::WMAXLIMPCT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wmaxlimpct_wintms: Self::WMAXLIMPCT_WINTMS.from_data(data)?,
            wmaxlimpct_rvrttms: Self::WMAXLIMPCT_RVRTTMS.from_data(data)?,
            wmaxlimpct_rmptms: Self::WMAXLIMPCT_RMPTMS.from_data(data)?,
            wmaxlim_ena: Self::WMAXLIM_ENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            outpfset: Self::OUTPFSET
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            outpfset_wintms: Self::OUTPFSET_WINTMS.from_data(data)?,
            outpfset_rvrttms: Self::OUTPFSET_RVRTTMS.from_data(data)?,
            outpfset_rmptms: Self::OUTPFSET_RMPTMS.from_data(data)?,
            outpfset_ena: Self::OUTPFSET_ENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varwmaxpct: Self::VARWMAXPCT.from_data(data)?,
            varmaxpct: Self::VARMAXPCT.from_data(data)?,
            varavalpct: Self::VARAVALPCT.from_data(data)?,
            varpct_wintms: Self::VARPCT_WINTMS.from_data(data)?,
            varpct_rvrttms: Self::VARPCT_RVRTTMS.from_data(data)?,
            varpct_rmptms: Self::VARPCT_RMPTMS.from_data(data)?,
            varpct_mod: Self::VARPCT_MOD.from_data(data)?,
            varpct_ena: Self::VARPCT_ENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wmaxlimpct_sf: Self::WMAXLIMPCT_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            outpfset_sf: Self::OUTPFSET_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            varpct_sf: Self::VARPCT_SF.from_data(data)?,
        })
    }
}

/// Storage
///
/// Basic Storage Controls
///
/// Notes: Ref 3: 8.7.4.2
#[derive(Debug)]
pub struct Model124 {
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    pub wchamax: u16,
    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    pub wchagra: u16,
    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    pub wdischagra: u16,
    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode. Bitfield value.
    pub storctl_mod: u16,
    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    pub vachamax: Option<u16>,
    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    pub minrsvpct: Option<u16>,
    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    pub chastate: Option<u16>,
    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    pub storaval: Option<u16>,
    /// InBatV
    ///
    /// Internal battery voltage.
    pub inbatv: Option<u16>,
    /// ChaSt
    ///
    /// Charge status of storage device. Enumerated value.
    pub chast: Option<u16>,
    /// OutWRte
    ///
    /// Percent of max discharge rate.
    pub outwrte: Option<i16>,
    /// InWRte
    ///
    /// Percent of max charging rate.
    pub inwrte: Option<i16>,
    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    pub inoutwrte_wintms: Option<u16>,
    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    pub inoutwrte_rvrttms: Option<u16>,
    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub inoutwrte_rmptms: Option<u16>,
    #[allow(missing_docs)]
    pub chagriset: Option<u16>,
    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    pub wchamax_sf: i16,
    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    pub wchadischagra_sf: i16,
    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    pub vachamax_sf: Option<i16>,
    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    pub minrsvpct_sf: Option<i16>,
    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    pub chastate_sf: Option<i16>,
    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    pub storaval_sf: Option<i16>,
    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    pub inbatv_sf: Option<i16>,
    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    pub inoutwrte_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model124 {
    pub const WCHAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const WCHAGRA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WDISCHAGRA: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const STORCTL_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const VACHAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const MINRSVPCT: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const CHASTATE: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const STORAVAL: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const INBATV: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const CHAST: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const OUTWRTE: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, true);
    pub const INWRTE: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, true);
    pub const INOUTWRTE_WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const INOUTWRTE_RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const INOUTWRTE_RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, true);
    pub const CHAGRISET: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, true);
    pub const WCHAMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WCHADISCHAGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const VACHAMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const MINRSVPCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const CHASTATE_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const STORAVAL_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const INBATV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const INOUTWRTE_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
}

impl crate::Model for Model124 {
    const ID: u16 = 124;
    const LENGTH: u16 = 24;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wchamax: Self::WCHAMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wchagra: Self::WCHAGRA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wdischagra: Self::WDISCHAGRA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            storctl_mod: Self::STORCTL_MOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vachamax: Self::VACHAMAX.from_data(data)?,
            minrsvpct: Self::MINRSVPCT.from_data(data)?,
            chastate: Self::CHASTATE.from_data(data)?,
            storaval: Self::STORAVAL.from_data(data)?,
            inbatv: Self::INBATV.from_data(data)?,
            chast: Self::CHAST.from_data(data)?,
            outwrte: Self::OUTWRTE.from_data(data)?,
            inwrte: Self::INWRTE.from_data(data)?,
            inoutwrte_wintms: Self::INOUTWRTE_WINTMS.from_data(data)?,
            inoutwrte_rvrttms: Self::INOUTWRTE_RVRTTMS.from_data(data)?,
            inoutwrte_rmptms: Self::INOUTWRTE_RMPTMS.from_data(data)?,
            chagriset: Self::CHAGRISET.from_data(data)?,
            wchamax_sf: Self::WCHAMAX_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wchadischagra_sf: Self::WCHADISCHAGRA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vachamax_sf: Self::VACHAMAX_SF.from_data(data)?,
            minrsvpct_sf: Self::MINRSVPCT_SF.from_data(data)?,
            chastate_sf: Self::CHASTATE_SF.from_data(data)?,
            storaval_sf: Self::STORAVAL_SF.from_data(data)?,
            inbatv_sf: Self::INBATV_SF.from_data(data)?,
            inoutwrte_sf: Self::INOUTWRTE_SF.from_data(data)?,
        })
    }
}

/// Pricing
///
/// Pricing Signal
///
/// Notes: Ref 3: 8.7.5.1; Ref 4: 6
#[derive(Debug)]
pub struct Model125 {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    pub modena: u16,
    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    pub sigtype: Option<u16>,
    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    pub sig: i16,
    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    pub wintms: Option<u16>,
    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    pub rvttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    pub rmptms: Option<u16>,
    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    pub sig_sf: i16,
}

#[allow(missing_docs)]

impl Model125 {
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const SIGTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const SIG: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RVTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const SIG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model125 {
    const ID: u16 = 125;
    const LENGTH: u16 = 8;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sigtype: Self::SIGTYPE.from_data(data)?,
            sig: Self::SIG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvttms: Self::RVTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            sig_sf: Self::SIG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Static Volt-VAR
///
/// Static Volt-VAR Arrays
///
/// Notes: Ref 3: 8.8.1.2
#[derive(Debug)]
pub struct Model126 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// Is Volt-VAR control active.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for volt-VAR change.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    /// DeptRef_SF
    ///
    /// scale factor for dependent variable.
    pub deptref_sf: i16,
    #[allow(missing_docs)]
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model126 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const DEPTREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model126 {
    const ID: u16 = 126;
    const LENGTH: u16 = 64;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            deptref_sf: Self::DEPTREF_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}

/// Freq-Watt Param
///
/// Parameterized Frequency-Watt
///
/// Notes: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
pub struct Model127 {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    pub wgra: u16,
    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    pub hzstr: i16,
    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    pub hzstop: i16,
    /// HysEna
    ///
    /// Enable hysteresis
    pub hysena: u16,
    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    pub modena: u16,
    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    pub hzstopwgra: Option<u16>,
    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    pub wgra_sf: Option<i16>,
    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    pub hzstrstop_sf: Option<i16>,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model127 {
    pub const WGRA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const HZSTR: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, true);
    pub const HZSTOP: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, true);
    pub const HYSENA: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const HZSTOPWGRA: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const WGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const HZSTRSTOP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model127 {
    const ID: u16 = 127;
    const LENGTH: u16 = 10;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wgra: Self::WGRA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hzstr: Self::HZSTR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hzstop: Self::HZSTOP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hysena: Self::HYSENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hzstopwgra: Self::HZSTOPWGRA.from_data(data)?,
            wgra_sf: Self::WGRA_SF.from_data(data)?,
            hzstrstop_sf: Self::HZSTRSTOP_SF.from_data(data)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}

/// Dynamic Reactive Current
///
/// Dynamic Reactive Current
///
/// Notes: Ref 3: 8.10.1.2; Ref 4: 12
#[derive(Debug)]
pub struct Model128 {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    pub argramod: u16,
    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    pub argrasag: u16,
    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current.  A value of 0 indicates no additional reactive current support.
    pub argraswell: u16,
    /// ModEna
    ///
    /// Activate dynamic reactive current model
    pub modena: u16,
    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    pub filtms: Option<u16>,
    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    pub dbvmin: Option<u16>,
    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    pub dbvmax: Option<u16>,
    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    pub blkznv: Option<u16>,
    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    pub hysblkznv: Option<u16>,
    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    pub blkzntmms: Option<u16>,
    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    pub holdtmms: Option<u16>,
    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    pub argra_sf: i16,
    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    pub vrefpct_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model128 {
    pub const ARGRAMOD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ARGRASAG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ARGRASWELL: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const FILTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const DBVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const DBVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const BLKZNV: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const HYSBLKZNV: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const BLKZNTMMS: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const HOLDTMMS: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const ARGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const VREFPCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
}

impl crate::Model for Model128 {
    const ID: u16 = 128;
    const LENGTH: u16 = 14;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            argramod: Self::ARGRAMOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            argrasag: Self::ARGRASAG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            argraswell: Self::ARGRASWELL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            filtms: Self::FILTMS.from_data(data)?,
            dbvmin: Self::DBVMIN.from_data(data)?,
            dbvmax: Self::DBVMAX.from_data(data)?,
            blkznv: Self::BLKZNV.from_data(data)?,
            hysblkznv: Self::HYSBLKZNV.from_data(data)?,
            blkzntmms: Self::BLKZNTMMS.from_data(data)?,
            holdtmms: Self::HOLDTMMS.from_data(data)?,
            argra_sf: Self::ARGRA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vrefpct_sf: Self::VREFPCT_SF.from_data(data)?,
        })
    }
}

/// LVRTD
///
/// LVRT Must Disconnect
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model129 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
}

#[allow(missing_docs)]

impl Model129 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model129 {
    const ID: u16 = 129;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// HVRTD
///
/// HVRT Must Disconnect
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model130 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// HVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for HVRT change.
    ///
    /// Notes: Setting is ignored for HVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for HVRT curve selection.
    ///
    /// Notes: Setting is ignored for HVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for HVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
}

#[allow(missing_docs)]

impl Model130 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model130 {
    const ID: u16 = 130;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Watt-PF
///
/// Watt-Power Factor
///
/// Notes: Ref 3: 8.11.1.2
#[derive(Debug)]
pub struct Model131 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// Is watt-PF mode active.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for watt-PF change.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Max number of points in array.
    pub npt: u16,
    /// W_SF
    ///
    /// Scale factor for percent WMax.
    pub w_sf: i16,
    /// PF_SF
    ///
    /// Scale factor for PF.
    pub pf_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model131 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model131 {
    const ID: u16 = 131;
    const LENGTH: u16 = 64;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            pf_sf: Self::PF_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}

/// Volt-Watt
///
/// Volt-Watt
///
/// Notes: Ref 3: 8.12.1.2
#[derive(Debug)]
pub struct Model132 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// Is Volt-Watt control active.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for volt-watt change.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of points in array (maximum 20).
    pub npt: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    /// DeptRef_SF
    ///
    /// Scale Factor for % DeptRef
    pub deptref_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model132 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const DEPTREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model132 {
    const ID: u16 = 132;
    const LENGTH: u16 = 64;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            deptref_sf: Self::DEPTREF_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}

/// Basic Scheduling
///
/// Basic Scheduling
///
/// Notes: Ref 2: 2.2.8
#[derive(Debug)]
pub struct Model133 {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    pub actschd: u32,
    /// ModEna
    ///
    /// Is basic scheduling active.
    pub modena: u16,
    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    pub nschd: u16,
    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    pub npts: u16,
}

#[allow(missing_docs)]

impl Model133 {
    pub const ACTSCHD: crate::PointDef<Self, u32> = crate::PointDef::new(0, 2, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const NSCHD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const NPTS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
}

impl crate::Model for Model133 {
    const ID: u16 = 133;
    const LENGTH: u16 = 66;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actschd: Self::ACTSCHD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            nschd: Self::NSCHD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npts: Self::NPTS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Freq-Watt Crv
///
/// Curve-Based Frequency-Watt
///
/// Notes: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
pub struct Model134 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for freq-watt change.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 10).
    pub npt: u16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
    /// W_SF
    ///
    /// Scale factor for percent WRef.
    pub w_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model134 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model134 {
    const ID: u16 = 134;
    const LENGTH: u16 = 68;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}

/// LFRT
///
/// Low Frequency Ride-through
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model135 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
}

#[allow(missing_docs)]

impl Model135 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model135 {
    const ID: u16 = 135;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// HFRT
///
/// High Frequency Ride-through
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model136 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Notes: Setting is ignored for HFRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Notes: Setting is ignored for HFRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for HFRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
}

#[allow(missing_docs)]

impl Model136 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model136 {
    const ID: u16 = 136;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// LVRTC
///
/// LVRT must remain connected
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model137 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
}

#[allow(missing_docs)]

impl Model137 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model137 {
    const ID: u16 = 137;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// HVRTC
///
/// HVRT must remain connected
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model138 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// HVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for HVRT change.
    ///
    /// Notes: Setting is ignored for HVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for HVRT curve selection.
    ///
    /// Notes: Setting is ignored for HVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for HVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
}

#[allow(missing_docs)]

impl Model138 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model138 {
    const ID: u16 = 138;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// LVRTX
///
/// LVRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model139 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub crvtype: u16,
}

#[allow(missing_docs)]

impl Model139 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRVTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model139 {
    const ID: u16 = 139;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            crvtype: Self::CRVTYPE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// HVRTX
///
/// HVRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model140 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub crvtype: u16,
}

#[allow(missing_docs)]

impl Model140 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRVTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model140 {
    const ID: u16 = 140;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            crvtype: Self::CRVTYPE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// LFRTC
///
/// LFRT must remain connected
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model141 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
}

#[allow(missing_docs)]

impl Model141 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model141 {
    const ID: u16 = 141;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// HFRTC
///
/// HFRT must remain connected
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model142 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
}

#[allow(missing_docs)]

impl Model142 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model142 {
    const ID: u16 = 142;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// LFRTX
///
/// LFRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model143 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
    #[allow(missing_docs)]
    pub crvtype: u16,
}

#[allow(missing_docs)]

impl Model143 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRVTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model143 {
    const ID: u16 = 143;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            crvtype: Self::CRVTYPE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// HFRTX
///
/// HFRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model144 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub npt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
    #[allow(missing_docs)]
    pub crvtype: u16,
}

#[allow(missing_docs)]

impl Model144 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRVTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model144 {
    const ID: u16 = 144;
    const LENGTH: u16 = 60;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tms_sf: Self::TMS_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            crvtype: Self::CRVTYPE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Extended Settings
///
/// Inverter controls extended settings
#[derive(Debug)]
pub struct Model145 {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    pub nomrmpuprte: Option<u16>,
    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    pub nomrmpdnrte: Option<u16>,
    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    pub emgrmpuprte: Option<u16>,
    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    pub emgrmpdnrte: Option<u16>,
    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    pub connrmpuprte: Option<u16>,
    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    pub connrmpdnrte: Option<u16>,
    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    pub agra: Option<u16>,
    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    pub rmp_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model145 {
    pub const NOMRMPUPRTE: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const NOMRMPDNRTE: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const EMGRMPUPRTE: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const EMGRMPDNRTE: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const CONNRMPUPRTE: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const CONNRMPDNRTE: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const AGRA: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const RMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
}

impl crate::Model for Model145 {
    const ID: u16 = 145;
    const LENGTH: u16 = 8;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nomrmpuprte: Self::NOMRMPUPRTE.from_data(data)?,
            nomrmpdnrte: Self::NOMRMPDNRTE.from_data(data)?,
            emgrmpuprte: Self::EMGRMPUPRTE.from_data(data)?,
            emgrmpdnrte: Self::EMGRMPDNRTE.from_data(data)?,
            connrmpuprte: Self::CONNRMPUPRTE.from_data(data)?,
            connrmpdnrte: Self::CONNRMPDNRTE.from_data(data)?,
            agra: Self::AGRA.from_data(data)?,
            rmp_sf: Self::RMP_SF.from_data(data)?,
        })
    }
}

/// Multiple MPPT Inverter Extension Model
#[derive(Debug)]
pub struct Model160 {
    /// Current Scale Factor
    pub dca_sf: Option<i16>,
    /// Voltage Scale Factor
    pub dcv_sf: Option<i16>,
    /// Power Scale Factor
    pub dcw_sf: Option<i16>,
    /// Energy Scale Factor
    pub dcwh_sf: Option<i16>,
    /// Global Events
    pub evt: Option<u32>,
    /// Number of Modules
    pub n: Option<u16>,
    /// Timestamp Period
    pub tmsper: Option<u16>,
}

#[allow(missing_docs)]

impl Model160 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMSPER: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
}

impl crate::Model for Model160 {
    const ID: u16 = 160;
    const LENGTH: u16 = 28;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            n: Self::N.from_data(data)?,
            tmsper: Self::TMSPER.from_data(data)?,
        })
    }
}

/// Meter (Single Phase)single phase (AN or AB) meter
///
/// Include this model for single phase (AN or AB) metering
#[derive(Debug)]
pub struct Model201 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: i16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: Option<i16>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<i16>,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ///
    /// Notes: Conditional AN connection
    pub phv: Option<i16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ///
    /// Notes: Conditional AN connection
    pub phvpha: Option<i16>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<i16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: Option<i16>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    ///
    /// Notes: Conditional AB connection
    pub ppvphab: Option<i16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<i16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<i16>,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Watts phase A
    pub wpha: Option<i16>,
    /// Watts phase B
    pub wphb: Option<i16>,
    /// Watts phase C
    pub wphc: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub vapha: Option<i16>,
    /// VA phase B
    pub vaphb: Option<i16>,
    /// VA phase C
    pub vaphc: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub varpha: Option<i16>,
    /// VAR phase B
    pub varphb: Option<i16>,
    /// VAR phase C
    pub varphc: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub pfpha: Option<i16>,
    /// PF phase B
    pub pfphb: Option<i16>,
    /// PF phase C
    pub pfphc: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<u32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<u32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<u32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<u32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<u32>,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<u32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<u32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<u32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<u32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model201 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const PHVPHA: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const PHVPHB: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHC: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const PPV: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const PPVPHAB: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const PPVPHBC: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PPVPHCA: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WPHA: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const WPHB: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const WPHC: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VAPHA: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VAPHB: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const VAPHC: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const VARPHA: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARPHB: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const VARPHC: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const PFPHA: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const PFPHB: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const PFPHC: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(53, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(55, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(57, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(59, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(61, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(63, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(65, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(67, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(69, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(70, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, u32> = crate::PointDef::new(72, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, u32> = crate::PointDef::new(74, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, u32> = crate::PointDef::new(76, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(78, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, u32> = crate::PointDef::new(80, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, u32> = crate::PointDef::new(82, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(86, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, u32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, u32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, u32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, u32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, u32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, u32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(102, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(103, 2, false);
}

impl crate::Model for Model201 {
    const ID: u16 = 201;
    const LENGTH: u16 = 105;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB.from_data(data)?,
            aphc: Self::APHC.from_data(data)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV.from_data(data)?,
            phvpha: Self::PHVPHA.from_data(data)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV.from_data(data)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// split single phase (ABN) meter
#[derive(Debug)]
pub struct Model202 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: Option<i16>,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: i16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: i16,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: i16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: i16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub phvphab: i16,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub phvphbc: Option<i16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub phvphca: Option<i16>,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Watts phase A
    pub wpha: Option<i16>,
    /// Watts phase B
    pub wphb: Option<i16>,
    /// Watts phase C
    pub wphc: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub vapha: Option<i16>,
    /// VA phase B
    pub vaphb: Option<i16>,
    /// VA phase C
    pub vaphc: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub varpha: Option<i16>,
    /// VAR phase B
    pub varphb: Option<i16>,
    /// VAR phase C
    pub varphc: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub pfpha: Option<i16>,
    /// PF phase B
    pub pfphb: Option<i16>,
    /// PF phase C
    pub pfphc: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<u32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<u32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<u32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<u32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<u32>,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<u32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<u32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<u32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<u32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model202 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const PHVPHA: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const PHVPHB: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHC: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const PPV: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHAB: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const PHVPHBC: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PHVPHCA: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WPHA: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const WPHB: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const WPHC: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VAPHA: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VAPHB: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const VAPHC: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const VARPHA: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARPHB: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const VARPHC: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const PFPHA: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const PFPHB: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const PFPHC: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(53, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(55, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(57, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(59, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(61, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(63, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(65, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(67, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(69, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(70, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, u32> = crate::PointDef::new(72, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, u32> = crate::PointDef::new(74, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, u32> = crate::PointDef::new(76, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(78, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, u32> = crate::PointDef::new(80, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, u32> = crate::PointDef::new(82, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(86, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, u32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, u32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, u32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, u32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, u32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, u32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(102, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(103, 2, false);
}

impl crate::Model for Model202 {
    const ID: u16 = 202;
    const LENGTH: u16 = 105;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA.from_data(data)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphab: Self::PHVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphbc: Self::PHVPHBC.from_data(data)?,
            phvphca: Self::PHVPHCA.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// wye-connect three phase (abcn) meter
#[derive(Debug)]
pub struct Model203 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: i16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: i16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: i16,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: i16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: i16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: i16,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub phvphab: i16,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub phvphbc: i16,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub phvphca: i16,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Watts phase A
    pub wpha: Option<i16>,
    /// Watts phase B
    pub wphb: Option<i16>,
    /// Watts phase C
    pub wphc: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub vapha: Option<i16>,
    /// VA phase B
    pub vaphb: Option<i16>,
    /// VA phase C
    pub vaphc: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub varpha: Option<i16>,
    /// VAR phase B
    pub varphb: Option<i16>,
    /// VAR phase C
    pub varphc: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub pfpha: Option<i16>,
    /// PF phase B
    pub pfphb: Option<i16>,
    /// PF phase C
    pub pfphc: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<u32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<u32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<u32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<u32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<u32>,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<u32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<u32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<u32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<u32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model203 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const PHVPHA: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const PHVPHB: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHC: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const PPV: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHAB: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const PHVPHBC: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PHVPHCA: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WPHA: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const WPHB: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const WPHC: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VAPHA: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VAPHB: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const VAPHC: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const VARPHA: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARPHB: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const VARPHC: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const PFPHA: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const PFPHB: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const PFPHC: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(53, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(55, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(57, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(59, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(61, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(63, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(65, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(67, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(69, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(70, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, u32> = crate::PointDef::new(72, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, u32> = crate::PointDef::new(74, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, u32> = crate::PointDef::new(76, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(78, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, u32> = crate::PointDef::new(80, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, u32> = crate::PointDef::new(82, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(86, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, u32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, u32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, u32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, u32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, u32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, u32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(102, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(103, 2, false);
}

impl crate::Model for Model203 {
    const ID: u16 = 203;
    const LENGTH: u16 = 105;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphab: Self::PHVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphbc: Self::PHVPHBC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphca: Self::PHVPHCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// delta-connect three phase (abc) meter
#[derive(Debug)]
pub struct Model204 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: i16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: i16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: Option<i16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: Option<i16>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<i16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub phvphab: i16,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub phvphbc: i16,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub phvphca: i16,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Watts phase A
    pub wpha: Option<i16>,
    /// Watts phase B
    pub wphb: Option<i16>,
    /// Watts phase C
    pub wphc: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub vapha: Option<i16>,
    /// VA phase B
    pub vaphb: Option<i16>,
    /// VA phase C
    pub vaphc: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub varpha: Option<i16>,
    /// VAR phase B
    pub varphb: Option<i16>,
    /// VAR phase C
    pub varphc: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub pfpha: Option<i16>,
    /// PF phase B
    pub pfphb: Option<i16>,
    /// PF phase C
    pub pfphc: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<u32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<u32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<u32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<u32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<u32>,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<u32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<u32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<u32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<u32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model204 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const PHVPHA: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const PHVPHB: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHC: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const PPV: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHAB: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const PHVPHBC: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PHVPHCA: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WPHA: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const WPHB: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const WPHC: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VAPHA: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VAPHB: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const VAPHC: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const VARPHA: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARPHB: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const VARPHC: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const PFPHA: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const PFPHB: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const PFPHC: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(53, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(55, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(57, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(59, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(61, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(63, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(65, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(67, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(69, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(70, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, u32> = crate::PointDef::new(72, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, u32> = crate::PointDef::new(74, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, u32> = crate::PointDef::new(76, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(78, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, u32> = crate::PointDef::new(80, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, u32> = crate::PointDef::new(82, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(86, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, u32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, u32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, u32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, u32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, u32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, u32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(102, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(103, 2, false);
}

impl crate::Model for Model204 {
    const ID: u16 = 204;
    const LENGTH: u16 = 105;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV.from_data(data)?,
            phvpha: Self::PHVPHA.from_data(data)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphab: Self::PHVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphbc: Self::PHVPHBC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphca: Self::PHVPHCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// single phase (AN or AB) meter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model211 {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: Option<f32>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<f32>,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: Option<f32>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub wpha: Option<f32>,
    /// Watts phase B
    pub wphb: Option<f32>,
    /// Watts phase C
    pub wphc: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub vapha: Option<f32>,
    /// VA phase B
    pub vaphb: Option<f32>,
    /// VA phase C
    pub vaphc: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub varpha: Option<f32>,
    /// VAR phase B
    pub varphb: Option<f32>,
    /// VAR phase C
    pub varphc: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub pfpha: Option<f32>,
    /// PF phase B
    pub pfphb: Option<f32>,
    /// PF phase C
    pub pfphc: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: f32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<f32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<f32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: f32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<f32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<f32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<f32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<f32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<f32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<f32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model211 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PHV: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PPV: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const WPHA: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WPHB: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const WPHC: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const VAPHA: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const VAPHB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const VAPHC: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const VARPHA: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const VARPHB: crate::PointDef<Self, f32> = crate::PointDef::new(46, 2, false);
    pub const VARPHC: crate::PointDef<Self, f32> = crate::PointDef::new(48, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(50, 2, false);
    pub const PFPHA: crate::PointDef<Self, f32> = crate::PointDef::new(52, 2, false);
    pub const PFPHB: crate::PointDef<Self, f32> = crate::PointDef::new(54, 2, false);
    pub const PFPHC: crate::PointDef<Self, f32> = crate::PointDef::new(56, 2, false);
    pub const TOTWHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(58, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(60, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(62, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(64, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(66, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(68, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(70, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(72, 2, false);
    pub const TOTVAHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(74, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(76, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(78, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(80, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(82, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(84, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(86, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, f32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, f32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, f32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, f32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, f32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, f32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, f32> = crate::PointDef::new(102, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, f32> = crate::PointDef::new(104, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, f32> = crate::PointDef::new(106, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, f32> = crate::PointDef::new(108, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, f32> = crate::PointDef::new(110, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, f32> = crate::PointDef::new(112, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, f32> = crate::PointDef::new(114, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, f32> = crate::PointDef::new(116, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, f32> = crate::PointDef::new(118, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, f32> = crate::PointDef::new(120, 2, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(122, 2, false);
}

impl crate::Model for Model211 {
    const ID: u16 = 211;
    const LENGTH: u16 = 124;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB.from_data(data)?,
            aphc: Self::APHC.from_data(data)?,
            phv: Self::PHV.from_data(data)?,
            phvpha: Self::PHVPHA.from_data(data)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV.from_data(data)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// split single phase (ABN) meter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model212 {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<f32>,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: f32,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: f32,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub wpha: Option<f32>,
    /// Watts phase B
    pub wphb: Option<f32>,
    /// Watts phase C
    pub wphc: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub vapha: Option<f32>,
    /// VA phase B
    pub vaphb: Option<f32>,
    /// VA phase C
    pub vaphc: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub varpha: Option<f32>,
    /// VAR phase B
    pub varphb: Option<f32>,
    /// VAR phase C
    pub varphc: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub pfpha: Option<f32>,
    /// PF phase B
    pub pfphb: Option<f32>,
    /// PF phase C
    pub pfphc: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: f32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<f32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<f32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: f32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<f32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<f32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<f32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<f32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<f32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<f32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model212 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PHV: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PPV: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const WPHA: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WPHB: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const WPHC: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const VAPHA: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const VAPHB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const VAPHC: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const VARPHA: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const VARPHB: crate::PointDef<Self, f32> = crate::PointDef::new(46, 2, false);
    pub const VARPHC: crate::PointDef<Self, f32> = crate::PointDef::new(48, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(50, 2, false);
    pub const PFPHA: crate::PointDef<Self, f32> = crate::PointDef::new(52, 2, false);
    pub const PFPHB: crate::PointDef<Self, f32> = crate::PointDef::new(54, 2, false);
    pub const PFPHC: crate::PointDef<Self, f32> = crate::PointDef::new(56, 2, false);
    pub const TOTWHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(58, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(60, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(62, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(64, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(66, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(68, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(70, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(72, 2, false);
    pub const TOTVAHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(74, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(76, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(78, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(80, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(82, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(84, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(86, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, f32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, f32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, f32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, f32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, f32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, f32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, f32> = crate::PointDef::new(102, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, f32> = crate::PointDef::new(104, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, f32> = crate::PointDef::new(106, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, f32> = crate::PointDef::new(108, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, f32> = crate::PointDef::new(110, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, f32> = crate::PointDef::new(112, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, f32> = crate::PointDef::new(114, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, f32> = crate::PointDef::new(116, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, f32> = crate::PointDef::new(118, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, f32> = crate::PointDef::new(120, 2, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(122, 2, false);
}

impl crate::Model for Model212 {
    const ID: u16 = 212;
    const LENGTH: u16 = 124;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC.from_data(data)?,
            phv: Self::PHV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// wye-connect three phase (abcn) meter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model213 {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: f32,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: f32,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: f32,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: f32,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: f32,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: f32,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub wpha: Option<f32>,
    /// Watts phase B
    pub wphb: Option<f32>,
    /// Watts phase C
    pub wphc: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub vapha: Option<f32>,
    /// VA phase B
    pub vaphb: Option<f32>,
    /// VA phase C
    pub vaphc: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub varpha: Option<f32>,
    /// VAR phase B
    pub varphb: Option<f32>,
    /// VAR phase C
    pub varphc: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub pfpha: Option<f32>,
    /// PF phase B
    pub pfphb: Option<f32>,
    /// PF phase C
    pub pfphc: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: f32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<f32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<f32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: f32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<f32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<f32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<f32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<f32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<f32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<f32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model213 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PHV: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PPV: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const WPHA: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WPHB: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const WPHC: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const VAPHA: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const VAPHB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const VAPHC: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const VARPHA: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const VARPHB: crate::PointDef<Self, f32> = crate::PointDef::new(46, 2, false);
    pub const VARPHC: crate::PointDef<Self, f32> = crate::PointDef::new(48, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(50, 2, false);
    pub const PFPHA: crate::PointDef<Self, f32> = crate::PointDef::new(52, 2, false);
    pub const PFPHB: crate::PointDef<Self, f32> = crate::PointDef::new(54, 2, false);
    pub const PFPHC: crate::PointDef<Self, f32> = crate::PointDef::new(56, 2, false);
    pub const TOTWHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(58, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(60, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(62, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(64, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(66, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(68, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(70, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(72, 2, false);
    pub const TOTVAHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(74, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(76, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(78, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(80, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(82, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(84, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(86, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, f32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, f32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, f32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, f32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, f32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, f32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, f32> = crate::PointDef::new(102, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, f32> = crate::PointDef::new(104, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, f32> = crate::PointDef::new(106, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, f32> = crate::PointDef::new(108, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, f32> = crate::PointDef::new(110, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, f32> = crate::PointDef::new(112, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, f32> = crate::PointDef::new(114, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, f32> = crate::PointDef::new(116, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, f32> = crate::PointDef::new(118, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, f32> = crate::PointDef::new(120, 2, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(122, 2, false);
}

impl crate::Model for Model213 {
    const ID: u16 = 213;
    const LENGTH: u16 = 124;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphbc: Self::PPVPHBC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphca: Self::PPVPHCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// delta-connect three phase (abc) meter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model214 {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: f32,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: Option<f32>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: f32,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: f32,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: f32,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub wpha: Option<f32>,
    /// Watts phase B
    pub wphb: Option<f32>,
    /// Watts phase C
    pub wphc: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub vapha: Option<f32>,
    /// VA phase B
    pub vaphb: Option<f32>,
    /// VA phase C
    pub vaphc: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub varpha: Option<f32>,
    /// VAR phase B
    pub varphb: Option<f32>,
    /// VAR phase C
    pub varphc: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub pfpha: Option<f32>,
    /// PF phase B
    pub pfphb: Option<f32>,
    /// PF phase C
    pub pfphc: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: f32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<f32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<f32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: f32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<f32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<f32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<f32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<f32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<f32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<f32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model214 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PHV: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PPV: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const WPHA: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WPHB: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const WPHC: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const VAPHA: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const VAPHB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const VAPHC: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const VARPHA: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const VARPHB: crate::PointDef<Self, f32> = crate::PointDef::new(46, 2, false);
    pub const VARPHC: crate::PointDef<Self, f32> = crate::PointDef::new(48, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(50, 2, false);
    pub const PFPHA: crate::PointDef<Self, f32> = crate::PointDef::new(52, 2, false);
    pub const PFPHB: crate::PointDef<Self, f32> = crate::PointDef::new(54, 2, false);
    pub const PFPHC: crate::PointDef<Self, f32> = crate::PointDef::new(56, 2, false);
    pub const TOTWHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(58, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(60, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(62, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(64, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(66, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(68, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(70, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(72, 2, false);
    pub const TOTVAHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(74, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(76, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(78, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(80, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(82, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(84, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(86, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, f32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, f32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, f32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, f32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, f32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, f32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, f32> = crate::PointDef::new(102, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, f32> = crate::PointDef::new(104, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, f32> = crate::PointDef::new(106, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, f32> = crate::PointDef::new(108, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, f32> = crate::PointDef::new(110, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, f32> = crate::PointDef::new(112, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, f32> = crate::PointDef::new(114, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, f32> = crate::PointDef::new(116, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, f32> = crate::PointDef::new(118, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, f32> = crate::PointDef::new(120, 2, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(122, 2, false);
}

impl crate::Model for Model214 {
    const ID: u16 = 214;
    const LENGTH: u16 = 124;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV.from_data(data)?,
            phvpha: Self::PHVPHA.from_data(data)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphbc: Self::PPVPHBC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphca: Self::PPVPHCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Secure AC Meter Selected Readings
///
/// Include this model for secure metering
#[derive(Debug)]
pub struct Model220 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage
    ///
    /// Average phase or line voltage
    pub phv: Option<i16>,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
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

impl Model220 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(14, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(16, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(19, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(21, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(30, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(33, 2, false);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, false);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, false);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
}

impl crate::Model for Model220 {
    const ID: u16 = 220;
    const LENGTH: u16 = 43;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
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
            alg: Self::ALG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Irradiance Model
///
/// Include to support various irradiance measurements
#[derive(Debug)]
pub struct Model302;

#[allow(missing_docs)]

impl Model302 {}

impl crate::Model for Model302 {
    const ID: u16 = 302;
    const LENGTH: u16 = 5;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}

/// Back of Module Temperature Model
///
/// Include to support variable number of  back of module temperature measurements
#[derive(Debug)]
pub struct Model303;

#[allow(missing_docs)]

impl Model303 {}

impl crate::Model for Model303 {
    const ID: u16 = 303;
    const LENGTH: u16 = 1;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}

/// Inclinometer Model
///
/// Include to support orientation measurements
#[derive(Debug)]
pub struct Model304;

#[allow(missing_docs)]

impl Model304 {}

impl crate::Model for Model304 {
    const ID: u16 = 304;
    const LENGTH: u16 = 6;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}

/// GPS
///
/// Include to support location measurements
#[derive(Debug)]
pub struct Model305 {
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    pub tm: Option<String>,
    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    pub date: Option<String>,
    /// Location
    ///
    /// Location string (40 chars max)
    pub loc: Option<String>,
    /// Lat
    ///
    /// Latitude with seven degrees of precision
    pub lat: Option<i32>,
    /// Long
    ///
    /// Longitude with seven degrees of precision
    pub long: Option<i32>,
    /// Altitude
    ///
    /// Altitude measurement in meters
    pub alt: Option<i32>,
}

#[allow(missing_docs)]

impl Model305 {
    pub const TM: crate::PointDef<Self, String> = crate::PointDef::new(0, 6, false);
    pub const DATE: crate::PointDef<Self, String> = crate::PointDef::new(6, 4, false);
    pub const LOC: crate::PointDef<Self, String> = crate::PointDef::new(10, 20, false);
    pub const LAT: crate::PointDef<Self, i32> = crate::PointDef::new(30, 2, false);
    pub const LONG: crate::PointDef<Self, i32> = crate::PointDef::new(32, 2, false);
    pub const ALT: crate::PointDef<Self, i32> = crate::PointDef::new(34, 2, false);
}

impl crate::Model for Model305 {
    const ID: u16 = 305;
    const LENGTH: u16 = 36;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            tm: Self::TM.from_data(data)?,
            date: Self::DATE.from_data(data)?,
            loc: Self::LOC.from_data(data)?,
            lat: Self::LAT.from_data(data)?,
            long: Self::LONG.from_data(data)?,
            alt: Self::ALT.from_data(data)?,
        })
    }
}

/// Reference Point Model
///
/// Include to support a standard reference point
#[derive(Debug)]
pub struct Model306 {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// Amps
    ///
    /// Current measurement at reference point
    pub a: Option<u16>,
    /// Voltage
    ///
    /// Voltage  measurement at reference point
    pub v: Option<u16>,
    /// Temperature
    ///
    /// Temperature measurement at reference point
    pub tmp: Option<u16>,
}

#[allow(missing_docs)]

impl Model306 {
    pub const GHI: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const A: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const TMP: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
}

impl crate::Model for Model306 {
    const ID: u16 = 306;
    const LENGTH: u16 = 4;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ghi: Self::GHI.from_data(data)?,
            a: Self::A.from_data(data)?,
            v: Self::V.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
        })
    }
}

/// Base Met
///
/// Base Meteorological Model
///
/// Notes: This model supersedes model 301
#[derive(Debug)]
pub struct Model307 {
    /// Ambient Temperature
    pub tmpamb: Option<i16>,
    /// Relative Humidity
    pub rh: Option<i16>,
    /// Barometric Pressure
    pub pres: Option<i16>,
    /// Wind Speed
    pub wndspd: Option<i16>,
    /// Wind Direction
    pub wnddir: Option<i16>,
    /// Rainfall
    pub rain: Option<i16>,
    /// Snow Depth
    pub snw: Option<i16>,
    /// Precipitation Type
    ///
    ///  Precipitation Type (WMO 4680 SYNOP code reference)
    pub ppt: Option<i16>,
    /// Electric Field
    pub elecfld: Option<i16>,
    /// Surface Wetness
    pub surwet: Option<i16>,
    /// Soil Wetness
    pub soilwet: Option<i16>,
}

#[allow(missing_docs)]

impl Model307 {
    pub const TMPAMB: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const RH: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const PRES: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const WNDSPD: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const WNDDIR: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const RAIN: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const SNW: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const PPT: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const ELECFLD: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const SURWET: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const SOILWET: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
}

impl crate::Model for Model307 {
    const ID: u16 = 307;
    const LENGTH: u16 = 11;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            tmpamb: Self::TMPAMB.from_data(data)?,
            rh: Self::RH.from_data(data)?,
            pres: Self::PRES.from_data(data)?,
            wndspd: Self::WNDSPD.from_data(data)?,
            wnddir: Self::WNDDIR.from_data(data)?,
            rain: Self::RAIN.from_data(data)?,
            snw: Self::SNW.from_data(data)?,
            ppt: Self::PPT.from_data(data)?,
            elecfld: Self::ELECFLD.from_data(data)?,
            surwet: Self::SURWET.from_data(data)?,
            soilwet: Self::SOILWET.from_data(data)?,
        })
    }
}

/// Mini Met Model
///
/// Include to support a few basic measurements
#[derive(Debug)]
pub struct Model308 {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// Temp
    ///
    /// Back of module temperature measurement
    pub tmpbom: Option<i16>,
    /// Ambient Temperature
    pub tmpamb: Option<i16>,
    /// Wind Speed
    pub wndspd: Option<u16>,
}

#[allow(missing_docs)]

impl Model308 {
    pub const GHI: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const TMPBOM: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const TMPAMB: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const WNDSPD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
}

impl crate::Model for Model308 {
    const ID: u16 = 308;
    const LENGTH: u16 = 4;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ghi: Self::GHI.from_data(data)?,
            tmpbom: Self::TMPBOM.from_data(data)?,
            tmpamb: Self::TMPAMB.from_data(data)?,
            wndspd: Self::WNDSPD.from_data(data)?,
        })
    }
}

/// String Combiner (Current)
///
/// A basic string combiner
///
/// Notes: This model is SUPERSEDED by model 403
#[derive(Debug)]
pub struct Model401 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dcahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dcamax: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: u32,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evtvnd: Option<u32>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dcahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
}

#[allow(missing_docs)]

impl Model401 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const DCAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const DCAHR: crate::PointDef<Self, u32> = crate::PointDef::new(10, 2, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const TMP: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
}

impl crate::Model for Model401 {
    const ID: u16 = 401;
    const LENGTH: u16 = 22;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr_sf: Self::DCAHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcamax: Self::DCAMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            dca: Self::DCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr: Self::DCAHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
        })
    }
}

/// String Combiner (Advanced)
///
/// An advanced string combiner
///
/// Notes: This model is SUPERSEDED by model 404
#[derive(Debug)]
pub struct Model402 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dcahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Power scale factor
    pub dcw_sf: Option<i16>,
    /// Energy scale factor
    pub dcwh_sf: i16,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dcamax: Option<u16>,
    /// N
    ///
    /// Number of Inputs
    pub n: Option<u16>,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: u32,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evtvnd: Option<u32>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dcahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Watts
    ///
    /// Output power
    pub dcw: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    pub dcpr: Option<u16>,
    /// Watt-hours
    ///
    /// Output energy
    pub dcwh: u32,
}

#[allow(missing_docs)]

impl Model402 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const DCAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(9, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const DCAHR: crate::PointDef<Self, u32> = crate::PointDef::new(12, 2, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const TMP: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const DCW: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const DCPR: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const DCWH: crate::PointDef<Self, u32> = crate::PointDef::new(18, 2, false);
}

impl crate::Model for Model402 {
    const ID: u16 = 402;
    const LENGTH: u16 = 34;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr_sf: Self::DCAHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcamax: Self::DCAMAX.from_data(data)?,
            n: Self::N.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            dca: Self::DCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr: Self::DCAHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcpr: Self::DCPR.from_data(data)?,
            dcwh: Self::DCWH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// String Combiner (Current)
///
/// A basic string combiner model
///
/// Notes: This model supersedes model 401
#[derive(Debug)]
pub struct Model403 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dcahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dcamax: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: u32,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evtvnd: Option<u32>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dcahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<i16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Current scale factor for inputs
    pub indca_sf: Option<i16>,
    /// Amp-hour scale factor for inputs
    pub indcahr_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model403 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const DCAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const DCAHR: crate::PointDef<Self, u32> = crate::PointDef::new(10, 2, false);
    pub const DCV: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const TMP: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const INDCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const INDCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
}

impl crate::Model for Model403 {
    const ID: u16 = 403;
    const LENGTH: u16 = 24;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr_sf: Self::DCAHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcamax: Self::DCAMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            dca: Self::DCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr: Self::DCAHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            indca_sf: Self::INDCA_SF.from_data(data)?,
            indcahr_sf: Self::INDCAHR_SF.from_data(data)?,
        })
    }
}

/// String Combiner (Advanced)
///
/// An advanced string combiner including voltage and energy measurements
///
/// Notes: This model supersedes model 402
#[derive(Debug)]
pub struct Model404 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dcahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Power scale factor
    pub dcw_sf: Option<i16>,
    /// Energy scale factor
    pub dcwh_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dcamax: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: u32,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evtvnd: Option<u32>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dcahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<i16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Watts
    ///
    /// Output power
    pub dcw: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    pub dcpr: Option<i16>,
    /// Watt-hours
    ///
    /// Output energy
    pub dcwh: Option<u32>,
    /// Current scale factor for inputs
    pub indca_sf: Option<i16>,
    /// Amp-hour scale factor for inputs
    pub indcahr_sf: Option<i16>,
    /// Voltage scale factor for inputs
    pub indcv_sf: Option<i16>,
    /// Power scale factor for inputs
    pub indcw_sf: Option<i16>,
    /// Energy scale factor for inputs
    pub indcwh_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model404 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const DCAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(9, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const DCAHR: crate::PointDef<Self, u32> = crate::PointDef::new(12, 2, false);
    pub const DCV: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const TMP: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const DCW: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const DCPR: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const DCWH: crate::PointDef<Self, u32> = crate::PointDef::new(18, 2, false);
    pub const INDCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const INDCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const INDCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const INDCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const INDCWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
}

impl crate::Model for Model404 {
    const ID: u16 = 404;
    const LENGTH: u16 = 39;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr_sf: Self::DCAHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            dcamax: Self::DCAMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            dca: Self::DCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr: Self::DCAHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcpr: Self::DCPR.from_data(data)?,
            dcwh: Self::DCWH.from_data(data)?,
            indca_sf: Self::INDCA_SF.from_data(data)?,
            indcahr_sf: Self::INDCAHR_SF.from_data(data)?,
            indcv_sf: Self::INDCV_SF.from_data(data)?,
            indcw_sf: Self::INDCW_SF.from_data(data)?,
            indcwh_sf: Self::INDCWH_SF.from_data(data)?,
        })
    }
}

/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model501 {
    /// Status
    ///
    /// Enumerated value.  Module Status Code
    pub stat: u16,
    /// Vendor Status
    ///
    /// Module Vendor Status Code
    pub statvend: Option<u16>,
    /// Events
    ///
    /// Bitmask value.  Module Event Flags
    pub evt: u32,
    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    pub evtvend: Option<u32>,
    /// Control
    ///
    /// Module Control
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor Module Control
    pub ctlvend: Option<u32>,
    /// Control Value
    ///
    /// Module Control Value
    pub ctlval: Option<i32>,
    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    pub tms: Option<u32>,
    /// Output Current
    ///
    /// Output Current
    pub outa: Option<f32>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub outv: Option<f32>,
    /// Output Energy
    ///
    /// Output Energy
    pub outwh: Option<f32>,
    /// Output Power
    ///
    /// Output Power
    pub outw: Option<f32>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<f32>,
    /// Input Current
    ///
    /// Input Current
    pub ina: Option<f32>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub inv: Option<f32>,
    /// Input Energy
    ///
    /// Input Energy
    pub inwh: Option<f32>,
    /// Input Power
    ///
    /// Input Power
    pub inw: Option<f32>,
}

#[allow(missing_docs)]

impl Model501 {
    pub const STAT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STATVEND: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const EVTVEND: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const CTLVEND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, true);
    pub const CTLVAL: crate::PointDef<Self, i32> = crate::PointDef::new(9, 2, true);
    pub const TMS: crate::PointDef<Self, u32> = crate::PointDef::new(11, 2, false);
    pub const OUTA: crate::PointDef<Self, f32> = crate::PointDef::new(13, 2, false);
    pub const OUTV: crate::PointDef<Self, f32> = crate::PointDef::new(15, 2, false);
    pub const OUTWH: crate::PointDef<Self, f32> = crate::PointDef::new(17, 2, false);
    pub const OUTW: crate::PointDef<Self, f32> = crate::PointDef::new(19, 2, false);
    pub const TMP: crate::PointDef<Self, f32> = crate::PointDef::new(21, 2, false);
    pub const INA: crate::PointDef<Self, f32> = crate::PointDef::new(23, 2, false);
    pub const INV: crate::PointDef<Self, f32> = crate::PointDef::new(25, 2, false);
    pub const INWH: crate::PointDef<Self, f32> = crate::PointDef::new(27, 2, false);
    pub const INW: crate::PointDef<Self, f32> = crate::PointDef::new(29, 2, false);
}

impl crate::Model for Model501 {
    const ID: u16 = 501;
    const LENGTH: u16 = 31;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stat: Self::STAT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            statvend: Self::STATVEND.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvend: Self::EVTVEND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctlvend: Self::CTLVEND.from_data(data)?,
            ctlval: Self::CTLVAL.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            outa: Self::OUTA.from_data(data)?,
            outv: Self::OUTV.from_data(data)?,
            outwh: Self::OUTWH.from_data(data)?,
            outw: Self::OUTW.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            ina: Self::INA.from_data(data)?,
            inv: Self::INV.from_data(data)?,
            inwh: Self::INWH.from_data(data)?,
            inw: Self::INW.from_data(data)?,
        })
    }
}

/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Notes: Integer
#[derive(Debug)]
pub struct Model502 {
    /// Current scale factor
    pub a_sf: Option<i16>,
    /// Voltage scale factor
    pub v_sf: Option<i16>,
    /// Power scale factor
    pub w_sf: Option<i16>,
    /// Energy scale factor
    pub wh_sf: Option<i16>,
    /// Status
    ///
    /// Enumerated value.  Module Status Code
    pub stat: u16,
    /// Vendor Status
    ///
    /// Module Vendor Status Code
    pub statvend: Option<u16>,
    /// Events
    ///
    /// Bitmask value.  Module Event Flags
    pub evt: u32,
    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    pub evtvend: Option<u32>,
    /// Control
    ///
    /// Module Control
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor Module Control
    pub ctlvend: Option<u32>,
    /// Control Value
    ///
    /// Module Control Value
    pub ctlval: Option<i32>,
    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    pub tms: Option<u32>,
    /// Output Current
    ///
    /// Output Current
    pub outa: Option<i16>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub outv: Option<i16>,
    /// Output Energy
    ///
    /// Output Energy
    pub outwh: Option<u32>,
    /// Output Power
    ///
    /// Output Power
    pub outpw: Option<i16>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<i16>,
    /// Input Current
    ///
    /// Input Current
    pub ina: Option<i16>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub inv: Option<i16>,
    /// Input Energy
    ///
    /// Input Energy
    pub inwh: Option<u32>,
    /// Input Power
    ///
    /// Input Power
    pub inw: Option<i16>,
}

#[allow(missing_docs)]

impl Model502 {
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const STAT: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const STATVEND: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(6, 2, false);
    pub const EVTVEND: crate::PointDef<Self, u32> = crate::PointDef::new(8, 2, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const CTLVEND: crate::PointDef<Self, u32> = crate::PointDef::new(11, 2, true);
    pub const CTLVAL: crate::PointDef<Self, i32> = crate::PointDef::new(13, 2, true);
    pub const TMS: crate::PointDef<Self, u32> = crate::PointDef::new(15, 2, false);
    pub const OUTA: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const OUTV: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const OUTWH: crate::PointDef<Self, u32> = crate::PointDef::new(19, 2, false);
    pub const OUTPW: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const TMP: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const INA: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const INV: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const INWH: crate::PointDef<Self, u32> = crate::PointDef::new(25, 2, false);
    pub const INW: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
}

impl crate::Model for Model502 {
    const ID: u16 = 502;
    const LENGTH: u16 = 28;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a_sf: Self::A_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            wh_sf: Self::WH_SF.from_data(data)?,
            stat: Self::STAT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            statvend: Self::STATVEND.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvend: Self::EVTVEND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctlvend: Self::CTLVEND.from_data(data)?,
            ctlval: Self::CTLVAL.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            outa: Self::OUTA.from_data(data)?,
            outv: Self::OUTV.from_data(data)?,
            outwh: Self::OUTWH.from_data(data)?,
            outpw: Self::OUTPW.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            ina: Self::INA.from_data(data)?,
            inv: Self::INV.from_data(data)?,
            inwh: Self::INWH.from_data(data)?,
            inw: Self::INW.from_data(data)?,
        })
    }
}

/// Tracker Controller DRAFT 2
///
/// Monitors and controls multiple trackers
///
/// Notes: Trackers may include GPS model 305 for location information
#[derive(Debug)]
pub struct Model601 {
    /// Controller
    ///
    /// Descriptive name for this control unit
    pub nam: Option<String>,
    /// Type
    ///
    /// Type of tracker
    pub typ: u16,
    /// Date
    ///
    /// Local date in YYYYMMDD format
    pub dtloc: Option<String>,
    /// Time
    ///
    /// 24 hour local time stamp to second
    pub tmloc: Option<String>,
    /// Day
    ///
    /// Number of the day in the year (1-366)
    pub day: Option<u16>,
    /// Manual Elevation
    ///
    /// Global manual override target position of elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type
    pub glblelctl: Option<i32>,
    /// Manual Azimuth
    ///
    /// Global manual override target position of azimuth in degrees from true north towards east.  Unimplemented for single axis azimuth tracker type
    pub glblazctl: Option<i32>,
    /// Global Mode
    ///
    /// Global Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    ///
    /// Notes: The global controls all trackers
    pub glblctl: Option<u16>,
    /// Global Alarm
    ///
    /// Global tracker alarm conditions
    ///
    /// Notes: Combined tracker alarm conditions.  See individual trackers for alarms
    pub glblalm: Option<u16>,
    /// SF
    ///
    /// Scale Factor for targets and position measurements in degrees
    pub dgr_sf: i16,
    /// Trackers
    ///
    /// Number of trackers being controlled.  Size of repeating block.
    pub n: u16,
}

#[allow(missing_docs)]

impl Model601 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 8, false);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const DTLOC: crate::PointDef<Self, String> = crate::PointDef::new(9, 5, false);
    pub const TMLOC: crate::PointDef<Self, String> = crate::PointDef::new(14, 3, false);
    pub const DAY: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const GLBLELCTL: crate::PointDef<Self, i32> = crate::PointDef::new(18, 2, true);
    pub const GLBLAZCTL: crate::PointDef<Self, i32> = crate::PointDef::new(20, 2, true);
    pub const GLBLCTL: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const GLBLALM: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, false);
    pub const DGR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
}

impl crate::Model for Model601 {
    const ID: u16 = 601;
    const LENGTH: u16 = 48;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dtloc: Self::DTLOC.from_data(data)?,
            tmloc: Self::TMLOC.from_data(data)?,
            day: Self::DAY.from_data(data)?,
            glblelctl: Self::GLBLELCTL.from_data(data)?,
            glblazctl: Self::GLBLAZCTL.from_data(data)?,
            glblctl: Self::GLBLCTL.from_data(data)?,
            glblalm: Self::GLBLALM.from_data(data)?,
            dgr_sf: Self::DGR_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Energy Storage Base Model (DEPRECATED)
///
/// This model has been deprecated.
#[derive(Debug)]
pub struct Model801 {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    pub deprecated: u16,
}

#[allow(missing_docs)]

impl Model801 {
    pub const DEPRECATED: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model801 {
    const ID: u16 = 801;
    const LENGTH: u16 = 1;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            deprecated: Self::DEPRECATED
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Battery Base Model
#[derive(Debug)]
pub struct Model802 {
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    pub ahrtg: u16,
    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    pub whrtg: u16,
    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    pub wchartemax: u16,
    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    pub wdischartemax: u16,
    /// Self Discharge Rate
    ///
    /// Self discharge rate.  Percentage of capacity (WHRtg) discharged per day.
    pub discharte: Option<u16>,
    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    pub socmax: Option<u16>,
    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    pub socmin: Option<u16>,
    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    pub socrsvmax: Option<u16>,
    /// Min Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    pub socrsvmin: Option<u16>,
    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub soc: u16,
    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub dod: Option<u16>,
    /// State of Health
    ///
    /// Percentage of battery life remaining.
    pub soh: Option<u16>,
    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    pub ncyc: Option<u32>,
    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    pub chast: Option<u16>,
    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Notes: Maps to DRCC.LocRemCtl in IEC 61850.
    pub locremctl: u16,
    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    pub hb: Option<u16>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    pub ctrlhb: Option<u16>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms.  1 = Reset.
    ///
    /// Notes: Battery should reset to 0 when reset is complete.
    pub almrst: u16,
    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Notes: Maps to DBAT.BatTyp in 61850.
    pub typ: u16,
    /// State of the Battery Bank
    ///
    /// State of the battery bank.  Enumeration.
    ///
    /// Notes: Must be reconciled with State in IEC 61850.
    pub state: u16,
    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state.  Enumeration.
    pub statevnd: Option<u16>,
    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Notes: Number of days since 1/1/2000.
    pub warrdt: Option<u32>,
    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.  Bit flags.
    pub evt1: u32,
    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.  Bit flags.
    ///
    /// Notes: Reserved for future use.
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evtvnd1: u32,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evtvnd2: u32,
    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Notes: Maps to ZBAT.V in IEC 61850.
    pub v: u16,
    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// Notes: If not implemented, must implement AChaMax and ADisChaMax.
    pub vmax: Option<u16>,
    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// Notes: If not implemented, must implement AChaMax and ADisChaMax.
    pub vmin: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Notes: Measurement.
    pub cellvmax: Option<u16>,
    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    pub cellvmaxstr: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    pub cellvmaxmod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Notes: Measurement.
    pub cellvmin: Option<u16>,
    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    pub cellvminstr: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    pub cellvminmod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: Option<u16>,
    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Notes: Measurement.
    pub a: i16,
    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Notes: Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    pub achamax: Option<u16>,
    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Notes: Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    pub adischamax: Option<u16>,
    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// Notes: Measurement.
    pub w: i16,
    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter.  Enumeration.
    ///
    /// Notes: Used in special states such as manual battery charging.
    pub reqinvstate: Option<u16>,
    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Notes: Used in special states such as string balancing.
    pub reqw: Option<i16>,
    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting.  Enumeration.
    pub setop: u16,
    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Notes: Information needed by battery for some operations.
    pub setinvstate: u16,
    /// Scale factor for charge capacity.
    pub ahrtg_sf: i16,
    /// Scale factor for energy capacity.
    pub whrtg_sf: i16,
    /// Scale factor for maximum charge and discharge rate.
    pub wchadischamax_sf: i16,
    /// Scale factor for self discharge rate.
    pub discharte_sf: Option<i16>,
    /// Scale factor for state of charge values.
    pub soc_sf: i16,
    /// Scale factor for depth of discharge.
    pub dod_sf: Option<i16>,
    /// Scale factor for state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for DC bus voltage.
    pub v_sf: i16,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for DC current.
    pub a_sf: i16,
    /// Scale factor for instantaneous DC charge/discharge current.
    pub amax_sf: i16,
    /// Scale factor for AC power request.
    pub w_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model802 {
    pub const AHRTG: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const WHRTG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const WCHARTEMAX: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const WDISCHARTEMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const DISCHARTE: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const SOCMAX: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const SOCMIN: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const SOCRSVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const SOCRSVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const DOD: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const SOH: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const NCYC: crate::PointDef<Self, u32> = crate::PointDef::new(12, 2, false);
    pub const CHAST: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const LOCREMCTL: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const HB: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const CTRLHB: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, true);
    pub const ALMRST: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const STATE: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const STATEVND: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const WARRDT: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(30, 2, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, false);
    pub const VMAX: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, false);
    pub const VMIN: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, false);
    pub const CELLVMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, false);
    pub const CELLVMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, false);
    pub const CELLVMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, false);
    pub const CELLVMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(42, 1, false);
    pub const ACHAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, false);
    pub const ADISCHAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(44, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(45, 1, false);
    pub const REQINVSTATE: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const REQW: crate::PointDef<Self, i16> = crate::PointDef::new(47, 1, false);
    pub const SETOP: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const SETINVSTATE: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const AHRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(50, 1, false);
    pub const WHRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(51, 1, false);
    pub const WCHADISCHAMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const DISCHARTE_SF: crate::PointDef<Self, i16> = crate::PointDef::new(53, 1, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(54, 1, false);
    pub const DOD_SF: crate::PointDef<Self, i16> = crate::PointDef::new(55, 1, false);
    pub const SOH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(56, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(57, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(58, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(59, 1, false);
    pub const AMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(60, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(61, 1, false);
}

impl crate::Model for Model802 {
    const ID: u16 = 802;
    const LENGTH: u16 = 62;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ahrtg: Self::AHRTG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            whrtg: Self::WHRTG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wchartemax: Self::WCHARTEMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wdischartemax: Self::WDISCHARTEMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            discharte: Self::DISCHARTE.from_data(data)?,
            socmax: Self::SOCMAX.from_data(data)?,
            socmin: Self::SOCMIN.from_data(data)?,
            socrsvmax: Self::SOCRSVMAX.from_data(data)?,
            socrsvmin: Self::SOCRSVMIN.from_data(data)?,
            soc: Self::SOC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dod: Self::DOD.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            ncyc: Self::NCYC.from_data(data)?,
            chast: Self::CHAST.from_data(data)?,
            locremctl: Self::LOCREMCTL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hb: Self::HB.from_data(data)?,
            ctrlhb: Self::CTRLHB.from_data(data)?,
            almrst: Self::ALMRST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            state: Self::STATE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            statevnd: Self::STATEVND.from_data(data)?,
            warrdt: Self::WARRDT.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd2: Self::EVTVND2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v: Self::V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vmax: Self::VMAX.from_data(data)?,
            vmin: Self::VMIN.from_data(data)?,
            cellvmax: Self::CELLVMAX.from_data(data)?,
            cellvmaxstr: Self::CELLVMAXSTR.from_data(data)?,
            cellvmaxmod: Self::CELLVMAXMOD.from_data(data)?,
            cellvmin: Self::CELLVMIN.from_data(data)?,
            cellvminstr: Self::CELLVMINSTR.from_data(data)?,
            cellvminmod: Self::CELLVMINMOD.from_data(data)?,
            cellvavg: Self::CELLVAVG.from_data(data)?,
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            achamax: Self::ACHAMAX.from_data(data)?,
            adischamax: Self::ADISCHAMAX.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            reqinvstate: Self::REQINVSTATE.from_data(data)?,
            reqw: Self::REQW.from_data(data)?,
            setop: Self::SETOP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            setinvstate: Self::SETINVSTATE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ahrtg_sf: Self::AHRTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            whrtg_sf: Self::WHRTG_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wchadischamax_sf: Self::WCHADISCHAMAX_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            discharte_sf: Self::DISCHARTE_SF.from_data(data)?,
            soc_sf: Self::SOC_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dod_sf: Self::DOD_SF.from_data(data)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            amax_sf: Self::AMAX_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF.from_data(data)?,
        })
    }
}

/// Lithium-Ion Battery Bank Model
#[derive(Debug)]
pub struct Model803 {
    /// String Count
    ///
    /// Number of strings in the bank.
    pub nstr: u16,
    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    pub nstrcon: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Notes: Measurement.
    pub modtmpmax: i16,
    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    pub modtmpmaxstr: Option<u16>,
    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    pub modtmpmaxmod: Option<u16>,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Notes: Measurement.
    pub modtmpmin: i16,
    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    pub modtmpminstr: Option<u16>,
    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    pub modtmpminmod: Option<u16>,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub modtmpavg: Option<i16>,
    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Notes: Measurement.
    pub strvmax: Option<u16>,
    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    pub strvmaxstr: Option<u16>,
    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Notes: Measurement.
    pub strvmin: Option<u16>,
    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    pub strvminstr: Option<u16>,
    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub strvavg: Option<u16>,
    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Notes: Measurement.
    pub stramax: Option<i16>,
    /// Max String Current String
    ///
    /// String with the maximum current.
    pub stramaxstr: Option<u16>,
    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Notes: Measurement.
    pub stramin: Option<i16>,
    /// Min String Current String
    ///
    /// String with the minimum current.
    pub straminstr: Option<u16>,
    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub straavg: Option<i16>,
    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    pub ncellbal: Option<u16>,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for module temperatures.
    pub modtmp_sf: i16,
    /// Scale factor for string currents.
    pub a_sf: i16,
    /// Scale factor for string state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for string state of charge.
    pub soc_sf: i16,
    /// Scale factor for string voltage.
    pub v_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model803 {
    pub const NSTR: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const NSTRCON: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const MODTMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const MODTMPMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const MODTMPMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const MODTMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const MODTMPMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const MODTMPMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const MODTMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const STRVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const STRVMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const STRVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const STRVMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const STRVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const STRAMAX: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const STRAMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const STRAMIN: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const STRAMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const STRAAVG: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const NCELLBAL: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const MODTMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const SOH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
}

impl crate::Model for Model803 {
    const ID: u16 = 803;
    const LENGTH: u16 = 58;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nstr: Self::NSTR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            nstrcon: Self::NSTRCON
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmax: Self::MODTMPMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmaxstr: Self::MODTMPMAXSTR.from_data(data)?,
            modtmpmaxmod: Self::MODTMPMAXMOD.from_data(data)?,
            modtmpmin: Self::MODTMPMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpminstr: Self::MODTMPMINSTR.from_data(data)?,
            modtmpminmod: Self::MODTMPMINMOD.from_data(data)?,
            modtmpavg: Self::MODTMPAVG.from_data(data)?,
            strvmax: Self::STRVMAX.from_data(data)?,
            strvmaxstr: Self::STRVMAXSTR.from_data(data)?,
            strvmin: Self::STRVMIN.from_data(data)?,
            strvminstr: Self::STRVMINSTR.from_data(data)?,
            strvavg: Self::STRVAVG.from_data(data)?,
            stramax: Self::STRAMAX.from_data(data)?,
            stramaxstr: Self::STRAMAXSTR.from_data(data)?,
            stramin: Self::STRAMIN.from_data(data)?,
            straminstr: Self::STRAMINSTR.from_data(data)?,
            straavg: Self::STRAAVG.from_data(data)?,
            ncellbal: Self::NCELLBAL.from_data(data)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmp_sf: Self::MODTMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            soc_sf: Self::SOC_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF.from_data(data)?,
        })
    }
}

/// Lithium-Ion String Model
#[derive(Debug)]
pub struct Model804 {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Notes: Indices are one-based.
    pub idx: u16,
    /// Module Count
    ///
    /// Count of modules in the string.
    pub nmod: u16,
    /// String Status
    ///
    /// Current status of the string.
    pub st: u32,
    /// Connection Failure Reason
    pub confail: Option<u16>,
    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    pub ncellbal: Option<u16>,
    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub soc: u16,
    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub dod: Option<u16>,
    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    pub ncyc: Option<u32>,
    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub soh: Option<u16>,
    /// String Current
    ///
    /// String current measurement.
    ///
    /// Notes: Measurement.
    pub a: i16,
    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Notes: Measurement.
    pub v: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmax: u16,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    pub cellvmaxmod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmin: u16,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    pub cellvminmod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modtmpmax: i16,
    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    pub modtmpmaxmod: u16,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modtmpmin: i16,
    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    pub modtmpminmod: u16,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub modtmpavg: i16,
    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    pub r#const: Option<u32>,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt1: u32,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    ///
    /// Notes: Reserved for future use.
    pub evt2: Option<u32>,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evtvnd2: Option<u32>,
    /// Enable/Disable String
    ///
    /// Enables and disables the string.  Should reset to 0 upon completion.
    pub setena: Option<u16>,
    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Notes: Should reset to 0 upon completion.
    pub setcon: Option<u16>,
    /// Scale factor for string state of charge.
    pub soc_sf: i16,
    /// Scale factor for string state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for string depth of discharge.
    pub dod_sf: Option<i16>,
    /// Scale factor for string current.
    pub a_sf: i16,
    /// Scale factor for string voltage.
    pub v_sf: Option<i16>,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for module temperature.
    pub modtmp_sf: i16,
}

#[allow(missing_docs)]

impl Model804 {
    pub const IDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const NMOD: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ST: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const CONFAIL: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const NCELLBAL: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const DOD: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const NCYC: crate::PointDef<Self, u32> = crate::PointDef::new(8, 2, false);
    pub const SOH: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CELLVMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const CELLVMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const MODTMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const MODTMPMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const MODTMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const MODTMPMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const MODTMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const CONST: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(30, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(32, 2, false);
    pub const SETENA: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const SETCON: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(36, 1, false);
    pub const SOH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(37, 1, false);
    pub const DOD_SF: crate::PointDef<Self, i16> = crate::PointDef::new(38, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(39, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(40, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(41, 1, false);
    pub const MODTMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(42, 1, false);
}

impl crate::Model for Model804 {
    const ID: u16 = 804;
    const LENGTH: u16 = 62;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            idx: Self::IDX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            nmod: Self::NMOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            confail: Self::CONFAIL.from_data(data)?,
            ncellbal: Self::NCELLBAL.from_data(data)?,
            soc: Self::SOC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dod: Self::DOD.from_data(data)?,
            ncyc: Self::NCYC.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v: Self::V.from_data(data)?,
            cellvmax: Self::CELLVMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmaxmod: Self::CELLVMAXMOD.from_data(data)?,
            cellvmin: Self::CELLVMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvminmod: Self::CELLVMINMOD.from_data(data)?,
            cellvavg: Self::CELLVAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmax: Self::MODTMPMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmaxmod: Self::MODTMPMAXMOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmin: Self::MODTMPMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpminmod: Self::MODTMPMINMOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpavg: Self::MODTMPAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            r#const: Self::CONST.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2.from_data(data)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            setena: Self::SETENA.from_data(data)?,
            setcon: Self::SETCON.from_data(data)?,
            soc_sf: Self::SOC_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            dod_sf: Self::DOD_SF.from_data(data)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF.from_data(data)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmp_sf: Self::MODTMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Lithium-Ion Module Model
#[derive(Debug)]
pub struct Model805 {
    /// String Index
    ///
    /// Index of the string containing the module.
    ///
    /// Notes: Indices are one-based.
    pub stridx: u16,
    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Notes: Indices are one-based.
    pub modidx: u16,
    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    pub ncell: u16,
    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    pub soc: Option<u16>,
    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Notes: Measurement.
    pub dod: Option<u16>,
    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    pub soh: Option<u16>,
    /// Cycle Count
    ///
    /// Count of cycles executed.
    pub ncyc: Option<u32>,
    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Notes: Measurement.
    pub v: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Notes: Measurement.
    pub cellvmax: u16,
    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    pub cellvmaxcell: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Notes: Measurement.
    pub cellvmin: u16,
    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    pub cellvmincell: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: u16,
    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Notes: Measurement.
    pub celltmpmax: i16,
    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    pub celltmpmaxcell: Option<u16>,
    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Notes: Measurement.
    pub celltmpmin: i16,
    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    pub celltmpmincell: Option<u16>,
    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Notes: Calculation based on measurements.
    pub celltmpavg: i16,
    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    pub ncellbal: Option<u16>,
    /// Serial Number
    ///
    /// Serial number for the module.
    pub sn: Option<String>,
    /// Scale factor for module state of charge.
    pub soc_sf: Option<i16>,
    /// Scale factor for module state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for module depth of discharge.
    pub dod_sf: Option<i16>,
    /// Scale factor for module voltage.
    pub v_sf: i16,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for module temperature.
    pub tmp_sf: i16,
}

#[allow(missing_docs)]

impl Model805 {
    pub const STRIDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const MODIDX: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const NCELL: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const DOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const SOH: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NCYC: crate::PointDef<Self, u32> = crate::PointDef::new(6, 2, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CELLVMAXCELL: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const CELLVMINCELL: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CELLTMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const CELLTMPMAXCELL: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const CELLTMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const CELLTMPMINCELL: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const CELLTMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const NCELLBAL: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const SN: crate::PointDef<Self, String> = crate::PointDef::new(20, 16, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(36, 1, false);
    pub const SOH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(37, 1, false);
    pub const DOD_SF: crate::PointDef<Self, i16> = crate::PointDef::new(38, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(39, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(40, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(41, 1, false);
}

impl crate::Model for Model805 {
    const ID: u16 = 805;
    const LENGTH: u16 = 46;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stridx: Self::STRIDX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modidx: Self::MODIDX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ncell: Self::NCELL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            soc: Self::SOC.from_data(data)?,
            dod: Self::DOD.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            ncyc: Self::NCYC.from_data(data)?,
            v: Self::V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmax: Self::CELLVMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmaxcell: Self::CELLVMAXCELL.from_data(data)?,
            cellvmin: Self::CELLVMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmincell: Self::CELLVMINCELL.from_data(data)?,
            cellvavg: Self::CELLVAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            celltmpmax: Self::CELLTMPMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            celltmpmaxcell: Self::CELLTMPMAXCELL.from_data(data)?,
            celltmpmin: Self::CELLTMPMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            celltmpmincell: Self::CELLTMPMINCELL.from_data(data)?,
            celltmpavg: Self::CELLTMPAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ncellbal: Self::NCELLBAL.from_data(data)?,
            sn: Self::SN.from_data(data)?,
            soc_sf: Self::SOC_SF.from_data(data)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            dod_sf: Self::DOD_SF.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Flow Battery Model
#[derive(Debug)]
pub struct Model806 {
    /// Battery Points To Be Determined
    pub battbd: u16,
}

#[allow(missing_docs)]

impl Model806 {
    pub const BATTBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model806 {
    const ID: u16 = 806;
    const LENGTH: u16 = 2;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            battbd: Self::BATTBD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Flow Battery String Model
#[derive(Debug)]
pub struct Model807 {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Notes: Indices are one-based.
    pub idx: u16,
    /// Module Count
    ///
    /// Number of modules in this string.
    pub nmod: u16,
    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    pub nmodcon: u16,
    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modvmax: u16,
    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    pub modvmaxmod: Option<u16>,
    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modvmin: u16,
    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    pub modvminmod: Option<u16>,
    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub modvavg: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmax: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    pub cellvmaxmod: Option<u16>,
    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    pub cellvmaxstk: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmin: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    pub cellvminmod: Option<u16>,
    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    pub cellvminstk: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: Option<u16>,
    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub tmpmax: i16,
    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    pub tmpmaxmod: Option<u16>,
    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub tmpmin: i16,
    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    pub tmpminmod: Option<u16>,
    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub tmpavg: i16,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt1: u32,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evtvnd1: u32,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evtvnd2: u32,
    #[allow(missing_docs)]
    pub modv_sf: i16,
    /// Scale factor for voltage.
    pub cellv_sf: i16,
    /// Scale factor for temperature.
    pub tmp_sf: i16,
    /// Scale factor for state of charge.
    pub soc_sf: i16,
    /// Scale factor for open circuit voltage.
    pub ocv_sf: i16,
}

#[allow(missing_docs)]

impl Model807 {
    pub const IDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const NMOD: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const NMODCON: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const MODVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const MODVMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const MODVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const MODVMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const MODVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const CELLVMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CELLVMAXSTK: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const CELLVMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const CELLVMINSTK: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const TMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const TMPMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const TMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const TMPMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, false);
    pub const TMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(20, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const MODV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const OCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
}

impl crate::Model for Model807 {
    const ID: u16 = 807;
    const LENGTH: u16 = 58;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            idx: Self::IDX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            nmod: Self::NMOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            nmodcon: Self::NMODCON
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modvmax: Self::MODVMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modvmaxmod: Self::MODVMAXMOD.from_data(data)?,
            modvmin: Self::MODVMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modvminmod: Self::MODVMINMOD.from_data(data)?,
            modvavg: Self::MODVAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmax: Self::CELLVMAX.from_data(data)?,
            cellvmaxmod: Self::CELLVMAXMOD.from_data(data)?,
            cellvmaxstk: Self::CELLVMAXSTK.from_data(data)?,
            cellvmin: Self::CELLVMIN.from_data(data)?,
            cellvminmod: Self::CELLVMINMOD.from_data(data)?,
            cellvminstk: Self::CELLVMINSTK.from_data(data)?,
            cellvavg: Self::CELLVAVG.from_data(data)?,
            tmpmax: Self::TMPMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpmaxmod: Self::TMPMAXMOD.from_data(data)?,
            tmpmin: Self::TMPMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpminmod: Self::TMPMINMOD.from_data(data)?,
            tmpavg: Self::TMPAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd2: Self::EVTVND2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modv_sf: Self::MODV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            soc_sf: Self::SOC_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ocv_sf: Self::OCV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Flow Battery Module Model
#[derive(Debug)]
pub struct Model808 {
    /// Module Points To Be Determined
    pub moduletbd: u16,
}

#[allow(missing_docs)]

impl Model808 {
    pub const MODULETBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model808 {
    const ID: u16 = 808;
    const LENGTH: u16 = 2;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            moduletbd: Self::MODULETBD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Flow Battery Stack Model
#[derive(Debug)]
pub struct Model809 {
    /// Stack Points To Be Determined
    pub stacktbd: u16,
}

#[allow(missing_docs)]

impl Model809 {
    pub const STACKTBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model809 {
    const ID: u16 = 809;
    const LENGTH: u16 = 2;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stacktbd: Self::STACKTBD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// SunSpec Test Model 1
#[derive(Debug)]
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
    pub bitfield16: Option<u16>,
    #[allow(missing_docs)]
    pub bitfield16_u: Option<u16>,
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
    pub bitfield32: Option<u32>,
    #[allow(missing_docs)]
    pub bitfield32_u: Option<u32>,
    #[allow(missing_docs)]
    pub ipaddr: Option<u32>,
    #[allow(missing_docs)]
    pub ipaddr_u: Option<u32>,
    #[allow(missing_docs)]
    pub int64: Option<i64>,
    #[allow(missing_docs)]
    pub int64_u: Option<i64>,
    #[allow(missing_docs)]
    pub acc64: Option<u64>,
    #[allow(missing_docs)]
    pub acc64_u: Option<u64>,
    #[allow(missing_docs)]
    pub ipv6addr: Option<u128>,
    #[allow(missing_docs)]
    pub ipv6addr_u: Option<u128>,
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
    pub const SUNSSF_1: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const SUNSSF_2: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const SUNSSF_3: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const SUNSSF_4: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const INT16_1: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const INT16_2: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const INT16_3: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const INT16_4: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, true);
    pub const INT16_5: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const INT16_U: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const UINT16_1: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const UINT16_2: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const UINT16_3: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const UINT16_4: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, true);
    pub const UINT16_5: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const UINT16_U: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const ACC16: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const ACC16_U: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const ENUM16: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, false);
    pub const ENUM16_U: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const BITFIELD16: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const BITFIELD16_U: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const INT32_1: crate::PointDef<Self, i32> = crate::PointDef::new(22, 2, false);
    pub const INT32_2: crate::PointDef<Self, i32> = crate::PointDef::new(24, 2, false);
    pub const INT32_3: crate::PointDef<Self, i32> = crate::PointDef::new(26, 2, true);
    pub const INT32_4: crate::PointDef<Self, i32> = crate::PointDef::new(28, 2, false);
    pub const INT32_5: crate::PointDef<Self, i32> = crate::PointDef::new(30, 2, false);
    pub const INT32_U: crate::PointDef<Self, i32> = crate::PointDef::new(32, 2, false);
    pub const UINT32_1: crate::PointDef<Self, u32> = crate::PointDef::new(34, 2, false);
    pub const UINT32_2: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const UINT32_3: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, true);
    pub const UINT32_4: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const UINT32_5: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const UINT32_U: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const ACC32: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const ACC32_U: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const ENUM32: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const ENUM32_U: crate::PointDef<Self, u32> = crate::PointDef::new(52, 2, false);
    pub const BITFIELD32: crate::PointDef<Self, u32> = crate::PointDef::new(54, 2, false);
    pub const BITFIELD32_U: crate::PointDef<Self, u32> = crate::PointDef::new(56, 2, false);
    pub const IPADDR: crate::PointDef<Self, u32> = crate::PointDef::new(58, 2, true);
    pub const IPADDR_U: crate::PointDef<Self, u32> = crate::PointDef::new(60, 2, false);
    pub const INT64: crate::PointDef<Self, i64> = crate::PointDef::new(62, 4, true);
    pub const INT64_U: crate::PointDef<Self, i64> = crate::PointDef::new(66, 4, false);
    pub const ACC64: crate::PointDef<Self, u64> = crate::PointDef::new(70, 4, false);
    pub const ACC64_U: crate::PointDef<Self, u64> = crate::PointDef::new(74, 4, false);
    pub const IPV6ADDR: crate::PointDef<Self, u128> = crate::PointDef::new(78, 8, false);
    pub const IPV6ADDR_U: crate::PointDef<Self, u128> = crate::PointDef::new(86, 8, false);
    pub const FLOAT32: crate::PointDef<Self, f32> = crate::PointDef::new(94, 2, true);
    pub const FLOAT32_U: crate::PointDef<Self, f32> = crate::PointDef::new(96, 2, false);
    pub const STRING: crate::PointDef<Self, String> = crate::PointDef::new(98, 16, true);
    pub const STRING_U: crate::PointDef<Self, String> = crate::PointDef::new(114, 16, false);
    pub const SUNSSF_5: crate::PointDef<Self, i16> = crate::PointDef::new(130, 1, false);
    pub const SUNSSF_6: crate::PointDef<Self, i16> = crate::PointDef::new(131, 1, false);
    pub const SUNSSF_7: crate::PointDef<Self, i16> = crate::PointDef::new(132, 1, false);
}

impl crate::Model for Model63001 {
    const ID: u16 = 63001;
    const LENGTH: u16 = 152;
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
}

/// SunSpec Test Model 2
#[derive(Debug)]
pub struct Model63002;

#[allow(missing_docs)]

impl Model63002 {}

impl crate::Model for Model63002 {
    const ID: u16 = 63002;
    const LENGTH: u16 = 4;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}

/// Veris Status and Configuration
#[derive(Debug)]
pub struct Model64001 {
    /// Command Code
    pub cmd: Option<u16>,
    /// Hardware Revision
    pub hwrev: Option<u16>,
    /// RS FW Revision
    pub rsfwrev: Option<u16>,
    /// OS FW Revision
    pub osfwrev: Option<u16>,
    /// Product Revision
    pub prodrev: Option<String>,
    /// Boot Count
    pub boots: Option<u16>,
    /// DIP Switches
    pub switch: Option<u16>,
    /// Num Detected Sensors
    pub sensors: Option<u16>,
    /// Num Communicating Sensors
    pub talking: Option<u16>,
    /// System Status
    pub status: Option<u16>,
    /// System Configuration
    pub config: Option<u16>,
    /// LED Blink Threshold
    pub ledblink: Option<u16>,
    /// LED On Threshold
    pub ledon: Option<u16>,
    #[allow(missing_docs)]
    pub reserved: Option<u16>,
    /// Location String
    pub loc: Option<String>,
    /// Sensor 1 Unit ID
    pub s1id: Option<u16>,
    /// Sensor 1 Address
    pub s1addr: Option<u16>,
    /// Sensor 1 OS Version
    pub s1osver: Option<u16>,
    /// Sensor 1 Product Version
    pub s1ver: Option<String>,
    /// Sensor 1 Serial Num
    pub s1serial: Option<String>,
    /// Sensor 2 Unit ID
    pub s2id: Option<u16>,
    /// Sensor 2 Address
    pub s2addr: Option<u16>,
    /// Sensor 2 OS Version
    pub s2osver: Option<u16>,
    /// Sensor 2 Product Version
    pub s2ver: Option<String>,
    /// Sensor 2 Serial Num
    pub s2serial: Option<String>,
    /// Sensor 3 Unit ID
    pub s3id: Option<u16>,
    /// Sensor 3 Address
    pub s3addr: Option<u16>,
    /// Sensor 3 OS Version
    pub s3osver: Option<u16>,
    /// Sensor 3 Product Version
    pub s3ver: Option<String>,
    /// Sensor 3 Serial Num
    pub s3serial: Option<String>,
    /// Sensor 4 Unit ID
    pub s4id: Option<u16>,
    /// Sensor 4 Address
    pub s4addr: Option<u16>,
    /// Sensor 4 OS Version
    pub s4osver: Option<u16>,
    /// Sensor 4 Product Version
    pub s4ver: Option<String>,
    /// Sensor 4 Serial Num
    pub s4serial: Option<String>,
}

#[allow(missing_docs)]

impl Model64001 {
    pub const CMD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const HWREV: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const RSFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const OSFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const PRODREV: crate::PointDef<Self, String> = crate::PointDef::new(4, 2, false);
    pub const BOOTS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const SWITCH: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const SENSORS: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const TALKING: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const STATUS: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const CONFIG: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const LEDBLINK: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const LEDON: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const RESERVED: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const LOC: crate::PointDef<Self, String> = crate::PointDef::new(15, 16, false);
    pub const S1ID: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, false);
    pub const S1ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, false);
    pub const S1OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, false);
    pub const S1VER: crate::PointDef<Self, String> = crate::PointDef::new(34, 2, false);
    pub const S1SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(36, 5, false);
    pub const S2ID: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
    pub const S2ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, false);
    pub const S2OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, false);
    pub const S2VER: crate::PointDef<Self, String> = crate::PointDef::new(44, 2, false);
    pub const S2SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(46, 5, false);
    pub const S3ID: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, false);
    pub const S3ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, false);
    pub const S3OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, false);
    pub const S3VER: crate::PointDef<Self, String> = crate::PointDef::new(54, 2, false);
    pub const S3SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(56, 5, false);
    pub const S4ID: crate::PointDef<Self, u16> = crate::PointDef::new(61, 1, false);
    pub const S4ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(62, 1, false);
    pub const S4OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(63, 1, false);
    pub const S4VER: crate::PointDef<Self, String> = crate::PointDef::new(64, 2, false);
    pub const S4SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(66, 5, false);
}

impl crate::Model for Model64001 {
    const ID: u16 = 64001;
    const LENGTH: u16 = 71;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            cmd: Self::CMD.from_data(data)?,
            hwrev: Self::HWREV.from_data(data)?,
            rsfwrev: Self::RSFWREV.from_data(data)?,
            osfwrev: Self::OSFWREV.from_data(data)?,
            prodrev: Self::PRODREV.from_data(data)?,
            boots: Self::BOOTS.from_data(data)?,
            switch: Self::SWITCH.from_data(data)?,
            sensors: Self::SENSORS.from_data(data)?,
            talking: Self::TALKING.from_data(data)?,
            status: Self::STATUS.from_data(data)?,
            config: Self::CONFIG.from_data(data)?,
            ledblink: Self::LEDBLINK.from_data(data)?,
            ledon: Self::LEDON.from_data(data)?,
            reserved: Self::RESERVED.from_data(data)?,
            loc: Self::LOC.from_data(data)?,
            s1id: Self::S1ID.from_data(data)?,
            s1addr: Self::S1ADDR.from_data(data)?,
            s1osver: Self::S1OSVER.from_data(data)?,
            s1ver: Self::S1VER.from_data(data)?,
            s1serial: Self::S1SERIAL.from_data(data)?,
            s2id: Self::S2ID.from_data(data)?,
            s2addr: Self::S2ADDR.from_data(data)?,
            s2osver: Self::S2OSVER.from_data(data)?,
            s2ver: Self::S2VER.from_data(data)?,
            s2serial: Self::S2SERIAL.from_data(data)?,
            s3id: Self::S3ID.from_data(data)?,
            s3addr: Self::S3ADDR.from_data(data)?,
            s3osver: Self::S3OSVER.from_data(data)?,
            s3ver: Self::S3VER.from_data(data)?,
            s3serial: Self::S3SERIAL.from_data(data)?,
            s4id: Self::S4ID.from_data(data)?,
            s4addr: Self::S4ADDR.from_data(data)?,
            s4osver: Self::S4OSVER.from_data(data)?,
            s4ver: Self::S4VER.from_data(data)?,
            s4serial: Self::S4SERIAL.from_data(data)?,
        })
    }
}

/// Mersen GreenString
#[derive(Debug)]
pub struct Model64020 {
    /// Aux 0 temperature
    pub aux0tmp: Option<i16>,
    /// Aux 1 temperature
    pub aux1tmp: Option<i16>,
    /// Aux 2 temperature
    pub aux2tmp: Option<i16>,
    /// Aux 3 temperature
    pub aux3tmp: Option<i16>,
    /// Aux 4 temperature
    pub aux4tmp: Option<i16>,
    /// Probe Temperature
    pub probetmp: i16,
    /// Main Temperature
    pub maintmp: i16,
    /// Voltage scale factor for the sensors
    pub sensorv_sf: i16,
    /// Current scale factor for the sensors
    pub sensora_sf: i16,
    /// Frequency scale factor for the sensors
    pub sensorhz_sf: i16,
    /// Sensor1 Voltage
    ///
    /// scale of 0-10V
    pub sensor1voltage: Option<i16>,
    /// Sensor2 Voltage
    ///
    /// scale of 0-10V
    pub sensor2voltage: Option<i16>,
    /// Sensor3 Voltage
    ///
    /// scale of 0-10V
    pub sensor3voltage: Option<i16>,
    /// Sensor4 Voltage
    ///
    /// scale of 0-10V
    pub sensor4voltage: Option<i16>,
    /// Sensor5 Voltage
    ///
    /// scale of 0-10V
    pub sensor5voltage: Option<i16>,
    /// Sensor6 Voltage
    ///
    /// scale of 0-10V
    pub sensor6voltage: Option<i16>,
    /// Sensor7 Voltage
    ///
    /// scale of 0-10V
    pub sensor7voltage: Option<i16>,
    /// Sensor1 Current
    ///
    /// scale of 4-20mA
    pub sensor1current: Option<i16>,
    /// Sensor2 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor2current: Option<i16>,
    /// Sensor3 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor3current: Option<i16>,
    /// Sensor4 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor4current: Option<i16>,
    /// Sensor5 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor5current: Option<i16>,
    /// Sensor6 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor6current: Option<i16>,
    /// Sensor7 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor7current: Option<i16>,
    /// Sensor8 frequency
    ///
    /// frequency in Hz
    pub sensor8: Option<u16>,
    /// Relay 1 state
    pub relay1: Option<u16>,
    /// Relay 2 state
    pub relay2: Option<u16>,
    /// Relay 3 state
    pub relay3: Option<u16>,
    /// Reset the accumulators
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting them
    pub resetaccumulators: Option<u16>,
    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    pub reset: Option<u16>,
}

#[allow(missing_docs)]

impl Model64020 {
    pub const AUX0TMP: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const AUX1TMP: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const AUX2TMP: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const AUX3TMP: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const AUX4TMP: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PROBETMP: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const MAINTMP: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const SENSORV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const SENSORA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const SENSORHZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const SENSOR1VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const SENSOR2VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const SENSOR3VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const SENSOR4VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const SENSOR5VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const SENSOR6VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const SENSOR7VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const SENSOR1CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const SENSOR2CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const SENSOR3CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const SENSOR4CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const SENSOR5CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const SENSOR6CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const SENSOR7CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const SENSOR8: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, false);
    pub const RELAY1: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const RELAY2: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, false);
    pub const RELAY3: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const RESETACCUMULATORS: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, false);
    pub const RESET: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
}

impl crate::Model for Model64020 {
    const ID: u16 = 64020;
    const LENGTH: u16 = 46;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            aux0tmp: Self::AUX0TMP.from_data(data)?,
            aux1tmp: Self::AUX1TMP.from_data(data)?,
            aux2tmp: Self::AUX2TMP.from_data(data)?,
            aux3tmp: Self::AUX3TMP.from_data(data)?,
            aux4tmp: Self::AUX4TMP.from_data(data)?,
            probetmp: Self::PROBETMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            maintmp: Self::MAINTMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensorv_sf: Self::SENSORV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensora_sf: Self::SENSORA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensorhz_sf: Self::SENSORHZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensor1voltage: Self::SENSOR1VOLTAGE.from_data(data)?,
            sensor2voltage: Self::SENSOR2VOLTAGE.from_data(data)?,
            sensor3voltage: Self::SENSOR3VOLTAGE.from_data(data)?,
            sensor4voltage: Self::SENSOR4VOLTAGE.from_data(data)?,
            sensor5voltage: Self::SENSOR5VOLTAGE.from_data(data)?,
            sensor6voltage: Self::SENSOR6VOLTAGE.from_data(data)?,
            sensor7voltage: Self::SENSOR7VOLTAGE.from_data(data)?,
            sensor1current: Self::SENSOR1CURRENT.from_data(data)?,
            sensor2current: Self::SENSOR2CURRENT.from_data(data)?,
            sensor3current: Self::SENSOR3CURRENT.from_data(data)?,
            sensor4current: Self::SENSOR4CURRENT.from_data(data)?,
            sensor5current: Self::SENSOR5CURRENT.from_data(data)?,
            sensor6current: Self::SENSOR6CURRENT.from_data(data)?,
            sensor7current: Self::SENSOR7CURRENT.from_data(data)?,
            sensor8: Self::SENSOR8.from_data(data)?,
            relay1: Self::RELAY1.from_data(data)?,
            relay2: Self::RELAY2.from_data(data)?,
            relay3: Self::RELAY3.from_data(data)?,
            resetaccumulators: Self::RESETACCUMULATORS.from_data(data)?,
            reset: Self::RESET.from_data(data)?,
        })
    }
}

/// Eltek Inverter Extension
#[derive(Debug)]
pub struct Model64101 {
    #[allow(missing_docs)]
    pub eltek_country_code: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_feeding_phase: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_apd_method: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_apd_power_ref: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_method: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_q_ref: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_cosphi_ref: Option<i16>,
}

#[allow(missing_docs)]

impl Model64101 {
    pub const ELTEK_COUNTRY_CODE: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const ELTEK_FEEDING_PHASE: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ELTEK_APD_METHOD: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ELTEK_APD_POWER_REF: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const ELTEK_RPS_METHOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const ELTEK_RPS_Q_REF: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const ELTEK_RPS_COSPHI_REF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model64101 {
    const ID: u16 = 64101;
    const LENGTH: u16 = 7;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            eltek_country_code: Self::ELTEK_COUNTRY_CODE.from_data(data)?,
            eltek_feeding_phase: Self::ELTEK_FEEDING_PHASE.from_data(data)?,
            eltek_apd_method: Self::ELTEK_APD_METHOD.from_data(data)?,
            eltek_apd_power_ref: Self::ELTEK_APD_POWER_REF.from_data(data)?,
            eltek_rps_method: Self::ELTEK_RPS_METHOD.from_data(data)?,
            eltek_rps_q_ref: Self::ELTEK_RPS_Q_REF.from_data(data)?,
            eltek_rps_cosphi_ref: Self::ELTEK_RPS_COSPHI_REF.from_data(data)?,
        })
    }
}

/// OutBack AXS device
#[derive(Debug)]
pub struct Model64110 {
    /// AXS Major Firmware Number
    pub majorfwrev: u16,
    /// AXS Mid Firmware Number
    pub midfwrev: u16,
    /// AXS Minor Firmware Number
    pub minorfwrev: u16,
    /// Encryption Key
    pub encrypkey: u16,
    /// MAC Address
    pub mac_address: String,
    /// Write Password
    pub writepassword: String,
    /// Enable DHCP
    pub enabledhcp: u16,
    /// TCPIP Address
    pub tcpip_address: u32,
    /// TCPIP Gateway
    pub gateway_address: u32,
    /// TCPIP Netmask
    pub tcpip_netmask: u32,
    /// TCPIP DNS1
    pub dns1_address: u32,
    /// TCPIP DNS2
    pub dns2_address: u32,
    /// ModBus Port
    pub modbus_port: u16,
    /// SMTP Server Name
    pub smtp_server_nm: String,
    /// SMTP Account Name
    pub smtp_account_nm: String,
    /// Enable SMTP SSL
    pub smtp_enable_ssl: u16,
    /// SMTP Password
    pub smtp_password: String,
    /// SMTP User Name
    pub smtp_user_nm: String,
    /// Status Email Interval
    pub stat_email_int: u16,
    /// Status Email Start Hour
    pub stat_start_hr: u16,
    /// Status Email Subject
    pub stat_email_sub: String,
    /// Status Email to Address 1
    pub stat_email_addr1: String,
    /// Status Email to Address 2
    pub stat_email_addr2: String,
    /// Enable Alarm Email
    pub alarm_email_en: u16,
    /// Alarm Email Subject
    pub alarm_email_sub: String,
    /// Alarm Email to Address 1
    pub alarm_email_addr1: String,
    /// Alarm Email to Address 2
    pub alarm_email_addr2: String,
    /// FTP Password
    pub ftp_password: String,
    /// Telnet Password
    pub telnet_password: String,
    /// SD-Card Datalog Write Interval
    pub log_write_int: u16,
    /// SD-Card Datalog Retain
    pub log_retain: u16,
    /// SD-Card Datalog Mode
    pub log_mode: u16,
    /// NTP Timer Server Name
    pub ntp_server_nm: String,
    /// Enable Network Time
    pub ntp_enable: u16,
    /// Time Zone
    pub timezone: i16,
    /// Year
    pub date_year: u16,
    /// Month
    pub date_month: u16,
    /// Day
    pub date_day: u16,
    /// Hour
    pub time_hour: u16,
    /// Minute
    pub time_minute: u16,
    /// Second
    pub time_second: u16,
    /// Battery Temperature
    pub battery_temp: i16,
    /// Ambient Temperature
    pub ambient_temp: i16,
    #[allow(missing_docs)]
    pub temp_sf: i16,
    /// AXS Error
    pub axs_error: u16,
    /// AXS Status
    pub axs_status: u16,
    /// Spare
    pub axs_spare: u16,
}

#[allow(missing_docs)]

impl Model64110 {
    pub const MAJORFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const MIDFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const MINORFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ENCRYPKEY: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const MAC_ADDRESS: crate::PointDef<Self, String> = crate::PointDef::new(4, 7, false);
    pub const WRITEPASSWORD: crate::PointDef<Self, String> = crate::PointDef::new(11, 8, false);
    pub const ENABLEDHCP: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const TCPIP_ADDRESS: crate::PointDef<Self, u32> = crate::PointDef::new(20, 2, false);
    pub const GATEWAY_ADDRESS: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const TCPIP_NETMASK: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const DNS1_ADDRESS: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const DNS2_ADDRESS: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const MODBUS_PORT: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, false);
    pub const SMTP_SERVER_NM: crate::PointDef<Self, String> = crate::PointDef::new(31, 20, false);
    pub const SMTP_ACCOUNT_NM: crate::PointDef<Self, String> = crate::PointDef::new(51, 16, false);
    pub const SMTP_ENABLE_SSL: crate::PointDef<Self, u16> = crate::PointDef::new(67, 1, false);
    pub const SMTP_PASSWORD: crate::PointDef<Self, String> = crate::PointDef::new(68, 8, false);
    pub const SMTP_USER_NM: crate::PointDef<Self, String> = crate::PointDef::new(76, 20, false);
    pub const STAT_EMAIL_INT: crate::PointDef<Self, u16> = crate::PointDef::new(96, 1, false);
    pub const STAT_START_HR: crate::PointDef<Self, u16> = crate::PointDef::new(97, 1, false);
    pub const STAT_EMAIL_SUB: crate::PointDef<Self, String> = crate::PointDef::new(98, 25, false);
    pub const STAT_EMAIL_ADDR1: crate::PointDef<Self, String> =
        crate::PointDef::new(123, 20, false);
    pub const STAT_EMAIL_ADDR2: crate::PointDef<Self, String> =
        crate::PointDef::new(143, 20, false);
    pub const ALARM_EMAIL_EN: crate::PointDef<Self, u16> = crate::PointDef::new(163, 1, false);
    pub const ALARM_EMAIL_SUB: crate::PointDef<Self, String> = crate::PointDef::new(164, 25, false);
    pub const ALARM_EMAIL_ADDR1: crate::PointDef<Self, String> =
        crate::PointDef::new(189, 20, false);
    pub const ALARM_EMAIL_ADDR2: crate::PointDef<Self, String> =
        crate::PointDef::new(209, 20, false);
    pub const FTP_PASSWORD: crate::PointDef<Self, String> = crate::PointDef::new(229, 8, false);
    pub const TELNET_PASSWORD: crate::PointDef<Self, String> = crate::PointDef::new(237, 8, false);
    pub const LOG_WRITE_INT: crate::PointDef<Self, u16> = crate::PointDef::new(245, 1, false);
    pub const LOG_RETAIN: crate::PointDef<Self, u16> = crate::PointDef::new(246, 1, false);
    pub const LOG_MODE: crate::PointDef<Self, u16> = crate::PointDef::new(247, 1, false);
    pub const NTP_SERVER_NM: crate::PointDef<Self, String> = crate::PointDef::new(248, 20, false);
    pub const NTP_ENABLE: crate::PointDef<Self, u16> = crate::PointDef::new(268, 1, false);
    pub const TIMEZONE: crate::PointDef<Self, i16> = crate::PointDef::new(269, 1, false);
    pub const DATE_YEAR: crate::PointDef<Self, u16> = crate::PointDef::new(270, 1, false);
    pub const DATE_MONTH: crate::PointDef<Self, u16> = crate::PointDef::new(271, 1, false);
    pub const DATE_DAY: crate::PointDef<Self, u16> = crate::PointDef::new(272, 1, false);
    pub const TIME_HOUR: crate::PointDef<Self, u16> = crate::PointDef::new(273, 1, false);
    pub const TIME_MINUTE: crate::PointDef<Self, u16> = crate::PointDef::new(274, 1, false);
    pub const TIME_SECOND: crate::PointDef<Self, u16> = crate::PointDef::new(275, 1, false);
    pub const BATTERY_TEMP: crate::PointDef<Self, i16> = crate::PointDef::new(276, 1, false);
    pub const AMBIENT_TEMP: crate::PointDef<Self, i16> = crate::PointDef::new(277, 1, false);
    pub const TEMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(278, 1, false);
    pub const AXS_ERROR: crate::PointDef<Self, u16> = crate::PointDef::new(279, 1, false);
    pub const AXS_STATUS: crate::PointDef<Self, u16> = crate::PointDef::new(280, 1, false);
    pub const AXS_SPARE: crate::PointDef<Self, u16> = crate::PointDef::new(281, 1, false);
}

impl crate::Model for Model64110 {
    const ID: u16 = 64110;
    const LENGTH: u16 = 282;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            majorfwrev: Self::MAJORFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            midfwrev: Self::MIDFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            minorfwrev: Self::MINORFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            encrypkey: Self::ENCRYPKEY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            mac_address: Self::MAC_ADDRESS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            writepassword: Self::WRITEPASSWORD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            enabledhcp: Self::ENABLEDHCP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tcpip_address: Self::TCPIP_ADDRESS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            gateway_address: Self::GATEWAY_ADDRESS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tcpip_netmask: Self::TCPIP_NETMASK
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dns1_address: Self::DNS1_ADDRESS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dns2_address: Self::DNS2_ADDRESS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modbus_port: Self::MODBUS_PORT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            smtp_server_nm: Self::SMTP_SERVER_NM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            smtp_account_nm: Self::SMTP_ACCOUNT_NM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            smtp_enable_ssl: Self::SMTP_ENABLE_SSL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            smtp_password: Self::SMTP_PASSWORD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            smtp_user_nm: Self::SMTP_USER_NM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stat_email_int: Self::STAT_EMAIL_INT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stat_start_hr: Self::STAT_START_HR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stat_email_sub: Self::STAT_EMAIL_SUB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stat_email_addr1: Self::STAT_EMAIL_ADDR1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stat_email_addr2: Self::STAT_EMAIL_ADDR2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alarm_email_en: Self::ALARM_EMAIL_EN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alarm_email_sub: Self::ALARM_EMAIL_SUB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alarm_email_addr1: Self::ALARM_EMAIL_ADDR1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alarm_email_addr2: Self::ALARM_EMAIL_ADDR2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ftp_password: Self::FTP_PASSWORD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            telnet_password: Self::TELNET_PASSWORD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            log_write_int: Self::LOG_WRITE_INT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            log_retain: Self::LOG_RETAIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            log_mode: Self::LOG_MODE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ntp_server_nm: Self::NTP_SERVER_NM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ntp_enable: Self::NTP_ENABLE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            timezone: Self::TIMEZONE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            date_year: Self::DATE_YEAR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            date_month: Self::DATE_MONTH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            date_day: Self::DATE_DAY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            time_hour: Self::TIME_HOUR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            time_minute: Self::TIME_MINUTE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            time_second: Self::TIME_SECOND
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            battery_temp: Self::BATTERY_TEMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ambient_temp: Self::AMBIENT_TEMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            temp_sf: Self::TEMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            axs_error: Self::AXS_ERROR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            axs_status: Self::AXS_STATUS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            axs_spare: Self::AXS_SPARE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// Basic Charge Controller
#[derive(Debug)]
pub struct Model64111 {
    /// Port Number
    pub port: u16,
    #[allow(missing_docs)]
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub a_sf: i16,
    #[allow(missing_docs)]
    pub p_sf: i16,
    #[allow(missing_docs)]
    pub ah_sf: i16,
    #[allow(missing_docs)]
    pub kwh_sf: i16,
    /// Battery Voltage
    pub battv: u16,
    /// Array Voltage
    pub arrayv: u16,
    /// Output Current
    pub outputa: u16,
    /// Array Current
    pub inputa: u16,
    /// Operating State
    pub chargerst: u16,
    /// Output Wattage
    pub outputw: u16,
    /// Today's Minimum Battery Voltage
    pub todayminbatv: u16,
    /// Today's Maximum Battery Voltage
    pub todaymaxbatv: u16,
    /// VOC
    pub vocv: u16,
    /// Today's Maximum VOC
    pub todaymaxvoc: u16,
    /// Today's kWh
    pub todaykwhoutput: u16,
    /// Today's AH
    pub todayahoutput: u16,
    /// Lifetime kWh
    pub lifetimekwhout: u16,
    /// Lifetime kAH
    pub lifetimeahout: u16,
    /// Lifetime Maximum Output Wattage
    pub lifetimemaxout: u16,
    /// Lifetime Maximum Battery Voltage
    pub lifetimemaxbatt: u16,
    /// Lifetime Maximum VOC Voltage
    pub lifetimemaxvoc: u16,
}

#[allow(missing_docs)]

impl Model64111 {
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const P_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const AH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const KWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const BATTV: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const ARRAYV: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const OUTPUTA: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const INPUTA: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CHARGERST: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const OUTPUTW: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const TODAYMINBATV: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const TODAYMAXBATV: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const VOCV: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const TODAYMAXVOC: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const TODAYKWHOUTPUT: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const TODAYAHOUTPUT: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const LIFETIMEKWHOUT: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, false);
    pub const LIFETIMEAHOUT: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const LIFETIMEMAXOUT: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const LIFETIMEMAXBATT: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const LIFETIMEMAXVOC: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, false);
}

impl crate::Model for Model64111 {
    const ID: u16 = 64111;
    const LENGTH: u16 = 23;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            port: Self::PORT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            p_sf: Self::P_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ah_sf: Self::AH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            kwh_sf: Self::KWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            battv: Self::BATTV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            arrayv: Self::ARRAYV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            outputa: Self::OUTPUTA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            inputa: Self::INPUTA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            chargerst: Self::CHARGERST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            outputw: Self::OUTPUTW
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            todayminbatv: Self::TODAYMINBATV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            todaymaxbatv: Self::TODAYMAXBATV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vocv: Self::VOCV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            todaymaxvoc: Self::TODAYMAXVOC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            todaykwhoutput: Self::TODAYKWHOUTPUT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            todayahoutput: Self::TODAYAHOUTPUT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            lifetimekwhout: Self::LIFETIMEKWHOUT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            lifetimeahout: Self::LIFETIMEAHOUT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            lifetimemaxout: Self::LIFETIMEMAXOUT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            lifetimemaxbatt: Self::LIFETIMEMAXBATT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            lifetimemaxvoc: Self::LIFETIMEMAXVOC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}

/// OutBack FM Charge Controller
#[derive(Debug)]
pub struct Model64112 {
    /// Port Number
    pub port: u16,
    #[allow(missing_docs)]
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub c_sf: i16,
    #[allow(missing_docs)]
    pub h_sf: i16,
    #[allow(missing_docs)]
    pub p_sf: i16,
    #[allow(missing_docs)]
    pub ah_sf: i16,
    #[allow(missing_docs)]
    pub kwh_sf: i16,
    /// Faults
    pub cc_config_fault: u16,
    /// Absorb
    pub cc_config_absorb_v: u16,
    /// Absorb Time
    pub cc_config_absorb_hr: u16,
    /// Absorb End
    pub cc_config_absorb_end_a: u16,
    /// Rebulk
    pub cc_config_rebulk_v: u16,
    /// Float
    pub cc_config_float_v: u16,
    /// Maximum Charge
    pub cc_config_max_chg_a: u16,
    /// Equalize
    pub cc_config_equalize_v: u16,
    /// Equalize Time
    pub cc_config_equalize_hr: u16,
    /// Auto Equalize Interval
    pub cc_config_auto_equalize: u16,
    /// MPPT mode
    pub cc_config_mppt_mode: u16,
    /// Sweep Width
    pub cc_config_sweep_width: u16,
    /// Sweep Maximum
    pub cc_config_sweep_max: u16,
    /// U-Pick PWM Duty Cycle
    pub cc_config_u_pick_duty_cyc: u16,
    /// Grid Tie Mode
    pub cc_config_grid_tie: u16,
    /// Temp Comp Mode
    pub cc_config_temp_comp: u16,
    /// Temp Comp Lower Limit
    pub cc_config_temp_comp_llimt: u16,
    /// Temp Comp Upper Limit
    pub cc_config_temp_comp_hlimt: u16,
    /// Auto Restart Mode
    pub cc_config_auto_restart: u16,
    /// Wakeup VOC Change
    pub cc_config_wakeup_voc: u16,
    /// Snooze Mode
    pub cc_config_snooze_mode_a: u16,
    /// Wakeup Interval
    pub cc_config_wakeup_interval: u16,
    /// AUX Output Mode
    pub cc_config_aux_mode: u16,
    /// AUX Output Control
    pub cc_config_aux_control: u16,
    /// AUX Output State
    pub cc_config_aux_state: u16,
    /// AUX Output Polarity
    pub cc_config_aux_polarity: u16,
    /// AUX Low Battery Disconnect
    pub cc_config_aux_l_batt_disc: u16,
    /// AUX Low Battery Reconnect
    pub cc_config_aux_l_batt_rcon: u16,
    /// AUX Low Battery Disconnect Delay
    pub cc_config_aux_l_batt_dly: u16,
    /// AUX Vent Fan
    pub cc_config_aux_vent_fan_v: u16,
    /// AUX PV Trigger
    pub cc_config_aux_pv_triggerv: u16,
    /// AUX PV Trigger Hold Time
    pub cc_config_aux_pv_trg_h_tm: u16,
    /// AUX Night Light Threshold
    pub cc_config_aux_nlite_thrsv: u16,
    /// AUX Night Light On Time
    pub cc_config_aux_nlite_on_tm: u16,
    /// AUX Night Light On Hysteresis
    pub cc_config_aux_nlite_on_hist: u16,
    /// AUX Night Light Off Hysteresis
    pub cc_config_aux_nlite_off_hist: u16,
    /// AUX Error Output Low Battery
    pub cc_config_aux_error_batt_v: u16,
    /// AUX Divert Hold Time
    pub cc_config_aux_divert_h_time: u16,
    /// AUX Divert Delay Time
    pub cc_config_aux_divert_dly_time: u16,
    /// AUX Divert Relative
    pub cc_config_aux_divert_rel_v: u16,
    /// AUX Divert Hysteresis
    pub cc_config_aux_divert_hyst_v: u16,
    /// FM CC Major Firmware Number
    pub cc_config_majorfwrev: u16,
    /// FM CC Mid Firmware Number
    pub cc_config_midfwrev: u16,
    /// FM CC Minor Firmware Number
    pub cc_config_minorfwrev: u16,
    /// Set Data Log Day Offset
    pub cc_config_datalog_day_offset: u16,
    /// Current Data Log Day Offset
    pub cc_config_datalog_cur_day_off: u16,
    /// Data Log Daily (Ah)
    pub cc_config_datalog_daily_ah: u16,
    /// Data Log Daily (kWh)
    pub cc_config_datalog_daily_kwh: u16,
    /// Data Log Daily Maximum Output (A)
    pub cc_config_datalog_max_out_a: u16,
    /// Data Log Daily Maximum Output (W)
    pub cc_config_datalog_max_out_w: u16,
    /// Data Log Daily Absorb Time
    pub cc_config_datalog_absorb_t: u16,
    /// Data Log Daily Float Time
    pub cc_config_datalog_float_t: u16,
    /// Data Log Daily Minimum Battery
    pub cc_config_datalog_min_batt_v: u16,
    /// Data Log Daily Maximum Battery
    pub cc_config_datalog_max_batt_v: u16,
    /// Data Log Daily Maximum Input
    pub cc_config_datalog_max_input_v: u16,
    /// Data Log Clear
    pub cc_config_datalog_clear: u16,
    /// Data Log Clear Complement
    pub cc_config_datalog_clr_comp: u16,
}

#[allow(missing_docs)]

impl Model64112 {
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const C_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const H_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const P_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const AH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const KWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const CC_CONFIG_FAULT: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const CC_CONFIG_ABSORB_V: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const CC_CONFIG_ABSORB_HR: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CC_CONFIG_ABSORB_END_A: crate::PointDef<Self, u16> =
        crate::PointDef::new(10, 1, false);
    pub const CC_CONFIG_REBULK_V: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const CC_CONFIG_FLOAT_V: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const CC_CONFIG_MAX_CHG_A: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CC_CONFIG_EQUALIZE_V: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const CC_CONFIG_EQUALIZE_HR: crate::PointDef<Self, u16> =
        crate::PointDef::new(15, 1, false);
    pub const CC_CONFIG_AUTO_EQUALIZE: crate::PointDef<Self, u16> =
        crate::PointDef::new(16, 1, false);
    pub const CC_CONFIG_MPPT_MODE: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const CC_CONFIG_SWEEP_WIDTH: crate::PointDef<Self, u16> =
        crate::PointDef::new(18, 1, false);
    pub const CC_CONFIG_SWEEP_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const CC_CONFIG_U_PICK_DUTY_CYC: crate::PointDef<Self, u16> =
        crate::PointDef::new(20, 1, false);
    pub const CC_CONFIG_GRID_TIE: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const CC_CONFIG_TEMP_COMP: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, false);
    pub const CC_CONFIG_TEMP_COMP_LLIMT: crate::PointDef<Self, u16> =
        crate::PointDef::new(23, 1, false);
    pub const CC_CONFIG_TEMP_COMP_HLIMT: crate::PointDef<Self, u16> =
        crate::PointDef::new(24, 1, false);
    pub const CC_CONFIG_AUTO_RESTART: crate::PointDef<Self, u16> =
        crate::PointDef::new(25, 1, false);
    pub const CC_CONFIG_WAKEUP_VOC: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, false);
    pub const CC_CONFIG_SNOOZE_MODE_A: crate::PointDef<Self, u16> =
        crate::PointDef::new(27, 1, false);
    pub const CC_CONFIG_WAKEUP_INTERVAL: crate::PointDef<Self, u16> =
        crate::PointDef::new(28, 1, false);
    pub const CC_CONFIG_AUX_MODE: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
    pub const CC_CONFIG_AUX_CONTROL: crate::PointDef<Self, u16> =
        crate::PointDef::new(30, 1, false);
    pub const CC_CONFIG_AUX_STATE: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, false);
    pub const CC_CONFIG_AUX_POLARITY: crate::PointDef<Self, u16> =
        crate::PointDef::new(32, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_DISC: crate::PointDef<Self, u16> =
        crate::PointDef::new(33, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_RCON: crate::PointDef<Self, u16> =
        crate::PointDef::new(34, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_DLY: crate::PointDef<Self, u16> =
        crate::PointDef::new(35, 1, false);
    pub const CC_CONFIG_AUX_VENT_FAN_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(36, 1, false);
    pub const CC_CONFIG_AUX_PV_TRIGGERV: crate::PointDef<Self, u16> =
        crate::PointDef::new(37, 1, false);
    pub const CC_CONFIG_AUX_PV_TRG_H_TM: crate::PointDef<Self, u16> =
        crate::PointDef::new(38, 1, false);
    pub const CC_CONFIG_AUX_NLITE_THRSV: crate::PointDef<Self, u16> =
        crate::PointDef::new(39, 1, false);
    pub const CC_CONFIG_AUX_NLITE_ON_TM: crate::PointDef<Self, u16> =
        crate::PointDef::new(40, 1, false);
    pub const CC_CONFIG_AUX_NLITE_ON_HIST: crate::PointDef<Self, u16> =
        crate::PointDef::new(41, 1, false);
    pub const CC_CONFIG_AUX_NLITE_OFF_HIST: crate::PointDef<Self, u16> =
        crate::PointDef::new(42, 1, false);
    pub const CC_CONFIG_AUX_ERROR_BATT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(43, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_H_TIME: crate::PointDef<Self, u16> =
        crate::PointDef::new(44, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_DLY_TIME: crate::PointDef<Self, u16> =
        crate::PointDef::new(45, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_REL_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(46, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_HYST_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(47, 1, false);
    pub const CC_CONFIG_MAJORFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, false);
    pub const CC_CONFIG_MIDFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, false);
    pub const CC_CONFIG_MINORFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, false);
    pub const CC_CONFIG_DATALOG_DAY_OFFSET: crate::PointDef<Self, u16> =
        crate::PointDef::new(51, 1, false);
    pub const CC_CONFIG_DATALOG_CUR_DAY_OFF: crate::PointDef<Self, u16> =
        crate::PointDef::new(52, 1, false);
    pub const CC_CONFIG_DATALOG_DAILY_AH: crate::PointDef<Self, u16> =
        crate::PointDef::new(53, 1, false);
    pub const CC_CONFIG_DATALOG_DAILY_KWH: crate::PointDef<Self, u16> =
        crate::PointDef::new(54, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_OUT_A: crate::PointDef<Self, u16> =
        crate::PointDef::new(55, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_OUT_W: crate::PointDef<Self, u16> =
        crate::PointDef::new(56, 1, false);
    pub const CC_CONFIG_DATALOG_ABSORB_T: crate::PointDef<Self, u16> =
        crate::PointDef::new(57, 1, false);
    pub const CC_CONFIG_DATALOG_FLOAT_T: crate::PointDef<Self, u16> =
        crate::PointDef::new(58, 1, false);
    pub const CC_CONFIG_DATALOG_MIN_BATT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(59, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_BATT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(60, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_INPUT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(61, 1, false);
    pub const CC_CONFIG_DATALOG_CLEAR: crate::PointDef<Self, u16> =
        crate::PointDef::new(62, 1, false);
    pub const CC_CONFIG_DATALOG_CLR_COMP: crate::PointDef<Self, u16> =
        crate::PointDef::new(63, 1, false);
}

impl crate::Model for Model64112 {
    const ID: u16 = 64112;
    const LENGTH: u16 = 64;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            port: Self::PORT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            c_sf: Self::C_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            h_sf: Self::H_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            p_sf: Self::P_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ah_sf: Self::AH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            kwh_sf: Self::KWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_fault: Self::CC_CONFIG_FAULT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_absorb_v: Self::CC_CONFIG_ABSORB_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_absorb_hr: Self::CC_CONFIG_ABSORB_HR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_absorb_end_a: Self::CC_CONFIG_ABSORB_END_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_rebulk_v: Self::CC_CONFIG_REBULK_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_float_v: Self::CC_CONFIG_FLOAT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_max_chg_a: Self::CC_CONFIG_MAX_CHG_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_equalize_v: Self::CC_CONFIG_EQUALIZE_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_equalize_hr: Self::CC_CONFIG_EQUALIZE_HR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_auto_equalize: Self::CC_CONFIG_AUTO_EQUALIZE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_mppt_mode: Self::CC_CONFIG_MPPT_MODE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_sweep_width: Self::CC_CONFIG_SWEEP_WIDTH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_sweep_max: Self::CC_CONFIG_SWEEP_MAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_u_pick_duty_cyc: Self::CC_CONFIG_U_PICK_DUTY_CYC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_grid_tie: Self::CC_CONFIG_GRID_TIE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_temp_comp: Self::CC_CONFIG_TEMP_COMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_temp_comp_llimt: Self::CC_CONFIG_TEMP_COMP_LLIMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_temp_comp_hlimt: Self::CC_CONFIG_TEMP_COMP_HLIMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_auto_restart: Self::CC_CONFIG_AUTO_RESTART
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_wakeup_voc: Self::CC_CONFIG_WAKEUP_VOC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_snooze_mode_a: Self::CC_CONFIG_SNOOZE_MODE_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_wakeup_interval: Self::CC_CONFIG_WAKEUP_INTERVAL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_mode: Self::CC_CONFIG_AUX_MODE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_control: Self::CC_CONFIG_AUX_CONTROL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_state: Self::CC_CONFIG_AUX_STATE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_polarity: Self::CC_CONFIG_AUX_POLARITY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_l_batt_disc: Self::CC_CONFIG_AUX_L_BATT_DISC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_l_batt_rcon: Self::CC_CONFIG_AUX_L_BATT_RCON
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_l_batt_dly: Self::CC_CONFIG_AUX_L_BATT_DLY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_vent_fan_v: Self::CC_CONFIG_AUX_VENT_FAN_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_pv_triggerv: Self::CC_CONFIG_AUX_PV_TRIGGERV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_pv_trg_h_tm: Self::CC_CONFIG_AUX_PV_TRG_H_TM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_thrsv: Self::CC_CONFIG_AUX_NLITE_THRSV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_on_tm: Self::CC_CONFIG_AUX_NLITE_ON_TM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_on_hist: Self::CC_CONFIG_AUX_NLITE_ON_HIST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_off_hist: Self::CC_CONFIG_AUX_NLITE_OFF_HIST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_error_batt_v: Self::CC_CONFIG_AUX_ERROR_BATT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_h_time: Self::CC_CONFIG_AUX_DIVERT_H_TIME
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_dly_time: Self::CC_CONFIG_AUX_DIVERT_DLY_TIME
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_rel_v: Self::CC_CONFIG_AUX_DIVERT_REL_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_hyst_v: Self::CC_CONFIG_AUX_DIVERT_HYST_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_majorfwrev: Self::CC_CONFIG_MAJORFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_midfwrev: Self::CC_CONFIG_MIDFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_minorfwrev: Self::CC_CONFIG_MINORFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_day_offset: Self::CC_CONFIG_DATALOG_DAY_OFFSET
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_cur_day_off: Self::CC_CONFIG_DATALOG_CUR_DAY_OFF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_daily_ah: Self::CC_CONFIG_DATALOG_DAILY_AH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_daily_kwh: Self::CC_CONFIG_DATALOG_DAILY_KWH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_out_a: Self::CC_CONFIG_DATALOG_MAX_OUT_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_out_w: Self::CC_CONFIG_DATALOG_MAX_OUT_W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_absorb_t: Self::CC_CONFIG_DATALOG_ABSORB_T
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_float_t: Self::CC_CONFIG_DATALOG_FLOAT_T
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_min_batt_v: Self::CC_CONFIG_DATALOG_MIN_BATT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_batt_v: Self::CC_CONFIG_DATALOG_MAX_BATT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_input_v: Self::CC_CONFIG_DATALOG_MAX_INPUT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_clear: Self::CC_CONFIG_DATALOG_CLEAR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_clr_comp: Self::CC_CONFIG_DATALOG_CLR_COMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
