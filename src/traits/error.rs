use std::fmt::Debug;

pub trait Error: Debug {
    fn source(&self) -> String;
    fn code(&self) -> u64;
    fn module(&self) -> String;

    fn new(source: String, code: u64, module: String) -> Self;
}
