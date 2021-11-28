use logos::Logos;
use std::fmt;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum AsmToken {
    #[token("(")]
    ParensOpen,

    #[token(")")]
    ParensClose,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("#")]
    ImmediateModifier,

    #[token("=")]
    AssignmentOperator,

    #[regex(r"(\$|0x)[0-9A_Fa-f]+")]
    HexInteger,

    #[regex(r"[1-9][0-9]*")]
    DecInteger,

    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier,

    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[token("section")]
    SectionKeyword,
    
    #[token(".str")]
    StrKeyword,

    #[token(".word")]
    WordKeyword,

    #[token(".byte")]
    ByteKeyword,

    #[token("\n")]
    Newline,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,

    End,
}

impl fmt::Display for AsmToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
