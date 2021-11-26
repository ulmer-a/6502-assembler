use super::AsmToken;
use crate::errors::ErrorMessage;

pub enum AsmParseError {
    UnexpectedToken(AsmToken),
    ImmediateTooLarge,
    AddressTooLarge,
    InvalidIndexRegister(String),
    ExcessTokens(usize),
}

impl ErrorMessage for AsmParseError {
    fn error_msg(&self) -> String {
        match self {
            AsmParseError::UnexpectedToken(token) => {
                if *token == AsmToken::Error {
                    "unrecognized token".into()
                } else {
                    format!("unexpected token: {}", token)
                }
            }
            AsmParseError::ImmediateTooLarge => "immediate value does not fit into 8 bits".into(),
            AsmParseError::AddressTooLarge => "address does not fit into 8 or 16 bits".into(),
            AsmParseError::InvalidIndexRegister(s) => {
                format!("unknown index register '{}', use X or Y", s)
            }
            AsmParseError::ExcessTokens(c) => format!("{} excess tokens after construct", c),
        }
    }
}
