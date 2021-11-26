use super::AsmToken;
use crate::errors::ErrorMessage;

pub enum AsmParseError {
    UnexpectedToken(AsmToken),
    ImmediateTooLarge,
    AddressTooLarge,
    InvalidIndexRegister,
}

impl ErrorMessage for AsmParseError {
    fn error_msg(&self) -> String {
        match self {
            AsmParseError::UnexpectedToken(_) => "unexpected token",
            AsmParseError::ImmediateTooLarge => "immediate value does not fit into 8 bits",
            AsmParseError::AddressTooLarge => "address does not fit into 8 or 16 bits",
            AsmParseError::InvalidIndexRegister => "unknown index register, use X or Y",
        }
        .into()
    }
}
