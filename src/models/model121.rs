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
    pub const VMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const VMIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const VAMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const VARMAXQ1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(6, 1, true);
    pub const VARMAXQ2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(7, 1, true);
    pub const VARMAXQ3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(8, 1, true);
    pub const VARMAXQ4: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(9, 1, true);
    pub const WGRA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, true);
    pub const PFMINQ1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(11, 1, true);
    pub const PFMINQ2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(12, 1, true);
    pub const PFMINQ3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(13, 1, true);
    pub const PFMINQ4: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(14, 1, true);
    pub const VARACT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(15, 1, true);
    pub const CLCTOTVA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, true);
    pub const MAXRMPRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, true);
    pub const ECPNOMHZ: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(18, 1, true);
    pub const CONNPH: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(19, 1, true);
    pub const WMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VREFOFS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VMINMAX_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
    pub const VAMAX_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(24, 1, false);
    pub const VARMAX_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(25, 1, false);
    pub const WGRA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(26, 1, false);
    pub const PFMIN_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(27, 1, false);
    pub const MAXRMPRTE_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(28, 1, false);
    pub const ECPNOMHZ_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(29, 1, false);
}

impl crate::Model for Model121 {
    const ID: u16 = 121;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wmax: Self::WMAX.from_data(data)?,
            vref: Self::VREF.from_data(data)?,
            vrefofs: Self::VREFOFS.from_data(data)?,
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
            wmax_sf: Self::WMAX_SF.from_data(data)?,
            vref_sf: Self::VREF_SF.from_data(data)?,
            vrefofs_sf: Self::VREFOFS_SF.from_data(data)?,
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
