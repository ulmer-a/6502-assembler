pub trait ErrorMessage {
    fn error_msg(&self) -> String;
}

pub struct CompileError<T: ErrorMessage> {
    error_type: T,
    line: u32,
}

impl<T: ErrorMessage> CompileError<T> {
    pub fn print(&self) {
        println!("parse error: line {}: {}", self.line, self.error_type.error_msg());
    }

    pub fn new(error_type: T, line: u32) -> CompileError<T> {
        CompileError { error_type, line }
    }
}
