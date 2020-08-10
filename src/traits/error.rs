pub trait Error<'a> {
    fn source(&self) -> &'a str;
    fn code(&self) -> u64;
    fn module(&self) -> &'a str;

    fn new(source: &'a str, code: u64, module: &'a str) -> Self;
}
