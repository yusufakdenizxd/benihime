#[derive(Debug, Eq, PartialEq)]
pub enum CharCategory {
    Whitespace,
    Eol,
    Word,
    Punctuation,
    Unknown,
}

pub fn char_is_line_ending(ch: char) -> bool {
    match ch {
        '\u{000A}' => true,
        '\u{000B}' => true,
        '\u{000C}' => true,
        '\u{000D}' => true,
        '\u{0085}' => true,
        '\u{2028}' => true,
        '\u{2029}' => true,
        _ => false,
    }
}

pub fn char_is_word(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

pub fn char_is_punctuation(ch: char) -> bool {
    use unicode_general_category::{GeneralCategory, get_general_category};

    matches!(
        get_general_category(ch),
        GeneralCategory::OtherPunctuation
            | GeneralCategory::OpenPunctuation
            | GeneralCategory::ClosePunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::FinalPunctuation
            | GeneralCategory::ConnectorPunctuation
            | GeneralCategory::DashPunctuation
            | GeneralCategory::MathSymbol
            | GeneralCategory::CurrencySymbol
            | GeneralCategory::ModifierSymbol
    )
}

pub fn categorize_char(ch: char) -> CharCategory {
    if char_is_line_ending(ch) {
        CharCategory::Eol
    } else if ch.is_whitespace() {
        CharCategory::Whitespace
    } else if char_is_word(ch) {
        CharCategory::Word
    } else if char_is_punctuation(ch) {
        CharCategory::Punctuation
    } else {
        CharCategory::Unknown
    }
}
