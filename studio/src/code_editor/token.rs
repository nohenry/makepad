#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Token {
    pub len: usize,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenKind {
    Comment,
    Identifier,
    Keyword(Keyword),
    Number,
    Punctuator(Punctuator),
    String,
    Whitespace,
    Unknown,
}

impl TokenKind {
    pub fn is_open_delimiter(&self) -> bool {
        match self {
            TokenKind::Punctuator(Punctuator::OpenDelimiter(_)) => true,
            _ => false
        }
    }

    pub fn is_close_delimiter(&self) -> bool {
        match self {
            TokenKind::Punctuator(Punctuator::CloseDelimiter(_)) => true,
            _ => false
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Keyword {
    Branch,
    Loop,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Punctuator {
    OpenDelimiter(Delimiter),
    CloseDelimiter(Delimiter),
    Other,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Delimiter {
    Paren,
    Bracket,
    Brace,
}