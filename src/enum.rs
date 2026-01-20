/// Represents supported languages.
///
/// Each variant corresponds to a specific language and locale combination.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {
    /// English (United States).
    EnUsUtf8,
    /// Chinese (China).
    #[default]
    ZhCnUtf8,
    /// French (France).
    FrFrUtf8,
    /// German (Germany).
    DeDeUtf8,
    /// Spanish (Spain).
    EsEsUtf8,
    /// Italian (Italy).
    ItItUtf8,
    /// Japanese (Japan).
    JaJpUtf8,
    /// Korean (South Korea).
    KoKrUtf8,
    /// Portuguese (Portugal).
    PtPtUtf8,
    /// Russian (Russia).
    RuRuUtf8,
    /// Arabic (Saudi Arabia).
    ArSaUtf8,
    /// Hindi (India).
    HiInUtf8,
    /// Thai (Thailand).
    ThThUtf8,
    /// Vietnamese (Vietnam).
    ViVnUtf8,
    /// Dutch (Netherlands).
    NlNlUtf8,
    /// Swedish (Sweden).
    SvSeUtf8,
    /// Finnish (Finland).
    FiFiUtf8,
}
