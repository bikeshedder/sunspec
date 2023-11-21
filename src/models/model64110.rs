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
    pub tcpip_address: std::net::Ipv4Addr,
    /// TCPIP Gateway
    pub gateway_address: std::net::Ipv4Addr,
    /// TCPIP Netmask
    pub tcpip_netmask: std::net::Ipv4Addr,
    /// TCPIP DNS1
    pub dns1_address: std::net::Ipv4Addr,
    /// TCPIP DNS2
    pub dns2_address: std::net::Ipv4Addr,
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
    pub const TCPIP_ADDRESS: crate::PointDef<Self, std::net::Ipv4Addr> =
        crate::PointDef::new(20, 2, false);
    pub const GATEWAY_ADDRESS: crate::PointDef<Self, std::net::Ipv4Addr> =
        crate::PointDef::new(22, 2, false);
    pub const TCPIP_NETMASK: crate::PointDef<Self, std::net::Ipv4Addr> =
        crate::PointDef::new(24, 2, false);
    pub const DNS1_ADDRESS: crate::PointDef<Self, std::net::Ipv4Addr> =
        crate::PointDef::new(26, 2, false);
    pub const DNS2_ADDRESS: crate::PointDef<Self, std::net::Ipv4Addr> =
        crate::PointDef::new(28, 2, false);
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
