//! Conversion of SunSpec point and group names into Rust identifiers.
//!
//! SunSpec names are camel case, but full of unit and acronym abbreviations
//! (`VAr`, `PF`, `DC`, `phA`, `L1`, ...) which a generic case converter
//! splits at the wrong places (`VArRtgQ1` would become `v_ar_rtg_q1`).
//! This module first splits a name into words using the tables and rules
//! below and then renders these words in the requested case.
//!
//! A name is scanned from left to right:
//!
//! 1. [`EXCEPTIONS`] and [`SPELLINGS`] are matched by their exact spelling
//!    and become the listed words. Digits directly following them stay
//!    attached to the last word (`VAr1` → `var1`).
//! 2. `ph` followed by one of the [`PHASES`] becomes a word of its own
//!    (`WphA` → `w_ph_a`).
//! 3. Everything else is split like regular camel case, with one addition:
//!    a digit followed by a capital letter also starts a new word
//!    (`S1ID` → `s1_id`).
//! 4. Runs of capitals (e.g. `DCA`, `PPV`, `VL1L2`) are split into
//!    [`CAPS_ATOMS`] if the run can be built entirely from them. Otherwise
//!    the run is kept as a single word (`AVAIL`, `PPT`, `AID`).
//!
//! These rules are meant for point and group names only. Enum and bitfield
//! symbols are all caps and already separated by underscores, so
//! [`symbol_upper_camel_case`] only applies the [`SPELLINGS`].

/// Abbreviations with a fixed mixed-case spelling.
///
/// They only match if they are not followed by a lowercase letter.
const SPELLINGS: &[(&str, &[&str])] = &[
    ("VArh", &["varh"]), // reactive energy
    ("VAr", &["var"]),   // reactive power
    ("VAh", &["vah"]),   // apparent energy
    ("kWh", &["kwh"]),   // kilowatt-hours
    ("SoC", &["soc"]),   // state of charge
    ("SoH", &["soh"]),   // state of health
    ("DoD", &["dod"]),   // depth of discharge
];

/// Spellings found in vendor models which no general rule can split.
const EXCEPTIONS: &[(&str, &[&str])] = &[
    ("LEDblink", &["led", "blink"]),
    ("LEDon", &["led", "on"]),
    ("Varmaxabs", &["var", "max", "abs"]),
    ("Varmaxinj", &["var", "max", "inj"]),
];

/// All-caps abbreviations a run of capitals may be split into.
const CAPS_ATOMS: &[&str] = &[
    "V", "A", "W", // volt, ampere, watt
    "VA", "WH", // volt-ampere, watt-hours
    "PF", // power factor
    "DC", // direct current
    "PR", // performance ratio
    "FW", "OS", "RS", // firmware, operating system, RS
    "PP", "LL", "LN", // phase-to-phase, line-to-line, line-to-neutral
    "ES", // enter service
    "L1", "L2", "L3", // lines
];

/// Phases following a `ph` prefix. Longer phases must come first.
const PHASES: &[&str] = &["AB", "BC", "CA", "A", "B", "C"];

/// Convert a point or group name to `snake_case`.
pub fn snake_case(name: &str) -> String {
    words(name).join("_")
}

/// Convert a point or group name to `SHOUTY_SNAKE_CASE`.
pub fn shouty_snake_case(name: &str) -> String {
    snake_case(name).to_ascii_uppercase()
}

/// Convert a point or group name to `UpperCamelCase`.
pub fn upper_camel_case(name: &str) -> String {
    words(name).iter().map(|word| capitalize(word)).collect()
}

/// Convert an enum or bitfield symbol name to `UpperCamelCase`.
pub fn symbol_upper_camel_case(name: &str) -> String {
    use heck::ToUpperCamelCase;
    let mut normalized = String::new();
    let mut rest = name;
    while let Some(c) = rest.chars().next() {
        let consumed = match match_spelling(SPELLINGS, rest) {
            Some((len, words)) => {
                normalized.extend(words.iter().map(|word| capitalize(word)));
                len
            }
            None => {
                normalized.push(c);
                c.len_utf8()
            }
        };
        rest = &rest[consumed..];
    }
    normalized.to_upper_camel_case()
}

