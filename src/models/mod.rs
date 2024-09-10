pub mod model1;
pub mod model10;
pub mod model101;
pub mod model102;
pub mod model103;
pub mod model11;
pub mod model111;
pub mod model112;
pub mod model113;
pub mod model12;
pub mod model120;
pub mod model121;
pub mod model122;
pub mod model123;
pub mod model124;
pub mod model125;
pub mod model126;
pub mod model127;
pub mod model128;
pub mod model129;
pub mod model13;
pub mod model130;
pub mod model131;
pub mod model132;
pub mod model133;
pub mod model134;
pub mod model135;
pub mod model136;
pub mod model137;
pub mod model138;
pub mod model139;
pub mod model14;
pub mod model140;
pub mod model141;
pub mod model142;
pub mod model143;
pub mod model144;
pub mod model145;
pub mod model15;
pub mod model16;
pub mod model160;
pub mod model17;
pub mod model18;
pub mod model19;
pub mod model2;
pub mod model201;
pub mod model202;
pub mod model203;
pub mod model204;
pub mod model211;
pub mod model212;
pub mod model213;
pub mod model214;
pub mod model220;
pub mod model3;
pub mod model302;
pub mod model303;
pub mod model304;
pub mod model305;
pub mod model306;
pub mod model307;
pub mod model308;
pub mod model4;
pub mod model401;
pub mod model402;
pub mod model403;
pub mod model404;
pub mod model5;
pub mod model501;
pub mod model502;
pub mod model6;
pub mod model601;
pub mod model63001;
pub mod model63002;
pub mod model64001;
pub mod model64020;
pub mod model64101;
pub mod model64111;
pub mod model64112;
pub mod model7;
pub mod model701;
pub mod model702;
pub mod model703;
pub mod model704;
pub mod model705;
pub mod model706;
pub mod model707;
pub mod model708;
pub mod model709;
pub mod model710;
pub mod model711;
pub mod model712;
pub mod model713;
pub mod model714;
pub mod model715;
pub mod model8;
pub mod model801;
pub mod model802;
pub mod model803;
pub mod model804;
pub mod model805;
pub mod model806;
pub mod model807;
pub mod model808;
pub mod model809;
pub mod model9;
/// This struct contains the addresses of all discovered models.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Models {
    /// Common
    pub m1: crate::ModelAddr<model1::Model1>,
    /// Basic Aggregator
    pub m2: crate::ModelAddr<model2::Model2>,
    /// Secure Dataset Read Request
    pub m3: crate::ModelAddr<model3::Model3>,
    /// Secure Dataset Read Response
    pub m4: crate::ModelAddr<model4::Model4>,
    /// Secure Write Request
    pub m5: crate::ModelAddr<model5::Model5>,
    /// Secure Write Sequential Request
    pub m6: crate::ModelAddr<model6::Model6>,
    /// Secure Write Response Model (DRAFT 1)
    pub m7: crate::ModelAddr<model7::Model7>,
    /// Get Device Security Certificate
    pub m8: crate::ModelAddr<model8::Model8>,
    /// Set Operator Security Certificate
    pub m9: crate::ModelAddr<model9::Model9>,
    /// Communication Interface Header
    pub m10: crate::ModelAddr<model10::Model10>,
    /// Ethernet Link Layer
    pub m11: crate::ModelAddr<model11::Model11>,
    /// IPv4
    pub m12: crate::ModelAddr<model12::Model12>,
    /// IPv6
    pub m13: crate::ModelAddr<model13::Model13>,
    /// Proxy Server
    pub m14: crate::ModelAddr<model14::Model14>,
    /// Interface Counters Model
    pub m15: crate::ModelAddr<model15::Model15>,
    /// Simple IP Network
    pub m16: crate::ModelAddr<model16::Model16>,
    /// Serial Interface
    pub m17: crate::ModelAddr<model17::Model17>,
    /// Cellular Link
    pub m18: crate::ModelAddr<model18::Model18>,
    /// PPP Link
    pub m19: crate::ModelAddr<model19::Model19>,
    /// Inverter (Single Phase)
    pub m101: crate::ModelAddr<model101::Model101>,
    /// Inverter (Split-Phase)
    pub m102: crate::ModelAddr<model102::Model102>,
    /// Inverter (Three Phase)
    pub m103: crate::ModelAddr<model103::Model103>,
    /// Inverter (Single Phase) FLOAT
    pub m111: crate::ModelAddr<model111::Model111>,
    /// Inverter (Split Phase) FLOAT
    pub m112: crate::ModelAddr<model112::Model112>,
    /// Inverter (Three Phase) FLOAT
    pub m113: crate::ModelAddr<model113::Model113>,
    /// Nameplate
    pub m120: crate::ModelAddr<model120::Model120>,
    /// Basic Settings
    pub m121: crate::ModelAddr<model121::Model121>,
    /// Measurements_Status
    pub m122: crate::ModelAddr<model122::Model122>,
    /// Immediate Controls
    pub m123: crate::ModelAddr<model123::Model123>,
    /// Storage
    pub m124: crate::ModelAddr<model124::Model124>,
    /// Pricing
    pub m125: crate::ModelAddr<model125::Model125>,
    /// Static Volt-VAR
    pub m126: crate::ModelAddr<model126::Model126>,
    /// Freq-Watt Param
    pub m127: crate::ModelAddr<model127::Model127>,
    /// Dynamic Reactive Current
    pub m128: crate::ModelAddr<model128::Model128>,
    /// LVRTD
    pub m129: crate::ModelAddr<model129::Model129>,
    /// HVRTD
    pub m130: crate::ModelAddr<model130::Model130>,
    /// Watt-PF
    pub m131: crate::ModelAddr<model131::Model131>,
    /// Volt-Watt
    pub m132: crate::ModelAddr<model132::Model132>,
    /// Basic Scheduling
    pub m133: crate::ModelAddr<model133::Model133>,
    /// Freq-Watt Crv
    pub m134: crate::ModelAddr<model134::Model134>,
    /// LFRT
    pub m135: crate::ModelAddr<model135::Model135>,
    /// HFRT
    pub m136: crate::ModelAddr<model136::Model136>,
    /// LVRTC
    pub m137: crate::ModelAddr<model137::Model137>,
    /// HVRTC
    pub m138: crate::ModelAddr<model138::Model138>,
    /// LVRTX
    pub m139: crate::ModelAddr<model139::Model139>,
    /// HVRTX
    pub m140: crate::ModelAddr<model140::Model140>,
    /// LFRTC
    pub m141: crate::ModelAddr<model141::Model141>,
    /// HFRTC
    pub m142: crate::ModelAddr<model142::Model142>,
    /// LFRTX
    pub m143: crate::ModelAddr<model143::Model143>,
    /// HFRTX
    pub m144: crate::ModelAddr<model144::Model144>,
    /// Extended Settings
    pub m145: crate::ModelAddr<model145::Model145>,
    /// Multiple MPPT Inverter Extension Model
    pub m160: crate::ModelAddr<model160::Model160>,
    /// Meter (Single Phase)single phase (AN or AB) meter
    pub m201: crate::ModelAddr<model201::Model201>,
    /// split single phase (ABN) meter
    pub m202: crate::ModelAddr<model202::Model202>,
    /// wye-connect three phase (abcn) meter
    pub m203: crate::ModelAddr<model203::Model203>,
    /// delta-connect three phase (abc) meter
    pub m204: crate::ModelAddr<model204::Model204>,
    /// single phase (AN or AB) meter
    pub m211: crate::ModelAddr<model211::Model211>,
    /// split single phase (ABN) meter
    pub m212: crate::ModelAddr<model212::Model212>,
    /// wye-connect three phase (abcn) meter
    pub m213: crate::ModelAddr<model213::Model213>,
    /// delta-connect three phase (abc) meter
    pub m214: crate::ModelAddr<model214::Model214>,
    /// Secure AC Meter Selected Readings
    pub m220: crate::ModelAddr<model220::Model220>,
    /// Irradiance Model
    pub m302: crate::ModelAddr<model302::Model302>,
    /// Back of Module Temperature Model
    pub m303: crate::ModelAddr<model303::Model303>,
    /// Inclinometer Model
    pub m304: crate::ModelAddr<model304::Model304>,
    /// GPS
    pub m305: crate::ModelAddr<model305::Model305>,
    /// Reference Point Model
    pub m306: crate::ModelAddr<model306::Model306>,
    /// Base Met
    pub m307: crate::ModelAddr<model307::Model307>,
    /// Mini Met Model
    pub m308: crate::ModelAddr<model308::Model308>,
    /// String Combiner (Current)
    pub m401: crate::ModelAddr<model401::Model401>,
    /// String Combiner (Advanced)
    pub m402: crate::ModelAddr<model402::Model402>,
    /// String Combiner (Current)
    pub m403: crate::ModelAddr<model403::Model403>,
    /// String Combiner (Advanced)
    pub m404: crate::ModelAddr<model404::Model404>,
    /// Solar Module
    pub m501: crate::ModelAddr<model501::Model501>,
    /// Solar Module
    pub m502: crate::ModelAddr<model502::Model502>,
    /// Tracker Controller DRAFT 2
    pub m601: crate::ModelAddr<model601::Model601>,
    /// DER AC Measurement
    pub m701: crate::ModelAddr<model701::Model701>,
    /// DER Capacity
    pub m702: crate::ModelAddr<model702::Model702>,
    /// Enter Service
    pub m703: crate::ModelAddr<model703::Model703>,
    /// DER AC Controls
    pub m704: crate::ModelAddr<model704::Model704>,
    /// DER Volt-Var
    pub m705: crate::ModelAddr<model705::Model705>,
    /// DER Volt-Watt
    pub m706: crate::ModelAddr<model706::Model706>,
    /// DER Trip LV
    pub m707: crate::ModelAddr<model707::Model707>,
    /// DER Trip HV
    pub m708: crate::ModelAddr<model708::Model708>,
    /// DER Trip LF
    pub m709: crate::ModelAddr<model709::Model709>,
    /// DER Trip HF
    pub m710: crate::ModelAddr<model710::Model710>,
    /// DER Frequency Droop
    pub m711: crate::ModelAddr<model711::Model711>,
    /// DER Watt-Var
    pub m712: crate::ModelAddr<model712::Model712>,
    /// DER Storage Capacity
    pub m713: crate::ModelAddr<model713::Model713>,
    /// DER DC Measurement
    pub m714: crate::ModelAddr<model714::Model714>,
    /// DERCtl
    pub m715: crate::ModelAddr<model715::Model715>,
    /// Energy Storage Base Model (DEPRECATED)
    pub m801: crate::ModelAddr<model801::Model801>,
    /// Battery Base Model
    pub m802: crate::ModelAddr<model802::Model802>,
    /// Lithium-Ion Battery Bank Model
    pub m803: crate::ModelAddr<model803::Model803>,
    /// Lithium-Ion String Model
    pub m804: crate::ModelAddr<model804::Model804>,
    /// Lithium-Ion Module Model
    pub m805: crate::ModelAddr<model805::Model805>,
    /// Flow Battery Model
    pub m806: crate::ModelAddr<model806::Model806>,
    /// Flow Battery String Model
    pub m807: crate::ModelAddr<model807::Model807>,
    /// Flow Battery Module Model
    pub m808: crate::ModelAddr<model808::Model808>,
    /// Flow Battery Stack Model
    pub m809: crate::ModelAddr<model809::Model809>,
    /// SunSpec Test Model 1
    pub m63001: crate::ModelAddr<model63001::Model63001>,
    /// SunSpec Test Model 2
    pub m63002: crate::ModelAddr<model63002::Model63002>,
    /// Veris Status and Configuration
    pub m64001: crate::ModelAddr<model64001::Model64001>,
    /// Mersen GreenString
    pub m64020: crate::ModelAddr<model64020::Model64020>,
    /// Eltek Inverter Extension
    pub m64101: crate::ModelAddr<model64101::Model64101>,
    /// Basic Charge Controller
    pub m64111: crate::ModelAddr<model64111::Model64111>,
    /// OutBack FM Charge Controller
    pub m64112: crate::ModelAddr<model64112::Model64112>,
}
impl Models {
    /// Returns a list of all supported model ids
    pub fn supported_model_ids(&self) -> Vec<u16> {
        let mut v = Vec::with_capacity(106usize);
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
        if self.m701.addr != 0 {
            v.push(701);
        }
        if self.m702.addr != 0 {
            v.push(702);
        }
        if self.m703.addr != 0 {
            v.push(703);
        }
        if self.m704.addr != 0 {
            v.push(704);
        }
        if self.m705.addr != 0 {
            v.push(705);
        }
        if self.m706.addr != 0 {
            v.push(706);
        }
        if self.m707.addr != 0 {
            v.push(707);
        }
        if self.m708.addr != 0 {
            v.push(708);
        }
        if self.m709.addr != 0 {
            v.push(709);
        }
        if self.m710.addr != 0 {
            v.push(710);
        }
        if self.m711.addr != 0 {
            v.push(711);
        }
        if self.m712.addr != 0 {
            v.push(712);
        }
        if self.m713.addr != 0 {
            v.push(713);
        }
        if self.m714.addr != 0 {
            v.push(714);
        }
        if self.m715.addr != 0 {
            v.push(715);
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
            701 => self.m701.set_addr(addr, len),
            702 => self.m702.set_addr(addr, len),
            703 => self.m703.set_addr(addr, len),
            704 => self.m704.set_addr(addr, len),
            705 => self.m705.set_addr(addr, len),
            706 => self.m706.set_addr(addr, len),
            707 => self.m707.set_addr(addr, len),
            708 => self.m708.set_addr(addr, len),
            709 => self.m709.set_addr(addr, len),
            710 => self.m710.set_addr(addr, len),
            711 => self.m711.set_addr(addr, len),
            712 => self.m712.set_addr(addr, len),
            713 => self.m713.set_addr(addr, len),
            714 => self.m714.set_addr(addr, len),
            715 => self.m715.set_addr(addr, len),
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
            64111 => self.m64111.set_addr(addr, len),
            64112 => self.m64112.set_addr(addr, len),
            _ => {
                return false;
            }
        }
        true
    }
}
