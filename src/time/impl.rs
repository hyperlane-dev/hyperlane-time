use crate::*;

/// Implementation of Display trait for Lang.
///
/// Provides a human-readable string representation for each language variant.
impl fmt::Display for Lang {
    /// Formats the language for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang_str: &str = match self {
            Lang::EnUsUtf8 => "English (US)",
            Lang::ZhCnUtf8 => "中文 (中国)",
            Lang::FrFrUtf8 => "Français (France)",
            Lang::DeDeUtf8 => "Deutsch (Deutschland)",
            Lang::EsEsUtf8 => "Español (España)",
            Lang::ItItUtf8 => "Italiano (Italia)",
            Lang::JaJpUtf8 => "日本語 (日本)",
            Lang::KoKrUtf8 => "한국어 (한국)",
            Lang::PtPtUtf8 => "Português (Portugal)",
            Lang::RuRuUtf8 => "Русский (Россия)",
            Lang::ArSaUtf8 => "العربية (السعودية)",
            Lang::HiInUtf8 => "हिन्दी (भारत)",
            Lang::ThThUtf8 => "ภาษาไทย (ประเทศไทย)",
            Lang::ViVnUtf8 => "Tiếng Việt (Việt Nam)",
            Lang::NlNlUtf8 => "Nederlands (Nederland)",
            Lang::SvSeUtf8 => "Svenska (Sverige)",
            Lang::FiFiUtf8 => "Suomi (Suomi)",
        };
        write!(f, "{lang_str}")
    }
}
impl Lang {
    /// Returns the UTC offset in seconds for the corresponding language.
    ///
    /// Each language is associated with a specific UTC offset,
    /// indicating the difference from Coordinated Universal Time (UTC).
    ///
    /// # Returns
    ///
    /// - `u64` - The UTC offset in seconds.
    pub fn value(&self) -> u64 {
        match self {
            Lang::EnUsUtf8 => 0,     // UTC
            Lang::ZhCnUtf8 => 28800, // UTC+8
            Lang::FrFrUtf8 => 3600,  // UTC+1
            Lang::DeDeUtf8 => 3600,  // UTC+1
            Lang::EsEsUtf8 => 3600,  // UTC+1
            Lang::ItItUtf8 => 3600,  // UTC+1
            Lang::JaJpUtf8 => 32400, // UTC+9
            Lang::KoKrUtf8 => 32400, // UTC+9
            Lang::PtPtUtf8 => 3600,  // UTC+1
            Lang::RuRuUtf8 => 10800, // UTC+3
            Lang::ArSaUtf8 => 10800, // UTC+3
            Lang::HiInUtf8 => 19800, // UTC+5:30
            Lang::ThThUtf8 => 25200, // UTC+7
            Lang::ViVnUtf8 => 25200, // UTC+7
            Lang::NlNlUtf8 => 3600,  // UTC+1
            Lang::SvSeUtf8 => 3600,  // UTC+1
            Lang::FiFiUtf8 => 3600,  // UTC+1
        }
    }
}

/// Implementation of FromStr trait for Lang.
///
/// Allows parsing a string into a Lang variant.
impl FromStr for Lang {
    /// The error type for parsing operations.
    type Err = ();

    /// Parses a string into a Lang variant.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The parsed Lang variant or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en_US.UTF-8" => Ok(Lang::EnUsUtf8),
            "zh_CN.UTF-8" => Ok(Lang::ZhCnUtf8),
            "fr_FR.UTF-8" => Ok(Lang::FrFrUtf8),
            "de_DE.UTF-8" => Ok(Lang::DeDeUtf8),
            "es_ES.UTF-8" => Ok(Lang::EsEsUtf8),
            "it_IT.UTF-8" => Ok(Lang::ItItUtf8),
            "ja_JP.UTF-8" => Ok(Lang::JaJpUtf8),
            "ko_KR.UTF-8" => Ok(Lang::KoKrUtf8),
            "pt_PT.UTF-8" => Ok(Lang::PtPtUtf8),
            "ru_RU.UTF-8" => Ok(Lang::RuRuUtf8),
            "ar_SA.UTF-8" => Ok(Lang::ArSaUtf8),
            "hi_IN.UTF-8" => Ok(Lang::HiInUtf8),
            "th_TH.UTF-8" => Ok(Lang::ThThUtf8),
            "vi_VN.UTF-8" => Ok(Lang::ViVnUtf8),
            "nl_NL.UTF-8" => Ok(Lang::NlNlUtf8),
            "sv_SE.UTF-8" => Ok(Lang::SvSeUtf8),
            "fi_FI.UTF-8" => Ok(Lang::FiFiUtf8),
            _ => Err(()),
        }
    }
}