/// Split a SunSpec name into lowercase words.
pub fn words(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut camel = String::new();
    let mut prev = None;
    let mut rest = name;
    while let Some(c) = rest.chars().next() {
        let consumed = if let Some((len, spelling)) =
            match_spelling(EXCEPTIONS, rest).or_else(|| match_spelling(SPELLINGS, rest))
        {
            let digits = count_digits(&rest[len..]);
            split_camel(&camel, &mut words);
            camel.clear();
            words.extend(spelling.iter().map(|word| word.to_string()));
            if let Some(last) = words.last_mut() {
                last.push_str(&rest[len..len + digits]);
            }
            len + digits
        } else if is_phase_prefix(rest, prev) {
            split_camel(&camel, &mut words);
            camel.clear();
            words.push("ph".into());
            2
        } else if c.is_ascii_alphanumeric() {
            camel.push(c);
            1
        } else {
            split_camel(&camel, &mut words);
            camel.clear();
            c.len_utf8()
        };
        prev = rest[..consumed].chars().last();
        rest = &rest[consumed..];
    }
    split_camel(&camel, &mut words);
    words
}

/// Find the longest spelling of `table` at the start of `s` and return
/// its length and words.
fn match_spelling(
    table: &'static [(&'static str, &'static [&'static str])],
    s: &str,
) -> Option<(usize, &'static [&'static str])> {
    table
        .iter()
        .filter(|(spelling, _)| s.starts_with(spelling))
        .filter(|(spelling, _)| !starts_with_lowercase(&s[spelling.len()..]))
        .max_by_key(|(spelling, _)| spelling.len())
        .map(|(spelling, words)| (spelling.len(), *words))
}

/// `ph` followed by a phase and preceded by a capital or digit.
fn is_phase_prefix(s: &str, prev: Option<char>) -> bool {
    let Some(after) = s.strip_prefix("ph") else {
        return false;
    };
    prev.is_some_and(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        && PHASES.iter().any(|phase| {
            after
                .strip_prefix(phase)
                .is_some_and(|rest| !starts_with_lowercase(rest))
        })
}

/// Split camel case text (letters and digits only) into words.
fn split_camel(text: &str, words: &mut Vec<String>) {
    let chars = text.as_bytes();
    let mut i = 0;
    while i < chars.len() {
        let mut j = i + 1;
        if chars[i].is_ascii_uppercase() && chars.get(j).is_some_and(u8::is_ascii_lowercase)
            || chars[i].is_ascii_lowercase()
        {
            // Capitalized or lowercase word, e.g. `Rtg`, `ph`, `addr`
            while j < chars.len() && (chars[j].is_ascii_lowercase() || chars[j].is_ascii_digit()) {
                j += 1;
            }
            words.push(text[i..j].to_ascii_lowercase());
        } else {
            // Run of capitals and digits, e.g. `DCA`, `VL1`, `Q3`
            while j < chars.len()
                && !chars[j].is_ascii_lowercase()
                && !(chars[j - 1].is_ascii_digit() && chars[j].is_ascii_uppercase())
                && !(chars[j].is_ascii_uppercase()
                    && chars.get(j + 1).is_some_and(u8::is_ascii_lowercase))
            {
                j += 1;
            }
            let run = &text[i..j];
            match split_caps(run) {
                Some(atoms) => words.extend(atoms.iter().map(|atom| atom.to_ascii_lowercase())),
                None => words.push(run.to_ascii_lowercase()),
            }
        }
        i = j;
    }
}

/// Split a run of capitals into the fewest possible [`CAPS_ATOMS`].
/// Returns `None` if the run can't be built from them entirely.
fn split_caps(run: &str) -> Option<Vec<&'static str>> {
    if run.is_empty() {
        return Some(Vec::new());
    }
    CAPS_ATOMS
        .iter()
        .filter(|atom| run.starts_with(**atom))
        .filter_map(|atom| {
            let mut atoms = split_caps(&run[atom.len()..])?;
            atoms.insert(0, *atom);
            Some(atoms)
        })
        .min_by_key(Vec::len)
}

fn count_digits(s: &str) -> usize {
    s.bytes().take_while(u8::is_ascii_digit).count()
}

