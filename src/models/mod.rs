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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Models {
    #[cfg(feature = "model1")]
    /// Common
    pub m1: Option<crate::ModelAddr<model1::Model1>>,
    #[cfg(feature = "model2")]
    /// Basic Aggregator
    pub m2: Option<crate::ModelAddr<model2::Model2>>,
    #[cfg(feature = "model3")]
    /// Secure Dataset Read Request
    pub m3: Option<crate::ModelAddr<model3::Model3>>,
    #[cfg(feature = "model4")]
    /// Secure Dataset Read Response
    pub m4: Option<crate::ModelAddr<model4::Model4>>,
    #[cfg(feature = "model5")]
    /// Secure Write Request
    pub m5: Option<crate::ModelAddr<model5::Model5>>,
    #[cfg(feature = "model6")]
    /// Secure Write Sequential Request
    pub m6: Option<crate::ModelAddr<model6::Model6>>,
    #[cfg(feature = "model7")]
    /// Secure Write Response Model (DRAFT 1)
    pub m7: Option<crate::ModelAddr<model7::Model7>>,
    #[cfg(feature = "model8")]
    /// Get Device Security Certificate
    pub m8: Option<crate::ModelAddr<model8::Model8>>,
    #[cfg(feature = "model9")]
    /// Set Operator Security Certificate
    pub m9: Option<crate::ModelAddr<model9::Model9>>,
    #[cfg(feature = "model10")]
    /// Communication Interface Header
    pub m10: Option<crate::ModelAddr<model10::Model10>>,
    #[cfg(feature = "model11")]
    /// Ethernet Link Layer
    pub m11: Option<crate::ModelAddr<model11::Model11>>,
    #[cfg(feature = "model12")]
    /// IPv4
    pub m12: Option<crate::ModelAddr<model12::Model12>>,
    #[cfg(feature = "model13")]
    /// IPv6
    pub m13: Option<crate::ModelAddr<model13::Model13>>,
    #[cfg(feature = "model14")]
    /// Proxy Server
    pub m14: Option<crate::ModelAddr<model14::Model14>>,
    #[cfg(feature = "model15")]
    /// Interface Counters Model
    pub m15: Option<crate::ModelAddr<model15::Model15>>,
    #[cfg(feature = "model16")]
    /// Simple IP Network
    pub m16: Option<crate::ModelAddr<model16::Model16>>,
    #[cfg(feature = "model17")]
    /// Serial Interface
    pub m17: Option<crate::ModelAddr<model17::Model17>>,
    #[cfg(feature = "model18")]
    /// Cellular Link
    pub m18: Option<crate::ModelAddr<model18::Model18>>,
    #[cfg(feature = "model19")]
    /// PPP Link
    pub m19: Option<crate::ModelAddr<model19::Model19>>,
    #[cfg(feature = "model101")]
    /// Inverter (Single Phase)
    pub m101: Option<crate::ModelAddr<model101::Model101>>,
    #[cfg(feature = "model102")]
    /// Inverter (Split-Phase)
    pub m102: Option<crate::ModelAddr<model102::Model102>>,
    #[cfg(feature = "model103")]
    /// Inverter (Three Phase)
    pub m103: Option<crate::ModelAddr<model103::Model103>>,
    #[cfg(feature = "model111")]
    /// Inverter (Single Phase) FLOAT
    pub m111: Option<crate::ModelAddr<model111::Model111>>,
    #[cfg(feature = "model112")]
    /// Inverter (Split Phase) FLOAT
    pub m112: Option<crate::ModelAddr<model112::Model112>>,
    #[cfg(feature = "model113")]
    /// Inverter (Three Phase) FLOAT
    pub m113: Option<crate::ModelAddr<model113::Model113>>,
    #[cfg(feature = "model120")]
    /// Nameplate
    pub m120: Option<crate::ModelAddr<model120::Model120>>,
    #[cfg(feature = "model121")]
    /// Basic Settings
    pub m121: Option<crate::ModelAddr<model121::Model121>>,
    #[cfg(feature = "model122")]
    /// Measurements_Status
    pub m122: Option<crate::ModelAddr<model122::Model122>>,
    #[cfg(feature = "model123")]
    /// Immediate Controls
    pub m123: Option<crate::ModelAddr<model123::Model123>>,
    #[cfg(feature = "model124")]
    /// Storage
    pub m124: Option<crate::ModelAddr<model124::Model124>>,
    #[cfg(feature = "model125")]
    /// Pricing
    pub m125: Option<crate::ModelAddr<model125::Model125>>,
    #[cfg(feature = "model126")]
    /// Static Volt-VAR
    pub m126: Option<crate::ModelAddr<model126::Model126>>,
    #[cfg(feature = "model127")]
    /// Freq-Watt Param
    pub m127: Option<crate::ModelAddr<model127::Model127>>,
    #[cfg(feature = "model128")]
    /// Dynamic Reactive Current
    pub m128: Option<crate::ModelAddr<model128::Model128>>,
    #[cfg(feature = "model129")]
    /// LVRTD
    pub m129: Option<crate::ModelAddr<model129::Model129>>,
    #[cfg(feature = "model130")]
    /// HVRTD
    pub m130: Option<crate::ModelAddr<model130::Model130>>,
    #[cfg(feature = "model131")]
    /// Watt-PF
    pub m131: Option<crate::ModelAddr<model131::Model131>>,
    #[cfg(feature = "model132")]
    /// Volt-Watt
    pub m132: Option<crate::ModelAddr<model132::Model132>>,
    #[cfg(feature = "model133")]
    /// Basic Scheduling
    pub m133: Option<crate::ModelAddr<model133::Model133>>,
    #[cfg(feature = "model134")]
    /// Freq-Watt Crv
    pub m134: Option<crate::ModelAddr<model134::Model134>>,
    #[cfg(feature = "model135")]
    /// LFRT
    pub m135: Option<crate::ModelAddr<model135::Model135>>,
    #[cfg(feature = "model136")]
    /// HFRT
    pub m136: Option<crate::ModelAddr<model136::Model136>>,
    #[cfg(feature = "model137")]
    /// LVRTC
    pub m137: Option<crate::ModelAddr<model137::Model137>>,
    #[cfg(feature = "model138")]
    /// HVRTC
    pub m138: Option<crate::ModelAddr<model138::Model138>>,
    #[cfg(feature = "model139")]
    /// LVRTX
    pub m139: Option<crate::ModelAddr<model139::Model139>>,
    #[cfg(feature = "model140")]
    /// HVRTX
    pub m140: Option<crate::ModelAddr<model140::Model140>>,
    #[cfg(feature = "model141")]
    /// LFRTC
    pub m141: Option<crate::ModelAddr<model141::Model141>>,
    #[cfg(feature = "model142")]
    /// HFRTC
    pub m142: Option<crate::ModelAddr<model142::Model142>>,
    #[cfg(feature = "model143")]
    /// LFRTX
    pub m143: Option<crate::ModelAddr<model143::Model143>>,
    #[cfg(feature = "model144")]
    /// HFRTX
    pub m144: Option<crate::ModelAddr<model144::Model144>>,
    #[cfg(feature = "model145")]
    /// Extended Settings
    pub m145: Option<crate::ModelAddr<model145::Model145>>,
    #[cfg(feature = "model160")]
    /// Multiple MPPT Inverter Extension Model
    pub m160: Option<crate::ModelAddr<model160::Model160>>,
    #[cfg(feature = "model201")]
    /// Meter (Single Phase) single phase (AN or AB) meter
    pub m201: Option<crate::ModelAddr<model201::Model201>>,
    #[cfg(feature = "model202")]
    /// split single phase (ABN) meter
    pub m202: Option<crate::ModelAddr<model202::Model202>>,
    #[cfg(feature = "model203")]
    /// wye-connect three phase (abcn) meter
    pub m203: Option<crate::ModelAddr<model203::Model203>>,
    #[cfg(feature = "model204")]
    /// delta-connect three phase (abc) meter
    pub m204: Option<crate::ModelAddr<model204::Model204>>,
    #[cfg(feature = "model211")]
    /// single phase (AN or AB) meter
    pub m211: Option<crate::ModelAddr<model211::Model211>>,
    #[cfg(feature = "model212")]
    /// split single phase (ABN) meter
    pub m212: Option<crate::ModelAddr<model212::Model212>>,
    #[cfg(feature = "model213")]
    /// wye-connect three phase (abcn) meter
    pub m213: Option<crate::ModelAddr<model213::Model213>>,
    #[cfg(feature = "model214")]
    /// delta-connect three phase (abc) meter
    pub m214: Option<crate::ModelAddr<model214::Model214>>,
    #[cfg(feature = "model220")]
    /// Secure AC Meter Selected Readings
    pub m220: Option<crate::ModelAddr<model220::Model220>>,
    #[cfg(feature = "model302")]
    /// Irradiance Model
    pub m302: Option<crate::ModelAddr<model302::Model302>>,
    #[cfg(feature = "model303")]
    /// Back of Module Temperature Model
    pub m303: Option<crate::ModelAddr<model303::Model303>>,
    #[cfg(feature = "model304")]
    /// Inclinometer Model
    pub m304: Option<crate::ModelAddr<model304::Model304>>,
    #[cfg(feature = "model305")]
    /// GPS
    pub m305: Option<crate::ModelAddr<model305::Model305>>,
    #[cfg(feature = "model306")]
    /// Reference Point Model
    pub m306: Option<crate::ModelAddr<model306::Model306>>,
    #[cfg(feature = "model307")]
    /// Base Met
    pub m307: Option<crate::ModelAddr<model307::Model307>>,
    #[cfg(feature = "model308")]
    /// Mini Met Model
    pub m308: Option<crate::ModelAddr<model308::Model308>>,
    #[cfg(feature = "model401")]
    /// String Combiner (Current)
    pub m401: Option<crate::ModelAddr<model401::Model401>>,
    #[cfg(feature = "model402")]
    /// String Combiner (Advanced)
    pub m402: Option<crate::ModelAddr<model402::Model402>>,
    #[cfg(feature = "model403")]
    /// String Combiner (Current)
    pub m403: Option<crate::ModelAddr<model403::Model403>>,
    #[cfg(feature = "model404")]
    /// String Combiner (Advanced)
    pub m404: Option<crate::ModelAddr<model404::Model404>>,
    #[cfg(feature = "model501")]
    /// Solar Module
    pub m501: Option<crate::ModelAddr<model501::Model501>>,
    #[cfg(feature = "model502")]
    /// Solar Module
    pub m502: Option<crate::ModelAddr<model502::Model502>>,
    #[cfg(feature = "model601")]
    /// Tracker Controller DRAFT 2
    pub m601: Option<crate::ModelAddr<model601::Model601>>,
    #[cfg(feature = "model701")]
    /// DER AC Measurement
    pub m701: Option<crate::ModelAddr<model701::Model701>>,
    #[cfg(feature = "model702")]
    /// DER Capacity
    pub m702: Option<crate::ModelAddr<model702::Model702>>,
    #[cfg(feature = "model703")]
    /// Enter Service
    pub m703: Option<crate::ModelAddr<model703::Model703>>,
    #[cfg(feature = "model704")]
    /// DER AC Controls
    pub m704: Option<crate::ModelAddr<model704::Model704>>,
    #[cfg(feature = "model705")]
    /// DER Volt-Var
    pub m705: Option<crate::ModelAddr<model705::Model705>>,
    #[cfg(feature = "model706")]
    /// DER Volt-Watt
    pub m706: Option<crate::ModelAddr<model706::Model706>>,
    #[cfg(feature = "model707")]
    /// DER Trip LV
    pub m707: Option<crate::ModelAddr<model707::Model707>>,
    #[cfg(feature = "model708")]
    /// DER Trip HV
    pub m708: Option<crate::ModelAddr<model708::Model708>>,
    #[cfg(feature = "model709")]
    /// DER Trip LF
    pub m709: Option<crate::ModelAddr<model709::Model709>>,
    #[cfg(feature = "model710")]
    /// DER Trip HF
    pub m710: Option<crate::ModelAddr<model710::Model710>>,
    #[cfg(feature = "model711")]
    /// DER Frequency Droop
    pub m711: Option<crate::ModelAddr<model711::Model711>>,
    #[cfg(feature = "model712")]
    /// DER Watt-Var
    pub m712: Option<crate::ModelAddr<model712::Model712>>,
    #[cfg(feature = "model713")]
    /// DER Storage Capacity
    pub m713: Option<crate::ModelAddr<model713::Model713>>,
    #[cfg(feature = "model714")]
    /// DER DC Measurement
    pub m714: Option<crate::ModelAddr<model714::Model714>>,
    #[cfg(feature = "model715")]
    /// DERCtl
    pub m715: Option<crate::ModelAddr<model715::Model715>>,
    #[cfg(feature = "model801")]
    /// Energy Storage Base Model (DEPRECATED)
    pub m801: Option<crate::ModelAddr<model801::Model801>>,
    #[cfg(feature = "model802")]
    /// Battery Base Model
    pub m802: Option<crate::ModelAddr<model802::Model802>>,
    #[cfg(feature = "model803")]
    /// Lithium-Ion Battery Bank Model
    pub m803: Option<crate::ModelAddr<model803::Model803>>,
    #[cfg(feature = "model804")]
    /// Lithium-Ion String Model
    pub m804: Option<crate::ModelAddr<model804::Model804>>,
    #[cfg(feature = "model805")]
    /// Lithium-Ion Module Model
    pub m805: Option<crate::ModelAddr<model805::Model805>>,
    #[cfg(feature = "model806")]
    /// Flow Battery Model
    pub m806: Option<crate::ModelAddr<model806::Model806>>,
    #[cfg(feature = "model807")]
    /// Flow Battery String Model
    pub m807: Option<crate::ModelAddr<model807::Model807>>,
    #[cfg(feature = "model808")]
    /// Flow Battery Module Model
    pub m808: Option<crate::ModelAddr<model808::Model808>>,
    #[cfg(feature = "model809")]
    /// Flow Battery Stack Model
    pub m809: Option<crate::ModelAddr<model809::Model809>>,
    #[cfg(feature = "model63001")]
    /// SunSpec Test Model 1
    pub m63001: Option<crate::ModelAddr<model63001::Model63001>>,
    #[cfg(feature = "model63002")]
    /// SunSpec Test Model 2
    pub m63002: Option<crate::ModelAddr<model63002::Model63002>>,
    #[cfg(feature = "model64001")]
    /// Veris Status and Configuration
    pub m64001: Option<crate::ModelAddr<model64001::Model64001>>,
    #[cfg(feature = "model64020")]
    /// Mersen GreenString
    pub m64020: Option<crate::ModelAddr<model64020::Model64020>>,
    #[cfg(feature = "model64101")]
    /// Eltek Inverter Extension
    pub m64101: Option<crate::ModelAddr<model64101::Model64101>>,
    #[cfg(feature = "model64111")]
    /// Basic Charge Controller
    pub m64111: Option<crate::ModelAddr<model64111::Model64111>>,
    #[cfg(feature = "model64112")]
    /// OutBack FM Charge Controller
    pub m64112: Option<crate::ModelAddr<model64112::Model64112>>,
    #[cfg(feature = "model64410")]
    /// DC Simulator Control Interface
    pub m64410: Option<crate::ModelAddr<model64410::Model64410>>,
    #[cfg(feature = "model64411")]
    /// AC Simulator Control Interface
    pub m64411: Option<crate::ModelAddr<model64411::Model64411>>,
    #[cfg(feature = "model64412")]
    /// DER Cyber Exploitation
    pub m64412: Option<crate::ModelAddr<model64412::Model64412>>,
    #[cfg(feature = "model64413")]
    /// PV Curves
    pub m64413: Option<crate::ModelAddr<model64413::Model64413>>,
    #[cfg(feature = "model64414")]
    /// DER Simulation Controls
    pub m64414: Option<crate::ModelAddr<model64414::Model64414>>,
    #[cfg(feature = "model64415")]
    /// CSIP Client Control
    pub m64415: Option<crate::ModelAddr<model64415::Model64415>>,
}
impl Models {
    /// Set address and length of the given model.
    ///
    /// This method is used by the model discovery.
    pub fn set_addr(&mut self, model_id: u16, _addr: std::num::NonZeroU16, _len: u16) -> bool {
        match model_id {
            #[cfg(feature = "model1")]
            1 => {
                self.m1 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model2")]
            2 => {
                self.m2 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model3")]
            3 => {
                self.m3 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model4")]
            4 => {
                self.m4 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model5")]
            5 => {
                self.m5 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model6")]
            6 => {
                self.m6 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model7")]
            7 => {
                self.m7 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model8")]
            8 => {
                self.m8 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model9")]
            9 => {
                self.m9 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model10")]
            10 => {
                self.m10 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model11")]
            11 => {
                self.m11 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model12")]
            12 => {
                self.m12 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model13")]
            13 => {
                self.m13 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model14")]
            14 => {
                self.m14 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model15")]
            15 => {
                self.m15 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model16")]
            16 => {
                self.m16 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model17")]
            17 => {
                self.m17 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model18")]
            18 => {
                self.m18 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model19")]
            19 => {
                self.m19 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model101")]
            101 => {
                self.m101 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model102")]
            102 => {
                self.m102 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model103")]
            103 => {
                self.m103 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model111")]
            111 => {
                self.m111 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model112")]
            112 => {
                self.m112 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model113")]
            113 => {
                self.m113 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model120")]
            120 => {
                self.m120 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model121")]
            121 => {
                self.m121 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model122")]
            122 => {
                self.m122 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model123")]
            123 => {
                self.m123 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model124")]
            124 => {
                self.m124 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model125")]
            125 => {
                self.m125 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model126")]
            126 => {
                self.m126 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model127")]
            127 => {
                self.m127 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model128")]
            128 => {
                self.m128 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model129")]
            129 => {
                self.m129 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model130")]
            130 => {
                self.m130 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model131")]
            131 => {
                self.m131 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model132")]
            132 => {
                self.m132 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model133")]
            133 => {
                self.m133 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model134")]
            134 => {
                self.m134 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model135")]
            135 => {
                self.m135 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model136")]
            136 => {
                self.m136 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model137")]
            137 => {
                self.m137 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model138")]
            138 => {
                self.m138 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model139")]
            139 => {
                self.m139 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model140")]
            140 => {
                self.m140 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model141")]
            141 => {
                self.m141 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model142")]
            142 => {
                self.m142 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model143")]
            143 => {
                self.m143 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model144")]
            144 => {
                self.m144 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model145")]
            145 => {
                self.m145 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model160")]
            160 => {
                self.m160 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model201")]
            201 => {
                self.m201 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model202")]
            202 => {
                self.m202 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model203")]
            203 => {
                self.m203 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model204")]
            204 => {
                self.m204 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model211")]
            211 => {
                self.m211 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model212")]
            212 => {
                self.m212 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model213")]
            213 => {
                self.m213 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model214")]
            214 => {
                self.m214 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model220")]
            220 => {
                self.m220 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model302")]
            302 => {
                self.m302 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model303")]
            303 => {
                self.m303 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model304")]
            304 => {
                self.m304 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model305")]
            305 => {
                self.m305 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model306")]
            306 => {
                self.m306 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model307")]
            307 => {
                self.m307 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model308")]
            308 => {
                self.m308 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model401")]
            401 => {
                self.m401 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model402")]
            402 => {
                self.m402 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model403")]
            403 => {
                self.m403 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model404")]
            404 => {
                self.m404 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model501")]
            501 => {
                self.m501 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model502")]
            502 => {
                self.m502 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model601")]
            601 => {
                self.m601 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model701")]
            701 => {
                self.m701 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model702")]
            702 => {
                self.m702 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model703")]
            703 => {
                self.m703 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model704")]
            704 => {
                self.m704 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model705")]
            705 => {
                self.m705 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model706")]
            706 => {
                self.m706 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model707")]
            707 => {
                self.m707 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model708")]
            708 => {
                self.m708 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model709")]
            709 => {
                self.m709 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model710")]
            710 => {
                self.m710 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model711")]
            711 => {
                self.m711 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model712")]
            712 => {
                self.m712 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model713")]
            713 => {
                self.m713 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model714")]
            714 => {
                self.m714 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model715")]
            715 => {
                self.m715 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model801")]
            801 => {
                self.m801 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model802")]
            802 => {
                self.m802 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model803")]
            803 => {
                self.m803 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model804")]
            804 => {
                self.m804 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model805")]
            805 => {
                self.m805 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model806")]
            806 => {
                self.m806 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model807")]
            807 => {
                self.m807 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model808")]
            808 => {
                self.m808 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model809")]
            809 => {
                self.m809 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model63001")]
            63001 => {
                self.m63001 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model63002")]
            63002 => {
                self.m63002 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64001")]
            64001 => {
                self.m64001 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64020")]
            64020 => {
                self.m64020 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64101")]
            64101 => {
                self.m64101 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64111")]
            64111 => {
                self.m64111 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64112")]
            64112 => {
                self.m64112 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64410")]
            64410 => {
                self.m64410 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64411")]
            64411 => {
                self.m64411 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64412")]
            64412 => {
                self.m64412 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64413")]
            64413 => {
                self.m64413 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64414")]
            64414 => {
                self.m64414 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            #[cfg(feature = "model64415")]
            64415 => {
                self.m64415 = Some(crate::ModelAddr::new(_addr, _len));
                true
            }
            _ => false,
        }
    }
}
/// Information about all models enabled via Cargo features, sorted by id.
pub static MODELS: &[&crate::ModelInfo] = &[
    #[cfg(feature = "model1")]
    &<model1::Model1 as crate::Model>::INFO,
    #[cfg(feature = "model2")]
    &<model2::Model2 as crate::Model>::INFO,
    #[cfg(feature = "model3")]
    &<model3::Model3 as crate::Model>::INFO,
    #[cfg(feature = "model4")]
    &<model4::Model4 as crate::Model>::INFO,
    #[cfg(feature = "model5")]
    &<model5::Model5 as crate::Model>::INFO,
    #[cfg(feature = "model6")]
    &<model6::Model6 as crate::Model>::INFO,
    #[cfg(feature = "model7")]
    &<model7::Model7 as crate::Model>::INFO,
    #[cfg(feature = "model8")]
    &<model8::Model8 as crate::Model>::INFO,
    #[cfg(feature = "model9")]
    &<model9::Model9 as crate::Model>::INFO,
    #[cfg(feature = "model10")]
    &<model10::Model10 as crate::Model>::INFO,
    #[cfg(feature = "model11")]
    &<model11::Model11 as crate::Model>::INFO,
    #[cfg(feature = "model12")]
    &<model12::Model12 as crate::Model>::INFO,
    #[cfg(feature = "model13")]
    &<model13::Model13 as crate::Model>::INFO,
    #[cfg(feature = "model14")]
    &<model14::Model14 as crate::Model>::INFO,
    #[cfg(feature = "model15")]
    &<model15::Model15 as crate::Model>::INFO,
    #[cfg(feature = "model16")]
    &<model16::Model16 as crate::Model>::INFO,
    #[cfg(feature = "model17")]
    &<model17::Model17 as crate::Model>::INFO,
    #[cfg(feature = "model18")]
    &<model18::Model18 as crate::Model>::INFO,
    #[cfg(feature = "model19")]
    &<model19::Model19 as crate::Model>::INFO,
    #[cfg(feature = "model101")]
    &<model101::Model101 as crate::Model>::INFO,
    #[cfg(feature = "model102")]
    &<model102::Model102 as crate::Model>::INFO,
    #[cfg(feature = "model103")]
    &<model103::Model103 as crate::Model>::INFO,
    #[cfg(feature = "model111")]
    &<model111::Model111 as crate::Model>::INFO,
    #[cfg(feature = "model112")]
    &<model112::Model112 as crate::Model>::INFO,
    #[cfg(feature = "model113")]
    &<model113::Model113 as crate::Model>::INFO,
    #[cfg(feature = "model120")]
    &<model120::Model120 as crate::Model>::INFO,
    #[cfg(feature = "model121")]
    &<model121::Model121 as crate::Model>::INFO,
    #[cfg(feature = "model122")]
    &<model122::Model122 as crate::Model>::INFO,
    #[cfg(feature = "model123")]
    &<model123::Model123 as crate::Model>::INFO,
    #[cfg(feature = "model124")]
    &<model124::Model124 as crate::Model>::INFO,
    #[cfg(feature = "model125")]
    &<model125::Model125 as crate::Model>::INFO,
    #[cfg(feature = "model126")]
    &<model126::Model126 as crate::Model>::INFO,
    #[cfg(feature = "model127")]
    &<model127::Model127 as crate::Model>::INFO,
    #[cfg(feature = "model128")]
    &<model128::Model128 as crate::Model>::INFO,
    #[cfg(feature = "model129")]
    &<model129::Model129 as crate::Model>::INFO,
    #[cfg(feature = "model130")]
    &<model130::Model130 as crate::Model>::INFO,
    #[cfg(feature = "model131")]
    &<model131::Model131 as crate::Model>::INFO,
    #[cfg(feature = "model132")]
    &<model132::Model132 as crate::Model>::INFO,
    #[cfg(feature = "model133")]
    &<model133::Model133 as crate::Model>::INFO,
    #[cfg(feature = "model134")]
    &<model134::Model134 as crate::Model>::INFO,
    #[cfg(feature = "model135")]
    &<model135::Model135 as crate::Model>::INFO,
    #[cfg(feature = "model136")]
    &<model136::Model136 as crate::Model>::INFO,
    #[cfg(feature = "model137")]
    &<model137::Model137 as crate::Model>::INFO,
    #[cfg(feature = "model138")]
    &<model138::Model138 as crate::Model>::INFO,
    #[cfg(feature = "model139")]
    &<model139::Model139 as crate::Model>::INFO,
    #[cfg(feature = "model140")]
    &<model140::Model140 as crate::Model>::INFO,
    #[cfg(feature = "model141")]
    &<model141::Model141 as crate::Model>::INFO,
    #[cfg(feature = "model142")]
    &<model142::Model142 as crate::Model>::INFO,
    #[cfg(feature = "model143")]
    &<model143::Model143 as crate::Model>::INFO,
    #[cfg(feature = "model144")]
    &<model144::Model144 as crate::Model>::INFO,
    #[cfg(feature = "model145")]
    &<model145::Model145 as crate::Model>::INFO,
    #[cfg(feature = "model160")]
    &<model160::Model160 as crate::Model>::INFO,
    #[cfg(feature = "model201")]
    &<model201::Model201 as crate::Model>::INFO,
    #[cfg(feature = "model202")]
    &<model202::Model202 as crate::Model>::INFO,
    #[cfg(feature = "model203")]
    &<model203::Model203 as crate::Model>::INFO,
    #[cfg(feature = "model204")]
    &<model204::Model204 as crate::Model>::INFO,
    #[cfg(feature = "model211")]
    &<model211::Model211 as crate::Model>::INFO,
    #[cfg(feature = "model212")]
    &<model212::Model212 as crate::Model>::INFO,
    #[cfg(feature = "model213")]
    &<model213::Model213 as crate::Model>::INFO,
    #[cfg(feature = "model214")]
    &<model214::Model214 as crate::Model>::INFO,
    #[cfg(feature = "model220")]
    &<model220::Model220 as crate::Model>::INFO,
    #[cfg(feature = "model302")]
    &<model302::Model302 as crate::Model>::INFO,
    #[cfg(feature = "model303")]
    &<model303::Model303 as crate::Model>::INFO,
    #[cfg(feature = "model304")]
    &<model304::Model304 as crate::Model>::INFO,
    #[cfg(feature = "model305")]
    &<model305::Model305 as crate::Model>::INFO,
    #[cfg(feature = "model306")]
    &<model306::Model306 as crate::Model>::INFO,
    #[cfg(feature = "model307")]
    &<model307::Model307 as crate::Model>::INFO,
    #[cfg(feature = "model308")]
    &<model308::Model308 as crate::Model>::INFO,
    #[cfg(feature = "model401")]
    &<model401::Model401 as crate::Model>::INFO,
    #[cfg(feature = "model402")]
    &<model402::Model402 as crate::Model>::INFO,
    #[cfg(feature = "model403")]
    &<model403::Model403 as crate::Model>::INFO,
    #[cfg(feature = "model404")]
    &<model404::Model404 as crate::Model>::INFO,
    #[cfg(feature = "model501")]
    &<model501::Model501 as crate::Model>::INFO,
    #[cfg(feature = "model502")]
    &<model502::Model502 as crate::Model>::INFO,
    #[cfg(feature = "model601")]
    &<model601::Model601 as crate::Model>::INFO,
    #[cfg(feature = "model701")]
    &<model701::Model701 as crate::Model>::INFO,
    #[cfg(feature = "model702")]
    &<model702::Model702 as crate::Model>::INFO,
    #[cfg(feature = "model703")]
    &<model703::Model703 as crate::Model>::INFO,
    #[cfg(feature = "model704")]
    &<model704::Model704 as crate::Model>::INFO,
    #[cfg(feature = "model705")]
    &<model705::Model705 as crate::Model>::INFO,
    #[cfg(feature = "model706")]
    &<model706::Model706 as crate::Model>::INFO,
    #[cfg(feature = "model707")]
    &<model707::Model707 as crate::Model>::INFO,
    #[cfg(feature = "model708")]
    &<model708::Model708 as crate::Model>::INFO,
    #[cfg(feature = "model709")]
    &<model709::Model709 as crate::Model>::INFO,
    #[cfg(feature = "model710")]
    &<model710::Model710 as crate::Model>::INFO,
    #[cfg(feature = "model711")]
    &<model711::Model711 as crate::Model>::INFO,
    #[cfg(feature = "model712")]
    &<model712::Model712 as crate::Model>::INFO,
    #[cfg(feature = "model713")]
    &<model713::Model713 as crate::Model>::INFO,
    #[cfg(feature = "model714")]
    &<model714::Model714 as crate::Model>::INFO,
    #[cfg(feature = "model715")]
    &<model715::Model715 as crate::Model>::INFO,
    #[cfg(feature = "model801")]
    &<model801::Model801 as crate::Model>::INFO,
    #[cfg(feature = "model802")]
    &<model802::Model802 as crate::Model>::INFO,
    #[cfg(feature = "model803")]
    &<model803::Model803 as crate::Model>::INFO,
    #[cfg(feature = "model804")]
    &<model804::Model804 as crate::Model>::INFO,
    #[cfg(feature = "model805")]
    &<model805::Model805 as crate::Model>::INFO,
    #[cfg(feature = "model806")]
    &<model806::Model806 as crate::Model>::INFO,
    #[cfg(feature = "model807")]
    &<model807::Model807 as crate::Model>::INFO,
    #[cfg(feature = "model808")]
    &<model808::Model808 as crate::Model>::INFO,
    #[cfg(feature = "model809")]
    &<model809::Model809 as crate::Model>::INFO,
    #[cfg(feature = "model63001")]
    &<model63001::Model63001 as crate::Model>::INFO,
    #[cfg(feature = "model63002")]
    &<model63002::Model63002 as crate::Model>::INFO,
    #[cfg(feature = "model64001")]
    &<model64001::Model64001 as crate::Model>::INFO,
    #[cfg(feature = "model64020")]
    &<model64020::Model64020 as crate::Model>::INFO,
    #[cfg(feature = "model64101")]
    &<model64101::Model64101 as crate::Model>::INFO,
    #[cfg(feature = "model64111")]
    &<model64111::Model64111 as crate::Model>::INFO,
    #[cfg(feature = "model64112")]
    &<model64112::Model64112 as crate::Model>::INFO,
    #[cfg(feature = "model64410")]
    &<model64410::Model64410 as crate::Model>::INFO,
    #[cfg(feature = "model64411")]
    &<model64411::Model64411 as crate::Model>::INFO,
    #[cfg(feature = "model64412")]
    &<model64412::Model64412 as crate::Model>::INFO,
    #[cfg(feature = "model64413")]
    &<model64413::Model64413 as crate::Model>::INFO,
    #[cfg(feature = "model64414")]
    &<model64414::Model64414 as crate::Model>::INFO,
    #[cfg(feature = "model64415")]
    &<model64415::Model64415 as crate::Model>::INFO,
];
/// Data of any model enabled via Cargo features.
///
/// This is useful when the model to be read is only known at runtime,
/// e.g. when it was selected via [`ModelInfo`](crate::ModelInfo).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize),
    serde(tag = "model")
)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum AnyModel {
    #[cfg(feature = "model1")]
    /// Common
    #[cfg_attr(feature = "serde", serde(rename = "common"))]
    M1(model1::Model1),
    #[cfg(feature = "model2")]
    /// Basic Aggregator
    #[cfg_attr(feature = "serde", serde(rename = "aggregator"))]
    M2(model2::Model2),
    #[cfg(feature = "model3")]
    /// Secure Dataset Read Request
    #[cfg_attr(feature = "serde", serde(rename = "model_3"))]
    M3(model3::Model3),
    #[cfg(feature = "model4")]
    /// Secure Dataset Read Response
    #[cfg_attr(feature = "serde", serde(rename = "model_4"))]
    M4(model4::Model4),
    #[cfg(feature = "model5")]
    /// Secure Write Request
    #[cfg_attr(feature = "serde", serde(rename = "model_5"))]
    M5(model5::Model5),
    #[cfg(feature = "model6")]
    /// Secure Write Sequential Request
    #[cfg_attr(feature = "serde", serde(rename = "model_6"))]
    M6(model6::Model6),
    #[cfg(feature = "model7")]
    /// Secure Write Response Model (DRAFT 1)
    #[cfg_attr(feature = "serde", serde(rename = "model_7"))]
    M7(model7::Model7),
    #[cfg(feature = "model8")]
    /// Get Device Security Certificate
    #[cfg_attr(feature = "serde", serde(rename = "model_8"))]
    M8(model8::Model8),
    #[cfg(feature = "model9")]
    /// Set Operator Security Certificate
    #[cfg_attr(feature = "serde", serde(rename = "model_9"))]
    M9(model9::Model9),
    #[cfg(feature = "model10")]
    /// Communication Interface Header
    #[cfg_attr(feature = "serde", serde(rename = "model_10"))]
    M10(model10::Model10),
    #[cfg(feature = "model11")]
    /// Ethernet Link Layer
    #[cfg_attr(feature = "serde", serde(rename = "model_11"))]
    M11(model11::Model11),
    #[cfg(feature = "model12")]
    /// IPv4
    #[cfg_attr(feature = "serde", serde(rename = "model_12"))]
    M12(model12::Model12),
    #[cfg(feature = "model13")]
    /// IPv6
    #[cfg_attr(feature = "serde", serde(rename = "model_13"))]
    M13(model13::Model13),
    #[cfg(feature = "model14")]
    /// Proxy Server
    #[cfg_attr(feature = "serde", serde(rename = "model_14"))]
    M14(model14::Model14),
    #[cfg(feature = "model15")]
    /// Interface Counters Model
    #[cfg_attr(feature = "serde", serde(rename = "model_15"))]
    M15(model15::Model15),
    #[cfg(feature = "model16")]
    /// Simple IP Network
    #[cfg_attr(feature = "serde", serde(rename = "model_16"))]
    M16(model16::Model16),
    #[cfg(feature = "model17")]
    /// Serial Interface
    #[cfg_attr(feature = "serde", serde(rename = "model_17"))]
    M17(model17::Model17),
    #[cfg(feature = "model18")]
    /// Cellular Link
    #[cfg_attr(feature = "serde", serde(rename = "model_18"))]
    M18(model18::Model18),
    #[cfg(feature = "model19")]
    /// PPP Link
    #[cfg_attr(feature = "serde", serde(rename = "model_19"))]
    M19(model19::Model19),
    #[cfg(feature = "model101")]
    /// Inverter (Single Phase)
    #[cfg_attr(feature = "serde", serde(rename = "inverter_single_phase"))]
    M101(model101::Model101),
    #[cfg(feature = "model102")]
    /// Inverter (Split-Phase)
    #[cfg_attr(feature = "serde", serde(rename = "inverter_split_phase"))]
    M102(model102::Model102),
    #[cfg(feature = "model103")]
    /// Inverter (Three Phase)
    #[cfg_attr(feature = "serde", serde(rename = "inverter_three_phase"))]
    M103(model103::Model103),
    #[cfg(feature = "model111")]
    /// Inverter (Single Phase) FLOAT
    #[cfg_attr(feature = "serde", serde(rename = "inverter_single_phase_float"))]
    M111(model111::Model111),
    #[cfg(feature = "model112")]
    /// Inverter (Split Phase) FLOAT
    #[cfg_attr(feature = "serde", serde(rename = "inverter_split_phase_float"))]
    M112(model112::Model112),
    #[cfg(feature = "model113")]
    /// Inverter (Three Phase) FLOAT
    #[cfg_attr(feature = "serde", serde(rename = "inverter_three_phase_float"))]
    M113(model113::Model113),
    #[cfg(feature = "model120")]
    /// Nameplate
    #[cfg_attr(feature = "serde", serde(rename = "nameplate"))]
    M120(model120::Model120),
    #[cfg(feature = "model121")]
    /// Basic Settings
    #[cfg_attr(feature = "serde", serde(rename = "settings"))]
    M121(model121::Model121),
    #[cfg(feature = "model122")]
    /// Measurements_Status
    #[cfg_attr(feature = "serde", serde(rename = "status"))]
    M122(model122::Model122),
    #[cfg(feature = "model123")]
    /// Immediate Controls
    #[cfg_attr(feature = "serde", serde(rename = "controls"))]
    M123(model123::Model123),
    #[cfg(feature = "model124")]
    /// Storage
    #[cfg_attr(feature = "serde", serde(rename = "storage_basic"))]
    M124(model124::Model124),
    #[cfg(feature = "model125")]
    /// Pricing
    #[cfg_attr(feature = "serde", serde(rename = "pricing"))]
    M125(model125::Model125),
    #[cfg(feature = "model126")]
    /// Static Volt-VAR
    #[cfg_attr(feature = "serde", serde(rename = "volt_var"))]
    M126(model126::Model126),
    #[cfg(feature = "model127")]
    /// Freq-Watt Param
    #[cfg_attr(feature = "serde", serde(rename = "freq_watt_param"))]
    M127(model127::Model127),
    #[cfg(feature = "model128")]
    /// Dynamic Reactive Current
    #[cfg_attr(feature = "serde", serde(rename = "reactive_current"))]
    M128(model128::Model128),
    #[cfg(feature = "model129")]
    /// LVRTD
    #[cfg_attr(feature = "serde", serde(rename = "lvrt"))]
    M129(model129::Model129),
    #[cfg(feature = "model130")]
    /// HVRTD
    #[cfg_attr(feature = "serde", serde(rename = "hvrt"))]
    M130(model130::Model130),
    #[cfg(feature = "model131")]
    /// Watt-PF
    #[cfg_attr(feature = "serde", serde(rename = "watt_pf"))]
    M131(model131::Model131),
    #[cfg(feature = "model132")]
    /// Volt-Watt
    #[cfg_attr(feature = "serde", serde(rename = "volt_watt"))]
    M132(model132::Model132),
    #[cfg(feature = "model133")]
    /// Basic Scheduling
    #[cfg_attr(feature = "serde", serde(rename = "schedule"))]
    M133(model133::Model133),
    #[cfg(feature = "model134")]
    /// Freq-Watt Crv
    #[cfg_attr(feature = "serde", serde(rename = "freq_watt"))]
    M134(model134::Model134),
    #[cfg(feature = "model135")]
    /// LFRT
    #[cfg_attr(feature = "serde", serde(rename = "lfrt"))]
    M135(model135::Model135),
    #[cfg(feature = "model136")]
    /// HFRT
    #[cfg_attr(feature = "serde", serde(rename = "hfrt"))]
    M136(model136::Model136),
    #[cfg(feature = "model137")]
    /// LVRTC
    #[cfg_attr(feature = "serde", serde(rename = "lvrtc"))]
    M137(model137::Model137),
    #[cfg(feature = "model138")]
    /// HVRTC
    #[cfg_attr(feature = "serde", serde(rename = "hvrtc"))]
    M138(model138::Model138),
    #[cfg(feature = "model139")]
    /// LVRTX
    #[cfg_attr(feature = "serde", serde(rename = "lvrtx"))]
    M139(model139::Model139),
    #[cfg(feature = "model140")]
    /// HVRTX
    #[cfg_attr(feature = "serde", serde(rename = "hvrtx"))]
    M140(model140::Model140),
    #[cfg(feature = "model141")]
    /// LFRTC
    #[cfg_attr(feature = "serde", serde(rename = "lfrtc"))]
    M141(model141::Model141),
    #[cfg(feature = "model142")]
    /// HFRTC
    #[cfg_attr(feature = "serde", serde(rename = "hfrtc"))]
    M142(model142::Model142),
    #[cfg(feature = "model143")]
    /// LFRTX
    #[cfg_attr(feature = "serde", serde(rename = "lfrtx"))]
    M143(model143::Model143),
    #[cfg(feature = "model144")]
    /// HFRTX
    #[cfg_attr(feature = "serde", serde(rename = "hfrtx"))]
    M144(model144::Model144),
    #[cfg(feature = "model145")]
    /// Extended Settings
    #[cfg_attr(feature = "serde", serde(rename = "ext_settings"))]
    M145(model145::Model145),
    #[cfg(feature = "model160")]
    /// Multiple MPPT Inverter Extension Model
    #[cfg_attr(feature = "serde", serde(rename = "mppt"))]
    M160(model160::Model160),
    #[cfg(feature = "model201")]
    /// Meter (Single Phase) single phase (AN or AB) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_an_or_ab"))]
    M201(model201::Model201),
    #[cfg(feature = "model202")]
    /// split single phase (ABN) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_abn"))]
    M202(model202::Model202),
    #[cfg(feature = "model203")]
    /// wye-connect three phase (abcn) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_abcn"))]
    M203(model203::Model203),
    #[cfg(feature = "model204")]
    /// delta-connect three phase (abc) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_abc"))]
    M204(model204::Model204),
    #[cfg(feature = "model211")]
    /// single phase (AN or AB) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_an_or_ab_float"))]
    M211(model211::Model211),
    #[cfg(feature = "model212")]
    /// split single phase (ABN) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_abn_float"))]
    M212(model212::Model212),
    #[cfg(feature = "model213")]
    /// wye-connect three phase (abcn) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_abcn_float"))]
    M213(model213::Model213),
    #[cfg(feature = "model214")]
    /// delta-connect three phase (abc) meter
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_abc_float"))]
    M214(model214::Model214),
    #[cfg(feature = "model220")]
    /// Secure AC Meter Selected Readings
    #[cfg_attr(feature = "serde", serde(rename = "ac_meter_secure"))]
    M220(model220::Model220),
    #[cfg(feature = "model302")]
    /// Irradiance Model
    #[cfg_attr(feature = "serde", serde(rename = "irradiance"))]
    M302(model302::Model302),
    #[cfg(feature = "model303")]
    /// Back of Module Temperature Model
    #[cfg_attr(feature = "serde", serde(rename = "bom_temp"))]
    M303(model303::Model303),
    #[cfg(feature = "model304")]
    /// Inclinometer Model
    #[cfg_attr(feature = "serde", serde(rename = "inclinometer"))]
    M304(model304::Model304),
    #[cfg(feature = "model305")]
    /// GPS
    #[cfg_attr(feature = "serde", serde(rename = "location"))]
    M305(model305::Model305),
    #[cfg(feature = "model306")]
    /// Reference Point Model
    #[cfg_attr(feature = "serde", serde(rename = "ref_point"))]
    M306(model306::Model306),
    #[cfg(feature = "model307")]
    /// Base Met
    #[cfg_attr(feature = "serde", serde(rename = "base_met"))]
    M307(model307::Model307),
    #[cfg(feature = "model308")]
    /// Mini Met Model
    #[cfg_attr(feature = "serde", serde(rename = "mini_met"))]
    M308(model308::Model308),
    #[cfg(feature = "model401")]
    /// String Combiner (Current)
    #[cfg_attr(feature = "serde", serde(rename = "string_combiner_current"))]
    M401(model401::Model401),
    #[cfg(feature = "model402")]
    /// String Combiner (Advanced)
    #[cfg_attr(feature = "serde", serde(rename = "string_combiner_advanced"))]
    M402(model402::Model402),
    #[cfg(feature = "model403")]
    /// String Combiner (Current)
    #[cfg_attr(feature = "serde", serde(rename = "string_combiner_current_input"))]
    M403(model403::Model403),
    #[cfg(feature = "model404")]
    /// String Combiner (Advanced)
    #[cfg_attr(feature = "serde", serde(rename = "string_combiner_advanced_inputs"))]
    M404(model404::Model404),
    #[cfg(feature = "model501")]
    /// Solar Module
    #[cfg_attr(feature = "serde", serde(rename = "solar_module_float"))]
    M501(model501::Model501),
    #[cfg(feature = "model502")]
    /// Solar Module
    #[cfg_attr(feature = "serde", serde(rename = "solar_module"))]
    M502(model502::Model502),
    #[cfg(feature = "model601")]
    /// Tracker Controller DRAFT 2
    #[cfg_attr(feature = "serde", serde(rename = "tracker_controller"))]
    M601(model601::Model601),
    #[cfg(feature = "model701")]
    /// DER AC Measurement
    #[cfg_attr(feature = "serde", serde(rename = "DERMeasureAC"))]
    M701(model701::Model701),
    #[cfg(feature = "model702")]
    /// DER Capacity
    #[cfg_attr(feature = "serde", serde(rename = "DERCapacity"))]
    M702(model702::Model702),
    #[cfg(feature = "model703")]
    /// Enter Service
    #[cfg_attr(feature = "serde", serde(rename = "DEREnterService"))]
    M703(model703::Model703),
    #[cfg(feature = "model704")]
    /// DER AC Controls
    #[cfg_attr(feature = "serde", serde(rename = "DERCtlAC"))]
    M704(model704::Model704),
    #[cfg(feature = "model705")]
    /// DER Volt-Var
    #[cfg_attr(feature = "serde", serde(rename = "DERVoltVar"))]
    M705(model705::Model705),
    #[cfg(feature = "model706")]
    /// DER Volt-Watt
    #[cfg_attr(feature = "serde", serde(rename = "DERVoltWatt"))]
    M706(model706::Model706),
    #[cfg(feature = "model707")]
    /// DER Trip LV
    #[cfg_attr(feature = "serde", serde(rename = "DERTripLV"))]
    M707(model707::Model707),
    #[cfg(feature = "model708")]
    /// DER Trip HV
    #[cfg_attr(feature = "serde", serde(rename = "DERTripHV"))]
    M708(model708::Model708),
    #[cfg(feature = "model709")]
    /// DER Trip LF
    #[cfg_attr(feature = "serde", serde(rename = "DERTripLF"))]
    M709(model709::Model709),
    #[cfg(feature = "model710")]
    /// DER Trip HF
    #[cfg_attr(feature = "serde", serde(rename = "DERTripHF"))]
    M710(model710::Model710),
    #[cfg(feature = "model711")]
    /// DER Frequency Droop
    #[cfg_attr(feature = "serde", serde(rename = "DERFreqDroop"))]
    M711(model711::Model711),
    #[cfg(feature = "model712")]
    /// DER Watt-Var
    #[cfg_attr(feature = "serde", serde(rename = "DERWattVar"))]
    M712(model712::Model712),
    #[cfg(feature = "model713")]
    /// DER Storage Capacity
    #[cfg_attr(feature = "serde", serde(rename = "DERStorageCapacity"))]
    M713(model713::Model713),
    #[cfg(feature = "model714")]
    /// DER DC Measurement
    #[cfg_attr(feature = "serde", serde(rename = "DERMeasureDC"))]
    M714(model714::Model714),
    #[cfg(feature = "model715")]
    /// DERCtl
    #[cfg_attr(feature = "serde", serde(rename = "DERCtl"))]
    M715(model715::Model715),
    #[cfg(feature = "model801")]
    /// Energy Storage Base Model (DEPRECATED)
    #[cfg_attr(feature = "serde", serde(rename = "storage"))]
    M801(model801::Model801),
    #[cfg(feature = "model802")]
    /// Battery Base Model
    #[cfg_attr(feature = "serde", serde(rename = "battery"))]
    M802(model802::Model802),
    #[cfg(feature = "model803")]
    /// Lithium-Ion Battery Bank Model
    #[cfg_attr(feature = "serde", serde(rename = "lithium_ion_bank"))]
    M803(model803::Model803),
    #[cfg(feature = "model804")]
    /// Lithium-Ion String Model
    #[cfg_attr(feature = "serde", serde(rename = "lithium_ion_string"))]
    M804(model804::Model804),
    #[cfg(feature = "model805")]
    /// Lithium-Ion Module Model
    #[cfg_attr(feature = "serde", serde(rename = "lithium-ion-module"))]
    M805(model805::Model805),
    #[cfg(feature = "model806")]
    /// Flow Battery Model
    #[cfg_attr(feature = "serde", serde(rename = "flow_battery"))]
    M806(model806::Model806),
    #[cfg(feature = "model807")]
    /// Flow Battery String Model
    #[cfg_attr(feature = "serde", serde(rename = "flow_battery_string"))]
    M807(model807::Model807),
    #[cfg(feature = "model808")]
    /// Flow Battery Module Model
    #[cfg_attr(feature = "serde", serde(rename = "flow_battery_module"))]
    M808(model808::Model808),
    #[cfg(feature = "model809")]
    /// Flow Battery Stack Model
    #[cfg_attr(feature = "serde", serde(rename = "flow_battery_stack"))]
    M809(model809::Model809),
    #[cfg(feature = "model63001")]
    /// SunSpec Test Model 1
    #[cfg_attr(feature = "serde", serde(rename = "model_63001"))]
    M63001(model63001::Model63001),
    #[cfg(feature = "model63002")]
    /// SunSpec Test Model 2
    #[cfg_attr(feature = "serde", serde(rename = "model_63002"))]
    M63002(model63002::Model63002),
    #[cfg(feature = "model64001")]
    /// Veris Status and Configuration
    #[cfg_attr(feature = "serde", serde(rename = "model_64001"))]
    M64001(model64001::Model64001),
    #[cfg(feature = "model64020")]
    /// Mersen GreenString
    #[cfg_attr(feature = "serde", serde(rename = "model_64020"))]
    M64020(model64020::Model64020),
    #[cfg(feature = "model64101")]
    /// Eltek Inverter Extension
    #[cfg_attr(feature = "serde", serde(rename = "model_64101"))]
    M64101(model64101::Model64101),
    #[cfg(feature = "model64111")]
    /// Basic Charge Controller
    #[cfg_attr(feature = "serde", serde(rename = "model_64111"))]
    M64111(model64111::Model64111),
    #[cfg(feature = "model64112")]
    /// OutBack FM Charge Controller
    #[cfg_attr(feature = "serde", serde(rename = "model_64112"))]
    M64112(model64112::Model64112),
    #[cfg(feature = "model64410")]
    /// DC Simulator Control Interface
    #[cfg_attr(feature = "serde", serde(rename = "DCSimInterface"))]
    M64410(model64410::Model64410),
    #[cfg(feature = "model64411")]
    /// AC Simulator Control Interface
    #[cfg_attr(feature = "serde", serde(rename = "ACSimInterface"))]
    M64411(model64411::Model64411),
    #[cfg(feature = "model64412")]
    /// DER Cyber Exploitation
    #[cfg_attr(feature = "serde", serde(rename = "DERExploitation"))]
    M64412(model64412::Model64412),
    #[cfg(feature = "model64413")]
    /// PV Curves
    #[cfg_attr(feature = "serde", serde(rename = "PVSimCurves"))]
    M64413(model64413::Model64413),
    #[cfg(feature = "model64414")]
    /// DER Simulation Controls
    #[cfg_attr(feature = "serde", serde(rename = "DERSimControls"))]
    M64414(model64414::Model64414),
    #[cfg(feature = "model64415")]
    /// CSIP Client Control
    #[cfg_attr(feature = "serde", serde(rename = "CSIPControl"))]
    M64415(model64415::Model64415),
}
impl AnyModel {
    /// Returns the contained model as trait object.
    pub fn as_dyn(&self) -> &dyn crate::DynModel {
        match *self {
            #[cfg(feature = "model1")]
            Self::M1(ref model) => model,
            #[cfg(feature = "model2")]
            Self::M2(ref model) => model,
            #[cfg(feature = "model3")]
            Self::M3(ref model) => model,
            #[cfg(feature = "model4")]
            Self::M4(ref model) => model,
            #[cfg(feature = "model5")]
            Self::M5(ref model) => model,
            #[cfg(feature = "model6")]
            Self::M6(ref model) => model,
            #[cfg(feature = "model7")]
            Self::M7(ref model) => model,
            #[cfg(feature = "model8")]
            Self::M8(ref model) => model,
            #[cfg(feature = "model9")]
            Self::M9(ref model) => model,
            #[cfg(feature = "model10")]
            Self::M10(ref model) => model,
            #[cfg(feature = "model11")]
            Self::M11(ref model) => model,
            #[cfg(feature = "model12")]
            Self::M12(ref model) => model,
            #[cfg(feature = "model13")]
            Self::M13(ref model) => model,
            #[cfg(feature = "model14")]
            Self::M14(ref model) => model,
            #[cfg(feature = "model15")]
            Self::M15(ref model) => model,
            #[cfg(feature = "model16")]
            Self::M16(ref model) => model,
            #[cfg(feature = "model17")]
            Self::M17(ref model) => model,
            #[cfg(feature = "model18")]
            Self::M18(ref model) => model,
            #[cfg(feature = "model19")]
            Self::M19(ref model) => model,
            #[cfg(feature = "model101")]
            Self::M101(ref model) => model,
            #[cfg(feature = "model102")]
            Self::M102(ref model) => model,
            #[cfg(feature = "model103")]
            Self::M103(ref model) => model,
            #[cfg(feature = "model111")]
            Self::M111(ref model) => model,
            #[cfg(feature = "model112")]
            Self::M112(ref model) => model,
            #[cfg(feature = "model113")]
            Self::M113(ref model) => model,
            #[cfg(feature = "model120")]
            Self::M120(ref model) => model,
            #[cfg(feature = "model121")]
            Self::M121(ref model) => model,
            #[cfg(feature = "model122")]
            Self::M122(ref model) => model,
            #[cfg(feature = "model123")]
            Self::M123(ref model) => model,
            #[cfg(feature = "model124")]
            Self::M124(ref model) => model,
            #[cfg(feature = "model125")]
            Self::M125(ref model) => model,
            #[cfg(feature = "model126")]
            Self::M126(ref model) => model,
            #[cfg(feature = "model127")]
            Self::M127(ref model) => model,
            #[cfg(feature = "model128")]
            Self::M128(ref model) => model,
            #[cfg(feature = "model129")]
            Self::M129(ref model) => model,
            #[cfg(feature = "model130")]
            Self::M130(ref model) => model,
            #[cfg(feature = "model131")]
            Self::M131(ref model) => model,
            #[cfg(feature = "model132")]
            Self::M132(ref model) => model,
            #[cfg(feature = "model133")]
            Self::M133(ref model) => model,
            #[cfg(feature = "model134")]
            Self::M134(ref model) => model,
            #[cfg(feature = "model135")]
            Self::M135(ref model) => model,
            #[cfg(feature = "model136")]
            Self::M136(ref model) => model,
            #[cfg(feature = "model137")]
            Self::M137(ref model) => model,
            #[cfg(feature = "model138")]
            Self::M138(ref model) => model,
            #[cfg(feature = "model139")]
            Self::M139(ref model) => model,
            #[cfg(feature = "model140")]
            Self::M140(ref model) => model,
            #[cfg(feature = "model141")]
            Self::M141(ref model) => model,
            #[cfg(feature = "model142")]
            Self::M142(ref model) => model,
            #[cfg(feature = "model143")]
            Self::M143(ref model) => model,
            #[cfg(feature = "model144")]
            Self::M144(ref model) => model,
            #[cfg(feature = "model145")]
            Self::M145(ref model) => model,
            #[cfg(feature = "model160")]
            Self::M160(ref model) => model,
            #[cfg(feature = "model201")]
            Self::M201(ref model) => model,
            #[cfg(feature = "model202")]
            Self::M202(ref model) => model,
            #[cfg(feature = "model203")]
            Self::M203(ref model) => model,
            #[cfg(feature = "model204")]
            Self::M204(ref model) => model,
            #[cfg(feature = "model211")]
            Self::M211(ref model) => model,
            #[cfg(feature = "model212")]
            Self::M212(ref model) => model,
            #[cfg(feature = "model213")]
            Self::M213(ref model) => model,
            #[cfg(feature = "model214")]
            Self::M214(ref model) => model,
            #[cfg(feature = "model220")]
            Self::M220(ref model) => model,
            #[cfg(feature = "model302")]
            Self::M302(ref model) => model,
            #[cfg(feature = "model303")]
            Self::M303(ref model) => model,
            #[cfg(feature = "model304")]
            Self::M304(ref model) => model,
            #[cfg(feature = "model305")]
            Self::M305(ref model) => model,
            #[cfg(feature = "model306")]
            Self::M306(ref model) => model,
            #[cfg(feature = "model307")]
            Self::M307(ref model) => model,
            #[cfg(feature = "model308")]
            Self::M308(ref model) => model,
            #[cfg(feature = "model401")]
            Self::M401(ref model) => model,
            #[cfg(feature = "model402")]
            Self::M402(ref model) => model,
            #[cfg(feature = "model403")]
            Self::M403(ref model) => model,
            #[cfg(feature = "model404")]
            Self::M404(ref model) => model,
            #[cfg(feature = "model501")]
            Self::M501(ref model) => model,
            #[cfg(feature = "model502")]
            Self::M502(ref model) => model,
            #[cfg(feature = "model601")]
            Self::M601(ref model) => model,
            #[cfg(feature = "model701")]
            Self::M701(ref model) => model,
            #[cfg(feature = "model702")]
            Self::M702(ref model) => model,
            #[cfg(feature = "model703")]
            Self::M703(ref model) => model,
            #[cfg(feature = "model704")]
            Self::M704(ref model) => model,
            #[cfg(feature = "model705")]
            Self::M705(ref model) => model,
            #[cfg(feature = "model706")]
            Self::M706(ref model) => model,
            #[cfg(feature = "model707")]
            Self::M707(ref model) => model,
            #[cfg(feature = "model708")]
            Self::M708(ref model) => model,
            #[cfg(feature = "model709")]
            Self::M709(ref model) => model,
            #[cfg(feature = "model710")]
            Self::M710(ref model) => model,
            #[cfg(feature = "model711")]
            Self::M711(ref model) => model,
            #[cfg(feature = "model712")]
            Self::M712(ref model) => model,
            #[cfg(feature = "model713")]
            Self::M713(ref model) => model,
            #[cfg(feature = "model714")]
            Self::M714(ref model) => model,
            #[cfg(feature = "model715")]
            Self::M715(ref model) => model,
            #[cfg(feature = "model801")]
            Self::M801(ref model) => model,
            #[cfg(feature = "model802")]
            Self::M802(ref model) => model,
            #[cfg(feature = "model803")]
            Self::M803(ref model) => model,
            #[cfg(feature = "model804")]
            Self::M804(ref model) => model,
            #[cfg(feature = "model805")]
            Self::M805(ref model) => model,
            #[cfg(feature = "model806")]
            Self::M806(ref model) => model,
            #[cfg(feature = "model807")]
            Self::M807(ref model) => model,
            #[cfg(feature = "model808")]
            Self::M808(ref model) => model,
            #[cfg(feature = "model809")]
            Self::M809(ref model) => model,
            #[cfg(feature = "model63001")]
            Self::M63001(ref model) => model,
            #[cfg(feature = "model63002")]
            Self::M63002(ref model) => model,
            #[cfg(feature = "model64001")]
            Self::M64001(ref model) => model,
            #[cfg(feature = "model64020")]
            Self::M64020(ref model) => model,
            #[cfg(feature = "model64101")]
            Self::M64101(ref model) => model,
            #[cfg(feature = "model64111")]
            Self::M64111(ref model) => model,
            #[cfg(feature = "model64112")]
            Self::M64112(ref model) => model,
            #[cfg(feature = "model64410")]
            Self::M64410(ref model) => model,
            #[cfg(feature = "model64411")]
            Self::M64411(ref model) => model,
            #[cfg(feature = "model64412")]
            Self::M64412(ref model) => model,
            #[cfg(feature = "model64413")]
            Self::M64413(ref model) => model,
            #[cfg(feature = "model64414")]
            Self::M64414(ref model) => model,
            #[cfg(feature = "model64415")]
            Self::M64415(ref model) => model,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn assert_impl_clone<T: Clone>() {}
    fn assert_impl_partial_eq<T: PartialEq>() {}
    fn assert_impl_eq<T: Eq>() {}
    /// Every model must implement `Clone` and `PartialEq`. Models
    /// without floating point values must implement `Eq`, too.
    #[test]
    fn models_implement_clone_and_eq() {
        assert_impl_clone::<Models>();
        assert_impl_partial_eq::<Models>();
        assert_impl_eq::<Models>();
        assert_impl_clone::<AnyModel>();
        assert_impl_partial_eq::<AnyModel>();
        #[cfg(feature = "model1")]
        {
            assert_impl_clone::<model1::Model1>();
            assert_impl_partial_eq::<model1::Model1>();
            assert_impl_eq::<model1::Model1>();
        }
        #[cfg(feature = "model2")]
        {
            assert_impl_clone::<model2::Model2>();
            assert_impl_partial_eq::<model2::Model2>();
            assert_impl_eq::<model2::Model2>();
        }
        #[cfg(feature = "model3")]
        {
            assert_impl_clone::<model3::Model3>();
            assert_impl_partial_eq::<model3::Model3>();
            assert_impl_eq::<model3::Model3>();
        }
        #[cfg(feature = "model4")]
        {
            assert_impl_clone::<model4::Model4>();
            assert_impl_partial_eq::<model4::Model4>();
            assert_impl_eq::<model4::Model4>();
        }
        #[cfg(feature = "model5")]
        {
            assert_impl_clone::<model5::Model5>();
            assert_impl_partial_eq::<model5::Model5>();
            assert_impl_eq::<model5::Model5>();
        }
        #[cfg(feature = "model6")]
        {
            assert_impl_clone::<model6::Model6>();
            assert_impl_partial_eq::<model6::Model6>();
            assert_impl_eq::<model6::Model6>();
        }
        #[cfg(feature = "model7")]
        {
            assert_impl_clone::<model7::Model7>();
            assert_impl_partial_eq::<model7::Model7>();
            assert_impl_eq::<model7::Model7>();
        }
        #[cfg(feature = "model8")]
        {
            assert_impl_clone::<model8::Model8>();
            assert_impl_partial_eq::<model8::Model8>();
            assert_impl_eq::<model8::Model8>();
        }
        #[cfg(feature = "model9")]
        {
            assert_impl_clone::<model9::Model9>();
            assert_impl_partial_eq::<model9::Model9>();
            assert_impl_eq::<model9::Model9>();
        }
        #[cfg(feature = "model10")]
        {
            assert_impl_clone::<model10::Model10>();
            assert_impl_partial_eq::<model10::Model10>();
            assert_impl_eq::<model10::Model10>();
        }
        #[cfg(feature = "model11")]
        {
            assert_impl_clone::<model11::Model11>();
            assert_impl_partial_eq::<model11::Model11>();
            assert_impl_eq::<model11::Model11>();
        }
        #[cfg(feature = "model12")]
        {
            assert_impl_clone::<model12::Model12>();
            assert_impl_partial_eq::<model12::Model12>();
            assert_impl_eq::<model12::Model12>();
        }
        #[cfg(feature = "model13")]
        {
            assert_impl_clone::<model13::Model13>();
            assert_impl_partial_eq::<model13::Model13>();
            assert_impl_eq::<model13::Model13>();
        }
        #[cfg(feature = "model14")]
        {
            assert_impl_clone::<model14::Model14>();
            assert_impl_partial_eq::<model14::Model14>();
            assert_impl_eq::<model14::Model14>();
        }
        #[cfg(feature = "model15")]
        {
            assert_impl_clone::<model15::Model15>();
            assert_impl_partial_eq::<model15::Model15>();
            assert_impl_eq::<model15::Model15>();
        }
        #[cfg(feature = "model16")]
        {
            assert_impl_clone::<model16::Model16>();
            assert_impl_partial_eq::<model16::Model16>();
            assert_impl_eq::<model16::Model16>();
        }
        #[cfg(feature = "model17")]
        {
            assert_impl_clone::<model17::Model17>();
            assert_impl_partial_eq::<model17::Model17>();
            assert_impl_eq::<model17::Model17>();
        }
        #[cfg(feature = "model18")]
        {
            assert_impl_clone::<model18::Model18>();
            assert_impl_partial_eq::<model18::Model18>();
            assert_impl_eq::<model18::Model18>();
        }
        #[cfg(feature = "model19")]
        {
            assert_impl_clone::<model19::Model19>();
            assert_impl_partial_eq::<model19::Model19>();
            assert_impl_eq::<model19::Model19>();
        }
        #[cfg(feature = "model101")]
        {
            assert_impl_clone::<model101::Model101>();
            assert_impl_partial_eq::<model101::Model101>();
            assert_impl_eq::<model101::Model101>();
        }
        #[cfg(feature = "model102")]
        {
            assert_impl_clone::<model102::Model102>();
            assert_impl_partial_eq::<model102::Model102>();
            assert_impl_eq::<model102::Model102>();
        }
        #[cfg(feature = "model103")]
        {
            assert_impl_clone::<model103::Model103>();
            assert_impl_partial_eq::<model103::Model103>();
            assert_impl_eq::<model103::Model103>();
        }
        #[cfg(feature = "model111")]
        {
            assert_impl_clone::<model111::Model111>();
            assert_impl_partial_eq::<model111::Model111>();
        }
        #[cfg(feature = "model112")]
        {
            assert_impl_clone::<model112::Model112>();
            assert_impl_partial_eq::<model112::Model112>();
        }
        #[cfg(feature = "model113")]
        {
            assert_impl_clone::<model113::Model113>();
            assert_impl_partial_eq::<model113::Model113>();
        }
        #[cfg(feature = "model120")]
        {
            assert_impl_clone::<model120::Model120>();
            assert_impl_partial_eq::<model120::Model120>();
            assert_impl_eq::<model120::Model120>();
        }
        #[cfg(feature = "model121")]
        {
            assert_impl_clone::<model121::Model121>();
            assert_impl_partial_eq::<model121::Model121>();
            assert_impl_eq::<model121::Model121>();
        }
        #[cfg(feature = "model122")]
        {
            assert_impl_clone::<model122::Model122>();
            assert_impl_partial_eq::<model122::Model122>();
            assert_impl_eq::<model122::Model122>();
        }
        #[cfg(feature = "model123")]
        {
            assert_impl_clone::<model123::Model123>();
            assert_impl_partial_eq::<model123::Model123>();
            assert_impl_eq::<model123::Model123>();
        }
        #[cfg(feature = "model124")]
        {
            assert_impl_clone::<model124::Model124>();
            assert_impl_partial_eq::<model124::Model124>();
            assert_impl_eq::<model124::Model124>();
        }
        #[cfg(feature = "model125")]
        {
            assert_impl_clone::<model125::Model125>();
            assert_impl_partial_eq::<model125::Model125>();
            assert_impl_eq::<model125::Model125>();
        }
        #[cfg(feature = "model126")]
        {
            assert_impl_clone::<model126::Model126>();
            assert_impl_partial_eq::<model126::Model126>();
            assert_impl_eq::<model126::Model126>();
        }
        #[cfg(feature = "model127")]
        {
            assert_impl_clone::<model127::Model127>();
            assert_impl_partial_eq::<model127::Model127>();
            assert_impl_eq::<model127::Model127>();
        }
        #[cfg(feature = "model128")]
        {
            assert_impl_clone::<model128::Model128>();
            assert_impl_partial_eq::<model128::Model128>();
            assert_impl_eq::<model128::Model128>();
        }
        #[cfg(feature = "model129")]
        {
            assert_impl_clone::<model129::Model129>();
            assert_impl_partial_eq::<model129::Model129>();
            assert_impl_eq::<model129::Model129>();
        }
        #[cfg(feature = "model130")]
        {
            assert_impl_clone::<model130::Model130>();
            assert_impl_partial_eq::<model130::Model130>();
            assert_impl_eq::<model130::Model130>();
        }
        #[cfg(feature = "model131")]
        {
            assert_impl_clone::<model131::Model131>();
            assert_impl_partial_eq::<model131::Model131>();
            assert_impl_eq::<model131::Model131>();
        }
        #[cfg(feature = "model132")]
        {
            assert_impl_clone::<model132::Model132>();
            assert_impl_partial_eq::<model132::Model132>();
            assert_impl_eq::<model132::Model132>();
        }
        #[cfg(feature = "model133")]
        {
            assert_impl_clone::<model133::Model133>();
            assert_impl_partial_eq::<model133::Model133>();
            assert_impl_eq::<model133::Model133>();
        }
        #[cfg(feature = "model134")]
        {
            assert_impl_clone::<model134::Model134>();
            assert_impl_partial_eq::<model134::Model134>();
            assert_impl_eq::<model134::Model134>();
        }
        #[cfg(feature = "model135")]
        {
            assert_impl_clone::<model135::Model135>();
            assert_impl_partial_eq::<model135::Model135>();
            assert_impl_eq::<model135::Model135>();
        }
        #[cfg(feature = "model136")]
        {
            assert_impl_clone::<model136::Model136>();
            assert_impl_partial_eq::<model136::Model136>();
            assert_impl_eq::<model136::Model136>();
        }
        #[cfg(feature = "model137")]
        {
            assert_impl_clone::<model137::Model137>();
            assert_impl_partial_eq::<model137::Model137>();
            assert_impl_eq::<model137::Model137>();
        }
        #[cfg(feature = "model138")]
        {
            assert_impl_clone::<model138::Model138>();
            assert_impl_partial_eq::<model138::Model138>();
            assert_impl_eq::<model138::Model138>();
        }
        #[cfg(feature = "model139")]
        {
            assert_impl_clone::<model139::Model139>();
            assert_impl_partial_eq::<model139::Model139>();
            assert_impl_eq::<model139::Model139>();
        }
        #[cfg(feature = "model140")]
        {
            assert_impl_clone::<model140::Model140>();
            assert_impl_partial_eq::<model140::Model140>();
            assert_impl_eq::<model140::Model140>();
        }
        #[cfg(feature = "model141")]
        {
            assert_impl_clone::<model141::Model141>();
            assert_impl_partial_eq::<model141::Model141>();
            assert_impl_eq::<model141::Model141>();
        }
        #[cfg(feature = "model142")]
        {
            assert_impl_clone::<model142::Model142>();
            assert_impl_partial_eq::<model142::Model142>();
            assert_impl_eq::<model142::Model142>();
        }
        #[cfg(feature = "model143")]
        {
            assert_impl_clone::<model143::Model143>();
            assert_impl_partial_eq::<model143::Model143>();
            assert_impl_eq::<model143::Model143>();
        }
        #[cfg(feature = "model144")]
        {
            assert_impl_clone::<model144::Model144>();
            assert_impl_partial_eq::<model144::Model144>();
            assert_impl_eq::<model144::Model144>();
        }
        #[cfg(feature = "model145")]
        {
            assert_impl_clone::<model145::Model145>();
            assert_impl_partial_eq::<model145::Model145>();
            assert_impl_eq::<model145::Model145>();
        }
        #[cfg(feature = "model160")]
        {
            assert_impl_clone::<model160::Model160>();
            assert_impl_partial_eq::<model160::Model160>();
            assert_impl_eq::<model160::Model160>();
        }
        #[cfg(feature = "model201")]
        {
            assert_impl_clone::<model201::Model201>();
            assert_impl_partial_eq::<model201::Model201>();
            assert_impl_eq::<model201::Model201>();
        }
        #[cfg(feature = "model202")]
        {
            assert_impl_clone::<model202::Model202>();
            assert_impl_partial_eq::<model202::Model202>();
            assert_impl_eq::<model202::Model202>();
        }
        #[cfg(feature = "model203")]
        {
            assert_impl_clone::<model203::Model203>();
            assert_impl_partial_eq::<model203::Model203>();
            assert_impl_eq::<model203::Model203>();
        }
        #[cfg(feature = "model204")]
        {
            assert_impl_clone::<model204::Model204>();
            assert_impl_partial_eq::<model204::Model204>();
            assert_impl_eq::<model204::Model204>();
        }
        #[cfg(feature = "model211")]
        {
            assert_impl_clone::<model211::Model211>();
            assert_impl_partial_eq::<model211::Model211>();
        }
        #[cfg(feature = "model212")]
        {
            assert_impl_clone::<model212::Model212>();
            assert_impl_partial_eq::<model212::Model212>();
        }
        #[cfg(feature = "model213")]
        {
            assert_impl_clone::<model213::Model213>();
            assert_impl_partial_eq::<model213::Model213>();
        }
        #[cfg(feature = "model214")]
        {
            assert_impl_clone::<model214::Model214>();
            assert_impl_partial_eq::<model214::Model214>();
        }
        #[cfg(feature = "model220")]
        {
            assert_impl_clone::<model220::Model220>();
            assert_impl_partial_eq::<model220::Model220>();
            assert_impl_eq::<model220::Model220>();
        }
        #[cfg(feature = "model302")]
        {
            assert_impl_clone::<model302::Model302>();
            assert_impl_partial_eq::<model302::Model302>();
            assert_impl_eq::<model302::Model302>();
        }
        #[cfg(feature = "model303")]
        {
            assert_impl_clone::<model303::Model303>();
            assert_impl_partial_eq::<model303::Model303>();
            assert_impl_eq::<model303::Model303>();
        }
        #[cfg(feature = "model304")]
        {
            assert_impl_clone::<model304::Model304>();
            assert_impl_partial_eq::<model304::Model304>();
            assert_impl_eq::<model304::Model304>();
        }
        #[cfg(feature = "model305")]
        {
            assert_impl_clone::<model305::Model305>();
            assert_impl_partial_eq::<model305::Model305>();
            assert_impl_eq::<model305::Model305>();
        }
        #[cfg(feature = "model306")]
        {
            assert_impl_clone::<model306::Model306>();
            assert_impl_partial_eq::<model306::Model306>();
            assert_impl_eq::<model306::Model306>();
        }
        #[cfg(feature = "model307")]
        {
            assert_impl_clone::<model307::Model307>();
            assert_impl_partial_eq::<model307::Model307>();
            assert_impl_eq::<model307::Model307>();
        }
        #[cfg(feature = "model308")]
        {
            assert_impl_clone::<model308::Model308>();
            assert_impl_partial_eq::<model308::Model308>();
            assert_impl_eq::<model308::Model308>();
        }
        #[cfg(feature = "model401")]
        {
            assert_impl_clone::<model401::Model401>();
            assert_impl_partial_eq::<model401::Model401>();
            assert_impl_eq::<model401::Model401>();
        }
        #[cfg(feature = "model402")]
        {
            assert_impl_clone::<model402::Model402>();
            assert_impl_partial_eq::<model402::Model402>();
            assert_impl_eq::<model402::Model402>();
        }
        #[cfg(feature = "model403")]
        {
            assert_impl_clone::<model403::Model403>();
            assert_impl_partial_eq::<model403::Model403>();
            assert_impl_eq::<model403::Model403>();
        }
        #[cfg(feature = "model404")]
        {
            assert_impl_clone::<model404::Model404>();
            assert_impl_partial_eq::<model404::Model404>();
            assert_impl_eq::<model404::Model404>();
        }
        #[cfg(feature = "model501")]
        {
            assert_impl_clone::<model501::Model501>();
            assert_impl_partial_eq::<model501::Model501>();
        }
        #[cfg(feature = "model502")]
        {
            assert_impl_clone::<model502::Model502>();
            assert_impl_partial_eq::<model502::Model502>();
            assert_impl_eq::<model502::Model502>();
        }
        #[cfg(feature = "model601")]
        {
            assert_impl_clone::<model601::Model601>();
            assert_impl_partial_eq::<model601::Model601>();
            assert_impl_eq::<model601::Model601>();
        }
        #[cfg(feature = "model701")]
        {
            assert_impl_clone::<model701::Model701>();
            assert_impl_partial_eq::<model701::Model701>();
            assert_impl_eq::<model701::Model701>();
        }
        #[cfg(feature = "model702")]
        {
            assert_impl_clone::<model702::Model702>();
            assert_impl_partial_eq::<model702::Model702>();
            assert_impl_eq::<model702::Model702>();
        }
        #[cfg(feature = "model703")]
        {
            assert_impl_clone::<model703::Model703>();
            assert_impl_partial_eq::<model703::Model703>();
            assert_impl_eq::<model703::Model703>();
        }
        #[cfg(feature = "model704")]
        {
            assert_impl_clone::<model704::Model704>();
            assert_impl_partial_eq::<model704::Model704>();
            assert_impl_eq::<model704::Model704>();
        }
        #[cfg(feature = "model705")]
        {
            assert_impl_clone::<model705::Model705>();
            assert_impl_partial_eq::<model705::Model705>();
            assert_impl_eq::<model705::Model705>();
        }
        #[cfg(feature = "model706")]
        {
            assert_impl_clone::<model706::Model706>();
            assert_impl_partial_eq::<model706::Model706>();
            assert_impl_eq::<model706::Model706>();
        }
        #[cfg(feature = "model707")]
        {
            assert_impl_clone::<model707::Model707>();
            assert_impl_partial_eq::<model707::Model707>();
            assert_impl_eq::<model707::Model707>();
        }
        #[cfg(feature = "model708")]
        {
            assert_impl_clone::<model708::Model708>();
            assert_impl_partial_eq::<model708::Model708>();
            assert_impl_eq::<model708::Model708>();
        }
        #[cfg(feature = "model709")]
        {
            assert_impl_clone::<model709::Model709>();
            assert_impl_partial_eq::<model709::Model709>();
            assert_impl_eq::<model709::Model709>();
        }
        #[cfg(feature = "model710")]
        {
            assert_impl_clone::<model710::Model710>();
            assert_impl_partial_eq::<model710::Model710>();
            assert_impl_eq::<model710::Model710>();
        }
        #[cfg(feature = "model711")]
        {
            assert_impl_clone::<model711::Model711>();
            assert_impl_partial_eq::<model711::Model711>();
            assert_impl_eq::<model711::Model711>();
        }
        #[cfg(feature = "model712")]
        {
            assert_impl_clone::<model712::Model712>();
            assert_impl_partial_eq::<model712::Model712>();
            assert_impl_eq::<model712::Model712>();
        }
        #[cfg(feature = "model713")]
        {
            assert_impl_clone::<model713::Model713>();
            assert_impl_partial_eq::<model713::Model713>();
            assert_impl_eq::<model713::Model713>();
        }
        #[cfg(feature = "model714")]
        {
            assert_impl_clone::<model714::Model714>();
            assert_impl_partial_eq::<model714::Model714>();
            assert_impl_eq::<model714::Model714>();
        }
        #[cfg(feature = "model715")]
        {
            assert_impl_clone::<model715::Model715>();
            assert_impl_partial_eq::<model715::Model715>();
            assert_impl_eq::<model715::Model715>();
        }
        #[cfg(feature = "model801")]
        {
            assert_impl_clone::<model801::Model801>();
            assert_impl_partial_eq::<model801::Model801>();
            assert_impl_eq::<model801::Model801>();
        }
        #[cfg(feature = "model802")]
        {
            assert_impl_clone::<model802::Model802>();
            assert_impl_partial_eq::<model802::Model802>();
            assert_impl_eq::<model802::Model802>();
        }
        #[cfg(feature = "model803")]
        {
            assert_impl_clone::<model803::Model803>();
            assert_impl_partial_eq::<model803::Model803>();
            assert_impl_eq::<model803::Model803>();
        }
        #[cfg(feature = "model804")]
        {
            assert_impl_clone::<model804::Model804>();
            assert_impl_partial_eq::<model804::Model804>();
            assert_impl_eq::<model804::Model804>();
        }
        #[cfg(feature = "model805")]
        {
            assert_impl_clone::<model805::Model805>();
            assert_impl_partial_eq::<model805::Model805>();
            assert_impl_eq::<model805::Model805>();
        }
        #[cfg(feature = "model806")]
        {
            assert_impl_clone::<model806::Model806>();
            assert_impl_partial_eq::<model806::Model806>();
            assert_impl_eq::<model806::Model806>();
        }
        #[cfg(feature = "model807")]
        {
            assert_impl_clone::<model807::Model807>();
            assert_impl_partial_eq::<model807::Model807>();
            assert_impl_eq::<model807::Model807>();
        }
        #[cfg(feature = "model808")]
        {
            assert_impl_clone::<model808::Model808>();
            assert_impl_partial_eq::<model808::Model808>();
            assert_impl_eq::<model808::Model808>();
        }
        #[cfg(feature = "model809")]
        {
            assert_impl_clone::<model809::Model809>();
            assert_impl_partial_eq::<model809::Model809>();
            assert_impl_eq::<model809::Model809>();
        }
        #[cfg(feature = "model63001")]
        {
            assert_impl_clone::<model63001::Model63001>();
            assert_impl_partial_eq::<model63001::Model63001>();
        }
        #[cfg(feature = "model63002")]
        {
            assert_impl_clone::<model63002::Model63002>();
            assert_impl_partial_eq::<model63002::Model63002>();
            assert_impl_eq::<model63002::Model63002>();
        }
        #[cfg(feature = "model64001")]
        {
            assert_impl_clone::<model64001::Model64001>();
            assert_impl_partial_eq::<model64001::Model64001>();
            assert_impl_eq::<model64001::Model64001>();
        }
        #[cfg(feature = "model64020")]
        {
            assert_impl_clone::<model64020::Model64020>();
            assert_impl_partial_eq::<model64020::Model64020>();
            assert_impl_eq::<model64020::Model64020>();
        }
        #[cfg(feature = "model64101")]
        {
            assert_impl_clone::<model64101::Model64101>();
            assert_impl_partial_eq::<model64101::Model64101>();
            assert_impl_eq::<model64101::Model64101>();
        }
        #[cfg(feature = "model64111")]
        {
            assert_impl_clone::<model64111::Model64111>();
            assert_impl_partial_eq::<model64111::Model64111>();
            assert_impl_eq::<model64111::Model64111>();
        }
        #[cfg(feature = "model64112")]
        {
            assert_impl_clone::<model64112::Model64112>();
            assert_impl_partial_eq::<model64112::Model64112>();
            assert_impl_eq::<model64112::Model64112>();
        }
        #[cfg(feature = "model64410")]
        {
            assert_impl_clone::<model64410::Model64410>();
            assert_impl_partial_eq::<model64410::Model64410>();
            assert_impl_eq::<model64410::Model64410>();
        }
        #[cfg(feature = "model64411")]
        {
            assert_impl_clone::<model64411::Model64411>();
            assert_impl_partial_eq::<model64411::Model64411>();
            assert_impl_eq::<model64411::Model64411>();
        }
        #[cfg(feature = "model64412")]
        {
            assert_impl_clone::<model64412::Model64412>();
            assert_impl_partial_eq::<model64412::Model64412>();
            assert_impl_eq::<model64412::Model64412>();
        }
        #[cfg(feature = "model64413")]
        {
            assert_impl_clone::<model64413::Model64413>();
            assert_impl_partial_eq::<model64413::Model64413>();
        }
        #[cfg(feature = "model64414")]
        {
            assert_impl_clone::<model64414::Model64414>();
            assert_impl_partial_eq::<model64414::Model64414>();
        }
        #[cfg(feature = "model64415")]
        {
            assert_impl_clone::<model64415::Model64415>();
            assert_impl_partial_eq::<model64415::Model64415>();
            assert_impl_eq::<model64415::Model64415>();
        }
    }
}
