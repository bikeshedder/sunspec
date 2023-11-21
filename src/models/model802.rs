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
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
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
    pub const DISCHARTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const SOCMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const SOCMIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const SOCRSVMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, true);
    pub const SOCRSVMIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, true);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const DOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const SOH: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(11, 1, false);
    pub const NCYC: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(12, 2, false);
    pub const CHAST: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const LOCREMCTL: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const HB: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, false);
    pub const CTRLHB: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, true);
    pub const ALMRST: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const STATE: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const STATEVND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(21, 1, false);
    pub const WARRDT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(22, 2, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(30, 2, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, false);
    pub const VMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(33, 1, false);
    pub const VMIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(34, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(35, 1, false);
    pub const CELLVMAXSTR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(36, 1, false);
    pub const CELLVMAXMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(37, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(38, 1, false);
    pub const CELLVMINSTR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(39, 1, false);
    pub const CELLVMINMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(40, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(41, 1, false);
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(42, 1, false);
    pub const ACHAMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(43, 1, false);
    pub const ADISCHAMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(44, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(45, 1, false);
    pub const REQINVSTATE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(46, 1, false);
    pub const REQW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(47, 1, false);
    pub const SETOP: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const SETINVSTATE: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const AHRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(50, 1, false);
    pub const WHRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(51, 1, false);
    pub const WCHADISCHAMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const DISCHARTE_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(53, 1, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(54, 1, false);
    pub const DOD_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(55, 1, false);
    pub const SOH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(56, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(57, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(58, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(59, 1, false);
    pub const AMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(60, 1, false);
    pub const W_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(61, 1, false);
}

impl crate::Model for Model802 {
    const ID: u16 = 802;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ahrtg: Self::AHRTG.from_data(data)?,
            whrtg: Self::WHRTG.from_data(data)?,
            wchartemax: Self::WCHARTEMAX.from_data(data)?,
            wdischartemax: Self::WDISCHARTEMAX.from_data(data)?,
            discharte: Self::DISCHARTE.from_data(data)?,
            socmax: Self::SOCMAX.from_data(data)?,
            socmin: Self::SOCMIN.from_data(data)?,
            socrsvmax: Self::SOCRSVMAX.from_data(data)?,
            socrsvmin: Self::SOCRSVMIN.from_data(data)?,
            soc: Self::SOC.from_data(data)?,
            dod: Self::DOD.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            ncyc: Self::NCYC.from_data(data)?,
            chast: Self::CHAST.from_data(data)?,
            locremctl: Self::LOCREMCTL.from_data(data)?,
            hb: Self::HB.from_data(data)?,
            ctrlhb: Self::CTRLHB.from_data(data)?,
            almrst: Self::ALMRST.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
            state: Self::STATE.from_data(data)?,
            statevnd: Self::STATEVND.from_data(data)?,
            warrdt: Self::WARRDT.from_data(data)?,
            evt1: Self::EVT1.from_data(data)?,
            evt2: Self::EVT2.from_data(data)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            v: Self::V.from_data(data)?,
            vmax: Self::VMAX.from_data(data)?,
            vmin: Self::VMIN.from_data(data)?,
            cellvmax: Self::CELLVMAX.from_data(data)?,
            cellvmaxstr: Self::CELLVMAXSTR.from_data(data)?,
            cellvmaxmod: Self::CELLVMAXMOD.from_data(data)?,
            cellvmin: Self::CELLVMIN.from_data(data)?,
            cellvminstr: Self::CELLVMINSTR.from_data(data)?,
            cellvminmod: Self::CELLVMINMOD.from_data(data)?,
            cellvavg: Self::CELLVAVG.from_data(data)?,
            a: Self::A.from_data(data)?,
            achamax: Self::ACHAMAX.from_data(data)?,
            adischamax: Self::ADISCHAMAX.from_data(data)?,
            w: Self::W.from_data(data)?,
            reqinvstate: Self::REQINVSTATE.from_data(data)?,
            reqw: Self::REQW.from_data(data)?,
            setop: Self::SETOP.from_data(data)?,
            setinvstate: Self::SETINVSTATE.from_data(data)?,
            ahrtg_sf: Self::AHRTG_SF.from_data(data)?,
            whrtg_sf: Self::WHRTG_SF.from_data(data)?,
            wchadischamax_sf: Self::WCHADISCHAMAX_SF.from_data(data)?,
            discharte_sf: Self::DISCHARTE_SF.from_data(data)?,
            soc_sf: Self::SOC_SF.from_data(data)?,
            dod_sf: Self::DOD_SF.from_data(data)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            cellv_sf: Self::CELLV_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            amax_sf: Self::AMAX_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
        })
    }
}
