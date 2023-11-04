pub use model1::Model1;
pub use model10::Model10;
pub use model101::Model101;
pub use model102::Model102;
pub use model103::Model103;
pub use model11::Model11;
pub use model111::Model111;
pub use model112::Model112;
pub use model113::Model113;
pub use model12::Model12;
pub use model120::Model120;
pub use model121::Model121;
pub use model122::Model122;
pub use model123::Model123;
pub use model124::Model124;
pub use model125::Model125;
pub use model126::Model126;
pub use model127::Model127;
pub use model128::Model128;
pub use model129::Model129;
pub use model13::Model13;
pub use model130::Model130;
pub use model131::Model131;
pub use model132::Model132;
pub use model133::Model133;
pub use model134::Model134;
pub use model135::Model135;
pub use model136::Model136;
pub use model137::Model137;
pub use model138::Model138;
pub use model139::Model139;
pub use model14::Model14;
pub use model140::Model140;
pub use model141::Model141;
pub use model142::Model142;
pub use model143::Model143;
pub use model144::Model144;
pub use model145::Model145;
pub use model15::Model15;
pub use model16::Model16;
pub use model160::Model160;
pub use model17::Model17;
pub use model18::Model18;
pub use model19::Model19;
pub use model2::Model2;
pub use model201::Model201;
pub use model202::Model202;
pub use model203::Model203;
pub use model204::Model204;
pub use model211::Model211;
pub use model212::Model212;
pub use model213::Model213;
pub use model214::Model214;
pub use model220::Model220;
pub use model3::Model3;
pub use model302::Model302;
pub use model303::Model303;
pub use model304::Model304;
pub use model305::Model305;
pub use model306::Model306;
pub use model307::Model307;
pub use model308::Model308;
pub use model4::Model4;
pub use model401::Model401;
pub use model402::Model402;
pub use model403::Model403;
pub use model404::Model404;
pub use model5::Model5;
pub use model501::Model501;
pub use model502::Model502;
pub use model6::Model6;
pub use model601::Model601;
pub use model63001::Model63001;
pub use model63002::Model63002;
pub use model64001::Model64001;
pub use model64020::Model64020;
pub use model64101::Model64101;
pub use model64110::Model64110;
pub use model64111::Model64111;
pub use model64112::Model64112;
pub use model7::Model7;
pub use model8::Model8;
pub use model801::Model801;
pub use model802::Model802;
pub use model803::Model803;
pub use model804::Model804;
pub use model805::Model805;
pub use model806::Model806;
pub use model807::Model807;
pub use model808::Model808;
pub use model809::Model809;
pub use model9::Model9;

mod model1;

mod model2;

mod model3;

mod model4;

mod model5;

mod model6;

mod model7;

mod model8;

mod model9;

mod model10;

mod model11;

mod model12;

mod model13;

mod model14;

mod model15;

mod model16;

mod model17;

mod model18;

mod model19;

mod model101;

mod model102;

mod model103;

mod model111;

mod model112;

mod model113;

mod model120;

mod model121;

mod model122;

mod model123;

mod model124;

mod model125;

mod model126;

mod model127;

mod model128;

mod model129;

mod model130;

mod model131;

mod model132;

mod model133;

mod model134;

mod model135;

mod model136;

mod model137;

mod model138;

mod model139;

mod model140;

mod model141;

mod model142;

mod model143;

mod model144;

mod model145;

mod model160;

mod model201;

mod model202;

mod model203;

mod model204;

mod model211;

mod model212;

mod model213;

mod model214;

mod model220;

mod model302;

mod model303;

mod model304;

mod model305;

mod model306;

mod model307;

mod model308;

mod model401;

mod model402;

mod model403;

mod model404;

mod model501;

mod model502;

mod model601;

mod model801;

mod model802;

mod model803;

mod model804;

mod model805;

mod model806;

mod model807;

mod model808;

mod model809;

mod model63001;

mod model63002;

mod model64001;

mod model64020;

mod model64101;

mod model64110;

mod model64111;

mod model64112;

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
