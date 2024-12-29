use std::{env, fmt, str::FromStr};

/// Represents supported languages.
#[derive(Debug, Clone, PartialEq)]
pub enum Lang {
    /// English (United States),
    EnUsUtf8,
    /// Chinese (China),
    ZhCnUtf8,
    /// French (France),
    FrFrUtf8,
    /// German (Germany),
    DeDeUtf8,
    /// Spanish (Spain),
    EsEsUtf8,
    /// Italian (Italy),
    ItItUtf8,
    /// Japanese (Japan),
    JaJpUtf8,
    /// Korean (South Korea),
    KoKrUtf8,
    /// Portuguese (Portugal),
    PtPtUtf8,
    /// Russian (Russia),
    RuRuUtf8,
    /// Arabic (Saudi Arabia),
    ArSaUtf8,
    /// Hindi (India),
    HiInUtf8,
    /// Thai (Thailand),
    ThThUtf8,
    /// Vietnamese (Vietnam),
    ViVnUtf8,
    /// Dutch (Netherlands),
    NlNlUtf8,
    /// Swedish (Sweden),
    SvSeUtf8,
    /// Finnish (Finland),
    FiFiUtf8,
}

impl fmt::Display for Lang {
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
        write!(f, "{}", lang_str)
    }
}

impl Lang {
    /// Returns the UTC offset in seconds for the corresponding language.
    ///
    /// Each language is associated with a specific UTC offset,
    /// indicating the difference from Coordinated Universal Time (UTC).
    ///
    /// # Returns
    /// - `u64`: The UTC offset in seconds. For example,
    ///   - `0` for UTC (English),
    ///   - `28800` for UTC+8 (Chinese),
    ///   - `3600` for UTC+1 (French, German, Spanish, etc.),
    ///   - `32400` for UTC+9 (Japanese, Korean),
    ///   - `10800` for UTC+3 (Russian, Arabic),
    ///   - `19800` for UTC+5:30 (Hindi),
    ///   - `25200` for UTC+7 (Thai, Vietnamese),
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

impl Default for Lang {
    fn default() -> Self {
        Lang::ZhCnUtf8
    }
}

impl FromStr for Lang {
    type Err = ();
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

/// Gets the time zone offset from the system environment variable.
///
/// This function retrieves the `LANG` environment variable and attempts to
/// parse it into a `Lang` value. If the variable is not set or cannot be
/// parsed, it defaults to `Lang::EnUsUtf8`.
///
/// # Returns
/// - `Lang`: The corresponding `Lang` value based on the `LANG` environment variable.
pub fn from_env_var() -> Lang {
    let lang: Lang = env::var("LANG")
        .unwrap_or_default()
        .parse::<Lang>()
        .unwrap_or_default();
    lang
}
