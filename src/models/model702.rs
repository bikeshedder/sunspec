/// DER Capacity
///
/// DER capacity model.
#[derive(Debug)]
pub struct Model702 {
    /// Active Power Max Rating
    ///
    /// Maximum active power rating at unity power factor in watts.
    ///
    /// Comments: Nameplate Ratings - Specifies capacity ratings
    pub wmaxrtg: Option<u16>,
    /// Active Power (Over-Excited) Rating
    ///
    /// Active power rating at specified over-excited power factor in watts.
    pub wovrextrtg: Option<u16>,
    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    pub wovrextrtgpf: Option<u16>,
    /// Active Power (Under-Excited) Rating
    ///
    /// Active power rating at specified under-excited power factor in watts.
    pub wundextrtg: Option<u16>,
    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    pub wundextrtgpf: Option<u16>,
    /// Apparent Power Max Rating
    ///
    /// Maximum apparent power rating in voltamperes.
    pub vamaxrtg: Option<u16>,
    /// Reactive Power Injected Rating
    ///
    /// Maximum injected reactive power rating in vars.
    pub varmaxinjrtg: Option<u16>,
    /// Reactive Power Absorbed Rating
    ///
    /// Maximum absorbed reactive power rating in vars.
    pub varmaxabsrtg: Option<u16>,
    /// Charge Rate Max Rating
    ///
    /// Maximum active power charge rate in watts.
    pub wchartemaxrtg: Option<u16>,
    /// Discharge Rate Max Rating
    ///
    /// Maximum active power discharge rate in watts.
    pub wdischartemaxrtg: Option<u16>,
    /// Charge Rate Max VA Rating
    ///
    /// Maximum apparent power charge rate in voltamperes.
    pub vachartemaxrtg: Option<u16>,
    /// Discharge Rate Max VA Rating
    ///
    /// Maximum apparent power discharge rate in voltamperes.
    pub vadischartemaxrtg: Option<u16>,
    /// AC Voltage Nominal Rating
    ///
    /// AC voltage nominal rating.
    pub vnomrtg: Option<u16>,
    /// AC Voltage Max Rating
    ///
    /// AC voltage maximum rating.
    pub vmaxrtg: Option<u16>,
    /// AC Voltage Min Rating
    ///
    /// AC voltage minimum rating.
    pub vminrtg: Option<u16>,
    /// AC Current Max Rating
    ///
    /// AC current maximum rating in amps.
    pub amaxrtg: Option<u16>,
    /// PF Over-Excited Rating
    ///
    /// Power factor over-excited rating.
    pub pfovrextrtg: Option<u16>,
    /// PF Under-Excited Rating
    ///
    /// Power factor under-excited rating.
    pub pfundextrtg: Option<u16>,
    /// Reactive Susceptance
    ///
    /// Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state.
    pub reactsusceptrtg: Option<u16>,
    /// Normal Operating Category
    ///
    /// Normal operating performance category as specified in IEEE 1547-2018.
    pub noropcatrtg: Option<u16>,
    /// Abnormal Operating Category
    ///
    /// Abnormal operating performance category as specified in IEEE 1547-2018.
    pub abnopcatrtg: Option<u16>,
    /// Supported Control Modes
    ///
    /// Supported control mode functions.
    pub ctrlmodes: Option<u32>,
    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    pub intislandcatrtg: Option<u16>,
    /// Active Power Max Setting
    ///
    /// Maximum active power setting used to adjust maximum active power setting.
    ///
    /// Comments: Settings - Used to adjust nameplate ratings
    pub wmax: Option<u16>,
    /// Active Power (Over-Excited) Setting
    ///
    /// Active power setting at specified over-excited power factor in watts.
    pub wmaxovrext: Option<u16>,
    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    pub wovrextpf: Option<u16>,
    /// Active Power (Under-Excited) Setting
    ///
    /// Active power setting at specified under-excited power factor in watts.
    pub wmaxundext: Option<u16>,
    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    pub wundextpf: Option<u16>,
    /// Apparent Power Max Setting
    ///
    /// Maximum apparent power setting used to adjust maximum apparent power rating.
    pub vamax: Option<u16>,
    /// Reactive Power Injected Setting
    ///
    /// Maximum injected reactive power setting used to adjust maximum injected reactive power rating.
    pub varmaxinj: Option<u16>,
    /// Reactive Power Absorbed Setting
    ///
    /// Maximum absorbed reactive power setting used to adjust maximum absorbed reactive power rating.
    pub varmaxabs: Option<u16>,
    /// Charge Rate Max Setting
    ///
    /// Maximum active power charge rate setting used to adjust maximum active power charge rate rating.
    pub wchartemax: Option<u16>,
    /// Discharge Rate Max Setting
    ///
    /// Maximum active power discharge rate setting used to adjust maximum active power discharge rate rating.
    pub wdischartemax: Option<u16>,
    /// Charge Rate Max VA Setting
    ///
    /// Maximum apparent power charge rate setting used to adjust maximum apparent power charge rate rating.
    pub vachartemax: Option<u16>,
    /// Discharge Rate Max VA Setting
    ///
    /// Maximum apparent power discharge rate setting used to adjust maximum apparent power discharge rate rating.
    pub vadischartemax: Option<u16>,
    /// Nominal AC Voltage Setting
    ///
    /// Nominal AC voltage setting.
    pub vnom: Option<u16>,
    /// AC Voltage Max Setting
    ///
    /// AC voltage maximum setting used to adjust AC voltage maximum rating.
    pub vmax: Option<u16>,
    /// AC Voltage Min Setting
    ///
    /// AC voltage minimum setting used to adjust AC voltage minimum rating.
    pub vmin: Option<u16>,
    /// AC Current Max Setting
    ///
    /// Maximum AC current setting used to adjust maximum AC current rating.
    pub amax: Option<u16>,
    /// PF Over-Excited Setting
    ///
    /// Power factor over-excited setting.
    pub pfovrext: Option<u16>,
    /// PF Under-Excited Setting
    ///
    /// Power factor under-excited setting.
    pub pfundext: Option<u16>,
    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    pub intislandcat: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    ///
    /// Comments: Scale Factors
    pub w_sf: Option<i16>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    pub pf_sf: Option<i16>,
    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    pub va_sf: Option<i16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    pub var_sf: Option<i16>,
    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    pub v_sf: Option<i16>,
    /// Current Scale Factor
    ///
    /// Current scale factor.
    pub a_sf: Option<i16>,
    /// Susceptance Scale Factor
    ///
    /// Susceptance scale factor.
    pub s_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model702 {
    pub const WMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const WOVREXTRTG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const WOVREXTRTGPF: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const WUNDEXTRTG: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const WUNDEXTRTGPF: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const VAMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const VARMAXINJRTG: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const VARMAXABSRTG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const WCHARTEMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const WDISCHARTEMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const VACHARTEMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const VADISCHARTEMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const VNOMRTG: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const VMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const VMINRTG: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const AMAXRTG: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const PFOVREXTRTG: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const PFUNDEXTRTG: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const REACTSUSCEPTRTG: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, false);
    pub const NOROPCATRTG: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const ABNOPCATRTG: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const CTRLMODES: crate::PointDef<Self, u32> = crate::PointDef::new(21, 2, false);
    pub const INTISLANDCATRTG: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, false);
    pub const WMAX: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, true);
    pub const WMAXOVREXT: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, true);
    pub const WOVREXTPF: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, true);
    pub const WMAXUNDEXT: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, true);
    pub const WUNDEXTPF: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, true);
    pub const VAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, true);
    pub const VARMAXINJ: crate::PointDef<Self, u16> = crate::PointDef::new(30, 1, true);
    pub const VARMAXABS: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, true);
    pub const WCHARTEMAX: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, true);
    pub const WDISCHARTEMAX: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, true);
    pub const VACHARTEMAX: crate::PointDef<Self, u16> = crate::PointDef::new(34, 1, true);
    pub const VADISCHARTEMAX: crate::PointDef<Self, u16> = crate::PointDef::new(35, 1, true);
    pub const VNOM: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, true);
    pub const VMAX: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, true);
    pub const VMIN: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, true);
    pub const AMAX: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, true);
    pub const PFOVREXT: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, true);
    pub const PFUNDEXT: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, true);
    pub const INTISLANDCAT: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, true);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(43, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(44, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(45, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(46, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(47, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(48, 1, false);
    pub const S_SF: crate::PointDef<Self, i16> = crate::PointDef::new(49, 1, false);
}

impl crate::Model for Model702 {
    const ID: u16 = 702;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wmaxrtg: Self::WMAXRTG.from_data(data)?,
            wovrextrtg: Self::WOVREXTRTG.from_data(data)?,
            wovrextrtgpf: Self::WOVREXTRTGPF.from_data(data)?,
            wundextrtg: Self::WUNDEXTRTG.from_data(data)?,
            wundextrtgpf: Self::WUNDEXTRTGPF.from_data(data)?,
            vamaxrtg: Self::VAMAXRTG.from_data(data)?,
            varmaxinjrtg: Self::VARMAXINJRTG.from_data(data)?,
            varmaxabsrtg: Self::VARMAXABSRTG.from_data(data)?,
            wchartemaxrtg: Self::WCHARTEMAXRTG.from_data(data)?,
            wdischartemaxrtg: Self::WDISCHARTEMAXRTG.from_data(data)?,
            vachartemaxrtg: Self::VACHARTEMAXRTG.from_data(data)?,
            vadischartemaxrtg: Self::VADISCHARTEMAXRTG.from_data(data)?,
            vnomrtg: Self::VNOMRTG.from_data(data)?,
            vmaxrtg: Self::VMAXRTG.from_data(data)?,
            vminrtg: Self::VMINRTG.from_data(data)?,
            amaxrtg: Self::AMAXRTG.from_data(data)?,
            pfovrextrtg: Self::PFOVREXTRTG.from_data(data)?,
            pfundextrtg: Self::PFUNDEXTRTG.from_data(data)?,
            reactsusceptrtg: Self::REACTSUSCEPTRTG.from_data(data)?,
            noropcatrtg: Self::NOROPCATRTG.from_data(data)?,
            abnopcatrtg: Self::ABNOPCATRTG.from_data(data)?,
            ctrlmodes: Self::CTRLMODES.from_data(data)?,
            intislandcatrtg: Self::INTISLANDCATRTG.from_data(data)?,
            wmax: Self::WMAX.from_data(data)?,
            wmaxovrext: Self::WMAXOVREXT.from_data(data)?,
            wovrextpf: Self::WOVREXTPF.from_data(data)?,
            wmaxundext: Self::WMAXUNDEXT.from_data(data)?,
            wundextpf: Self::WUNDEXTPF.from_data(data)?,
            vamax: Self::VAMAX.from_data(data)?,
            varmaxinj: Self::VARMAXINJ.from_data(data)?,
            varmaxabs: Self::VARMAXABS.from_data(data)?,
            wchartemax: Self::WCHARTEMAX.from_data(data)?,
            wdischartemax: Self::WDISCHARTEMAX.from_data(data)?,
            vachartemax: Self::VACHARTEMAX.from_data(data)?,
            vadischartemax: Self::VADISCHARTEMAX.from_data(data)?,
            vnom: Self::VNOM.from_data(data)?,
            vmax: Self::VMAX.from_data(data)?,
            vmin: Self::VMIN.from_data(data)?,
            amax: Self::AMAX.from_data(data)?,
            pfovrext: Self::PFOVREXT.from_data(data)?,
            pfundext: Self::PFUNDEXT.from_data(data)?,
            intislandcat: Self::INTISLANDCAT.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            s_sf: Self::S_SF.from_data(data)?,
        })
    }
}
