pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '@'
}

pub fn is_punctuation(c: char) -> bool {
    !c.is_whitespace() && !is_word_char(c)
}
