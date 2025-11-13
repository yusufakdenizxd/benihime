pub fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'@' || b >= 192
}

pub fn is_punctuation(b: u8) -> bool {
    !b.is_ascii_whitespace() && !is_word_char(b)
}
