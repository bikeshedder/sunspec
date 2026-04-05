#[cfg(feature = "model1")]
pub mod model1;
#[cfg(feature = "model10")]
pub mod model10;
#[cfg(feature = "model101")]
pub mod model101;
#[cfg(feature = "model102")]
pub mod model102;
#[cfg(feature = "model103")]
pub mod model103;
#[cfg(feature = "model11")]
pub mod model11;
#[cfg(feature = "model111")]
pub mod model111;
#[cfg(feature = "model112")]
pub mod model112;
#[cfg(feature = "model113")]
pub mod model113;
#[cfg(feature = "model12")]
pub mod model12;
#[cfg(feature = "model120")]
pub mod model120;
#[cfg(feature = "model121")]
pub mod model121;
#[cfg(feature = "model122")]
pub mod model122;
#[cfg(feature = "model123")]
pub mod model123;
#[cfg(feature = "model124")]
pub mod model124;
#[cfg(feature = "model125")]
pub mod model125;
#[cfg(feature = "model126")]
pub mod model126;
#[cfg(feature = "model127")]
pub mod model127;
#[cfg(feature = "model128")]
pub mod model128;
#[cfg(feature = "model129")]
pub mod model129;
#[cfg(feature = "model13")]
pub mod model13;
#[cfg(feature = "model130")]
pub mod model130;
#[cfg(feature = "model131")]
pub mod model131;
#[cfg(feature = "model132")]
pub mod model132;
#[cfg(feature = "model133")]
pub mod model133;
#[cfg(feature = "model134")]
pub mod model134;
#[cfg(feature = "model135")]
pub mod model135;
#[cfg(feature = "model136")]
pub mod model136;
#[cfg(feature = "model137")]
pub mod model137;
#[cfg(feature = "model138")]
pub mod model138;
#[cfg(feature = "model139")]
pub mod model139;
#[cfg(feature = "model14")]
pub mod model14;
#[cfg(feature = "model140")]
pub mod model140;
#[cfg(feature = "model141")]
pub mod model141;
#[cfg(feature = "model142")]
pub mod model142;
#[cfg(feature = "model143")]
pub mod model143;
#[cfg(feature = "model144")]
pub mod model144;
#[cfg(feature = "model145")]
pub mod model145;
#[cfg(feature = "model15")]
pub mod model15;
#[cfg(feature = "model16")]
pub mod model16;
#[cfg(feature = "model160")]
pub mod model160;
#[cfg(feature = "model17")]
pub mod model17;
#[cfg(feature = "model18")]
pub mod model18;
#[cfg(feature = "model19")]
pub mod model19;
#[cfg(feature = "model2")]
pub mod model2;
#[cfg(feature = "model201")]
pub mod model201;
#[cfg(feature = "model202")]
pub mod model202;
#[cfg(feature = "model203")]
pub mod model203;
#[cfg(feature = "model204")]
pub mod model204;
#[cfg(feature = "model211")]
pub mod model211;
#[cfg(feature = "model212")]
pub mod model212;
#[cfg(feature = "model213")]
pub mod model213;
#[cfg(feature = "model214")]
pub mod model214;
#[cfg(feature = "model220")]
pub mod model220;
#[cfg(feature = "model3")]
pub mod model3;
#[cfg(feature = "model302")]
pub mod model302;
#[cfg(feature = "model303")]
pub mod model303;
#[cfg(feature = "model304")]
pub mod model304;
#[cfg(feature = "model305")]
pub mod model305;
#[cfg(feature = "model306")]
pub mod model306;
#[cfg(feature = "model307")]
pub mod model307;
#[cfg(feature = "model308")]
pub mod model308;
#[cfg(feature = "model4")]
pub mod model4;
#[cfg(feature = "model401")]
pub mod model401;
#[cfg(feature = "model402")]
pub mod model402;
#[cfg(feature = "model403")]
pub mod model403;
#[cfg(feature = "model404")]
pub mod model404;
#[cfg(feature = "model5")]
pub mod model5;
#[cfg(feature = "model501")]
pub mod model501;
#[cfg(feature = "model502")]
pub mod model502;
#[cfg(feature = "model6")]
pub mod model6;
#[cfg(feature = "model601")]
pub mod model601;
#[cfg(feature = "model63001")]
pub mod model63001;
#[cfg(feature = "model63002")]
pub mod model63002;
#[cfg(feature = "model64001")]
pub mod model64001;
#[cfg(feature = "model64020")]
pub mod model64020;
#[cfg(feature = "model64101")]
pub mod model64101;
#[cfg(feature = "model64111")]
pub mod model64111;
#[cfg(feature = "model64112")]
pub mod model64112;
#[cfg(feature = "model64410")]
pub mod model64410;
#[cfg(feature = "model64411")]
pub mod model64411;
#[cfg(feature = "model64412")]
pub mod model64412;
#[cfg(feature = "model64413")]
pub mod model64413;
#[cfg(feature = "model64414")]
pub mod model64414;
#[cfg(feature = "model64415")]
pub mod model64415;
#[cfg(feature = "model7")]
pub mod model7;
#[cfg(feature = "model701")]
pub mod model701;
#[cfg(feature = "model702")]
pub mod model702;
#[cfg(feature = "model703")]
pub mod model703;
#[cfg(feature = "model704")]
pub mod model704;
#[cfg(feature = "model705")]
pub mod model705;
#[cfg(feature = "model706")]
pub mod model706;
#[cfg(feature = "model707")]
pub mod model707;
#[cfg(feature = "model708")]
pub mod model708;
#[cfg(feature = "model709")]
pub mod model709;
#[cfg(feature = "model710")]
pub mod model710;
#[cfg(feature = "model711")]
pub mod model711;
#[cfg(feature = "model712")]
pub mod model712;
#[cfg(feature = "model713")]
pub mod model713;
#[cfg(feature = "model714")]
pub mod model714;
#[cfg(feature = "model715")]
pub mod model715;
#[cfg(feature = "model8")]
pub mod model8;
#[cfg(feature = "model801")]
pub mod model801;
#[cfg(feature = "model802")]
pub mod model802;
#[cfg(feature = "model803")]
pub mod model803;
#[cfg(feature = "model804")]
pub mod model804;
#[cfg(feature = "model805")]
pub mod model805;
#[cfg(feature = "model806")]
pub mod model806;
#[cfg(feature = "model807")]
pub mod model807;
#[cfg(feature = "model808")]
pub mod model808;
#[cfg(feature = "model809")]
pub mod model809;
#[cfg(feature = "model9")]
pub mod model9;
/// This struct contains the addresses of all discovered models enabled via Cargo features.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Models {
    #[cfg(feature = "model1")]
    /// Common
    pub m1: crate::ModelAddr<model1::Model1>,
    #[cfg(feature = "model2")]
    /// Basic Aggregator
    pub m2: crate::ModelAddr<model2::Model2>,
    #[cfg(feature = "model3")]
    /// Secure Dataset Read Request
    pub m3: crate::ModelAddr<model3::Model3>,
    #[cfg(feature = "model4")]
    /// Secure Dataset Read Response
    pub m4: crate::ModelAddr<model4::Model4>,
    #[cfg(feature = "model5")]
    /// Secure Write Request
    pub m5: crate::ModelAddr<model5::Model5>,
    #[cfg(feature = "model6")]
    /// Secure Write Sequential Request
    pub m6: crate::ModelAddr<model6::Model6>,
    #[cfg(feature = "model7")]
    /// Secure Write Response Model (DRAFT 1)
    pub m7: crate::ModelAddr<model7::Model7>,
    #[cfg(feature = "model8")]
    /// Get Device Security Certificate
    pub m8: crate::ModelAddr<model8::Model8>,
    #[cfg(feature = "model9")]
    /// Set Operator Security Certificate
    pub m9: crate::ModelAddr<model9::Model9>,
    #[cfg(feature = "model10")]
    /// Communication Interface Header
    pub m10: crate::ModelAddr<model10::Model10>,
    #[cfg(feature = "model11")]
    /// Ethernet Link Layer
    pub m11: crate::ModelAddr<model11::Model11>,
    #[cfg(feature = "model12")]
    /// IPv4
    pub m12: crate::ModelAddr<model12::Model12>,
    #[cfg(feature = "model13")]
    /// IPv6
    pub m13: crate::ModelAddr<model13::Model13>,
    #[cfg(feature = "model14")]
    /// Proxy Server
    pub m14: crate::ModelAddr<model14::Model14>,
    #[cfg(feature = "model15")]
    /// Interface Counters Model
    pub m15: crate::ModelAddr<model15::Model15>,
    #[cfg(feature = "model16")]
    /// Simple IP Network
    pub m16: crate::ModelAddr<model16::Model16>,
    #[cfg(feature = "model17")]
    /// Serial Interface
    pub m17: crate::ModelAddr<model17::Model17>,
    #[cfg(feature = "model18")]
    /// Cellular Link
    pub m18: crate::ModelAddr<model18::Model18>,
    #[cfg(feature = "model19")]
    /// PPP Link
    pub m19: crate::ModelAddr<model19::Model19>,
    #[cfg(feature = "model101")]
    /// Inverter (Single Phase)
    pub m101: crate::ModelAddr<model101::Model101>,
    #[cfg(feature = "model102")]
    /// Inverter (Split-Phase)
    pub m102: crate::ModelAddr<model102::Model102>,
    #[cfg(feature = "model103")]
    /// Inverter (Three Phase)
    pub m103: crate::ModelAddr<model103::Model103>,
    #[cfg(feature = "model111")]
    /// Inverter (Single Phase) FLOAT
    pub m111: crate::ModelAddr<model111::Model111>,
    #[cfg(feature = "model112")]
    /// Inverter (Split Phase) FLOAT
    pub m112: crate::ModelAddr<model112::Model112>,
    #[cfg(feature = "model113")]
    /// Inverter (Three Phase) FLOAT
    pub m113: crate::ModelAddr<model113::Model113>,
    #[cfg(feature = "model120")]
    /// Nameplate
    pub m120: crate::ModelAddr<model120::Model120>,
    #[cfg(feature = "model121")]
    /// Basic Settings
    pub m121: crate::ModelAddr<model121::Model121>,
    #[cfg(feature = "model122")]
    /// Measurements_Status
    pub m122: crate::ModelAddr<model122::Model122>,
    #[cfg(feature = "model123")]
    /// Immediate Controls
    pub m123: crate::ModelAddr<model123::Model123>,
    #[cfg(feature = "model124")]
    /// Storage
    pub m124: crate::ModelAddr<model124::Model124>,
    #[cfg(feature = "model125")]
    /// Pricing
    pub m125: crate::ModelAddr<model125::Model125>,
    #[cfg(feature = "model126")]
    /// Static Volt-VAR
    pub m126: crate::ModelAddr<model126::Model126>,
    #[cfg(feature = "model127")]
    /// Freq-Watt Param
    pub m127: crate::ModelAddr<model127::Model127>,
    #[cfg(feature = "model128")]
    /// Dynamic Reactive Current
    pub m128: crate::ModelAddr<model128::Model128>,
    #[cfg(feature = "model129")]
    /// LVRTD
    pub m129: crate::ModelAddr<model129::Model129>,
    #[cfg(feature = "model130")]
    /// HVRTD
    pub m130: crate::ModelAddr<model130::Model130>,
    #[cfg(feature = "model131")]
    /// Watt-PF
    pub m131: crate::ModelAddr<model131::Model131>,
    #[cfg(feature = "model132")]
    /// Volt-Watt
    pub m132: crate::ModelAddr<model132::Model132>,
    #[cfg(feature = "model133")]
    /// Basic Scheduling
    pub m133: crate::ModelAddr<model133::Model133>,
    #[cfg(feature = "model134")]
    /// Freq-Watt Crv
    pub m134: crate::ModelAddr<model134::Model134>,
    #[cfg(feature = "model135")]
    /// LFRT
    pub m135: crate::ModelAddr<model135::Model135>,
    #[cfg(feature = "model136")]
    /// HFRT
    pub m136: crate::ModelAddr<model136::Model136>,
    #[cfg(feature = "model137")]
    /// LVRTC
    pub m137: crate::ModelAddr<model137::Model137>,
    #[cfg(feature = "model138")]
    /// HVRTC
    pub m138: crate::ModelAddr<model138::Model138>,
    #[cfg(feature = "model139")]
    /// LVRTX
    pub m139: crate::ModelAddr<model139::Model139>,
    #[cfg(feature = "model140")]
    /// HVRTX
    pub m140: crate::ModelAddr<model140::Model140>,
    #[cfg(feature = "model141")]
    /// LFRTC
    pub m141: crate::ModelAddr<model141::Model141>,
    #[cfg(feature = "model142")]
    /// HFRTC
    pub m142: crate::ModelAddr<model142::Model142>,
    #[cfg(feature = "model143")]
    /// LFRTX
    pub m143: crate::ModelAddr<model143::Model143>,
    #[cfg(feature = "model144")]
    /// HFRTX
    pub m144: crate::ModelAddr<model144::Model144>,
    #[cfg(feature = "model145")]
    /// Extended Settings
    pub m145: crate::ModelAddr<model145::Model145>,
    #[cfg(feature = "model160")]
    /// Multiple MPPT Inverter Extension Model
    pub m160: crate::ModelAddr<model160::Model160>,
    #[cfg(feature = "model201")]
    /// Meter (Single Phase) single phase (AN or AB) meter
    pub m201: crate::ModelAddr<model201::Model201>,
    #[cfg(feature = "model202")]
    /// split single phase (ABN) meter
    pub m202: crate::ModelAddr<model202::Model202>,
    #[cfg(feature = "model203")]
    /// wye-connect three phase (abcn) meter
    pub m203: crate::ModelAddr<model203::Model203>,
    #[cfg(feature = "model204")]
    /// delta-connect three phase (abc) meter
    pub m204: crate::ModelAddr<model204::Model204>,
    #[cfg(feature = "model211")]
    /// single phase (AN or AB) meter
    pub m211: crate::ModelAddr<model211::Model211>,
    #[cfg(feature = "model212")]
    /// split single phase (ABN) meter
    pub m212: crate::ModelAddr<model212::Model212>,
    #[cfg(feature = "model213")]
    /// wye-connect three phase (abcn) meter
    pub m213: crate::ModelAddr<model213::Model213>,
    #[cfg(feature = "model214")]
    /// delta-connect three phase (abc) meter
    pub m214: crate::ModelAddr<model214::Model214>,
    #[cfg(feature = "model220")]
    /// Secure AC Meter Selected Readings
    pub m220: crate::ModelAddr<model220::Model220>,
    #[cfg(feature = "model302")]
    /// Irradiance Model
    pub m302: crate::ModelAddr<model302::Model302>,
    #[cfg(feature = "model303")]
    /// Back of Module Temperature Model
    pub m303: crate::ModelAddr<model303::Model303>,
    #[cfg(feature = "model304")]
    /// Inclinometer Model
    pub m304: crate::ModelAddr<model304::Model304>,
    #[cfg(feature = "model305")]
    /// GPS
    pub m305: crate::ModelAddr<model305::Model305>,
    #[cfg(feature = "model306")]
    /// Reference Point Model
    pub m306: crate::ModelAddr<model306::Model306>,
    #[cfg(feature = "model307")]
    /// Base Met
    pub m307: crate::ModelAddr<model307::Model307>,
    #[cfg(feature = "model308")]
    /// Mini Met Model
    pub m308: crate::ModelAddr<model308::Model308>,
    #[cfg(feature = "model401")]
    /// String Combiner (Current)
    pub m401: crate::ModelAddr<model401::Model401>,
    #[cfg(feature = "model402")]
    /// String Combiner (Advanced)
    pub m402: crate::ModelAddr<model402::Model402>,
    #[cfg(feature = "model403")]
    /// String Combiner (Current)
    pub m403: crate::ModelAddr<model403::Model403>,
    #[cfg(feature = "model404")]
    /// String Combiner (Advanced)
    pub m404: crate::ModelAddr<model404::Model404>,
    #[cfg(feature = "model501")]
    /// Solar Module
    pub m501: crate::ModelAddr<model501::Model501>,
    #[cfg(feature = "model502")]
    /// Solar Module
    pub m502: crate::ModelAddr<model502::Model502>,
    #[cfg(feature = "model601")]
    /// Tracker Controller DRAFT 2
    pub m601: crate::ModelAddr<model601::Model601>,
    #[cfg(feature = "model701")]
    /// DER AC Measurement
    pub m701: crate::ModelAddr<model701::Model701>,
    #[cfg(feature = "model702")]
    /// DER Capacity
    pub m702: crate::ModelAddr<model702::Model702>,
    #[cfg(feature = "model703")]
    /// Enter Service
    pub m703: crate::ModelAddr<model703::Model703>,
    #[cfg(feature = "model704")]
    /// DER AC Controls
    pub m704: crate::ModelAddr<model704::Model704>,
    #[cfg(feature = "model705")]
    /// DER Volt-Var
    pub m705: crate::ModelAddr<model705::Model705>,
    #[cfg(feature = "model706")]
    /// DER Volt-Watt
    pub m706: crate::ModelAddr<model706::Model706>,
    #[cfg(feature = "model707")]
    /// DER Trip LV
    pub m707: crate::ModelAddr<model707::Model707>,
    #[cfg(feature = "model708")]
    /// DER Trip HV
    pub m708: crate::ModelAddr<model708::Model708>,
    #[cfg(feature = "model709")]
    /// DER Trip LF
    pub m709: crate::ModelAddr<model709::Model709>,
    #[cfg(feature = "model710")]
    /// DER Trip HF
    pub m710: crate::ModelAddr<model710::Model710>,
    #[cfg(feature = "model711")]
    /// DER Frequency Droop
    pub m711: crate::ModelAddr<model711::Model711>,
    #[cfg(feature = "model712")]
    /// DER Watt-Var
    pub m712: crate::ModelAddr<model712::Model712>,
    #[cfg(feature = "model713")]
    /// DER Storage Capacity
    pub m713: crate::ModelAddr<model713::Model713>,
    #[cfg(feature = "model714")]
    /// DER DC Measurement
    pub m714: crate::ModelAddr<model714::Model714>,
    #[cfg(feature = "model715")]
    /// DERCtl
    pub m715: crate::ModelAddr<model715::Model715>,
    #[cfg(feature = "model801")]
    /// Energy Storage Base Model (DEPRECATED)
    pub m801: crate::ModelAddr<model801::Model801>,
    #[cfg(feature = "model802")]
    /// Battery Base Model
    pub m802: crate::ModelAddr<model802::Model802>,
    #[cfg(feature = "model803")]
    /// Lithium-Ion Battery Bank Model
    pub m803: crate::ModelAddr<model803::Model803>,
    #[cfg(feature = "model804")]
    /// Lithium-Ion String Model
    pub m804: crate::ModelAddr<model804::Model804>,
    #[cfg(feature = "model805")]
    /// Lithium-Ion Module Model
    pub m805: crate::ModelAddr<model805::Model805>,
    #[cfg(feature = "model806")]
    /// Flow Battery Model
    pub m806: crate::ModelAddr<model806::Model806>,
    #[cfg(feature = "model807")]
    /// Flow Battery String Model
    pub m807: crate::ModelAddr<model807::Model807>,
    #[cfg(feature = "model808")]
    /// Flow Battery Module Model
    pub m808: crate::ModelAddr<model808::Model808>,
    #[cfg(feature = "model809")]
    /// Flow Battery Stack Model
    pub m809: crate::ModelAddr<model809::Model809>,
    #[cfg(feature = "model63001")]
    /// SunSpec Test Model 1
    pub m63001: crate::ModelAddr<model63001::Model63001>,
    #[cfg(feature = "model63002")]
    /// SunSpec Test Model 2
    pub m63002: crate::ModelAddr<model63002::Model63002>,
    #[cfg(feature = "model64001")]
    /// Veris Status and Configuration
    pub m64001: crate::ModelAddr<model64001::Model64001>,
    #[cfg(feature = "model64020")]
    /// Mersen GreenString
    pub m64020: crate::ModelAddr<model64020::Model64020>,
    #[cfg(feature = "model64101")]
    /// Eltek Inverter Extension
    pub m64101: crate::ModelAddr<model64101::Model64101>,
    #[cfg(feature = "model64111")]
    /// Basic Charge Controller
    pub m64111: crate::ModelAddr<model64111::Model64111>,
    #[cfg(feature = "model64112")]
    /// OutBack FM Charge Controller
    pub m64112: crate::ModelAddr<model64112::Model64112>,
    #[cfg(feature = "model64410")]
    /// DC Simulator Control Interface
    pub m64410: crate::ModelAddr<model64410::Model64410>,
    #[cfg(feature = "model64411")]
    /// AC Simulator Control Interface
    pub m64411: crate::ModelAddr<model64411::Model64411>,
    #[cfg(feature = "model64412")]
    /// DER Cyber Exploitation
    pub m64412: crate::ModelAddr<model64412::Model64412>,
    #[cfg(feature = "model64413")]
    /// PV Curves
    pub m64413: crate::ModelAddr<model64413::Model64413>,
    #[cfg(feature = "model64414")]
    /// DER Simulation Controls
    pub m64414: crate::ModelAddr<model64414::Model64414>,
    #[cfg(feature = "model64415")]
    /// CSIP Client Control
    pub m64415: crate::ModelAddr<model64415::Model64415>,
}
impl Models {
    /// Returns a list of all supported model ids
    pub fn supported_model_ids(&self) -> Vec<u16> {
        #[allow(unused_mut)]
        let mut v = Vec::new();
        #[cfg(feature = "model1")]
        {
            if self.m1.addr != 0 {
                v.push(1);
            }
        }
        #[cfg(feature = "model2")]
        {
            if self.m2.addr != 0 {
                v.push(2);
            }
        }
        #[cfg(feature = "model3")]
        {
            if self.m3.addr != 0 {
                v.push(3);
            }
        }
        #[cfg(feature = "model4")]
        {
            if self.m4.addr != 0 {
                v.push(4);
            }
        }
        #[cfg(feature = "model5")]
        {
            if self.m5.addr != 0 {
                v.push(5);
            }
        }
        #[cfg(feature = "model6")]
        {
            if self.m6.addr != 0 {
                v.push(6);
            }
        }
        #[cfg(feature = "model7")]
        {
            if self.m7.addr != 0 {
                v.push(7);
            }
        }
        #[cfg(feature = "model8")]
        {
            if self.m8.addr != 0 {
                v.push(8);
            }
        }
        #[cfg(feature = "model9")]
        {
            if self.m9.addr != 0 {
                v.push(9);
            }
        }
        #[cfg(feature = "model10")]
        {
            if self.m10.addr != 0 {
                v.push(10);
            }
        }
        #[cfg(feature = "model11")]
        {
            if self.m11.addr != 0 {
                v.push(11);
            }
        }
        #[cfg(feature = "model12")]
        {
            if self.m12.addr != 0 {
                v.push(12);
            }
        }
        #[cfg(feature = "model13")]
        {
            if self.m13.addr != 0 {
                v.push(13);
            }
        }
        #[cfg(feature = "model14")]
        {
            if self.m14.addr != 0 {
                v.push(14);
            }
        }
        #[cfg(feature = "model15")]
        {
            if self.m15.addr != 0 {
                v.push(15);
            }
        }
        #[cfg(feature = "model16")]
        {
            if self.m16.addr != 0 {
                v.push(16);
            }
        }
        #[cfg(feature = "model17")]
        {
            if self.m17.addr != 0 {
                v.push(17);
            }
        }
        #[cfg(feature = "model18")]
        {
            if self.m18.addr != 0 {
                v.push(18);
            }
        }
        #[cfg(feature = "model19")]
        {
            if self.m19.addr != 0 {
                v.push(19);
            }
        }
        #[cfg(feature = "model101")]
        {
            if self.m101.addr != 0 {
                v.push(101);
            }
        }
        #[cfg(feature = "model102")]
        {
            if self.m102.addr != 0 {
                v.push(102);
            }
        }
        #[cfg(feature = "model103")]
        {
            if self.m103.addr != 0 {
                v.push(103);
            }
        }
        #[cfg(feature = "model111")]
        {
            if self.m111.addr != 0 {
                v.push(111);
            }
        }
        #[cfg(feature = "model112")]
        {
            if self.m112.addr != 0 {
                v.push(112);
            }
        }
        #[cfg(feature = "model113")]
        {
            if self.m113.addr != 0 {
                v.push(113);
            }
        }
        #[cfg(feature = "model120")]
        {
            if self.m120.addr != 0 {
                v.push(120);
            }
        }
        #[cfg(feature = "model121")]
        {
            if self.m121.addr != 0 {
                v.push(121);
            }
        }
        #[cfg(feature = "model122")]
        {
            if self.m122.addr != 0 {
                v.push(122);
            }
        }
        #[cfg(feature = "model123")]
        {
            if self.m123.addr != 0 {
                v.push(123);
            }
        }
        #[cfg(feature = "model124")]
        {
            if self.m124.addr != 0 {
                v.push(124);
            }
        }
        #[cfg(feature = "model125")]
        {
            if self.m125.addr != 0 {
                v.push(125);
            }
        }
        #[cfg(feature = "model126")]
        {
            if self.m126.addr != 0 {
                v.push(126);
            }
        }
        #[cfg(feature = "model127")]
        {
            if self.m127.addr != 0 {
                v.push(127);
            }
        }
        #[cfg(feature = "model128")]
        {
            if self.m128.addr != 0 {
                v.push(128);
            }
        }
        #[cfg(feature = "model129")]
        {
            if self.m129.addr != 0 {
                v.push(129);
            }
        }
        #[cfg(feature = "model130")]
        {
            if self.m130.addr != 0 {
                v.push(130);
            }
        }
        #[cfg(feature = "model131")]
        {
            if self.m131.addr != 0 {
                v.push(131);
            }
        }
        #[cfg(feature = "model132")]
        {
            if self.m132.addr != 0 {
                v.push(132);
            }
        }
        #[cfg(feature = "model133")]
        {
            if self.m133.addr != 0 {
                v.push(133);
            }
        }
        #[cfg(feature = "model134")]
        {
            if self.m134.addr != 0 {
                v.push(134);
            }
        }
        #[cfg(feature = "model135")]
        {
            if self.m135.addr != 0 {
                v.push(135);
            }
        }
        #[cfg(feature = "model136")]
        {
            if self.m136.addr != 0 {
                v.push(136);
            }
        }
        #[cfg(feature = "model137")]
        {
            if self.m137.addr != 0 {
                v.push(137);
            }
        }
        #[cfg(feature = "model138")]
        {
            if self.m138.addr != 0 {
                v.push(138);
            }
        }
        #[cfg(feature = "model139")]
        {
            if self.m139.addr != 0 {
                v.push(139);
            }
        }
        #[cfg(feature = "model140")]
        {
            if self.m140.addr != 0 {
                v.push(140);
            }
        }
        #[cfg(feature = "model141")]
        {
            if self.m141.addr != 0 {
                v.push(141);
            }
        }
        #[cfg(feature = "model142")]
        {
            if self.m142.addr != 0 {
                v.push(142);
            }
        }
        #[cfg(feature = "model143")]
        {
            if self.m143.addr != 0 {
                v.push(143);
            }
        }
        #[cfg(feature = "model144")]
        {
            if self.m144.addr != 0 {
                v.push(144);
            }
        }
        #[cfg(feature = "model145")]
        {
            if self.m145.addr != 0 {
                v.push(145);
            }
        }
        #[cfg(feature = "model160")]
        {
            if self.m160.addr != 0 {
                v.push(160);
            }
        }
        #[cfg(feature = "model201")]
        {
            if self.m201.addr != 0 {
                v.push(201);
            }
        }
        #[cfg(feature = "model202")]
        {
            if self.m202.addr != 0 {
                v.push(202);
            }
        }
        #[cfg(feature = "model203")]
        {
            if self.m203.addr != 0 {
                v.push(203);
            }
        }
        #[cfg(feature = "model204")]
        {
            if self.m204.addr != 0 {
                v.push(204);
            }
        }
        #[cfg(feature = "model211")]
        {
            if self.m211.addr != 0 {
                v.push(211);
            }
        }
        #[cfg(feature = "model212")]
        {
            if self.m212.addr != 0 {
                v.push(212);
            }
        }
        #[cfg(feature = "model213")]
        {
            if self.m213.addr != 0 {
                v.push(213);
            }
        }
        #[cfg(feature = "model214")]
        {
            if self.m214.addr != 0 {
                v.push(214);
            }
        }
        #[cfg(feature = "model220")]
        {
            if self.m220.addr != 0 {
                v.push(220);
            }
        }
        #[cfg(feature = "model302")]
        {
            if self.m302.addr != 0 {
                v.push(302);
            }
        }
        #[cfg(feature = "model303")]
        {
            if self.m303.addr != 0 {
                v.push(303);
            }
        }
        #[cfg(feature = "model304")]
        {
            if self.m304.addr != 0 {
                v.push(304);
            }
        }
        #[cfg(feature = "model305")]
        {
            if self.m305.addr != 0 {
                v.push(305);
            }
        }
        #[cfg(feature = "model306")]
        {
            if self.m306.addr != 0 {
                v.push(306);
            }
        }
        #[cfg(feature = "model307")]
        {
            if self.m307.addr != 0 {
                v.push(307);
            }
        }
        #[cfg(feature = "model308")]
        {
            if self.m308.addr != 0 {
                v.push(308);
            }
        }
        #[cfg(feature = "model401")]
        {
            if self.m401.addr != 0 {
                v.push(401);
            }
        }
        #[cfg(feature = "model402")]
        {
            if self.m402.addr != 0 {
                v.push(402);
            }
        }
        #[cfg(feature = "model403")]
        {
            if self.m403.addr != 0 {
                v.push(403);
            }
        }
        #[cfg(feature = "model404")]
        {
            if self.m404.addr != 0 {
                v.push(404);
            }
        }
        #[cfg(feature = "model501")]
        {
            if self.m501.addr != 0 {
                v.push(501);
            }
        }
        #[cfg(feature = "model502")]
        {
            if self.m502.addr != 0 {
                v.push(502);
            }
        }
        #[cfg(feature = "model601")]
        {
            if self.m601.addr != 0 {
                v.push(601);
            }
        }
        #[cfg(feature = "model701")]
        {
            if self.m701.addr != 0 {
                v.push(701);
            }
        }
        #[cfg(feature = "model702")]
        {
            if self.m702.addr != 0 {
                v.push(702);
            }
        }
        #[cfg(feature = "model703")]
        {
            if self.m703.addr != 0 {
                v.push(703);
            }
        }
        #[cfg(feature = "model704")]
        {
            if self.m704.addr != 0 {
                v.push(704);
            }
        }
        #[cfg(feature = "model705")]
        {
            if self.m705.addr != 0 {
                v.push(705);
            }
        }
        #[cfg(feature = "model706")]
        {
            if self.m706.addr != 0 {
                v.push(706);
            }
        }
        #[cfg(feature = "model707")]
        {
            if self.m707.addr != 0 {
                v.push(707);
            }
        }
        #[cfg(feature = "model708")]
        {
            if self.m708.addr != 0 {
                v.push(708);
            }
        }
        #[cfg(feature = "model709")]
        {
            if self.m709.addr != 0 {
                v.push(709);
            }
        }
        #[cfg(feature = "model710")]
        {
            if self.m710.addr != 0 {
                v.push(710);
            }
        }
        #[cfg(feature = "model711")]
        {
            if self.m711.addr != 0 {
                v.push(711);
            }
        }
        #[cfg(feature = "model712")]
        {
            if self.m712.addr != 0 {
                v.push(712);
            }
        }
        #[cfg(feature = "model713")]
        {
            if self.m713.addr != 0 {
                v.push(713);
            }
        }
        #[cfg(feature = "model714")]
        {
            if self.m714.addr != 0 {
                v.push(714);
            }
        }
        #[cfg(feature = "model715")]
        {
            if self.m715.addr != 0 {
                v.push(715);
            }
        }
        #[cfg(feature = "model801")]
        {
            if self.m801.addr != 0 {
                v.push(801);
            }
        }
        #[cfg(feature = "model802")]
        {
            if self.m802.addr != 0 {
                v.push(802);
            }
        }
        #[cfg(feature = "model803")]
        {
            if self.m803.addr != 0 {
                v.push(803);
            }
        }
        #[cfg(feature = "model804")]
        {
            if self.m804.addr != 0 {
                v.push(804);
            }
        }
        #[cfg(feature = "model805")]
        {
            if self.m805.addr != 0 {
                v.push(805);
            }
        }
        #[cfg(feature = "model806")]
        {
            if self.m806.addr != 0 {
                v.push(806);
            }
        }
        #[cfg(feature = "model807")]
        {
            if self.m807.addr != 0 {
                v.push(807);
            }
        }
        #[cfg(feature = "model808")]
        {
            if self.m808.addr != 0 {
                v.push(808);
            }
        }
        #[cfg(feature = "model809")]
        {
            if self.m809.addr != 0 {
                v.push(809);
            }
        }
        #[cfg(feature = "model63001")]
        {
            if self.m63001.addr != 0 {
                v.push(63001);
            }
        }
        #[cfg(feature = "model63002")]
        {
            if self.m63002.addr != 0 {
                v.push(63002);
            }
        }
        #[cfg(feature = "model64001")]
        {
            if self.m64001.addr != 0 {
                v.push(64001);
            }
        }
        #[cfg(feature = "model64020")]
        {
            if self.m64020.addr != 0 {
                v.push(64020);
            }
        }
        #[cfg(feature = "model64101")]
        {
            if self.m64101.addr != 0 {
                v.push(64101);
            }
        }
        #[cfg(feature = "model64111")]
        {
            if self.m64111.addr != 0 {
                v.push(64111);
            }
        }
        #[cfg(feature = "model64112")]
        {
            if self.m64112.addr != 0 {
                v.push(64112);
            }
        }
        #[cfg(feature = "model64410")]
        {
            if self.m64410.addr != 0 {
                v.push(64410);
            }
        }
        #[cfg(feature = "model64411")]
        {
            if self.m64411.addr != 0 {
                v.push(64411);
            }
        }
        #[cfg(feature = "model64412")]
        {
            if self.m64412.addr != 0 {
                v.push(64412);
            }
        }
        #[cfg(feature = "model64413")]
        {
            if self.m64413.addr != 0 {
                v.push(64413);
            }
        }
        #[cfg(feature = "model64414")]
        {
            if self.m64414.addr != 0 {
                v.push(64414);
            }
        }
        #[cfg(feature = "model64415")]
        {
            if self.m64415.addr != 0 {
                v.push(64415);
            }
        }
        v
    }
    /// Returns all discovered models together with their static metadata.
    pub fn discovered_models(&self) -> Vec<crate::DiscoveredModel> {
        let mut v = Vec::with_capacity(112usize);
        if self.m1.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model1::Model1 as crate::Model>::info(),
                addr: self.m1.addr,
                len: self.m1.len,
            });
        }
        if self.m2.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model2::Model2 as crate::Model>::info(),
                addr: self.m2.addr,
                len: self.m2.len,
            });
        }
        if self.m3.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model3::Model3 as crate::Model>::info(),
                addr: self.m3.addr,
                len: self.m3.len,
            });
        }
        if self.m4.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model4::Model4 as crate::Model>::info(),
                addr: self.m4.addr,
                len: self.m4.len,
            });
        }
        if self.m5.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model5::Model5 as crate::Model>::info(),
                addr: self.m5.addr,
                len: self.m5.len,
            });
        }
        if self.m6.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model6::Model6 as crate::Model>::info(),
                addr: self.m6.addr,
                len: self.m6.len,
            });
        }
        if self.m7.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model7::Model7 as crate::Model>::info(),
                addr: self.m7.addr,
                len: self.m7.len,
            });
        }
        if self.m8.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model8::Model8 as crate::Model>::info(),
                addr: self.m8.addr,
                len: self.m8.len,
            });
        }
        if self.m9.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model9::Model9 as crate::Model>::info(),
                addr: self.m9.addr,
                len: self.m9.len,
            });
        }
        if self.m10.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model10::Model10 as crate::Model>::info(),
                addr: self.m10.addr,
                len: self.m10.len,
            });
        }
        if self.m11.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model11::Model11 as crate::Model>::info(),
                addr: self.m11.addr,
                len: self.m11.len,
            });
        }
        if self.m12.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model12::Model12 as crate::Model>::info(),
                addr: self.m12.addr,
                len: self.m12.len,
            });
        }
        if self.m13.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model13::Model13 as crate::Model>::info(),
                addr: self.m13.addr,
                len: self.m13.len,
            });
        }
        if self.m14.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model14::Model14 as crate::Model>::info(),
                addr: self.m14.addr,
                len: self.m14.len,
            });
        }
        if self.m15.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model15::Model15 as crate::Model>::info(),
                addr: self.m15.addr,
                len: self.m15.len,
            });
        }
        if self.m16.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model16::Model16 as crate::Model>::info(),
                addr: self.m16.addr,
                len: self.m16.len,
            });
        }
        if self.m17.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model17::Model17 as crate::Model>::info(),
                addr: self.m17.addr,
                len: self.m17.len,
            });
        }
        if self.m18.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model18::Model18 as crate::Model>::info(),
                addr: self.m18.addr,
                len: self.m18.len,
            });
        }
        if self.m19.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model19::Model19 as crate::Model>::info(),
                addr: self.m19.addr,
                len: self.m19.len,
            });
        }
        if self.m101.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model101::Model101 as crate::Model>::info(),
                addr: self.m101.addr,
                len: self.m101.len,
            });
        }
        if self.m102.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model102::Model102 as crate::Model>::info(),
                addr: self.m102.addr,
                len: self.m102.len,
            });
        }
        if self.m103.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model103::Model103 as crate::Model>::info(),
                addr: self.m103.addr,
                len: self.m103.len,
            });
        }
        if self.m111.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model111::Model111 as crate::Model>::info(),
                addr: self.m111.addr,
                len: self.m111.len,
            });
        }
        if self.m112.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model112::Model112 as crate::Model>::info(),
                addr: self.m112.addr,
                len: self.m112.len,
            });
        }
        if self.m113.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model113::Model113 as crate::Model>::info(),
                addr: self.m113.addr,
                len: self.m113.len,
            });
        }
        if self.m120.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model120::Model120 as crate::Model>::info(),
                addr: self.m120.addr,
                len: self.m120.len,
            });
        }
        if self.m121.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model121::Model121 as crate::Model>::info(),
                addr: self.m121.addr,
                len: self.m121.len,
            });
        }
        if self.m122.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model122::Model122 as crate::Model>::info(),
                addr: self.m122.addr,
                len: self.m122.len,
            });
        }
        if self.m123.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model123::Model123 as crate::Model>::info(),
                addr: self.m123.addr,
                len: self.m123.len,
            });
        }
        if self.m124.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model124::Model124 as crate::Model>::info(),
                addr: self.m124.addr,
                len: self.m124.len,
            });
        }
        if self.m125.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model125::Model125 as crate::Model>::info(),
                addr: self.m125.addr,
                len: self.m125.len,
            });
        }
        if self.m126.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model126::Model126 as crate::Model>::info(),
                addr: self.m126.addr,
                len: self.m126.len,
            });
        }
        if self.m127.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model127::Model127 as crate::Model>::info(),
                addr: self.m127.addr,
                len: self.m127.len,
            });
        }
        if self.m128.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model128::Model128 as crate::Model>::info(),
                addr: self.m128.addr,
                len: self.m128.len,
            });
        }
        if self.m129.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model129::Model129 as crate::Model>::info(),
                addr: self.m129.addr,
                len: self.m129.len,
            });
        }
        if self.m130.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model130::Model130 as crate::Model>::info(),
                addr: self.m130.addr,
                len: self.m130.len,
            });
        }
        if self.m131.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model131::Model131 as crate::Model>::info(),
                addr: self.m131.addr,
                len: self.m131.len,
            });
        }
        if self.m132.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model132::Model132 as crate::Model>::info(),
                addr: self.m132.addr,
                len: self.m132.len,
            });
        }
        if self.m133.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model133::Model133 as crate::Model>::info(),
                addr: self.m133.addr,
                len: self.m133.len,
            });
        }
        if self.m134.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model134::Model134 as crate::Model>::info(),
                addr: self.m134.addr,
                len: self.m134.len,
            });
        }
        if self.m135.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model135::Model135 as crate::Model>::info(),
                addr: self.m135.addr,
                len: self.m135.len,
            });
        }
        if self.m136.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model136::Model136 as crate::Model>::info(),
                addr: self.m136.addr,
                len: self.m136.len,
            });
        }
        if self.m137.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model137::Model137 as crate::Model>::info(),
                addr: self.m137.addr,
                len: self.m137.len,
            });
        }
        if self.m138.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model138::Model138 as crate::Model>::info(),
                addr: self.m138.addr,
                len: self.m138.len,
            });
        }
        if self.m139.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model139::Model139 as crate::Model>::info(),
                addr: self.m139.addr,
                len: self.m139.len,
            });
        }
        if self.m140.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model140::Model140 as crate::Model>::info(),
                addr: self.m140.addr,
                len: self.m140.len,
            });
        }
        if self.m141.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model141::Model141 as crate::Model>::info(),
                addr: self.m141.addr,
                len: self.m141.len,
            });
        }
        if self.m142.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model142::Model142 as crate::Model>::info(),
                addr: self.m142.addr,
                len: self.m142.len,
            });
        }
        if self.m143.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model143::Model143 as crate::Model>::info(),
                addr: self.m143.addr,
                len: self.m143.len,
            });
        }
        if self.m144.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model144::Model144 as crate::Model>::info(),
                addr: self.m144.addr,
                len: self.m144.len,
            });
        }
        if self.m145.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model145::Model145 as crate::Model>::info(),
                addr: self.m145.addr,
                len: self.m145.len,
            });
        }
        if self.m160.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model160::Model160 as crate::Model>::info(),
                addr: self.m160.addr,
                len: self.m160.len,
            });
        }
        if self.m201.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model201::Model201 as crate::Model>::info(),
                addr: self.m201.addr,
                len: self.m201.len,
            });
        }
        if self.m202.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model202::Model202 as crate::Model>::info(),
                addr: self.m202.addr,
                len: self.m202.len,
            });
        }
        if self.m203.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model203::Model203 as crate::Model>::info(),
                addr: self.m203.addr,
                len: self.m203.len,
            });
        }
        if self.m204.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model204::Model204 as crate::Model>::info(),
                addr: self.m204.addr,
                len: self.m204.len,
            });
        }
        if self.m211.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model211::Model211 as crate::Model>::info(),
                addr: self.m211.addr,
                len: self.m211.len,
            });
        }
        if self.m212.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model212::Model212 as crate::Model>::info(),
                addr: self.m212.addr,
                len: self.m212.len,
            });
        }
        if self.m213.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model213::Model213 as crate::Model>::info(),
                addr: self.m213.addr,
                len: self.m213.len,
            });
        }
        if self.m214.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model214::Model214 as crate::Model>::info(),
                addr: self.m214.addr,
                len: self.m214.len,
            });
        }
        if self.m220.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model220::Model220 as crate::Model>::info(),
                addr: self.m220.addr,
                len: self.m220.len,
            });
        }
        if self.m302.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model302::Model302 as crate::Model>::info(),
                addr: self.m302.addr,
                len: self.m302.len,
            });
        }
        if self.m303.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model303::Model303 as crate::Model>::info(),
                addr: self.m303.addr,
                len: self.m303.len,
            });
        }
        if self.m304.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model304::Model304 as crate::Model>::info(),
                addr: self.m304.addr,
                len: self.m304.len,
            });
        }
        if self.m305.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model305::Model305 as crate::Model>::info(),
                addr: self.m305.addr,
                len: self.m305.len,
            });
        }
        if self.m306.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model306::Model306 as crate::Model>::info(),
                addr: self.m306.addr,
                len: self.m306.len,
            });
        }
        if self.m307.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model307::Model307 as crate::Model>::info(),
                addr: self.m307.addr,
                len: self.m307.len,
            });
        }
        if self.m308.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model308::Model308 as crate::Model>::info(),
                addr: self.m308.addr,
                len: self.m308.len,
            });
        }
        if self.m401.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model401::Model401 as crate::Model>::info(),
                addr: self.m401.addr,
                len: self.m401.len,
            });
        }
        if self.m402.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model402::Model402 as crate::Model>::info(),
                addr: self.m402.addr,
                len: self.m402.len,
            });
        }
        if self.m403.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model403::Model403 as crate::Model>::info(),
                addr: self.m403.addr,
                len: self.m403.len,
            });
        }
        if self.m404.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model404::Model404 as crate::Model>::info(),
                addr: self.m404.addr,
                len: self.m404.len,
            });
        }
        if self.m501.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model501::Model501 as crate::Model>::info(),
                addr: self.m501.addr,
                len: self.m501.len,
            });
        }
        if self.m502.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model502::Model502 as crate::Model>::info(),
                addr: self.m502.addr,
                len: self.m502.len,
            });
        }
        if self.m601.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model601::Model601 as crate::Model>::info(),
                addr: self.m601.addr,
                len: self.m601.len,
            });
        }
        if self.m701.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model701::Model701 as crate::Model>::info(),
                addr: self.m701.addr,
                len: self.m701.len,
            });
        }
        if self.m702.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model702::Model702 as crate::Model>::info(),
                addr: self.m702.addr,
                len: self.m702.len,
            });
        }
        if self.m703.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model703::Model703 as crate::Model>::info(),
                addr: self.m703.addr,
                len: self.m703.len,
            });
        }
        if self.m704.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model704::Model704 as crate::Model>::info(),
                addr: self.m704.addr,
                len: self.m704.len,
            });
        }
        if self.m705.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model705::Model705 as crate::Model>::info(),
                addr: self.m705.addr,
                len: self.m705.len,
            });
        }
        if self.m706.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model706::Model706 as crate::Model>::info(),
                addr: self.m706.addr,
                len: self.m706.len,
            });
        }
        if self.m707.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model707::Model707 as crate::Model>::info(),
                addr: self.m707.addr,
                len: self.m707.len,
            });
        }
        if self.m708.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model708::Model708 as crate::Model>::info(),
                addr: self.m708.addr,
                len: self.m708.len,
            });
        }
        if self.m709.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model709::Model709 as crate::Model>::info(),
                addr: self.m709.addr,
                len: self.m709.len,
            });
        }
        if self.m710.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model710::Model710 as crate::Model>::info(),
                addr: self.m710.addr,
                len: self.m710.len,
            });
        }
        if self.m711.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model711::Model711 as crate::Model>::info(),
                addr: self.m711.addr,
                len: self.m711.len,
            });
        }
        if self.m712.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model712::Model712 as crate::Model>::info(),
                addr: self.m712.addr,
                len: self.m712.len,
            });
        }
        if self.m713.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model713::Model713 as crate::Model>::info(),
                addr: self.m713.addr,
                len: self.m713.len,
            });
        }
        if self.m714.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model714::Model714 as crate::Model>::info(),
                addr: self.m714.addr,
                len: self.m714.len,
            });
        }
        if self.m715.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model715::Model715 as crate::Model>::info(),
                addr: self.m715.addr,
                len: self.m715.len,
            });
        }
        if self.m801.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model801::Model801 as crate::Model>::info(),
                addr: self.m801.addr,
                len: self.m801.len,
            });
        }
        if self.m802.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model802::Model802 as crate::Model>::info(),
                addr: self.m802.addr,
                len: self.m802.len,
            });
        }
        if self.m803.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model803::Model803 as crate::Model>::info(),
                addr: self.m803.addr,
                len: self.m803.len,
            });
        }
        if self.m804.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model804::Model804 as crate::Model>::info(),
                addr: self.m804.addr,
                len: self.m804.len,
            });
        }
        if self.m805.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model805::Model805 as crate::Model>::info(),
                addr: self.m805.addr,
                len: self.m805.len,
            });
        }
        if self.m806.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model806::Model806 as crate::Model>::info(),
                addr: self.m806.addr,
                len: self.m806.len,
            });
        }
        if self.m807.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model807::Model807 as crate::Model>::info(),
                addr: self.m807.addr,
                len: self.m807.len,
            });
        }
        if self.m808.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model808::Model808 as crate::Model>::info(),
                addr: self.m808.addr,
                len: self.m808.len,
            });
        }
        if self.m809.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model809::Model809 as crate::Model>::info(),
                addr: self.m809.addr,
                len: self.m809.len,
            });
        }
        if self.m63001.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model63001::Model63001 as crate::Model>::info(),
                addr: self.m63001.addr,
                len: self.m63001.len,
            });
        }
        if self.m63002.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model63002::Model63002 as crate::Model>::info(),
                addr: self.m63002.addr,
                len: self.m63002.len,
            });
        }
        if self.m64001.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64001::Model64001 as crate::Model>::info(),
                addr: self.m64001.addr,
                len: self.m64001.len,
            });
        }
        if self.m64020.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64020::Model64020 as crate::Model>::info(),
                addr: self.m64020.addr,
                len: self.m64020.len,
            });
        }
        if self.m64101.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64101::Model64101 as crate::Model>::info(),
                addr: self.m64101.addr,
                len: self.m64101.len,
            });
        }
        if self.m64111.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64111::Model64111 as crate::Model>::info(),
                addr: self.m64111.addr,
                len: self.m64111.len,
            });
        }
        if self.m64112.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64112::Model64112 as crate::Model>::info(),
                addr: self.m64112.addr,
                len: self.m64112.len,
            });
        }
        if self.m64410.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64410::Model64410 as crate::Model>::info(),
                addr: self.m64410.addr,
                len: self.m64410.len,
            });
        }
        if self.m64411.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64411::Model64411 as crate::Model>::info(),
                addr: self.m64411.addr,
                len: self.m64411.len,
            });
        }
        if self.m64412.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64412::Model64412 as crate::Model>::info(),
                addr: self.m64412.addr,
                len: self.m64412.len,
            });
        }
        if self.m64413.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64413::Model64413 as crate::Model>::info(),
                addr: self.m64413.addr,
                len: self.m64413.len,
            });
        }
        if self.m64414.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64414::Model64414 as crate::Model>::info(),
                addr: self.m64414.addr,
                len: self.m64414.len,
            });
        }
        if self.m64415.addr != 0 {
            v.push(crate::DiscoveredModel {
                info: <model64415::Model64415 as crate::Model>::info(),
                addr: self.m64415.addr,
                len: self.m64415.len,
            });
        }
        v
    }
    /// Set address and length of the given model.
    ///
    /// This method is used by the model discovery.
    pub fn set_addr(&mut self, model_id: u16, _addr: u16, _len: u16) -> bool {
        match model_id {
            #[cfg(feature = "model1")]
            1 => {
                self.m1.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model2")]
            2 => {
                self.m2.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model3")]
            3 => {
                self.m3.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model4")]
            4 => {
                self.m4.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model5")]
            5 => {
                self.m5.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model6")]
            6 => {
                self.m6.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model7")]
            7 => {
                self.m7.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model8")]
            8 => {
                self.m8.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model9")]
            9 => {
                self.m9.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model10")]
            10 => {
                self.m10.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model11")]
            11 => {
                self.m11.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model12")]
            12 => {
                self.m12.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model13")]
            13 => {
                self.m13.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model14")]
            14 => {
                self.m14.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model15")]
            15 => {
                self.m15.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model16")]
            16 => {
                self.m16.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model17")]
            17 => {
                self.m17.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model18")]
            18 => {
                self.m18.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model19")]
            19 => {
                self.m19.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model101")]
            101 => {
                self.m101.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model102")]
            102 => {
                self.m102.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model103")]
            103 => {
                self.m103.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model111")]
            111 => {
                self.m111.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model112")]
            112 => {
                self.m112.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model113")]
            113 => {
                self.m113.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model120")]
            120 => {
                self.m120.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model121")]
            121 => {
                self.m121.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model122")]
            122 => {
                self.m122.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model123")]
            123 => {
                self.m123.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model124")]
            124 => {
                self.m124.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model125")]
            125 => {
                self.m125.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model126")]
            126 => {
                self.m126.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model127")]
            127 => {
                self.m127.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model128")]
            128 => {
                self.m128.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model129")]
            129 => {
                self.m129.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model130")]
            130 => {
                self.m130.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model131")]
            131 => {
                self.m131.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model132")]
            132 => {
                self.m132.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model133")]
            133 => {
                self.m133.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model134")]
            134 => {
                self.m134.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model135")]
            135 => {
                self.m135.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model136")]
            136 => {
                self.m136.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model137")]
            137 => {
                self.m137.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model138")]
            138 => {
                self.m138.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model139")]
            139 => {
                self.m139.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model140")]
            140 => {
                self.m140.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model141")]
            141 => {
                self.m141.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model142")]
            142 => {
                self.m142.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model143")]
            143 => {
                self.m143.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model144")]
            144 => {
                self.m144.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model145")]
            145 => {
                self.m145.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model160")]
            160 => {
                self.m160.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model201")]
            201 => {
                self.m201.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model202")]
            202 => {
                self.m202.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model203")]
            203 => {
                self.m203.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model204")]
            204 => {
                self.m204.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model211")]
            211 => {
                self.m211.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model212")]
            212 => {
                self.m212.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model213")]
            213 => {
                self.m213.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model214")]
            214 => {
                self.m214.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model220")]
            220 => {
                self.m220.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model302")]
            302 => {
                self.m302.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model303")]
            303 => {
                self.m303.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model304")]
            304 => {
                self.m304.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model305")]
            305 => {
                self.m305.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model306")]
            306 => {
                self.m306.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model307")]
            307 => {
                self.m307.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model308")]
            308 => {
                self.m308.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model401")]
            401 => {
                self.m401.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model402")]
            402 => {
                self.m402.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model403")]
            403 => {
                self.m403.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model404")]
            404 => {
                self.m404.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model501")]
            501 => {
                self.m501.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model502")]
            502 => {
                self.m502.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model601")]
            601 => {
                self.m601.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model701")]
            701 => {
                self.m701.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model702")]
            702 => {
                self.m702.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model703")]
            703 => {
                self.m703.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model704")]
            704 => {
                self.m704.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model705")]
            705 => {
                self.m705.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model706")]
            706 => {
                self.m706.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model707")]
            707 => {
                self.m707.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model708")]
            708 => {
                self.m708.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model709")]
            709 => {
                self.m709.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model710")]
            710 => {
                self.m710.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model711")]
            711 => {
                self.m711.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model712")]
            712 => {
                self.m712.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model713")]
            713 => {
                self.m713.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model714")]
            714 => {
                self.m714.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model715")]
            715 => {
                self.m715.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model801")]
            801 => {
                self.m801.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model802")]
            802 => {
                self.m802.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model803")]
            803 => {
                self.m803.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model804")]
            804 => {
                self.m804.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model805")]
            805 => {
                self.m805.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model806")]
            806 => {
                self.m806.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model807")]
            807 => {
                self.m807.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model808")]
            808 => {
                self.m808.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model809")]
            809 => {
                self.m809.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model63001")]
            63001 => {
                self.m63001.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model63002")]
            63002 => {
                self.m63002.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64001")]
            64001 => {
                self.m64001.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64020")]
            64020 => {
                self.m64020.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64101")]
            64101 => {
                self.m64101.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64111")]
            64111 => {
                self.m64111.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64112")]
            64112 => {
                self.m64112.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64410")]
            64410 => {
                self.m64410.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64411")]
            64411 => {
                self.m64411.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64412")]
            64412 => {
                self.m64412.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64413")]
            64413 => {
                self.m64413.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64414")]
            64414 => {
                self.m64414.set_addr(_addr, _len);
                true
            }
            #[cfg(feature = "model64415")]
            64415 => {
                self.m64415.set_addr(_addr, _len);
                true
            }
            _ => false,
        }
    }
}
