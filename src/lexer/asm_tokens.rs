use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum AsmToken {
    #[token("brk")]
    #[token("jsr")]
    #[token("rts")]
    #[token("rti")]
    Mnemonic,

    #[token("(")]
    ParensOpen,
    #[token(")")]
    ParensClose,
    #[token(",")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("#")]
    ImmediateModifier,

    #[regex(r"(\$|0x)[0-9A_Fa-f]+")]
    HexInteger,
    #[regex(r"[1-9][0-9]*")]
    DecInteger,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,

    #[token("\n")]
    Newline,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}