fn starts_with_lowercase(s: &str) -> bool {
    s.bytes().next().is_some_and(|c| c.is_ascii_lowercase())
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spellings() {
        assert_eq!(snake_case("VArRtgQ1"), "var_rtg_q1");
        assert_eq!(snake_case("TotVArhImpQ1"), "tot_varh_imp_q1");
        assert_eq!(snake_case("TotVAhExp"), "tot_vah_exp");
        assert_eq!(snake_case("TodaykWhOutput"), "today_kwh_output");
        assert_eq!(snake_case("SoCMax"), "soc_max");
        assert_eq!(snake_case("DoD_SF"), "dod_sf");
        assert_eq!(snake_case("VAr1"), "var1");
        assert_eq!(snake_case("Volt-VAr"), "volt_var");
    }

    #[test]
    fn exceptions() {
        assert_eq!(snake_case("LEDblink"), "led_blink");
        assert_eq!(
            snake_case("SettingsHighVarmaxabs"),
            "settings_high_var_max_abs"
        );
    }

    #[test]
    fn phases() {
        assert_eq!(snake_case("AphA"), "a_ph_a");
        assert_eq!(snake_case("PhVphAB"), "ph_v_ph_ab");
        assert_eq!(snake_case("PFphA"), "pf_ph_a");
        assert_eq!(snake_case("VARphA"), "var_ph_a");
        assert_eq!(snake_case("TotVArhExpQ3phA"), "tot_varh_exp_q3_ph_a");
        assert_eq!(snake_case("TotVArhExpQ3PhA"), "tot_varh_exp_q3_ph_a");
    }

    #[test]
    fn caps_atoms() {
        assert_eq!(snake_case("DCA"), "dc_a");
        assert_eq!(snake_case("InDCAhr"), "in_dc_ahr");
        assert_eq!(snake_case("DCWH_SF"), "dc_wh_sf");
        assert_eq!(snake_case("PFWAbsEna"), "pf_w_abs_ena");
        assert_eq!(snake_case("VAL1"), "va_l1");
        assert_eq!(snake_case("VL1L2"), "v_l1_l2");
        assert_eq!(snake_case("OSFWRev"), "os_fw_rev");
        assert_eq!(snake_case("PPVphAB"), "pp_v_ph_ab");
        assert_eq!(snake_case("ESVHi"), "es_v_hi");
    }

    #[test]
    fn digit_boundary() {
        assert_eq!(snake_case("S1ID"), "s1_id");
        assert_eq!(snake_case("S1OSVer"), "s1_os_ver");
        assert_eq!(snake_case("MeasHighL1V"), "meas_high_l1_v");
    }

    #[test]
    fn unchanged() {
        for (name, expected) in [
            ("VARtg", "va_rtg"),
            ("VAR", "var"),
            ("VAR_SF", "var_sf"),
            ("CellVAvg", "cell_v_avg"),
            ("WMaxLimPct_RvrtTms", "w_max_lim_pct_rvrt_tms"),
            ("RmpPT1Tms", "rmp_pt1_tms"),
            ("PF1", "pf1"),
            ("PPT", "ppt"),
            ("AID", "aid"),
            ("IA", "ia"),
            ("VOCV", "vocv"),
            ("ipv6addr", "ipv6addr"),
            ("EN50530", "en50530"),
            ("COMM004Cert", "comm004_cert"),
            ("HWRev", "hw_rev"),
            ("ID", "id"),
        ] {
            assert_eq!(snake_case(name), expected, "{name}");
        }
    }

    #[test]
    fn cases() {
        assert_eq!(shouty_snake_case("VArRtgQ1"), "VAR_RTG_Q1");
        assert_eq!(upper_camel_case("VArRtgQ1"), "VarRtgQ1");
        assert_eq!(upper_camel_case("PFWAbs"), "PfWAbs");
        assert_eq!(symbol_upper_camel_case("VAR_AVAIL_PCT"), "VarAvailPct");
        assert_eq!(symbol_upper_camel_case("SoC_LOW"), "SocLow");
        assert_eq!(symbol_upper_camel_case("Volt-VAr"), "VoltVar");
    }
}
